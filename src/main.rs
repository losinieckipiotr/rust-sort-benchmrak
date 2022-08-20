use clap::Parser;

/// Simple program like linux grep
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CommandLineArguments {
  pattern: String,
  #[clap(parse(from_os_str))]
  path: std::path::PathBuf,
}

fn get_file_content(path: std::path::PathBuf) -> String {
  let result = std::fs::read_to_string(&path);
  match result {
    Ok(content) => { content }
    Err(_) => { panic!("could not read file: {}", path.display().to_string()) }
  }
}

fn main() {


  let args = CommandLineArguments::parse();
  let content = get_file_content(args.path);

  let mut line_num: i32 = 0;
  for line in content.lines() {
    if line.contains(&args.pattern) {
      println!("{}:\t{}", line_num, line);
    }
    line_num = line_num + 1;
  }
}
