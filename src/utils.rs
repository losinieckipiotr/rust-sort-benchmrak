use std::time::SystemTime;
use rand::{rngs::SmallRng, SeedableRng, seq::SliceRandom};

pub fn get_seconds() -> u64 {
  std::time::SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .expect("duration from unix epoch")
    .as_secs()
}

// TODO rename
pub fn my_vec_factory(size: usize) -> Vec<i32> {
  let mut vec: Vec<i32> = Vec::new();
  vec.reserve(size);

  for i in 0..size {
    let num = i32::try_from(i).expect("converted usize to i32");
    vec.push(num);
  }

  vec
}

// TODO rename
pub fn shuffle_my_vec(vec: &mut Vec<i32>, seed: u64) {
  let mut rng = SmallRng::seed_from_u64(seed);
  vec.shuffle(&mut rng);
}

pub fn measure_time_mili<F>(start_msg: &str, verbose: bool, mut operation: F) -> f64
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

pub fn is_sorted(vec: &Vec<i32>) -> bool {
  vec.iter()
    .zip(vec.iter().skip(1))
    .all(|(a, b)| { a <= b })
}

pub fn swap(i: usize, j: usize, vec: &mut Vec<i32>) {
  let temp = vec[i];
  vec[i] = vec[j];
  vec[j] = temp;
}

pub fn compare_vectors(vec_a: &Vec<i32>, vec_b: &Vec<i32>) -> bool {
  let len_a = vec_a.len();
  let len_b = vec_b.len();
  if len_a == len_b {
    for i in 0..len_a - 1 {
      if vec_a[i] != vec_b[i] {
        return false
      }
    }
    return true;
  } else {
    return false;
  }
}

