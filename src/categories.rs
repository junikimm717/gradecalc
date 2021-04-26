#[derive(Debug)]
pub enum Score {
    Points(f64, f64),
    Percentage(f64)
}

#[derive(Debug)]
pub struct Category {
    pub name: String,
    pub weight: f64,
    pub scores: Vec<Score>
}

impl Category {
    fn all_points(&self) -> bool {
        if self.scores.len() == 0 {
            return false;
        }
        for x in &self.scores {
            match x {
                Score::Points(_a, _b) => {},
                Score::Percentage(_a) => {return false;}
            }
        }
        return true;
    }
    fn all_percentage(&self) -> bool {
        if self.scores.len() == 0 {
            return false;
        }
        for x in &self.scores {
            match x {
                Score::Points(_a, _b) => {return false;},
                Score::Percentage(_a) => {}
            }
        }
        return true;
    }
    pub fn avg(&self) -> Result<f64, ()> {
        if self.all_points() {
            return Ok(
                self.scores.iter().fold(
                    0 as f64, |acc, next| {
                        acc + match next {
                            Score::Points(a, _b) => a,
                            Score::Percentage(_a) => panic!("imposs")
                        }
                    }
                ) / self.scores.iter().fold(
                    0 as f64, |acc, next| {
                        acc + match next {
                            Score::Points(_a, b) => b,
                            Score::Percentage(_a) => panic!("imposs")
                        }
                    }
                )
            )

        } else if self.all_percentage() {
            let it = self.scores.iter();
            return Ok(
                it.fold(
                    0 as f64, |acc, next| {
                        acc + match next {
                            Score::Percentage(a) => a/(100 as f64),
                            Score::Points(_a, _b) => panic! ("imposs")
                        }
                    }
                ) / (self.scores.len() as f64)
            );
        }
        else {
            println!("Error! category {} must consist of scores that are all
            numbers or all based on points (do not mix and match them)", self.name);
            Err(())
        }
    }
}

pub fn average(v: Vec<Category>) -> Result<f64, ()> {
    let mut sum: f64 = 0 as f64;
    let mut weights: f64 = 0 as f64;
    for cat in v {
        if cat.scores.len() == 0 {continue;}
        weights += cat.weight;
        sum += cat.weight * match cat.avg() {
            Ok(avg) => {
                println!("The average for category {} is {}%", cat.name, (100 as f64)*avg);
                avg
            },
            Err(_) => {
                println!("failed to calculate grade average");
                return Err(());
            }
        };
    }
    if weights == 0 as f64 {
        println!("Error! Attempting to calculate grade with no categories put in.");
        return Err(());
    }
    return Ok(sum/weights);
}

#[test]
fn test_1() {
    let cat1 = Category{
        name: String::from("WA"),
        weight: 10 as f64,
        scores: vec![Score::Percentage(99.1) ]
    };
    match cat1.avg() {
        Ok(x) => assert_eq!(x, 99.1 as f64),
        Err(()) => panic!()
    }
}
