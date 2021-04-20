use std::io;
use crate::categories;

fn get_one_line() -> String{
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read line");
    return line;
}

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
    loop {
        let v: Vec<String> = get_one_line()
                            .split_whitespace()
                            .map(|x| String::from(x))
                            .collect();
        if v.len() == 0 {continue;}
        let cmd = (&v[0]).as_str();
        match cmd {
            "cat" | "category" | "c" => {
                if v.len() != 3 {
                    println!("Error! the following lines do not have the appropriate number of arguments.");
                    println!("{:?}", &v);
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
                if res.len() == 0 {
                    println!("Error! Unclear which category the following score belongs to");
                    println!("{:?}", &v);
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
                    println!("Error! score does not have enough arguments.");
                    return Err(());
                }
            }
            "end" | "e" => {
                return Ok(res);
            }
            u => {
                println!("Error! {} is an undefined command", u);
                return Err(());
            }
        }
    }
}