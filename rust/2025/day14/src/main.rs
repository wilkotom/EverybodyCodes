use std::{collections::{HashMap, HashSet}, error::Error};

use aochelpers::{Coordinate, get_everybodycodes_input};

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_everybodycodes_input(14, 2025, 1)?;
    let (grid, bounds) = gen_grid(&data);
    println!("Part 1: {}", part1(grid, bounds, 10));
    let data = get_everybodycodes_input(14, 2025, 2)?;
    let (grid, bounds) = gen_grid(&data);
    println!("Part 2: {}", part1(grid, bounds, 2025));
    let data = get_everybodycodes_input(14, 2025, 3)?;
    let (grid, _) = gen_grid(&data);
    println!("Part 3: {}", part3(&grid));
    Ok(())
}
fn part1(mut grid: HashSet<Coordinate<usize>>, bounds: Coordinate<usize>, rounds: usize) -> usize {
    let mut result = 0;
    for _ in 0..rounds {
        grid = generation(grid, &bounds);
        result += grid.len();
    }
    result
}


fn part3(sought: &HashSet<Coordinate<usize>>) -> usize {
    let mut seen_before = HashMap::new();
    let mut round = 0;
    let mut score = 0;
    let mut grid: HashSet<Coordinate<usize>> = HashSet::new();

    while round < 1000000000 {
        let board_vec = hashable_board(&grid, &Coordinate { x: 33, y: 33 });
        let mut skipped = false;
        if !skipped {
            if let Some((prev_round, prev_score)) = seen_before.get(&board_vec)  {
                    skipped = true;
                    let elapsed_time = round - prev_round;
                    let score_boost = score - prev_score;
                    while round + elapsed_time < 1000000000 {
                        round += elapsed_time;
                        score += score_boost;
                    }
            } else {
                seen_before.insert(board_vec, (round, score));
            }
        }

        if grid_match(&grid, &sought) {
            score += grid.len();
        }
        grid = generation(grid, &Coordinate { x: 33, y: 33 });
        round +=1;
        
    }
    score
}

fn grid_match(board: &HashSet<Coordinate<usize>>, sought: &HashSet<Coordinate<usize>>) -> bool {
    for x in 0..=4 {
        for y in 0..=4 {
            if (sought.contains(&Coordinate { x, y }) && !board.contains(&Coordinate { x: x + 13, y: y + 13 }) ) || 
                (!sought.contains(&Coordinate { x, y }) && board.contains(&Coordinate { x: x + 13, y: y + 13 })) {
                    return false;
                }
        }
    }

    true
}

fn hashable_board(grid: &HashSet<Coordinate<usize>>, bounds: &Coordinate<usize>) -> Vec<usize> {
    let mut rows = Vec::new();
    for y in 0..=bounds.y /2 {
        let mut row_sum = 0;
        for x in 0..=bounds.x /2{
            row_sum <<= 1;
            if grid.contains(&Coordinate { x, y }) {
                row_sum |= 1;
            }
        }
        rows.push(row_sum);
    }
    rows
}

fn generation(grid: HashSet<Coordinate<usize>>, bounds: &Coordinate<usize>) -> HashSet<Coordinate<usize>>{
    let mut next_grid = HashSet::new();
    for x in 0..=bounds.x {
        for y in 0..=bounds.y {
            if grid.contains(&Coordinate{x,y}) {
                if (Coordinate{x,y}).checked_extended_neighbours().filter(|c| c.x != x && c.y !=y && grid.contains(c)).count() % 2 == 1 {
                    next_grid.insert(Coordinate { x, y });
                }
            } else {
                if (Coordinate{x,y}).checked_extended_neighbours().filter(|c| c.x != x && c.y !=y && grid.contains(c)).count() % 2 == 0 {
                    next_grid.insert(Coordinate { x, y });
                }
            }
        }
    }
    next_grid
}

fn gen_grid(data: &str) -> (HashSet<Coordinate<usize>>, Coordinate<usize>) {
    let mut grid = HashSet::new();
    let mut bounds = Coordinate{x:0,y:0};
    for (y,line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid.insert(Coordinate { x, y});
            }
            bounds.x = x.max(bounds.x);
        }
        bounds.y = y.max(bounds.y);
    }


    (grid, bounds)
}

#[cfg(test)] 
mod tests {
    use super::*;

    const P1TESTDATA: &str = ".#.##.
##..#.
..##.#
.#.##.
.###..
###.##";


    const P3TESTDATA: &str = "#......#
..#..#..
.##..##.
...##...
...##...
.##..##.
..#..#..
#......#";

    #[test]
    fn test_p1() {
        let (grid, bounds) = gen_grid(P1TESTDATA);
        assert_eq!(part1(grid, bounds, 10), 200);

    }
    #[test]
    fn test_board_hasher() {
        assert_eq!(hashable_board(&HashSet::new(), &Coordinate { x: 33, y: 33}), vec![0;17]);

        let (grid, bounds) = gen_grid("#······#·#··#··####··#··#·#······#");
        assert_eq!(hashable_board(&grid, &bounds), vec![66195]);

        let (grid, bounds) = gen_grid(P3TESTDATA);
        assert_eq!(hashable_board(&grid, &bounds), vec![8,2,6,1]);
    }
    #[test]
    fn test_p3() {
        let (mask, _) = gen_grid(P3TESTDATA);
        assert_eq!(part3(&mask), 278388552);
    }
}