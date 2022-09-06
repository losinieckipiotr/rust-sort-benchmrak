use clap::Parser;
use crate::sorts::*;

use test::TestOptions;

pub mod test;
pub mod utils;
pub mod sort;
pub mod sorts;
pub mod print;

// Simple program to benchmark sort algorithms
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CommandLineArguments {
  max_number: i32,
  iterations: i32,
}

fn main() {
  let args = CommandLineArguments::parse();
  let max_number = args.max_number;
  let iterations = args.iterations;
  // let seed = utils::get_seconds();
  let seed = 0; // TODO add as cmd argument

  let options = TestOptions {
    iterations,
    max_number,
    seed
  };

  options.run(DefaultSort {});
  options.run(QuickSort {});
  options.run(QuickSort2 {});
  options.run(QuickSort3 {});
  options.run(InsertionSort {});
  options.run(SelectionSort {});
  options.run(BubbleSort {});
}
