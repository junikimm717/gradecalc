use crate::categories;
use std::io::{Read};

#[path = "read_file.rs"] mod read_file;


macro_rules! parse_or_fail {
    ($x:expr) => {
        match $x.parse::<f64>() {
            Ok(o) => o,
            Err(_e) =>  {
                eprintln!("Error! Unable to parse {} into a floating-point", $x);
                return Err(());
            }
        }
    };
}


/*
reads in a file and returns a vector of categories for calculation.
 */
pub fn read_section() -> Result<Vec<categories::Category>, ()>  {
    let mut res : Vec<categories::Category> = vec![];
    let mut reader = match read_file::get_reader() {
        Some(x) => x,
        None => {
            eprintln!("No File Given! Please provide a file name to gradecalc.");
            return Err(());
        }
    };
    let mut lines: Vec<String>;
    {
        let mut s: String = String::new();
        reader.read_to_string(&mut s).ok().expect("failed parse");
        lines = s.split('\n')
                    .map(|x| x.to_string())
                    .rev()
                    .collect();
    }
    let mut line_number: usize = 0;
    loop {
        line_number += 1;
        let line = match lines.pop() {
            Some(x) => x,
            None => {return Ok(res);}
        };
        let v: Vec<String> = line
                            .split_whitespace()
                            .map(|x| String::from(x))
                            .collect();
        if v.len() == 0 {
            continue;
        }
        if line.as_str().chars().next().unwrap() == '#' {
            // processes comments
            continue;
        }
        let cmd = (&v[0]).as_str();
        match cmd {
            "cat" | "category" | "c" => {
                // defining a new category
                if v.len() != 3 {
                    eprintln!("Error on line {}! Does not have the appropriate number of arguments.", line_number);
                    eprintln!("line {}: {:?}", line_number, &line);
                    eprintln!("Format should be cat (name) (weight of grade, just a floating point number)");
                    return Err(())
                }
                res.push(categories::Category{
                    name: v[1].clone(),
                    scores: vec![],
                    weight: parse_or_fail!(v[2])
                })
            }
            "score" | "s" | "sc" => {
                // a certain score.
                if res.len() == 0 {
                    eprintln!("Error on line {}! Unclear which category the following score belongs to", line_number);
                    eprintln!("line {}: {:?}", line_number, &line);
                    return Err(());
                }
                let ind = res.len()-1;
                if v.len() == 2 {
                    res[ind].scores.push(
                        categories::Score::Percentage(parse_or_fail!(v[1]))
                    )
                } else if v.len() == 3 {
                    res[ind].scores.push(
                        categories::Score::Points(parse_or_fail!(v[1]), parse_or_fail!(v[2]))
                    )
                } else {
                    eprintln!("Error on line {}! score does not have enough arguments.", line_number);
                    eprintln!("line {}: {:?}", line_number, &line);
                    return Err(());
                }
            }
            u => {
                eprintln!("Error on line {}! {} is an undefined command", line_number, u);
                eprintln!("line {}: {:?}", line_number, &line);
                return Err(());
            }
        }
    }
}
