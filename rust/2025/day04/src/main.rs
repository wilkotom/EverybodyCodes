use aochelpers::get_everybodycodes_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let p1data = get_everybodycodes_input(4, 2025, 1)?;
    let parsed = parse_data(&p1data)?;
    println!("Part 1: {}", part1(&parsed));

    let p2data = get_everybodycodes_input(4, 2025, 2)?;
    let parsed = parse_data(&p2data)?;
    println!("Part 2: {}", part2(&parsed));

    let p3data = get_everybodycodes_input(4, 2025, 3)?;
    let (first, last, everything) = parse_data_3(&p3data)?;
    println!("Part 3: {}", part3(first, last, &everything));
    Ok(())
}
fn part1(gear_teeth: &[usize]) -> usize {
    gear_teeth.iter().next().unwrap_or(&0) * 2025 / gear_teeth.iter().last().unwrap_or(&1)
}

fn part2(gear_teeth: &[usize]) -> usize {
    let first = *gear_teeth.iter().next().unwrap_or(&0);
    let last = *gear_teeth.iter().last().unwrap_or(&1);
    (10000000000000 * last).div_ceil(first)
}

fn part3(start_gear: usize, end_gear: usize, ratios: &[usize]) -> usize {
    ratios.iter().fold(start_gear * 100, |g, r| g * r ) / end_gear
}

fn parse_data(input: &str) ->  Result<Vec<usize>, Box<dyn Error>> {
    Ok(input.lines().map(|c| c.parse().expect("Not an integer")).collect())
}

fn parse_data_3(input: &str) ->  Result<(usize, usize, Vec<usize>), Box<dyn Error>> {
    let mut lines = input.lines();
    let first = lines.next().ok_or("No first line in input")?.parse()?;
    let mut last: usize = 0;
    let mut ratios = Vec::new();
    while let Some(line) = lines.next() {
        if line.contains("|") {
            let mut sections = line.split('|');
            let in_teeth = sections.next().unwrap_or_default().parse::<usize>().expect("Can't parse first field");
            let out_teeth = sections.next().unwrap_or_default().parse::<usize>().expect("Can't parse second field");
            ratios.push( out_teeth / in_teeth );
        } else {
            last = line.parse()?;
        }
    }

    Ok((first, last, ratios))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2() {
        assert_eq!(part2(&[128,64,32,16,8]), 625000000000);
        assert_eq!(part2(&[102,75,50,35,13]), 1274509803922);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part1(&[102,75,50,35,13]), 15888)
    }

    #[test]
    fn test_p3() {
        let (first, last, everything) = parse_data_3("5
7|21
18|36
27|27
10|50
10|50
11").expect("Parsing failed");
        assert_eq!(part3(first, last, &everything), 6818)
    }
}