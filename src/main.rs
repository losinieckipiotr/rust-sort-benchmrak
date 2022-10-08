use std::thread;
use std::sync::mpsc;
use rand::seq::SliceRandom;
use std::time::SystemTime;

use clap::Parser;

pub mod test;
pub mod utils;
pub mod sorts;
pub mod print;

use crate::sorts::*;
use test::{Options, MyTest};

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

  /// Number of threads
  #[clap(short, long, default_value_t = 1, value_parser)]
  threads: i32,

  /// Shuffle jobs
  #[clap(short, long, value_parser)]
  no_shuffle: bool,
}

fn get_options() -> Options {
  let args = CommandLineArguments::parse();

  Options {
    iterations: args.iterations,
    max_number: args.max_number,
    seed: match args.seed {
      Some(s) => s,
      None => utils::get_seconds()
    }
  }
}

#[allow(dead_code)]

#[derive(Debug, Copy, Clone)]
struct SortResult<'a> {
  id: i32,
  name: &'a str,
  seed: u64,
  result: f64,
}

fn main() {
  let options = get_options();
  println!("{:?}", options);

  let args = CommandLineArguments::parse();
  let threads = args.threads;
  let no_shuffle = args.no_shuffle;

  let now = SystemTime::now();
  thread::scope(|scope| {
    let new_test = |name, sorter| {
      MyTest::new(name, sorter, options.max_number, 0)
    };

    let original_tests = vec![
      new_test("bubble", bubble_sort),
      new_test("insertion", insertion_sort),
      new_test("quick", quick_sort),
      new_test("quick2", quick_sort_2),
      new_test("quick3", quick_sort_3),
      new_test("default", default_sort),
      new_test("default_unstable", default_unstable_sort),
      new_test("merge", merge_sort),
    ];

    let mut tests = original_tests.clone();

    for _ in 1..options.iterations {
      let mut t = original_tests.clone();
      tests.append(&mut t);
    }

    for (i, test) in tests.iter_mut().enumerate() {
      test.seed = options.seed + i as u64;
    }

    tests.reverse();

    if !no_shuffle {
      let mut rng = rand::thread_rng();
      tests.shuffle(&mut rng);
    }

    let (tx, rx) = mpsc::channel();
    let mut worker_id = -1;

    let mut do_jobs = |th_num| {
      for _ in 0..th_num {
        let test = tests.pop();
        worker_id += 1;
        let t = match test {
          Some(t) => t,
          None => break
        };
        let my_tx = mpsc::Sender::clone(&tx);
        scope.spawn(move || {
          let result = t.run_single();
          let r = SortResult {
            id: worker_id,
            name: t.name.clone(),
            seed: t.seed,
            result: result
          };

          my_tx.send(r).unwrap();
        });
      }

      return tests.is_empty();
    };

    // start workers, all threads
    let mut is_empty = do_jobs(threads);
    while !is_empty {
      // receive one result
      let received = rx.recv().unwrap();
      // start one thread
      is_empty = do_jobs(1);
      println!("{:?}", received);
    }
    drop(tx);

    // collect last results
    for received in rx.iter() {
      println!("{:?}", received);
    }
  });
  let operation_time = now
  .elapsed()
  .unwrap()
  .as_secs();

  println!("operation took {} seconds", operation_time);
}
