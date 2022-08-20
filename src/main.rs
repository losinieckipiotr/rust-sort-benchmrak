use clap::Parser;

#[derive(Parser)]
struct CommandLineArguments {
  pattern: String,
  #[clap(parse(from_os_str))]
  path: std::path::PathBuf,
}

/// Simple program to greet a person
// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//    /// Name of the person to greet
//    #[clap(short, long, value_parser)]
//    name: String,

//    /// Number of times to greet
//    #[clap(short, long, value_parser, default_value_t = 1)]
//    count: u8,
// }

fn main() {
  // let args = Args::parse();
  //  for _ in 0..args.count {
  //      println!("Hello {}!", args.name)
  //  }

  // let pattern: String = std::env::args().nth(1).expect("no pattern given");
  // let path: String = std::env::args().nth(2).expect("no path given");

  // let args: CommandLineArguments = CommandLineArguments {
  //   pattern: pattern,
  //   path: std::path::PathBuf::from(path),
  // };

  // println!("pattern: {}", &args.pattern);
  // println!("path: {}", args.path.to_str())

  // println!("Hello, world!");

  let args = CommandLineArguments::parse();

  println!("pattern: {}", &args.pattern);
  println!("path: {}", &args.path.display().to_string());
}
