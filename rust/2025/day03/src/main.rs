use aochelpers::get_everybodycodes_input;
use std::{collections::{HashMap, HashSet}, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let p1data = get_everybodycodes_input(3, 2025, 1)?;
    let parsed = parse_data(&p1data)?;
    println!("Part 1: {}", part1(&parsed));

    let p2data = get_everybodycodes_input(3, 2025, 2)?;
    let parsed = parse_data(&p2data)?;
    println!("Part 2: {}", part2(&parsed));

    let p3data = get_everybodycodes_input(3, 2025, 3)?;
    let parsed = parse_data(&p3data)?;
    println!("Part 3: {}", part3(&parsed));
    Ok(())
}

fn part1(crate_sizes: &[i32]) -> i32 {
    let unique_sizes: HashSet<_> = crate_sizes.iter().collect();
    unique_sizes.iter().map(|s| *s).sum()
}

fn part2(crate_sizes: &[i32]) -> i32 {
    let mut unique_sizes= crate_sizes.iter().collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();
    unique_sizes.sort_unstable();
    unique_sizes.iter().take(20).map(|s| *s).sum()
}

fn part3(crate_sizes: &[i32]) -> i32 {
    let mut unique_crates: HashMap<i32, i32> = HashMap::new();

    crate_sizes.iter().for_each(|s| *unique_crates.entry(*s).or_default() += 1);
    println!("{:?}", unique_crates);
    *(unique_crates.values().max().unwrap_or(&0))

}

fn parse_data(input: &str) ->  Result<Vec<i32>, Box<dyn Error>> {
    Ok(input.split(',').map(|c| c.parse().expect("Not an integer")).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse_data("10,5,1,10,3,8,5,2,2").expect("Parse failed"), vec![10,5,1,10,3,8,5,2,2])
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[10,5,1,10,3,8,5,2,2]), 29);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&[4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77]), 781);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(&[4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77]), 3);
    }
}