/*
Possible Errors:

Failing to Parse,

Failing to generate Average,


*/

pub enum Error {
    Parse_fail,
    AverageFail(&'static str),
    No_file,
}

pub fn handler(e: Error) {
    match e {
        Error::Parse_fail => {
            eprintln!("Failed to parse the YAML file. Please change the syntax.");
        }
        Error::AverageFail(s) => {
            eprintln!("Failed to calculate the average. {}", s);
        }
        Error::No_file => {
            eprintln!("File not found. You must add a file to the arguments.")
        }
    };
}