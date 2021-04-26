mod categories;
mod parser;

fn main() {
    match parser::read_section() {
        Ok(x) => {
            println!("{}%", match categories::average(x) {
                Ok(a) => a*(100 as f64),
                Err(_) => {return;}
            });
        }
        Err(_) => {
            eprintln!("parse failed. please fix bugs above.");
        }
    }
}

