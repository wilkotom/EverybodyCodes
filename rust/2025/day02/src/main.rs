use aochelpers::get_everybodycodes_input;
use std::{collections::HashMap, error::Error, fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub}};

#[derive(Debug, PartialEq, Eq,Copy,Clone)]
struct ComplexNumber {
    real: i128,
    imaginary: i128
}

impl Display for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{},{}]", self.real, self.imaginary)
    }
}

impl Add for ComplexNumber {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary
        }
    }
}

impl AddAssign for ComplexNumber {
    fn add_assign(&mut self, other: Self) {
        self.real =  self.real + other.real;
        self.imaginary =  self.imaginary + other.imaginary;
    }
}

impl Sub for ComplexNumber {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary
        }
    }
}

impl Mul for ComplexNumber {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let real = self.real * other.real - self.imaginary * other.imaginary;
        let imaginary = self.real * other.imaginary + self.imaginary * other.real;
        Self {
            real,
            imaginary
        }
    }
}

impl MulAssign for ComplexNumber {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl Div for ComplexNumber {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            real: self.real / other.real,
            imaginary: self.imaginary / other.imaginary
        }
    }
}


impl DivAssign for ComplexNumber {
    fn div_assign(&mut self, other: Self) {
            self.real = self.real / other.real;
            self.imaginary = self.imaginary / other.imaginary;
    }
}


fn main() -> Result<(), Box<dyn Error>>{
    let p1data = get_everybodycodes_input(2, 2025, 1)?;
    let parsed = parse_data(&p1data)?;
    println!("Part 1: {}", part1(*(parsed.get("A").expect("No label A in part 1 data"))));
    let p2data = get_everybodycodes_input(2, 2025, 2)?;
    let parsed = parse_data(&p2data)?;
    println!("Part 2: {}", part2(*(parsed.get("A").expect("No label A in part 2 data"))));
    let p3data = get_everybodycodes_input(2, 2025, 2)?;
    let parsed = parse_data(&p3data)?;
    println!("Part 3: {}", part3(*(parsed.get("A").expect("No label A in part 2 data"))));
    Ok(())
}

fn parse_data<'a>(data: &'a str) -> Result<HashMap<&'a str, ComplexNumber>, Box<dyn Error>> {
    let mut numbers = HashMap::new();
    for line in data.lines() {
        let mut tokens = line.split("=[");
        let label = tokens.next().expect("No label found");
        let mut values = tokens.next().expect("No value supplied").trim_end_matches(']').split(',');
        numbers.insert(label, ComplexNumber { 
            real: values.next().expect("First value missing").parse().expect("Value cannot be parsed"), 
            imaginary: values.next().expect("First value missing").parse().expect("Value cannot be parsed")
        });

    }
    Ok(numbers)
}

fn part2(seed: ComplexNumber) -> usize {
    let mut counter = 0;
    for y in 0..=100 {
        for x in 0..=100 {
            if let Some(_) = cycle( seed + ComplexNumber { real: x *10 , imaginary: y*10}, 100, ComplexNumber{real: 100000, imaginary: 100000})
            {
                counter +=1;
            } 
        }
    }

    counter
}

fn part3(seed: ComplexNumber) -> usize {
    let mut counter = 0;
    for y in 0..=1000 {
        for x in 0..=1000 {
            if let Some(_) = cycle( seed + ComplexNumber { real: x , imaginary: y}, 100, ComplexNumber{real: 100000, imaginary: 100000})
            {
                counter +=1;
            } else {
            }
        }
    }
    counter
}

fn part1(number: ComplexNumber) -> ComplexNumber {

    cycle(number, 3, ComplexNumber{real: 10, imaginary: 10}).expect("No Answer for Part 1")
}

fn cycle(number: ComplexNumber, repetitions: usize, divisor: ComplexNumber) -> Option<ComplexNumber> {
    let mut res = ComplexNumber{real: 0, imaginary: 0};

    for _ in 0..repetitions {
        res = res * res / divisor + number;
        if res.real.abs() > 100_0000 || res.imaginary.abs() > 100_0000 {
            return None
        }
    }

    Some(res)
}


#[cfg(test)]
mod tests {
    use super::*;

    const P1TESTDATA: &str = "A=[25,9]";


    #[test]
    fn test_parser() {
        let test_cases = [
           ( "A", ComplexNumber{ real: 25, imaginary: 9})
        ];
        let parsed = parse_data(P1TESTDATA).expect("Parsing failed");
        for (label, result) in test_cases {
            assert_eq!(parsed.get(label), Some(&result));
        }
    }
    #[test]
    fn test_add() {
        let test_cases = [
            (ComplexNumber{real:1, imaginary:1}, ComplexNumber{real:2, imaginary:2}, ComplexNumber{real:3, imaginary:3}),
            (ComplexNumber{real:2, imaginary:5}, ComplexNumber{real:3, imaginary:7}, ComplexNumber{real:5, imaginary:12}),
            (ComplexNumber{real:-2, imaginary:5}, ComplexNumber{real:10, imaginary:-1}, ComplexNumber{real:8, imaginary:4}),
            (ComplexNumber{real:-1, imaginary:-2}, ComplexNumber{real:-3, imaginary:-4}, ComplexNumber{real:-4, imaginary:-6})];
        for (left, right, result) in test_cases {
            assert_eq!(left+right, result)
        }
    }

    #[test]
    fn test_mul() {
        let test_cases = [
            (ComplexNumber{real:1, imaginary:1}, ComplexNumber{real:2, imaginary:2}, ComplexNumber{real:0, imaginary:4}),
            (ComplexNumber{real:2, imaginary:5}, ComplexNumber{real:3, imaginary:7}, ComplexNumber{real:-29, imaginary:29}),
            (ComplexNumber{real:-2, imaginary:5}, ComplexNumber{real:10, imaginary:-1}, ComplexNumber{real:-15, imaginary:52}),
            (ComplexNumber{real:-1, imaginary:-2}, ComplexNumber{real:-3, imaginary:-4}, ComplexNumber{real:-5, imaginary:10})];

        for (left, right, result) in test_cases {
            assert_eq!(left*right, result)
        }
    }

    #[test]
    fn test_div() {
        let test_cases = vec![
            (ComplexNumber{real:10, imaginary:12}, ComplexNumber{real:2, imaginary:2}, ComplexNumber{real:5, imaginary:6}),
            (ComplexNumber{real:11, imaginary:12}, ComplexNumber{real:3, imaginary:5}, ComplexNumber{real: 3, imaginary:2}),
            (ComplexNumber{real:-10, imaginary:-12}, ComplexNumber{real:2, imaginary:2}, ComplexNumber{real:-5, imaginary:-6}),
            (ComplexNumber{real:-11, imaginary:-12}, ComplexNumber{real:3, imaginary:5}, ComplexNumber{real:-3, imaginary:-2})];
            
        for (left, right, result) in test_cases {
            assert_eq!(left/right, result)
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(ComplexNumber { real: 25, imaginary: 9 }), ComplexNumber{real: 357, imaginary: 862})
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ComplexNumber { real: 35300, imaginary: -64910 }), 4076);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(ComplexNumber { real: 35300, imaginary: -64910 }), 406954);

    }
}