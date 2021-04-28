mod categories;
mod serialization;
mod errors;

use std::fs::File;
use std::env;
use std::io;
use std::result::Result;

fn file_name() -> Option<String> {
    let args: Vec<String> =env::args().collect();
    if args.len() >= 2 {
        return Some((&args[1]).to_string());
    }
    return None;
}


// returns a BufReader that can read a file.
pub fn get_reader() -> Result<io::BufReader<File>, errors::Error> {
    let path = match file_name() {
        Some(x) => x,
        None => {return Err(errors::Error::No_file);}
    };
    let reader = io::BufReader::new(match File::open(path).ok() { Some(x) => x, 
        None => {return Err(errors::Error::No_file)} });
    return Ok(reader);
}

fn main() {
    let mut reader = match get_reader() {
        Ok(x) => x,
        Err(x) => {
            errors::handler(x);
            return;
        }
    };
    let v = match serialization::get_scores(&mut reader) {
        Ok(x) => x,
        Err(x) => {
            errors::handler(x);
            return;
        }
    };
    match categories::average(v) {
        Ok(x) => {println!("The average is {}%", x*(100 as f64))},
        Err(x) => {
            errors::handler(x);
            return;
        }
    }
}

