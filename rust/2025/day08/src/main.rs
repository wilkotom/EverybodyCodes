use std::error::Error;

use aochelpers::get_everybodycodes_input;

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = get_everybodycodes_input(8, 2025, 1)?;
    let pins: Vec<i32> = input_data.split(',').flat_map(|v| v.parse::<i32>()).collect::<Vec<_>>();
    println!("Part 1: {}", part1(&pins, 32));
    let input_data = get_everybodycodes_input(8, 2025, 2)?;
    let pins: Vec<i32> = input_data.split(',').flat_map(|v| v.parse::<i32>()).collect::<Vec<_>>();
    println!("Part 2: {}", part2(&pins));
    let input_data = get_everybodycodes_input(8, 2025, 3)?;
    let pins: Vec<i32> = input_data.split(',').flat_map(|v| v.parse::<i32>()).collect::<Vec<_>>();
    println!("Part 3: {}", part3(&pins,256));
    Ok(())
}

fn part1(pins: &[i32], circle_size: i32) -> usize {
    if circle_size % 2 == 1 {
        0
    } else {
        pins.windows(2).map(|w| (w[1] - w[0]).abs()).filter(|v| *v == circle_size / 2).count()
    }
}

fn part2(pins: &[i32]) -> usize {
    let mut cords = Vec::new();
    for pair in pins.windows(2) {
        cords.push( if pair[0] < pair[1] {
            (pair[0], pair[1])
        } else {
            (pair[1], pair[0])
        });
    }

    let mut answer = 0 ;
    for (i, pair) in cords.iter().enumerate() {
        for comp in &cords[..i] {
            if pair.0 < comp.0 && comp.0 < pair.1 && pair.1 < comp.1 ||
                comp.0 < pair.0 && pair.0 < comp.1 && comp.1 < pair.1 {
                    answer+=1;
                }
        }
    }
    answer
}

fn part3(pins: &[i32], circle_size: i32) -> usize {
    let cords = pins.windows(2).map(|pair| if pair[0] < pair[1] {(pair[0], pair[1])} else {(pair[1], pair[0])}).collect::<Vec<_>>();
    
    let mut best = 0;
    for i in 1..=circle_size {
        for j in i+1..=circle_size {
            let score = cords.iter().filter(|c|
                        i < c.0 && c.0 < j && j < c.1 || c.0 < i && i < c.1 && c.1 < j || c.0 ==i && c.1 == j).collect::<Vec<_>>();
            best = best.max(score.len());
        }
    }
    best
}

#[cfg(test)]

mod test {

    use super::*;

  

    #[test]
    fn test_p1() {
        let pins = vec![1,5,2,6,8,4,1,7,3];
        assert_eq!(part1(&pins, 8), 4)
    }
    #[test]
    fn test_p2() {
        let pins = vec![1,5,2,6,8,4,1,7,3,5,7,8,2];
        assert_eq!(part2(&pins), 21)
    }

    #[test]
    fn test_p3_2() {
        let pins = vec![1,5,2,6,8,4,1,7,3,6];
        assert_eq!(part3(&pins, 8), 7);
    }

    // #[test]
    // fn test_p3() {
    //     let (names, rules) = parse_data(P3TESTDATA);
    //     assert_eq!(part3(&names, &rules), 25)
    // }

    // #[test]
    // fn test_p3_2() {
    //     let (names, rules) = parse_data(P3TESTDATA2);
    //     assert_eq!(part3(&names, &rules), 1154)
    // }

}