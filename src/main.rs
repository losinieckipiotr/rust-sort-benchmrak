use clap::Parser;
use std::time::SystemTime;
use rand::{rngs::SmallRng, SeedableRng, seq::SliceRandom};

// Simple program to benchmark sort algorithms
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CommandLineArguments {
  max_number: usize,
  iterations: u32,
}

fn get_seconds() -> u64 {
  SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .expect("duration from unix epoch")
    .as_secs()
}

fn my_vec_factory(size: usize) -> Vec<i32> {
  let mut vec: Vec<i32> = Vec::new();
  vec.reserve(size);

  for i in 0..size {
    let num = i32::try_from(i).expect("converted usize to i32");
    vec.push(num);
  }

  vec
}

// fn print_random_numbers(iterations: u32, max_num: u32, seed: u64) {
//   use rand::distributions::{Distribution, Uniform};
//   let mut rng = SmallRng::seed_from_u64(seed);
//   // let mut rng = rand::thread_rng();
//
//   let die = Uniform::from(1..max_num);
//
//   for _ in 0..iterations {
//     let number = die.sample(&mut rng);
//     println!("random number: {}", number);
//   }
// }

// fn print_vec(vec: &Vec<i32>) {
//   println!("-----------------");
//   for it in vec.iter() {
//     println!("Value in vec is {}", it);
//   }
//   println!("-----------------");
// }

// fn print_vec_first(vec: &Vec<i32>, first: usize) {
//   let mut it = vec.iter();
//   println!("-----------------");
//   for i in 0..first {
//     if let Some(value) = it.next() {
//       println!("Value at {} is {}", i, value);
//     }
//   }
//   println!("-----------------");
// }

// fn print_vec_last(vec: &Vec<i32>, last: usize) {
//   let len = vec.len();
//   let mut it = vec.iter().rev();
//   println!("-----------------");
//   for i in 0..last {
//     if let Some(value) = it.next() {
//       println!("Value at {} is {}", len - 1 - i, value);
//     }
//   }
//   println!("-----------------");
// }

fn shuffle_my_vec(vec: &mut Vec<i32>, seed: u64) {
  let mut rng = SmallRng::seed_from_u64(seed);
  vec.shuffle(&mut rng);
}

fn measure_time_mili<F>(start_msg: &str, verbose: bool, mut operation: F) -> f64
  where F: FnMut() {

  if verbose {
    println!("{}", start_msg);
  }

  let now = SystemTime::now();

  // let result = operation();
  operation();

  let operation_time = now
  .elapsed()
  .unwrap()
  .as_millis();

  if verbose {
    println!("operation took {} mili seconds", operation_time);
  }

  operation_time as f64
}

fn is_sorted(vec: &Vec<i32>) -> bool {
  vec.iter()
    .zip(vec.iter().skip(1))
    .all(|(a, b)| { a <= b })
}

fn bubble_sort(vec: &mut Vec<i32>) {
  let n = vec.len();
  let mut temp;
  let mut j;

  for i in 0..n {
    j = 0;
    while j < n - 1 - i {
      if vec[j] > vec[j + 1] {
        temp = vec[j + 1];
        vec[j + 1] = vec[j];
        vec[j] = temp;
      }
      j = j + 1;
    }
  }
}

fn selection_sort(vec: &mut Vec<i32>) {
  let n = vec.len();
  let mut i = 0;
  let mut j;
  let mut k;

  while i < n {
    k = i;
    j = i + 1;
    while j < n {
      if vec[j] < vec[k] {
        k = j
      };
      j = j + 1;
    }
    let temp = vec[k];
    vec[k] = vec[i];
    vec[i] = temp;
    i = i + 1;
  }
}

fn insertion_sort(vec: &mut Vec<i32>) {
  let mut key;
  let mut j;
  let len = vec.len();

  let mut i = 1;
  while i < len {
    key = vec[i];
    j = i;

    loop {
      if j > 0 && vec[j - 1] > key {
        vec[j] = vec[j - 1];
        j = j - 1;
      } else {
        break;
      }
    }
    vec[j] = key;
    i = i + 1;
  }
}

fn quick_sort(vec: &mut Vec<i32>) {
}

// fn compare_vectors(vec_a: &Vec<i32>, vec_b: &Vec<i32>) -> bool {
//   let len_a = vec_a.len();
//   let len_b = vec_b.len();
//   if len_a == len_b {
//     for i in 0..len_a - 1 {
//       if vec_a[i] != vec_b[i] {
//         return false
//       }
//     }
//     return true;
//   } else {
//     return false;
//   }
// }

trait Sort {
  fn name(&self) -> &str;
  fn sort(&self, vec: &mut Vec<i32>);
}

struct DefaultSort {}

impl Sort for DefaultSort {
  fn name(&self) -> &str {
    "Default"
  }

  fn sort(&self, vec: &mut Vec<i32>) {
    vec.sort();
  }
}

struct BubbleSort {}

impl Sort for BubbleSort {
  fn name(&self) -> &str {
    "bubble"
  }

  fn sort(&self, vec: &mut Vec<i32>) {
    bubble_sort(vec);
  }
}

struct SelectionSort {}

impl Sort for SelectionSort {
  fn name(&self) -> &str {
    "selection"
  }

  fn sort(&self, vec: &mut Vec<i32>) {
    selection_sort(vec);
  }
}

struct InsertionSort {}

impl Sort for InsertionSort {
  fn name(&self) -> &str {
    "insertion"
  }

  fn sort(&self, vec: &mut Vec<i32>) {
    insertion_sort(vec);
  }
}

struct QuickSort {}

impl Sort for QuickSort {
  fn name(&self) -> &str {
    "quick"
  }

  fn sort(&self, vec: &mut Vec<i32>) {
    quick_sort(vec);
  }
}

fn test (
  max_number: usize,
  seed: u64,
  verbose: bool,
  sorter: &impl Sort
) -> f64 {
  let mut my_vec = Vec::new();

  measure_time_mili("constructing vec", verbose, || {
    my_vec = my_vec_factory(max_number);
  });

  measure_time_mili("shuffling vec", verbose, || {
    shuffle_my_vec(&mut my_vec, seed);
  });

   let result = measure_time_mili(
    // Polymorphism
    sorter.name(),
    verbose,
    || {
      // Polymorphism
      sorter.sort(&mut my_vec);
    }
  );

  let mut sorted = false;
  measure_time_mili("Checking", verbose, || {
    sorted = is_sorted(&my_vec);
  });

  if !sorted {
    panic!("vector is not sorted");
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
  let seed = get_seconds();
  // let seed = 0;

  average_test(iterations, max_number, seed, &QuickSort {});
  average_test(iterations, max_number, seed, &DefaultSort {});
  average_test(iterations, max_number, seed, &InsertionSort {});
  average_test(iterations, max_number, seed, &SelectionSort {});
  average_test(iterations, max_number, seed, &BubbleSort {});
}
