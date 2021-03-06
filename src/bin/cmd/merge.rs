extern crate exitcode;
extern crate regex;
extern crate serde_json;

use log::*;
use preqc_pack::fastqc;
use regex::Regex;
use std::path::Path;
use structopt::StructOpt;

/// A collection of metadata, such as file size, md5sum
#[derive(StructOpt, PartialEq, Debug)]
#[structopt(setting=structopt::clap::AppSettings::ColoredHelp, name="PreQC Tool Suite - Merge", author="Jingcheng Yang <yjcyxky@163.com>")]
pub struct Arguments {
  /// Bam file to process
  #[structopt(name = "FILE", multiple = true, takes_value = true)]
  inputs: Vec<String>,

  /// Output file.
  #[structopt(name = "output", short = "o", long = "output")]
  output: String,
}

fn is_fastq_file(filepath: &str) -> bool {
  // Import at the crate root - preqc-pack.rs
  lazy_static! {
    static ref RE: Regex = Regex::new(".*(.fq|.fastq)$").unwrap();
  }

  RE.is_match(filepath)
}

fn is_fastq_gz_file(filepath: &str) -> bool {
  // Import at the crate root - preqc-pack.rs
  lazy_static! {
    static ref RE: Regex = Regex::new(".*(.fq.gz|.fastq.gz)$").unwrap();
  }

  RE.is_match(filepath)
}

fn exists(files: &Vec<String>) -> bool {
  let items = files.into_iter().filter(|file| !Path::new(&file).exists());
  if items.count() > 0 {
    return false;
  } else {
    return true;
  }
}

pub fn run(args: &Arguments) {
  let outputs: Vec<String> = vec![args.output.clone()];
  if !exists(&outputs) {
    if exists(&args.inputs) {
      // TODO: Multi threads?
      println!("Merge all intputs {:?} to {}", args.inputs, args.output);
      for input in args.inputs.clone() {
        fastqc::zcat(&input, &args.output)
      }
    } else {
      error!("{} - Not Found: {:?}", module_path!(), args.inputs);
    }
  } else {
    error!("{} exists", args.output);
  }
}
