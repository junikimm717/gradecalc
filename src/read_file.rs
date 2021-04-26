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


pub fn get_reader() -> Option<io::BufReader<File>> {
    let path = match file_name() {
        Some(x) => x,
        None => {return None;}
    };
    let reader = io::BufReader::new(File::open(path).ok()?);
    // read from a file and return one string.
    return Some(reader);
}
