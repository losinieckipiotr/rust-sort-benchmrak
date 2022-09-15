use crate::utils;

#[derive(Debug, Copy, Clone)]
pub struct Options {
  pub iterations: i32,
  pub max_number: i32,
  pub seed: u64,
}

#[derive(Copy, Clone)]
pub struct MyTest<'a> {
  pub name: &'a str,
  pub sorter: fn(&mut Vec<i32>) -> (),
  pub max_number: i32,
  pub seed: u64,
}

impl<'a> MyTest<'a> {
  pub fn new(
    name: &'a str,
    sorter: fn(&mut Vec<i32>) -> (),
    max_number: i32,
    seed: u64
  ) -> Self {
    MyTest {
      name,
      sorter,
      max_number,
      seed
    }
  }

  pub fn run_single(&self) -> f64 {
    test_fn_pointer(false, self)
  }
}

//   println!("Sort: {}, results num {}", test.name, results.len());
//   let sum: f64 = results.iter().sum();
//   let avg = sum / (iterations as f64);
//   println!("Sort: {} - avg: {} ms, runs: {}", test.name, avg, iterations);
// }

pub fn test_fn_pointer(verbose: bool, test: &MyTest) -> f64 {
  let MyTest {
    name,
    sorter,
    max_number,
    seed
  } = *test;
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
