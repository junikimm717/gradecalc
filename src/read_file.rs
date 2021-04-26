use std::env;
use std::io;
use std::fs::File;
use std::io::{BufRead};

fn file_name() -> Option<String> {
    let args: Vec<String> =env::args().collect();
    if args.len() >= 2 {
        return Some((&args[1]).to_string());
    }
    return None;
}

pub fn get_one_line(reader: &mut Option<io::BufReader<File>>) -> String {
    return match reader {
        Some(r) => {
            let mut s: String = String::new();
            r.read_line(&mut s).ok().expect("failed parse of file");
            s
        }
        None => {line_from_stdin()}
    }
}

fn line_from_stdin() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read line");
    return line;
}

pub fn get_reader() -> Option<io::BufReader<File>> {
    let path = match file_name() {
        Some(x) => x,
        None => {return None;}
    };
    let reader = io::BufReader::new(File::open(path).ok()?);
    // read from a file and return one string.
    return Some(reader);
}
