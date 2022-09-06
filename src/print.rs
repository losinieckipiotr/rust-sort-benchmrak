use rand::{rngs::SmallRng, SeedableRng};

pub fn print_random_numbers(iterations: u32, max_num: u32, seed: u64) {
  use rand::distributions::{Distribution, Uniform};
  let mut rng = SmallRng::seed_from_u64(seed);
  // let mut rng = rand::thread_rng();

  let die = Uniform::from(1..max_num);

  for _ in 0..iterations {
    let number = die.sample(&mut rng);
    println!("random number: {}", number);
  }
}

pub fn print_vec(vec: &Vec<i32>) {
  println!("-----------------");
  for it in vec.iter() {
    println!("Value in vec is {}", it);
  }
  println!("-----------------");
}

pub fn print_vec_first(vec: &Vec<i32>, first: usize) {
  let mut it = vec.iter();
  println!("-----------------");
  for i in 0..first {
    if let Some(value) = it.next() {
      println!("Value at {} is {}", i, value);
    }
  }
  println!("-----------------");
}

pub fn print_vec_last(vec: &Vec<i32>, last: usize) {
  let len = vec.len();
  let mut it = vec.iter().rev();
  println!("-----------------");
  for i in 0..last {
    if let Some(value) = it.next() {
      println!("Value at {} is {}", len - 1 - i, value);
    }
  }
  println!("-----------------");
}
