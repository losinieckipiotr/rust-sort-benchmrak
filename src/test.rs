use crate::sort::Sort;
use crate::utils;

pub struct Options {
  pub iterations: i32,
  pub max_number: i32,
  pub seed: u64,
}

pub fn run<S>(options: &Options, sorter: S) where S: Sort {
  average_test(
    options.iterations,
    options.max_number,
    options.seed,
    &sorter
  );
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
    my_vec = utils::test_vector_factory(max_number as usize);
  });

  utils::measure_time_mili("shuffling vec", verbose, || {
    utils::shuffle_vec(&mut my_vec, seed);
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

pub fn dyn_average_test(
  iterations: i32,
  max_number: i32,
  seed: u64,
  name: &str,
  sorter: Box<dyn Fn(&mut Vec<i32>) -> ()>,
) {
  let mut results:Vec<f64> = Vec::new();

  for i in 0..iterations {
    let result = dyn_test(max_number, seed + i as u64, false, name, &sorter);

    results.push(result);

    let last = i == iterations - 1;

    if last {
      let sum: f64 = results.iter().sum();
      let avg = sum / (iterations as f64);

      println!("Sort: {} - avg: {} ms, runs: {}", name, avg, iterations);
    }
  }
}

pub fn dyn_test(
  max_number: i32,
  seed: u64,
  verbose: bool,
  name: &str,
  sorter: &Box<dyn Fn(&mut Vec<i32>) -> ()>
) -> f64 {
  let mut my_vec = Vec::new();

  utils::measure_time_mili("constructing vec", verbose, || {
    my_vec = utils::test_vector_factory(max_number as usize);
  });

  utils::measure_time_mili("shuffling vec", verbose, || {
    utils::shuffle_vec(&mut my_vec, seed);
  });

  let result = utils::measure_time_mili(
    name,
    verbose,
    || {
      // use function from param
      sorter(&mut my_vec);
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
