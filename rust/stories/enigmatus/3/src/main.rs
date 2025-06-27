use std::error::Error;
use aochelpers::{Coordinate, get_everybodycodes_input};

fn main() -> Result<(), Box<dyn Error>> {
    let data: String = get_everybodycodes_input(3, 1, 1)?;
    println!("Part 1: {}", part1(parse_data(&data),100));
    let data: String = get_everybodycodes_input(3, 1, 2)?;
    println!("Part 2: {}", part2(&parse_data(&data)));
    let data: String = get_everybodycodes_input(3, 1, 3)?;
    println!("Part 3: {}", part2(&parse_data(&data)));
    Ok(())
}

fn part1(mut snails: Vec<Coordinate<i64>>, period: usize) -> i64 {

    for _ in 0..period {
        for snail in snails.iter_mut() {
            if snail.y == 1 {
                snail.y = snail.x;
                snail.x = 1;
            } else {
                snail.y -=1;
                snail.x +=1;
            }
        }
    }
    let scores = snails.iter().map(|s| s.x + 100 * s.y).collect::<Vec<_>>();
    scores.iter().sum()
    
}

fn part2(snails: &[Coordinate<i64>]) -> i64 {
    // Chinese Remainder Theorem
    let (mut period,mut delay) = (0,1);

    for snail in snails.iter(){
        let snail_period = snail.x + snail.y -1;
        while (snail.x + period) % snail_period != 0 {
            
            period += delay;
        }
        delay *= snail_period;
    }
    period

}

fn parse_data(data: &str) -> Vec<Coordinate<i64>> {

    data.lines()
    .map(|l| l.split_ascii_whitespace()
        .map(|s| s[2..].parse::<i64>().unwrap_or_default()))
    .map(|mut v| Coordinate::<i64>{x: v.next().unwrap(), y: v.next().unwrap()})
    .collect()

}

#[cfg(test)] 
mod tests {
    use super::*;

    const P1DATA: &str = "x=1 y=2
x=2 y=3
x=3 y=4
x=4 y=4";

    #[test]
    fn test_parse_data() {
        assert_eq!(parse_data(P1DATA), vec![
            Coordinate::<i64>{x:1, y:2},
            Coordinate::<i64>{x:2, y:3},
            Coordinate::<i64>{x:3, y:4},
            Coordinate::<i64>{x:4, y:4}])
    }

    #[test]
    fn test_part1(){
        let snails = parse_data(P1DATA);
        assert_eq!(part1(snails, 100), 1310)
    }

    #[test]
    fn test_part2(){
        let snails = parse_data("x=12 y=2
x=8 y=4
x=7 y=1
x=1 y=5
x=1 y=3");
        assert_eq!(part2(&snails), 14);
    

    let snails = parse_data("x=3 y=1
x=3 y=9
x=1 y=5
x=4 y=10
x=5 y=3");
        assert_eq!(part2(&snails),  13659 )
    }
}