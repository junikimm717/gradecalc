use std::fs;
use std::env;
use std::io;
use std::fs::File;

fn file_name() -> Option<String> {
    let args: Vec<String> =env::args().collect();
    if args.len() >= 2 {
        return Some((&args[1]).to_string());
    }
    return None;
}

fn line_from_stdin() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read line");
    return line;
}

/*
fn line_from_fs() -> io::BufReader {
    let path = match file_name() {
        Some(x) => x,
        None => {return None;}
    };
    let mut reader = io::BufReader::new(File::open(path)?);
    // read from a file and return one string.
}
*/

pub fn get_one_line() -> String{
    return line_from_stdin();
}