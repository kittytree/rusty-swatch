use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (file_path, num_colours) = parse_args(&args);

    println!("path of image is {}", file_path);
    println!("Number of colours: {}", num_colours);
}

fn parse_args(args: &[String]) -> (&str, usize) {
    let filename = &args[1];
    let num_colours = args[2].parse::<usize>().unwrap();
    (filename, num_colours)
}