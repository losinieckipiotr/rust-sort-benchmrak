use std::thread;
use clap::Parser;
use crate::sorts::*;

use test::{Options, run};

pub mod test;
pub mod utils;
pub mod sort;
pub mod sorts;
pub mod print;

// Simple program to benchmark sort algorithms
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CommandLineArguments {
  /// Size of array to sort
  #[clap(value_parser)]
  max_number: i32,

  /// Number of iterations
  #[clap(value_parser)]
  iterations: i32,

  /// Optional seed
  #[clap(short, long, value_parser)]
  seed: Option<u64>,
}

fn get_options() -> Options {
  let args = CommandLineArguments::parse();

  println!("{:?}", args);

  Options {
    iterations: args.iterations,
    max_number: args.max_number,
    seed: match args.seed {
      Some(s) => s,
      None => utils::get_seconds()
    }
  }
}

fn main() {
  let options = get_options();

  thread::scope(|s| {
    s.spawn(|| { run(&options, BubbleSort); });
    s.spawn(|| { run(&options, DefaultSort); });
    s.spawn(|| { run(&options, InsertionSort); });
    s.spawn(|| { run(&options, QuickSort) });
    s.spawn(|| { run(&options, QuickSort2); });
    s.spawn(|| { run(&options, QuickSort3); });
    s.spawn(|| { run(&options, SelectionSort) });

    // let mut runners: Vec<Runner<DefaultSort>> = Vec::new();
    // runners.push(Runner::new(&options, DefaultSort));
    // runners.push(Runner::new(&options, DefaultSort));
    // runners.push(Runner::new(&options, DefaultSort));

    // s.spawn(|| {
    //   test::dyn_average_test(
    //     iterations,
    //     max_number,
    //     seed,
    //     "quick",
    //     Box::new(|vec: &mut Vec<i32>| { quick::quick_sort(vec); })
    //   );
    // });
  });

  // let vec: Vec<i32> = vec!(101, 102, 103);
  // let s: &str = "123";
  // let ptr: *const u8 = s.as_ptr();
  // let ptr_vec: *const i32 = vec.as_ptr();

  // unsafe {
  //   println!("{}", *ptr.offset(1) as char);
  //   println!("{}", *ptr.offset(2) as char);

  //   println!("{}", *ptr_vec);
  //   println!("{}", *ptr_vec.offset(1));
  //   println!("{}", *ptr_vec.offset(2));
  // }
}
