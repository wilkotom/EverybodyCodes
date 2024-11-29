use std::{collections::{HashMap, HashSet, VecDeque}, error::Error};

use aochelpers::{Coordinate, get_everybodycodes_input};

#[derive(Debug,Clone, Copy, PartialEq, Eq)]
enum GridTile {
    Empty,
    Tree
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = get_everybodycodes_input(18, 2024, 1)?;
    let grid = parse_data(&input);
    println!("Part 1: {}", part1(&grid,false));
    let input: String = get_everybodycodes_input(18, 2024, 2)?;
    let grid = parse_data(&input);
    println!("Part 1: {}", part1(&grid,true));
    let input: String = get_everybodycodes_input(18, 2024, 3)?;
    let grid = parse_data(&input);
    println!("Part 1: {}", part3(&grid));
    Ok(())
}

fn parse_data(input: &str) -> HashMap<Coordinate<usize>, GridTile> {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            match c {
                '.' => {grid.insert(Coordinate{x:x+1, y:y+1}, GridTile::Empty);},
                'P' => {grid.insert(Coordinate{x:x+1, y:y+1}, GridTile::Tree);},
                _ => {}
            }
        }
    }
    grid
}

fn part1(grid: &HashMap<Coordinate<usize>, GridTile>, part2: bool) -> usize {
    let tree_count = grid.values().filter(|v| **v == GridTile::Tree).count();
    let start = grid.keys().find(|k| k.x ==1).unwrap();
    let mut unvisited = VecDeque::new();
    let mut visited = HashSet::new();
    let mut trees_seen = 0;
    unvisited.push_back((*start, 0));
    if part2 {
        let max_x = grid.keys().map(|c|c.x).max().unwrap();
        let start2 = grid.keys().find(|k: &&Coordinate<usize>| k.x ==max_x).unwrap();
        unvisited.push_back((*start2, 0));
    }
    while let Some((location, steps)) = unvisited.pop_front() {
        if visited.contains(&location) {
            continue;
        }
        visited.insert(location);
        trees_seen += if grid.get(&location) == Some(&GridTile::Tree) {1} else {0};
        if trees_seen == tree_count {
            return steps;
        }
        for neighbour in location.neighbours() {
            if grid.contains_key(&neighbour) && !visited.contains(&neighbour){
                unvisited.push_back((neighbour, steps+1));
            }
        }
    }
    0
}

fn part3(grid: &HashMap<Coordinate<usize>, GridTile>) -> usize {
    let tree_count = grid.values().filter(|v| **v == GridTile::Tree).count();

    let mut best_answer = usize::MAX;

    'outer: for starting_point in grid.keys().filter(|k| grid.get(k) != Some(&GridTile::Tree)) {
        let mut unvisited: VecDeque<(Coordinate<usize>, usize)> = VecDeque::new();
        let mut visited = HashSet::new();
        let mut trees_seen = 0;
        unvisited.push_back((*starting_point, 0));
        let mut total_distance = 0;
        while let Some((location, steps)) = unvisited.pop_front() {
            if visited.contains(&location) {
                continue;
            }
            visited.insert(location);
            if grid.get(&location) == Some(&GridTile::Tree) {
                trees_seen += 1;
                total_distance += steps;
                if total_distance > best_answer {
                    continue 'outer;
                }
            
                if trees_seen == tree_count {
                    best_answer = total_distance.min(best_answer);
                    continue 'outer;
                }
            }
            for neighbour in location.neighbours() {
                if grid.contains_key(&neighbour) && !visited.contains(&neighbour){
                    unvisited.push_back((neighbour, steps+1));
                }
            }
        }
    }
    best_answer
}


#[cfg(test)]
mod tests {
    use super::*;

    const P1DATA: &str = "##########
..#......#
#.P.####P#
#.#...P#.#
##########";

const P2DATA: &str ="#######################
...P..P...#P....#.....#
#.#######.#.#.#.#####.#
#.....#...#P#.#..P....#
#.#####.#####.#########
#...P....P.P.P.....P#.#
#.#######.#####.#.#.#.#
#...#.....#P...P#.#....
#######################";

const P3DATA: &str = "##########
#.#......#
#.P.####P#
#.#...P#.#
##########";

    #[test]
    fn test_p1() {
        let grid = parse_data(P1DATA);
        assert_eq!(part1(&grid,false),11);
    }

    #[test]
    fn test_p2() {
        let grid = parse_data(P2DATA);
        assert_eq!(part1(&grid,true),21);
    }

    #[test]
    fn test_p3() {
        let grid = parse_data(P3DATA);
        assert_eq!(part3(&grid),12);
    }
}