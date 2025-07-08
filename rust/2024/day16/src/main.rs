use std::error::Error;
use aochelpers::{lcm, get_everybodycodes_input};


fn main() -> Result<(), Box<dyn Error>> {
    let input = get_everybodycodes_input(16, 2024, 1)?;
    let (advances, reels) = parse_data(&input);
    println!("Part 1 answer: {:?}", part1(&advances, &reels, 100, 0));
    
    let input = get_everybodycodes_input(16, 2024,2)?;
    let (advances, reels) = parse_data(&input);
    println!("Part 2 answer: {:?}", part2(&advances, &reels, 202420242024));
    
    let input = get_everybodycodes_input(16, 2024, 3)?;
    let (advances, reels) = parse_data(&input);
    let p3answer =  part3(&advances, &reels, 256);
    println!("Part 3 answer: {} {}", p3answer.0, p3answer.1);

    Ok(())
}

fn parse_data(input: &str) -> (Vec<usize>, Vec<Vec<String>>) {
    let mut lines = input.lines();
    let advances = lines.next().unwrap().split(',').map(|v| v.parse().unwrap()).collect::<Vec<_>>();
    let mut reels = vec![Vec::new(); advances.len()];
    lines.next();
    for line in lines {
        for pos in (0..line.len()).step_by(4){
            if &line[pos..pos+3] != "   "{
                reels[pos / 4].push(line[pos..pos+3].to_string());
            }
        }
    }
    (advances,reels)
}

fn part1(advances: &[usize], reels: &[Vec<String>], spins: isize, offset: isize) -> String {
    let mut result = String::new();
    for (i, reel) in reels.iter().enumerate() {
        let index = ((offset + reel.len() as isize + (advances[i] as isize * spins) ) as usize ) % reel.len();
        result.push_str(&reel[index]);
        result.push(' ');
    }
    result.pop();
    result
}

fn part2(advances: &[usize], reels: &[Vec<String>], spins: isize) -> isize {
    // Data is constructed such that cycle time of each reel = len(reel)
    let cycle_time = reels.iter().fold(1, |acc, n| lcm(acc,n.len())) as isize;

    let mut score = 0;
    for i in 1..=cycle_time {
        let readout = part1(advances, reels, i, 0);
        score += score_readout(&readout, 2);
    }

    score *= spins / cycle_time;
    for i in 1..=(spins % cycle_time ) {
        let readout = part1(advances, reels, i, 0);
        score += score_readout(&readout, 2);
    }
    score
}

fn part3(advances: &[usize], reels: &[Vec<String>], pulls: isize) -> (isize, isize) {

    let mut best_scores:Vec<Vec<Option<isize>>>  = vec![vec![None; ((pulls +1) *2 +1 ) as usize]; (pulls +1) as usize]; 
    let mut worst_scores: Vec<Vec<Option<isize>>>  = vec![vec![None; ((pulls +1) *2 +1 ) as usize]; (pulls +1) as usize]; 
 
    for round in 1..=pulls {
        for offset in -round..=round {
            let cat_face = part1(advances, reels, round, offset);
            let score = score_readout(&cat_face,2);

            if let Some(prev_best) = best_scores[(round-1)as usize]
                                                       [(offset+pulls) as usize..=(offset+pulls+2) as usize]
                                                       .iter().filter_map(|c| *c).max() { 
                best_scores[round as usize][(offset+pulls +1) as usize] = Some(score + prev_best);
            } else {
                best_scores[round as usize][(offset+pulls +1) as usize] = Some(score);
            }

            if let Some(prev_worst) = worst_scores[(round-1)as usize]
                                                         [(offset+pulls) as usize..=(offset+pulls+2) as usize]
                                                         .iter().filter_map(|c| *c).min() {
                worst_scores[round as usize][(offset+pulls +1) as usize] = Some(score + prev_worst);
            } else {
                worst_scores[round as usize][(offset+pulls +1) as usize] = Some(score);
            }

        }
    }

    (*best_scores.iter().last().unwrap().iter().flatten().max().unwrap(), *worst_scores.iter().last().unwrap().iter().flatten().min().unwrap())

}


fn score_readout(readout: &str, part: usize) -> isize {
    let mut ascii_count = [0;128];
    for c in readout.chars().enumerate().filter(|(i,_)| i % if part == 2 {2} else {4} ==0).map(|(_,c)| c) {
        ascii_count[c as usize] +=1;
    };
    ascii_count.iter().map(|c| if c < &3 {0} else {c -2}).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1TEST: &str = "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>";

    const P3TEST: &str = "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- ^.^
    -.^ >.<
    >.>";

    #[test]
    fn test_p1() {
        let (advances, reels) = parse_data(P1TEST);
        assert_eq!(advances, vec![1,2,3]);
        assert_eq!(reels, vec![vec!["^_^".to_string(), ">.-".to_string(),"-_-".to_string()],
        vec!["-.-".to_string(), "^_^".to_string(), "-.-".to_string(), "-.^".to_string(), ">.>".to_string()],
        vec!["^,-".to_string(), ">.<".to_string(), ">.<".to_string(), "^_^".to_string()]]);
        assert_eq!(part1(&advances, &reels, 0, 0), String::from("^_^ -.- ^,-"));
        assert_eq!(part1(&advances, &reels, 1, 0), String::from(">.- -.- ^_^"));
        assert_eq!(part1(&advances, &reels, 5, 0), String::from("-_- -.- ^_^"));
        assert_eq!(part1(&advances, &reels, 21, 0), String::from("^_^ -.- ^_^"));
        assert_eq!(part1(&advances, &reels, 33, 0), String::from("^_^ ^_^ ^_^"));
        assert_eq!(part1(&advances, &reels, 100, 0), String::from(">.- -.- ^,-"));

    }

    #[test]
    fn test_part2_score(){
        assert_eq!(score_readout("^_^ ^_^ ^_^",2), 4);
        let (advances, reels) = parse_data(P1TEST);

        for (spin, expected_accumulation) in [(0,0_isize),(1,1),(2,2),(3,4),(4,5),(5,7),(10,15),(100,138),(1000,1383),(10000,13833),(202420242024, 280014668134)] {
            assert_eq!(part2(&advances, &reels, spin), expected_accumulation);
        }
    }

    #[test]
    fn test_part3(){
        let (advances, reels) = parse_data(P3TEST);
        for (pulls, result) in [(2, (6,1)), (3,(9,2)), (10,(26,5)),(100,(246,50)),(256,(627,128))] {
            assert_eq!(part3(&advances, &reels,pulls), result)
        }
    }
}