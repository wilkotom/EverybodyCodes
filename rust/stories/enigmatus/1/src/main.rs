use std::{collections::{HashMap, VecDeque}, error::Error};
use aochelpers::get_everybodycodes_input;

fn main() -> Result<(), Box<dyn Error>>{
    let part1_input = get_everybodycodes_input(1, 1, 1)?;
    println!("Part 1: {}", part1(&part1_input));
    let part2_input = get_everybodycodes_input(1, 1, 2)?;
    println!("Part 2: {}", part2(&part2_input));
    let part3_input = get_everybodycodes_input(1, 1, 3)?;
    println!("Part 3: {}", part3(&part3_input));
    Ok(())
}

fn eni(n: i128, exp: i128, modulus: i128) -> i128 {
    let mut score = 1;
    let mut remainders = Vec::new();
    for _ in 0..exp {
        score *= n;
        score %= modulus;
        remainders.push(score);

    }
    let mut result = 0;
    while let Some(n) = remainders.pop() {
        result *=  10_i128.pow(n.checked_ilog10().unwrap_or(0)+1);
        result += n;
    }
    result
}

fn eni2(n: i128, exp: i128, modulus: i128) -> i128 {
    #[derive(Eq,PartialEq,Hash)]
    struct State {
        score: i128,
        values: [i128;5]
    }
    let mut score = 1;
    let mut remainders = VecDeque::from([0,0,0,0,0]);
    let mut previous = HashMap::<State, i128>::new();
    let mut i = 0;
    while i < exp {
        i += 1;
        score *= n;
        score %= modulus;
        remainders.push_front(score);
        remainders.pop_back();
        let state = State{
            score,
            values: [remainders[0], remainders[1], remainders[2], remainders[3], remainders[4]]
        };
        if let Some(prev_state_point) = previous.get(&state) {
            let period = i - prev_state_point;
            let repetitions = (exp - prev_state_point ) / period;
            i += period * repetitions;
            if i > exp {
                i -= period;
            }
        } else {
            previous.insert(state, i);
        } 
    }
    let mut result = 0;
    for _ in 0..5.min(i) {
        if let Some(n) = remainders.pop_front() {
            result *=  10_i128.pow(n.checked_ilog10().unwrap_or(0)+1);
            result += n;
        }
    }
    result
}

fn eni3(n: i128, exp: i128, modulus: i128) -> i128 {
    let mut score = 1;
    let mut previous = HashMap::<i128, (i128,i128)>::new();
    let mut i = 0;
    let mut total = 0;
    while i < exp {
        i += 1;
        score *= n;
        score %= modulus;
        total += score;
        if let Some((prev_score_point, prev_total)) = previous.get(&score) {
            let period = i - prev_score_point;
            if i + period > exp {
                continue;
            }
            let period_score_total = total - prev_total;
            let repetitions = (exp -i) / period;
            i += repetitions * period;
            total += repetitions * period_score_total;
        } else {
            previous.insert(score, (i,total));
        } 
    }
    total
}

fn parse_line(line: &str) -> Vec<i128> {
    line.split_ascii_whitespace().map(|t| t[2..].parse::<i128>().unwrap_or_default()).collect()
}

fn part1(data: &str) -> i128 {
    data.lines().map(parse_line).map(|nums| 
        eni(nums[0], nums[3], nums[6]) + 
        eni(nums[1], nums[4], nums[6]) + 
        eni(nums[2], nums[5], nums[6])).max().unwrap_or_default()
}

fn part2(data: &str) -> i128 {
    data.lines().map(parse_line).map(|nums| 
        eni2(nums[0], nums[3], nums[6]) + 
        eni2(nums[1], nums[4], nums[6]) + 
        eni2(nums[2], nums[5], nums[6])).max().unwrap_or_default()
}

