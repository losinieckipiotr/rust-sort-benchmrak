use rand::{rngs::SmallRng, SeedableRng, seq::SliceRandom};

// use clap::Parser;

/// Simple program like linux grep
// #[d&erive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct CommandLineArguments {
//   pattern: String,
//   #[clap(parse(from_os_str))]
//   path: std::path::PathBuf,
// }

// fn get_file_content(path: std::path::PathBuf) -> String {
//   let result = std::fs::read_to_string(&path);
//   match result {
//     Ok(content) => { content }
//     Err(_) => { panic!("could not read file: {}", path.display().to_string()) }
//   }
// }

fn myVecFactory(size: usize) -> Vec<i32> {
  let mut vec: Vec<i32> = Vec::new();
  vec.reserve(size);

  for i in 0..size {
    let num = i32::try_from(i).expect("usize to i32 conversion");
    vec.push(num);
  }

  vec
}

fn printRandomNumbers(iterations: u32, max_num: u32, seed: u64) {
  use rand::distributions::{Distribution, Uniform};
  let mut rng = SmallRng::seed_from_u64(seed);
  // let mut rng = rand::thread_rng();

  let die = Uniform::from(1..max_num);

  for _ in 0..iterations {
    let number = die.sample(&mut rng);
    println!("random number: {}", number);
  }
}

fn printVec(vec: &Vec<i32>) {
  println!("-----------------");
  for it in vec.iter() {
    println!("Value in vec is {}", it);
  }
  println!("-----------------");
}

fn shuffleMyVec(vec: &mut Vec<i32>, seed: u64) -> &mut Vec<i32> {
  let mut rng = SmallRng::seed_from_u64(seed);
  vec.shuffle(&mut rng);

  vec
}

fn main() {
  // let iterations = 100;
  // let max_num = 10;
  let seed = 0;
  // printRandomNumbers(iterations, max_num, seed);

  let mut my_vec = myVecFactory(5);
  printVec(&my_vec);

  shuffleMyVec(&mut my_vec, seed);
  printVec(&my_vec);

  // let args = CommandLineArguments::parse();
  // let content = get_file_content(args.path);

  // let mut line_num: i32 = 0;
  // for line in content.lines() {
  //   if line.contains(&args.pattern) {
  //     println!("{}:\t{}", line_num, line);
  //   }
  //   line_num = line_num + 1;
  // }

  // let mut vec: Vec<i32> = Vec::new();
  // vec.reserve(100);

  // for i in 0..100 {
  //   vec.push(i);
  // }

  // for it in &vec {
  //   println!("Value in vec is {}", it);
  // }

  // println!("vec capacity is {}", vec.capacity());
  // println!("vec len is {}", vec.len());

  // match numbers.get(10) {
  //   Some(third) => println!("The third element is {}", third),
  //   None => println!("There is no third element."),
  // }
}
