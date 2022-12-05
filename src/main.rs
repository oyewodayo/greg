use std::env;
use std::fs;
fn main() {
    let args:Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    let contents = fs::read_to_string(file_path).expect("SHould have been able to the file ffrom thte file path");

    println!("Searching for {}",query);
    println!("In the file {}", file_path);  
    println!("The content: {}",contents);
}

fn parse_config(args:&[String]) -> (&str, &str) {
    let query:&String = &args[1];
    let file_path:&String = &args[2];

    (query,file_path)
}
