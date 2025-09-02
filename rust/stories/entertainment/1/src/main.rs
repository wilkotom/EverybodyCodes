use std::{collections::{BinaryHeap, HashSet}, error::Error, isize};
use aochelpers::{get_everybodycodes_input, Coordinate, Direction, ScoredItem};

fn main() -> Result<(), Box<dyn Error>>{
    let in_data: String = get_everybodycodes_input(1, 2, 1)?;
    let (board, tokens) = parse_data(&in_data);
    println!("Part 1: {}", part1(&board, &tokens));

    let in_data: String = get_everybodycodes_input(1, 2, 2)?;
    let (board, tokens) = parse_data(&in_data);
    println!("Part 2: {}", part2(&board, &tokens));

    let in_data: String = get_everybodycodes_input(1, 2, 3)?;
    let (board, tokens) = parse_data(&in_data);
    println!("Part 3: {} {}", part3_min(&board, &tokens), part3_max(&board, &tokens));

    Ok(())
}

fn part1(board: &HashSet<Coordinate<isize>>, tokens: &[Vec<Direction>]) -> isize {

    tokens.iter().enumerate().map(
        |(i, t)| token_score(&board, t, i as isize +1)
    ).sum()
}

fn part2(board: &HashSet<Coordinate<isize>>, tokens: &[Vec<Direction>]) -> isize {

    tokens.iter().map(
        |t| (0..=12).map(
            |n| token_score(&board, t, n as isize +1)
        ).max().unwrap_or_default()
    ).sum()

}

fn part3_min(board: &HashSet<Coordinate<isize>>, tokens: &[Vec<Direction>]) -> isize{

    let mut unvisited = BinaryHeap::new();
    unvisited.push(ScoredItem{cost: 0, item: Vec::new()});
    let seen: HashSet<Vec<isize>>  = HashSet::new();
    while let Some(state) = unvisited.pop() {
        if seen.contains(&state.item) {
            continue;
        }
        if state.item.len() == tokens.len() {
            return  state.cost;
        }
        for slot in 1..=20 {
            if state.item.contains(&slot) {
                continue;
            }
            let extra_score = token_score(board, &tokens[state.item.len()], slot);
            let mut new_state = state.clone();
            new_state.item.push(slot);
            new_state.cost += extra_score;
            unvisited.push(new_state);
        }
    }
    unreachable!()
}


fn part3_max(board: &HashSet<Coordinate<isize>>, tokens: &[Vec<Direction>]) -> isize{
    let mut unvisited = BinaryHeap::new();
    unvisited.push((0, Vec::new()));
    let seen: HashSet<Vec<isize>>  = HashSet::new();
    while let Some(state) = unvisited.pop() {
        if seen.contains(&state.1) {
            continue;
        }
        if state.1.len() == tokens.len() {
            return state.0;
        }
        for slot in 1..=20 {
            if state.1.contains(&slot) {
                continue;
            }
            let extra_score = token_score(board, &tokens[state.1.len()], slot);
            let mut new_state = state.clone();
            new_state.1.push(slot);
            new_state.0 += extra_score;
            unvisited.push(new_state);
        }
    }
    unreachable!()
}


fn token_score(board: &HashSet<Coordinate<isize>>, path: &[Direction], entrypoint: isize) -> isize {
    let mut token = Coordinate{x: entrypoint *2 -1, y:0};
    let max_x: isize = board.iter().map(|c| c.x).max().unwrap_or_default();
    let max_y:isize = board.iter().map(|c| c.y).max().unwrap_or_default();
    let mut stage = 0;

    while token.y <= max_y {
        token.y +=1;
        if board.contains(&token) {
            match path[stage % path.len()] {
                Direction::East => {
                    token.x +=1;
                    if token.x > max_x {
                        token.x -=2;
                    }
                }
                Direction::West => {
                    token.x -=1;
                    if token.x < 1 {
                        token.x =2;
                    }
                }
                _ => unimplemented!()
            }
            stage +=1;
        }
    }
    (((token.x+1)/2) * 2 - entrypoint).max(0)
}

fn parse_data(data: &str) -> (HashSet<Coordinate<isize>>,Vec<Vec<Direction>>) {
    let mut nails = HashSet::new();
    let mut sections = data.split("\n\n");
    for( y, line )in sections.next().unwrap_or_default().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                nails.insert(Coordinate { x: x as isize+1, y: y as isize +1});
            }
        }
    }

    let tokens = sections.next().unwrap_or_default().lines()
        .map(|l | l.chars().map(|c| match c {
            'L' => Direction::West,
            'R' => Direction::East,
            _ => unimplemented!()
        }).collect())
        .collect();

    (nails, tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1DATA: &str = "*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*...*..
.*.*.*.*.*...*.*.
*.*.....*...*.*.*
.*.*.*.*.*.*.*.*.
*...*...*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*
.*...*...*.*.*.*.
*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.

RRRLRLRRRRRL
LLLLRLRRRRRR
RLLLLLRLRLRL
LRLLLRRRLRLR
LLRLLRLLLRRL
LRLRLLLRRRRL
LRLLLLLLRLLL
RRLLLRLLRLRR
RLLLLLRLLLRL";

    #[test]
    fn test_token_score() {
        let (board, tokens) = parse_data(P1DATA);
        assert_eq!(token_score(&board, &tokens[0], 1), 5);    
        assert_eq!(token_score(&board, &tokens[1], 2), 4);   
        assert_eq!(token_score(&board, &tokens[2], 3), 0);    
        assert_eq!(token_score(&board, &tokens[3], 4), 2);
        assert_eq!(token_score(&board, &tokens[4], 5), 3);
        assert_eq!(token_score(&board, &tokens[5], 6), 4);
        assert_eq!(token_score(&board, &tokens[6], 7), 1);
        assert_eq!(token_score(&board, &tokens[7], 8), 6);
        assert_eq!(token_score(&board, &tokens[8], 9), 1);
    }

    #[test]
    fn test_part_1() {
        let (board, tokens) = parse_data(P1DATA);
        assert_eq!(part1(&board, &tokens), 26)
    }

    const P2DATA: &str = "*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
..*.*.*.*...*.*...*.*.*..
.*...*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.......*.
*.*.*.*.*.*.*.*.*.*...*..
.*.*.*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*.*.*....
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*...*.*.
*.*.*.*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.....*.*.
*.*.*.*.*.*.*.*...*...*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*.*.*.*.*
.*...*.*.*.*...*.*.*...*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.

RRRLLRRRLLRLRRLLLRLR
RRRRRRRRRRLRRRRRLLRR
LLLLLLLLRLRRLLRRLRLL
RRRLLRRRLLRLLRLLLRRL
RLRLLLRRLRRRLRRLRRRL
LLLLLLLLRLLRRLLRLLLL
LRLLRRLRLLLLLLLRLRRL
LRLLRRLLLRRRRRLRRLRR
LRLLRRLRLLRLRRLLLRLL
RLLRRRRLRLRLRLRLLRRL";

    #[test]
    fn test_part_2() {
        let (board, tokens) = parse_data(P2DATA);
        assert_eq!(part2(&board, &tokens), 115)
    }

    
}