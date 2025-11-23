use aochelpers::get_everybodycodes_input;
use std::{collections::VecDeque, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_everybodycodes_input(13, 2025, 1)?;
    let wheel = build_wheel(&data);
    println!("{:?}", wheel);
    println!("Part 1: {}", part1(&wheel, 2025));

    let data = get_everybodycodes_input(13, 2025, 2)?;
    let wheel = build_wheel(&data);
    println!("Part 2: {}", part1(&wheel, 20252025));


    let data = get_everybodycodes_input(13, 2025, 3)?;
    let wheel = build_wheel(&data);
    println!("Part 3: {}", part1(&wheel, 202520252025));
    Ok(())
}
fn build_wheel(data: &str) -> VecDeque<i32>{
    let mut wheel = VecDeque::new();
    wheel.push_back(1);
    for (i, line) in data.lines().enumerate() {
        if line.contains('-') {
            let mut nums = line.split('-').map(|s| s.parse::<i32>().unwrap_or_default());
            for j in nums.next().unwrap_or_default()..=nums.next().unwrap_or_default() {
                if i %2 == 0 {
                    wheel.push_back(j);
                } else {
                    wheel.push_front(j);
                }
            }
        } else {
            if i %2 == 0 {
                wheel.push_back(line.parse().unwrap());
            } else {
                wheel.push_front(line.parse().unwrap());
            }
        }   
    }
    while wheel[0] != 1 {
        wheel.rotate_right(1);
    }
    wheel
}


fn part1(nums: &VecDeque<i32>,pos: usize) -> i32 {
    println!("Part 2 len: {} remainder: {}", nums.len(), pos % nums.len());
    nums[pos % nums.len()]

}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;


    const P1TESTDATA: &str = "72
58
47
61
67";

    const P2TESTDATA: &str = "10-15
12-13
20-21
19-23
30-37";

    #[test]
    fn test_build_wheel() {
        assert_eq!(build_wheel(P1TESTDATA), VecDeque::from([1, 72, 47, 67, 61, 58]));
        assert_eq!(build_wheel(P2TESTDATA), VecDeque::from([1,10,11,12,13,14,15,20,21,30,31,32,33,34,35,36,37,23,22,21,20,19,13,12 ]));
    }
    #[test]
    fn test_p1() {
        let wheel = build_wheel(P1TESTDATA);
        assert_eq!(part1(&wheel, 2025), 67);
        assert_eq!(part1(&wheel, 2024), 47);
    }

    #[test]
    fn test_p2() {
        let wheel = build_wheel(P2TESTDATA);
        assert_eq!(part1(&wheel, 20252025), 30);

        println!("{:?}", wheel);
    }
}