use std::{collections::{HashMap}, error::Error};

use aochelpers::get_everybodycodes_input;


fn main() -> Result<(), Box<dyn Error>> {
    let p1data = get_everybodycodes_input(6, 2025, 1)?;
    println!("Part 1: {}", part1(&p1data));
    let p2data = get_everybodycodes_input(6, 2025, 2)?;
    println!("Part 1: {}", part2(&p2data));
    let p3data = get_everybodycodes_input(6, 2025, 3)?;
    println!("Part 1: {}", part3(&p3data, 1000,1000));
    Ok(())
}


fn part1(letters: &str) -> usize {
    let mut participants: HashMap<char, usize> = HashMap::new();
    for c in letters.chars() {
        if c == 'A' {
            *participants.entry(c).or_default() +=1;
        } else if c == 'a' {
            *participants.entry(c).or_default() += *participants.get(&c.to_ascii_uppercase()).unwrap_or(&0);
        }
    }
    participants.iter().map(|(k,v)| if k.is_ascii_lowercase() {*v} else {0}).sum()
}


fn part2(letters: &str) -> usize {
    let mut participants: HashMap<char, usize> = HashMap::new();
    for c in letters.chars() {
        if c.is_ascii_uppercase() {
            *participants.entry(c).or_default() +=1;
        } else if c.is_ascii_lowercase() {
            // let mentor_count = *participants.get(&c.to_ascii_uppercase()).unwrap_or(&0)
            *participants.entry(c).or_default() += *participants.get(&c.to_ascii_uppercase()).unwrap_or(&0);
        }
    }
    participants.iter().map(|(k,v)| if k.is_ascii_lowercase() {*v} else {0}).sum()
}

fn part3(letters: &str, distance: usize, repeats: usize) -> usize {
    let full_letters = letters.repeat(repeats).chars().collect::<Vec<_>>();
    let mut right_pos = 0;
    let mut available_instructors: HashMap<char, usize> = HashMap::new();
    let mut result = 0;
    while right_pos <= distance {
        let c = full_letters[right_pos];
        if c.is_ascii_uppercase() {
            *available_instructors.entry(c).or_default() +=1;
        }
        right_pos +=1;
    }
    let mut pos = 0;
    while pos < full_letters.len() {
        let c = full_letters[pos];
        if c.is_ascii_lowercase() {
            
            result += available_instructors.get(&c.to_ascii_uppercase()).unwrap_or(&0);
        }
        pos +=1;
        if pos +distance < full_letters.len() {
            let c: char = full_letters[pos+distance];
            if c.is_ascii_uppercase() {
               *available_instructors.entry(c).or_default() +=1;
            }
        }
        if pos > distance {
            let c: char = full_letters[pos - (distance +1)];
            if c.is_ascii_uppercase() {
               *available_instructors.entry(c).or_default() -=1;
            }

        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_p1() {
        assert_eq!(part1("AaAaa"), 5);
        // assert_eq!(part1("ABabACacBCbca"), 11 );
    }


    #[test]
    fn test_p3() {
        assert_eq!(part3("AABCBABCABCabcabcABCCBAACBCa", 10, 1), 34);
        assert_eq!(part3("AABCBABCABCabcabcABCCBAACBCa", 10, 2), 72);
        assert_eq!(part3("AABCBABCABCabcabcABCCBAACBCa", 1000, 1000), 3442321);

        // assert_eq!(part1("ABabACacBCbca"), 11 );
    }
}