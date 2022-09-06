use crate::sort::Sort;
use crate::utils;

pub struct TestOptions {
  pub iterations: i32,
  pub max_number: i32,
  pub seed: u64,
}

impl TestOptions {
  pub fn run(&self, sorter: impl Sort) {
    average_test(
      self.iterations,
      self.max_number,
      self.seed,
      &sorter
    );
  }
}

fn average_test(
  iterations: i32,
  max_number: i32,
  seed: u64,
  sorter: &impl Sort,
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

fn test(
  max_number: i32,
  seed: u64,
  verbose: bool,
  sorter: &impl Sort,
) -> f64 {
  let mut my_vec = Vec::new();

  utils::measure_time_mili("constructing vec", verbose, || {
    my_vec = utils::my_vec_factory(max_number as usize);
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
