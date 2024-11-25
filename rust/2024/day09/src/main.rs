use std::fs::read_to_string;
use cached::proc_macro::cached;

fn main() {
    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q09_p1.txt").unwrap();
    println!("Part 1: {}", part1(&input, 1));
    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q09_p2.txt").unwrap();
    println!("Part 2: {}", part1(&input, 2));
    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q09_p3.txt").unwrap();
    println!("Part 3: {}", part3(&input));
}


fn part1(input: &str, part: usize) -> i64 {
    let brightnesses: Vec<i64> = input.split('\n').map(|x: &str| x.parse::<i64>().unwrap_or(0)).collect::<Vec<_>>();
    let mut total = 0;
    for target in brightnesses {
        total += stamp_beetle(target, part -1,0)
    }
    total
}

fn part3(input: &str) -> i64 {
    let brightnesses: Vec<i64> = input.split('\n').map(|x: &str| x.parse::<i64>().unwrap_or(0)).collect::<Vec<_>>();
    let mut total = 0;
    let mut results= vec![];
    for target in brightnesses {
        let mut best = target;
        for left in (target/2)..=(target/2 + 50){
            let right = target - left;
            let attempt = stamp_beetle(left, 2, 0) + stamp_beetle(right, 2, 0);
            if attempt < best {
                best = best.min(attempt);
            }

            }
        results.push(best);
        total += best
    }
    total
}

#[cached]
fn stamp_beetle(beetle: i64, part: usize, offset: usize) -> i64 {
    let stamps = [
        vec![10,5,3,1], 
        vec![30,25,24,20,16,15,10,5,3,1],
        vec![101,100,75,74,50,49,38,37,30,25,24,20,16,15,10,5,3,1]
];
    if stamps[part][offset] == 1 {
        beetle
    }  else if stamps[part][offset] == beetle {
      1
    } else if stamps[part][offset] < beetle {
        (1 + stamp_beetle(beetle - stamps[part][offset], part, offset) ).min(stamp_beetle(beetle, part, offset+1))
    } else {
        stamp_beetle(beetle, part, offset+1)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {

        assert_eq!(part1("2",1), 2);
        assert_eq!(part1("4",1), 2);
        assert_eq!(part1("7",1), 3);
        assert_eq!(part1("16",1), 3);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part1("33",2), 2);
        assert_eq!(part1("41",2), 2);
        assert_eq!(part1("55",2), 2);
        assert_eq!(part1("99",2), 4);

    }

    #[test]
    fn test_p3() {
        assert_eq!(part3("156488"), 1550);
        assert_eq!(part3("352486"), 3490);
        assert_eq!(part3("546212"), 5409);
        assert_eq!(part3("156488\n352486\n546212"), 10449);

    }


}

//1909 wrong answer