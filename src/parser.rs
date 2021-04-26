use crate::categories;
use std::io::BufReader;

#[path = "read_file.rs"] mod read_file;


macro_rules! parse_or_fail {
    ($x:expr) => {
        match $x.parse::<f64>() {
            Ok(o) => o,
            Err(_e) =>  {
                println!("Error! Unable to parse {} into a floating-point", $x);
                return Err(());
            }
        }
    };
}


pub fn read_section() -> Result<Vec<categories::Category>, ()>  {
    let mut res : Vec<categories::Category> = vec![];
    let mut reader: Option<BufReader<std::fs::File>> = match read_file::get_reader() {
        Some(x) => Some(x),
        None => None
    };
    let mut line_number: usize = 0;
    loop {
        line_number += 1;
        let line = read_file::get_one_line(&mut reader);
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
                    println!("Error on line {}! Does not have the appropriate number of arguments.", line_number);
                    println!("line {}: {:?}", line_number, &line);
                    println!("Format should be cat (name) (weight of grade, just a floating point number)");
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
                    println!("Error on line {}! Unclear which category the following score belongs to", line_number);
                    println!("line {}: {:?}", line_number, &line);
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
                    println!("Error on line {}! score does not have enough arguments.", line_number);
                    println!("line {}: {:?}", line_number, &line);
                    return Err(());
                }
            }
            "end" | "e" => {
                return Ok(res);
            }
            u => {
                println!("Error on line {}! {} is an undefined command", line_number, u);
                println!("line {}: {:?}", line_number, &line);
                return Err(());
            }
        }
    }
}
