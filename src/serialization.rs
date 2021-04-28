extern crate serde;
use serde::{Serialize, Deserialize};

#[path="categories.rs"] mod categories;
#[path="errors.rs"] mod errors;

use crate::categories::*;
use std::result::Result;
use std::io::*;


pub fn get_scores<'a>(reader: &mut BufReader<std::fs::File>) -> Result<Vec<Category>, crate::errors::Error> {
    let mut s = String::new();
    reader.read_to_string(&mut s).ok().expect(("failed parse"));
    return match serde_yaml::from_str(&(s.as_str())) {
        Ok(x) => Ok(x),
        Err(_) => Err(crate::errors::Error::Parse_fail)
    };
}

#[test]
fn test_serialize() {
    let raw = r#"
        -
            name: WA
            weight: 30
            scores:
                - Points:
                    - 30
                    - 30
                - Points:
                    - 30
                    - 40
        -
            name: WA
            weight: 30
            scores:
                - Points:
                    - 30
                    - 30
                - Points:
                    - 30
                    - 40
            
    "#;
    let score = vec![Category{
        name: String::from("WA"),
        weight: 30 as f64,
        scores: vec![ Score::Points(30 as f64, 30 as f64), Score::Points(20 as f64, 30 as f64)]
    }];
    match serde_yaml::to_string::<Vec<Category>>(&score) {
        Ok(x) => {println!("{:?}", x)}
        Err(x) => {panic!("{:?}", x)}
    };
    match serde_yaml::from_str::<Vec<Category>>(&raw) {
        Ok(x) => {println!("{:?}", x)}
        Err(x) => {panic!("{:?}", x)}
    };
}