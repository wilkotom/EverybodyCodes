use aochelpers::get_everybodycodes_input;
use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
enum Step {
    Left(i32),
    Right(i32)
}

fn main() -> Result<(), Box<dyn Error>>{
    let p1data = get_everybodycodes_input(1, 2025, 1)?;
    let (names,steps) = parse_data(&p1data)?;
    println!("Part 1: {}", part1(&names, &steps));
    let p2data = get_everybodycodes_input(1, 2025, 2)?;
    let (names,steps) = parse_data(&p2data)?;
    println!("Part 2: {}", part2(&names, &steps));
    let p3data = get_everybodycodes_input(1, 2025, 3)?;
    let (names,steps) = parse_data(&p3data)?;

    println!("Part 3: {}", part3(names, &steps));

    Ok(())
}

fn parse_data<'a>(data: &'a str) -> Result<(Vec<&'a str>, Vec<Step>), Box<dyn Error>> {
    let mut sections = data.split("\n\n");
    let names = sections.next().unwrap_or_default().split(",").collect();
    let steps = sections.next().expect("No Directions section")
        .split(",")
        .map(|s| if s.starts_with("L") {
                Step::Left(s[1..].parse().expect("Not a number"))
            } else {
                Step::Right(s[1..].parse().expect("Not a number"))
            })
        .collect();

    Ok((names, steps))
}

fn part1<'a>(names: &[&'a str], steps: &[Step]) -> &'a str {
    let mut pos = 0;
    for step in steps {
        pos += match step{
            Step::Left(n) => -n,
            Step::Right(n) => *n,
        };
        pos = pos.max(0);
        pos = pos.min(names.len() as i32 -1);
    }
    names[pos as usize]
}

fn part2<'a>(names: &[&'a str], steps: &[Step]) -> &'a str {
    let mut pos = 0;
    let name_count = names.len() as i32;
    for step in steps {
        pos += match step{
            Step::Left(n) => -n,
            Step::Right(n) => *n,
        };
        pos += name_count;
        pos %= name_count;

    }
    while pos < 0 {
        pos += name_count;
    }
    names[(pos % name_count) as usize]
}

fn part3<'a>(mut names: Vec<&'a str>, steps: &[Step]) -> &'a str {
    let name_count = names.len() as i32;
    for step in steps {
        let pos = (match step{
            Step::Left(n) => name_count - *n,
            Step::Right(n) => name_count + *n,
        } + name_count) % name_count;
        (names[0], names[pos as usize]) = (names[pos as usize], names[0]);
    }
    names[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1TESTDATA: &str = "Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1";

    const P3TESTDATA: &str = "Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L3";

    #[test]
    fn test_parser() {
        if let Ok((names,steps)) = parse_data(P1TESTDATA) {
            assert_eq!(names, vec!["Vyrdax","Drakzyph","Fyrryn","Elarzris"]);
            assert_eq!(steps, vec![Step::Right(3), Step::Left(2), Step::Right(3), Step::Left(1)]);
        }
        else {
            panic!()
        }
    }


    #[test]
    fn test_part1() {
        if let Ok((names,steps)) = parse_data(P1TESTDATA) {
            assert_eq!(part1(&names, &steps), "Fyrryn")
        }
        else {
            panic!()
        }
    }

    #[test]
    fn test_part2() {
        if let Ok((names,steps)) = parse_data(P1TESTDATA) {
            assert_eq!(part2(&names, &steps), "Elarzris")
        }
        else {
            panic!()
        }
    }

        #[test]
    fn test_part3() {
        if let Ok((names,steps)) = parse_data(P3TESTDATA) {
            assert_eq!(part3(names, &steps), "Drakzyph")
        }
        else {
            panic!()
        }
    }
}