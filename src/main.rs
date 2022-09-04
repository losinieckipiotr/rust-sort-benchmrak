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

fn shuffle_my_vec(vec: &mut Vec<i32>, seed: u64) -> &mut Vec<i32> {
  let mut rng = SmallRng::seed_from_u64(seed);
  vec.shuffle(&mut rng);

  vec
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
  let max_index = vec.len() - 1;
  let mut temp;

  for _ in 0..max_index {
    for j in 0..max_index {
      if vec[j] > vec[j + 1] {
        temp = vec[j + 1];
        vec[j + 1] = vec[j];
        vec[j] = temp;
      }
    }
  }
}

fn bubble_sort_2(vec: &mut Vec<i32>) {
  let max_index = vec.len() - 1;
  let mut temp;
  let mut change;

  for _ in 0..max_index {
    change = false;
    for j in 0..max_index {
      if vec[j + 1] < vec[j] {
        temp = vec[j];
        vec[j] = vec[j + 1];
        vec[j + 1] = temp;
        change = true;
      }
    }
    if !change {
      break;
    }
  }
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

enum SortType {
  Bubble,
  Bubble2,
  Default,
}

// struct DefaultSort {
//   name: String,
//   vec_to_sort: Vec<i32>,
//   sort_time: u128,
//   done: bool,
//   }

// impl DefaultSort {
//   fn new (max_number: usize) -> Self {
//     DefaultSort {
//       name: String::from("default_sort"),
//       vec_to_sort: my_vec_factory(max_number),
//       sort_time: 0,
//       done: false,
//     }
//   }

//   fn run (&mut self, max_number: usize, seed: u64) {
//     shuffle_my_vec(&mut self.vec_to_sort, seed);
//     self.sort_time = measure_time_mili(Some(&self.name), || {
//       self.vec_to_sort.sort();
//     });
//     self.done = true;
//     let sorted = is_sorted(&self.vec_to_sort);
//     if sorted {
//       println!("Sorted !");
//     } else {
//       println!("Not sorted :(");
//     }
//     self.done = true;
//   }
// }

// struct BubbleSort {
//   name: String,
//   sort_time: u128,
//   done: bool,
// }

// impl BubbleSort {
//   fn new () -> Self {
//     BubbleSort {
//       name: String::from("bubble_sort"),
//       sort_time: 0,
//       done: false,
//     }
//   }

//   fn run (&mut self, max_number: usize, seed: u64) {
//     let mut my_vec = my_vec_factory(max_number);
//     shuffle_my_vec(&mut my_vec, seed);
//     self.sort_time = measure_time_mili(Some(&self.name), || {
//       bubble_sort(&mut my_vec);
//     });
// }

// struct TestOptions {
//   max_number: usize,
//   seed: u64,
//   sort_type: SortType,
// }

fn test (
  max_number: usize,
  seed: u64,
  sort_type: SortType,
  verbose: bool
) -> f64 {
  let mut my_vec: Vec<i32> = Vec::new();

  measure_time_mili("constructing vec", verbose, || {
    my_vec = my_vec_factory(max_number);
  });

  measure_time_mili("shuffling vec", verbose, || {
    shuffle_my_vec(&mut my_vec, seed);
  });

  let result = match sort_type {
    SortType::Bubble => measure_time_mili("bubble_sort", verbose, || {
       bubble_sort(&mut my_vec);
    }),
    SortType::Bubble2 => measure_time_mili("bubble_sort_2", verbose, || {
      bubble_sort_2(&mut my_vec);
    }),
    SortType::Default => measure_time_mili("default_sort", verbose, || {
      my_vec.sort();
    }),
  };

  let mut sorted = false;
  measure_time_mili("Checking", verbose, || {
    sorted = is_sorted(&my_vec)
  });

  if !sorted {
    panic!("Sorting error!");
  }

  result
}

fn average_test (iterations: u32, max_number: usize, seed: u64, sort_type: SortType) {
  let mut results:Vec<f64> = Vec::new();

  for i in 0..iterations {
    // println!("SortType::Bubble - iteration {}", i);


    let result = match sort_type {
      SortType::Bubble => test(max_number, seed + i as u64, SortType::Bubble, false),
      SortType::Bubble2 => test(max_number, seed + i as u64, SortType::Bubble2, false),
      SortType::Default => test(max_number, seed + i as u64, SortType::Default, false),
    };

    // println!("result: {} ms", result);

    results.push(result);

    let last = i == iterations - 1;

    if last {
      let sum: f64 = results.iter().sum();
      let avg = sum / (iterations as f64);

      match sort_type {
        SortType::Bubble => println!("SortType::Bubble Avg: {} ms", avg),
        SortType::Bubble2 => println!("SortType::Bubble2 Avg: {} ms", avg),
        SortType::Default => println!("SortType::Default Avg: {} ms", avg),
      }
    }
  }
}

fn main() {
  let args = CommandLineArguments::parse();
  let max_number = args.max_number;
  let iterations = args.iterations;
  let seed = get_seconds();
  // let seed = 0;

  average_test(iterations, max_number, seed, SortType::Default);
  average_test(iterations, max_number, seed, SortType::Bubble);
  average_test(iterations, max_number, seed, SortType::Bubble2);
}
