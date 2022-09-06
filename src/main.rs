use clap::Parser;
use crate::sort::Sort;
use crate::sorts::*;

pub mod utils;
pub mod sort;
pub mod sorts;

// Simple program to benchmark sort algorithms
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CommandLineArguments {
  max_number: usize,
  iterations: u32,
}

fn test (
  max_number: usize,
  seed: u64,
  verbose: bool,
  sorter: &impl Sort
) -> f64 {
  let mut my_vec = Vec::new();

  utils::measure_time_mili("constructing vec", verbose, || {
    my_vec = utils::my_vec_factory(max_number);
  });

  utils::measure_time_mili("shuffling vec", verbose, || {
    utils::shuffle_my_vec(&mut my_vec, seed);
  });

   let result = utils::measure_time_mili(
    // Polymorphism
    sorter.name(),
    verbose,
    || {
      // Polymorphism
      sorter.sort(&mut my_vec);
    }
  );

  let mut sorted = false;
  utils::measure_time_mili("Checking", verbose, || {
    sorted = utils::is_sorted(&my_vec);
  });

  if !sorted {
    panic!("vector is not sorted {:?}", &my_vec);
  }

  result
}

fn average_test (
  iterations: u32,
  max_number: usize,
  seed: u64,
  sorter: &impl Sort
) {
  let mut results:Vec<f64> = Vec::new();

  for i in 0..iterations {
    // Polymorphism
    let result = test(max_number, seed + i as u64, false, sorter);

    // println!("result: {} ms", result);

    results.push(result);

    let last = i == iterations - 1;

    if last {
      let sum: f64 = results.iter().sum();
      let avg = sum / (iterations as f64);

      // Polymorphism
      println!("Sort: {} - avg: {} ms, runs: {}", sorter.name(), avg, iterations);
    }
  }
}

fn main() {
  let args = CommandLineArguments::parse();
  let max_number = args.max_number;
  let iterations = args.iterations;
  let seed = utils::get_seconds();
  // let seed = 0;

  average_test(iterations, max_number, seed, &DefaultSort {});
  average_test(iterations, max_number, seed, &QuickSort {});
  average_test(iterations, max_number, seed, &QuickSort2 {});
  average_test(iterations, max_number, seed, &QuickSort3 {});
  average_test(iterations, max_number, seed, &InsertionSort {});
  average_test(iterations, max_number, seed, &SelectionSort {});
  average_test(iterations, max_number, seed, &BubbleSort {});
}
