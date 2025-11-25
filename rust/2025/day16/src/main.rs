use aochelpers::get_everybodycodes_input;
use std::{error::Error, thread::panicking};

fn main() -> Result<(), Box<dyn Error>> {
    let data: Vec<usize> = get_everybodycodes_input(16,2025,1)?
        .split(",").map(|c| c.parse().expect("Not a number")).collect();
    println!("Part 1:: {}", part1(&data, 90));
    let data: Vec<usize> = get_everybodycodes_input(16,2025,2)?
        .split(",").map(|c| c.parse().expect("Not a number")).collect();
    println!("Part 2: {}", part2(&data).iter().fold(1, |a,b| a*b));
    let data: Vec<usize> = get_everybodycodes_input(16,2025,3)?
        .split(",").map(|c| c.parse().expect("Not a number")).collect();
    println!("Part 3: {}", part3(data, 202520252025000));

    Ok(())
}

fn part1(nums: &Vec<usize>, target: usize) -> usize{
    nums.iter().map(|n| target / n).sum()
}

fn part2(nums:&Vec<usize>) -> Vec<usize> {
    let mut result = vec![];
    for (i,n) in nums.iter().enumerate() {
        if *n > result.iter().filter(|&v| (i+1) % v == 0).count() {
            result.push(i+1);
        }
    }
    result
}

fn part3(nums:Vec<usize>, target: usize) -> usize {
    let pattern: Vec<usize> = part2(&nums);
    let mut current_column_count = 1;
    while part1(&pattern, current_column_count) < target {
        current_column_count *=2;
    }
    let mut window_size = current_column_count /2;

    while window_size > 0 {
        match part1(&pattern, current_column_count).cmp(&target) {
            std::cmp::Ordering::Less => {current_column_count += window_size;}
            std::cmp::Ordering::Equal => {return current_column_count;}
            std::cmp::Ordering::Greater => {current_column_count -=window_size;}
        }
        
        window_size /=2;
    }
    // if the final column is incomplete, disregard it.
    // This can happen because we always increment the column count if
    // the current column count requires less than the target number of blocks
    
    if part1(&pattern, current_column_count) > target {
        current_column_count -1
    } else {
        current_column_count
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(part1(&vec![1,2,3,5,9], 90),193);
    }


    #[test]
    fn test_p2() {
        assert_eq!(part2(&vec![1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2]),[1, 2, 3, 5, 9]);
    }

    #[test]
    fn test_p3() {
        for (blocks,columns) in [(1,1),(10,5),(100,47),(1000,467),(10000,4664),(202520252025000,94439495762954)] {
            assert_eq!(part3(vec![1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2], blocks),columns);
        }
    }
}