fn part3(data: &str) -> i128 {
    data.lines().map(parse_line).map(|nums| 
        eni3(nums[0], nums[3], nums[6]) + 
        eni3(nums[1], nums[4], nums[6]) + 
        eni3(nums[2], nums[5], nums[6])).max().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eni(){
        assert_eq!(eni(2,4,5),1342);
        assert_eq!(eni(3,5,16),311193);
    }

        #[test]
    fn test_eni2(){
        assert_eq!(eni2(2,7,5),34213);
        assert_eq!(eni2(4,3,11),954);
        assert_eq!(eni2(4,14,11),39541);
        assert_eq!(eni2(6,15,11),109736);
        assert_eq!(eni2(8,8,12),48484);
        assert_eq!(eni2(4,14,12),44444);
        assert_eq!(eni2(7,16,12),17171);
        assert_eq!(eni2(2,2,13),42);
        assert_eq!(eni2(8,14,13),1281512);
        assert_eq!(eni2(6,15,13),8106111);
    }
    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("A=4 B=4 C=6 X=3 Y=4 Z=5 M=11"), vec![4,4,6,3,4,5,11]);
    }

        #[test]
    fn test_part_1() {
        assert_eq!(part1("A=4 B=4 C=6 X=3 Y=4 Z=5 M=11
A=8 B=4 C=7 X=8 Y=4 Z=6 M=12
A=2 B=8 C=6 X=2 Y=4 Z=5 M=13
A=5 B=9 C=6 X=8 Y=6 Z=8 M=14
A=5 B=9 C=7 X=6 Y=6 Z=8 M=15
A=8 B=8 C=8 X=6 Y=9 Z=6 M=16"), 11611972920);
    }

#[test]
fn test_part_2() {
    assert_eq!(part2("A=4 B=4 C=6 X=3 Y=14 Z=15 M=11
A=8 B=4 C=7 X=8 Y=14 Z=16 M=12
A=2 B=8 C=6 X=2 Y=14 Z=15 M=13
A=5 B=9 C=6 X=8 Y=16 Z=18 M=14
A=5 B=9 C=7 X=6 Y=16 Z=18 M=15
A=8 B=8 C=8 X=6 Y=19 Z=16 M=16"), 11051340);

   assert_eq!(part2("A=3657 B=3583 C=9716 X=903056852 Y=9283895500 Z=85920867478 M=188
A=6061 B=4425 C=5082 X=731145782 Y=1550090416 Z=87586428967 M=107
A=7818 B=5395 C=9975 X=122388873 Y=4093041057 Z=58606045432 M=102
A=7681 B=9603 C=5681 X=716116871 Y=6421884967 Z=66298999264 M=196
A=7334 B=9016 C=8524 X=297284338 Y=1565962337 Z=86750102612 M=145"), 1507702060886);
    }

    #[test]
    fn test_part_3() {
        assert_eq!(part3("A=4 B=4 C=6 X=3000 Y=14000 Z=15000 M=110
        A=8 B=4 C=7 X=8000 Y=14000 Z=16000 M=120
        A=2 B=8 C=6 X=2000 Y=14000 Z=15000 M=130
        A=5 B=9 C=6 X=8000 Y=16000 Z=18000 M=140
        A=5 B=9 C=7 X=6000 Y=16000 Z=18000 M=150
        A=8 B=8 C=8 X=6000 Y=19000 Z=16000 M=160"), 3279640);

        assert_eq!(part3("A=3657 B=3583 C=9716 X=903056852 Y=9283895500 Z=85920867478 M=188
        A=6061 B=4425 C=5082 X=731145782 Y=1550090416 Z=87586428967 M=107
        A=7818 B=5395 C=9975 X=122388873 Y=4093041057 Z=58606045432 M=102
        A=7681 B=9603 C=5681 X=716116871 Y=6421884967 Z=66298999264 M=196
        A=7334 B=9016 C=8524 X=297284338 Y=1565962337 Z=86750102612 M=145"), 7276515438396);
    }

    #[test]
    fn test_eni3(){
        assert_eq!(eni3(2,7,5), 19);
        assert_eq!(eni3(3,8,16), 48);

        assert_eq!(eni3(4,3000,110), 132000);
    }
}