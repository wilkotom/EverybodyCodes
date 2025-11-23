use std::{error::Error, usize};

use aochelpers::get_everybodycodes_input;

fn main() -> Result<(), Box<dyn Error>> {
    let nums = get_everybodycodes_input(11, 2025, 1)?.lines().map(|v|v.parse().unwrap()).collect::<Vec<_>>();
    println!("Part 1: {}", part1(nums,10));
    let nums = get_everybodycodes_input(11, 2025, 2)?.lines().map(|v|v.parse().unwrap()).collect::<Vec<_>>();
    println!("Part 1: {}", part1(nums,usize::MAX));
    let nums = get_everybodycodes_input(11, 2025, 3)?.lines().map(|v|v.parse().unwrap()).collect::<Vec<_>>();
    println!("Part 3: {}", part3(nums));
    Ok(())
}

fn part1(mut columns: Vec<usize>, rounds: usize) -> usize {

    let mut phase_1  = true;
    let mut round_counter = 0;
    while round_counter < rounds {
        if phase_1 {
            phase_1 = false;
            for i in 0..columns.len()-1 {
                if columns[i+1] < columns[i] {
                    columns[i] -= 1;
                    columns[i+1] +=1;
                    phase_1 = true;
                }
            } 
        } 
        if ! phase_1 {
            for i in 0..columns.len()-1 {
                if columns[i+1] > columns[i] {
                    columns[i] += 1;
                    columns[i+1] -=1;
                }
            }
        }
        round_counter +=1;
        if columns.iter().all(|&n| n == columns[0]) {
            return round_counter;
        }
    }
    checksum(&columns)
}

fn part3(columns: Vec<usize>) -> usize {
    let target_column = columns.iter().sum::<usize>() / columns.len();
    columns.iter().filter(|&&v| v < target_column).map(|v| target_column - v).sum()


}



fn checksum(columns: &Vec<usize>) -> usize {
    columns.iter().enumerate().map(|(i,v)| (i+1) * *v).sum()
}

#[cfg(test)] 
mod test {

    use std::usize;

    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(part1(vec![9,1,1,4,9,6], 10), 109)
    }

    #[test]
    fn test_p2() {
        assert_eq!(part1(vec![9,1,1,4,9,6], usize::MAX), 11);
        assert_eq!(part1(vec![805,706,179,48,158,150,232,885,598,524,423], usize::MAX), 1579);

    }
}