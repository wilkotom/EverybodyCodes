use std::{collections::VecDeque, error::Error};
use aochelpers::get_everybodycodes_input;

#[derive(Debug, PartialEq, Eq)]
enum Colour {
    Red,
    Green,
    Blue
}

const ORDER: [Colour; 3] = [Colour::Red, Colour::Green, Colour::Blue];

fn main() -> Result<(), Box<dyn Error>>{
    let part1_balloons = parse_data(&get_everybodycodes_input(2, 2, 1)?);
    println!("Part 1: {}", part1(&part1_balloons));

    let part2_input = parse_data(&get_everybodycodes_input(2, 2, 2)?.repeat(100));
    println!("Part 2: {}", part2(part2_input));

    let part3_input = parse_data(&get_everybodycodes_input(2, 2, 3)?.repeat(100000));
    println!("Part 3: {}", part2(part3_input));


    Ok(())
}


fn parse_data(data: &str) -> Vec<Colour> {
    data.chars()
        .map(|c| match c { 'R' => Colour::Red, 'G' => Colour::Green, 'B' => Colour::Blue, _ => unimplemented!()})
        .collect()
}

fn part1(balloons: &[Colour]) -> usize {
    let mut arrow_count: usize = 0;
    let mut position = 0;
    while position < balloons.len() {
        let current_colour = &ORDER[arrow_count % 3];
        while position < balloons.len() && balloons[position] == *current_colour{
            position +=1
        }
        arrow_count +=1;
        position +=1;
    }
    arrow_count
}

fn part2(balloons: Vec<Colour>) -> usize {
    let mut first_semicircle = VecDeque::from(balloons);
    let mut second_semicircle = first_semicircle.split_off(first_semicircle.len() / 2);
    let mut arrow_count = 0;
    while !first_semicircle.is_empty() {
        let current_colour = &ORDER[arrow_count % 3];
        if first_semicircle.len() == second_semicircle.len() {
            let first_balloon = first_semicircle.pop_front().unwrap();
            if first_balloon == *current_colour {
                second_semicircle.pop_front();
            }
        } else {
            first_semicircle.pop_front();
        }
        if first_semicircle.len() < second_semicircle.len() && ! second_semicircle.is_empty() {
            first_semicircle.push_back(second_semicircle.pop_front().unwrap());
        }
        arrow_count +=1;
    }
    arrow_count
}


#[cfg(test)]
mod test{ 
    use super::*;

    const P1TEST: &str = "GRBGGGBBBRRRRRRRR";

    #[test]
    fn test_part_1() {
        let balloons = parse_data(P1TEST);
        assert_eq!(part1(&balloons), 7);
    }
    #[test]
    fn test_part_2() {
        let balloons = parse_data(&"GGBR".repeat(5));
        assert_eq!(part2(balloons), 14);
    }
}