use std::{collections::HashSet, error::Error, fmt};

use aochelpers::{Coordinate, Direction, Rectangle, get_everybodycodes_input};

const DIRECTIONS: [Direction; 4] = [Direction::North, Direction::East, Direction::South, Direction::West];
const DIRECTIONS3: [Direction; 12] = [Direction::North, Direction::North,Direction::North,
                                Direction::East, Direction::East, Direction::East,
                                Direction::South,Direction::South,Direction::South,
                                Direction::West,Direction::West,Direction::West];

fn main() -> Result<(), Box<dyn Error>>{
    if let Ok((start, goals)) = parse_data(&get_everybodycodes_input(2, 3, 1)?) {
        println!("Part 1: {}", part1(start, goals[0]));
    }
    if let Ok((start, goals)) = parse_data(&get_everybodycodes_input(2, 3, 2)?) {
        println!("Part 2: {}", part3(start, &goals,1));
    }
    if let Ok((start, goals)) = parse_data(&get_everybodycodes_input(2, 3, 3)?) {
        println!("Part 3: {}", part3(start, &goals, 3));
    }

    Ok(())
}

fn parse_data(data: &str) -> Result<(Coordinate<i32>, Vec<Coordinate<i32>>), Box<dyn Error>> {
    let mut start = None;
    let mut bones = Vec::new();
    for (y,line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {bones.push(Coordinate { x: x as i32, y: y as i32})},
                '@' => {start = Some(Coordinate {x: x as i32, y: y as i32})},
                _ => {}
            }
        }
    }
    if start.is_some() {
        Ok((start.unwrap(),bones))
    } else {
        Err("No Start Point".into())
    }
    
}

fn part1(mut position: Coordinate<i32>, goal: Coordinate<i32>) -> i32 {

    let mut facing = 0;
    let mut visited = HashSet::new();
    let mut steps = 0;
    while position != goal {
        visited.insert(position);
        while visited.contains(&(position.neighbour(DIRECTIONS[facing % 4]))) {
            facing +=1;
        }
        position = position.neighbour(DIRECTIONS[facing % 4]);
        steps +=1;
        facing +=1;
    }

    steps

}

fn part3(mut position: Coordinate<i32>, bones: &[Coordinate<i32>], repeats: usize) -> i32 {

    let mut facing = 0;
    let mut visited= bones.iter().copied().collect::<HashSet<_>>();
    let mut steps = 0;

    // This is a pretty slow approach
    // After each step, we run a flood fill on the area surrounding the whole map
    // Any squares we can't reach are marked as filled-in.
    while ! bones.iter().all(|b| b.neighbours().all(|n| visited.contains(&n))) {
        visited.insert(position);
    
        fill_unreachable_squares(&position, &mut visited, bones);
        while visited.contains(&(position.neighbour(DIRECTIONS[(facing / repeats) % 4])))   {
            facing +=1;
            facing %= 12;
        }
        position = position.neighbour(DIRECTIONS[(facing / repeats) % 4]);
        steps +=1;
        facing +=1;
    }

    steps -1

}

fn fill_unreachable_squares(position: &Coordinate<i32>, grid: &mut HashSet<Coordinate<i32>>, bones: &[Coordinate<i32>]) {
    let min_x = grid.iter().map(|c| c.x).min().unwrap_or_default();
    let max_x = grid.iter().map(|c| c.x).max().unwrap_or_default();

    let min_y = grid.iter().map(|c| c.y).min().unwrap_or_default();
    let max_y = grid.iter().map(|c| c.y).max().unwrap_or_default();
   
    let bounding_box = Rectangle::new(Coordinate { x: min_x -1, y: min_y -1}, Coordinate { x: max_x +1, y: max_y +1});
    let mut visited = bones.iter().copied().collect::<HashSet<_>>();
    visited.insert(*position);
    // grid.iter().for_each(|c: &Coordinate<i32>| {visited.insert(*c);});
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if grid.contains(&Coordinate { x, y}) {
                visited.insert(Coordinate { x, y});
            }
        }
    }
    let mut unvisited = vec![Coordinate{x: min_x-1, y: min_y-1}];
    while let Some(loc) = unvisited.pop() {
        visited.insert(loc);
        for neighbour in loc.neighbours().filter(|n| !visited.contains(&n) && bounding_box.contains(&n)) {
            unvisited.push(neighbour);
        }
    }
    for x in min_x..=max_x {
        for y in min_y ..=max_y {
            if !visited.contains(&Coordinate { x, y}) {
                grid.insert(Coordinate { x, y});
            }
        }
    }
}



fn fill_unreachable_squares_old(position: &Coordinate<i32>, grid: &mut HashSet<Coordinate<i32>>, bones: &[Coordinate<i32>]) {
    let min_x = grid.iter().map(|c| c.x).min().unwrap_or_default() -1;
    let max_x = grid.iter().map(|c| c.x).max().unwrap_or_default() +1;

    let min_y = grid.iter().map(|c| c.y).min().unwrap_or_default() -1;
    let max_y = grid.iter().map(|c| c.y).max().unwrap_or_default() +1;
   
    let bounding_box = Rectangle::new(Coordinate { x: min_x, y: min_y }, Coordinate { x: max_x, y: max_y });
    let mut visited = bones.iter().copied().collect::<HashSet<_>>();
    visited.insert(*position);
    grid.iter().for_each(|c| {visited.insert(*c);});
    let mut unvisited = vec![Coordinate{x: min_x, y: min_y}];
    while let Some(loc) = unvisited.pop() {
        visited.insert(loc);
        for neighbour in loc.neighbours().filter(|n| !visited.contains(&n) && bounding_box.contains(&n)) {
            unvisited.push(neighbour);
        }
    }
    for x in min_x..=max_x {
        for y in min_y ..=max_y {
            if !visited.contains(&Coordinate { x, y}) {
                grid.insert(Coordinate { x, y});
            }
        }
    }
}



mod tests {
    use super ::*;

    const P1TESTDATA: &str = ".......
.......
.......
.#.@...
.......
.......
.......";

const P3TESTDATA: &str = "#..#.......#...
...#...........
...#...........
#######........
...#....#######
...#...@...#...
...#.......#...
...........#...
...........#...
#..........#...
##......#######";

    #[test]
    fn test_parser() {
        let result = parse_data(P1TESTDATA);
        assert!(result.is_ok_and(|r| r == (Coordinate{x: 3, y:3}, vec![Coordinate{x: 1, y:3}])));

        let result = parse_data("");
        assert!(result.is_err());
    }

    #[test]
    fn test_p1() {
        assert_eq!(part1(Coordinate{x: 3, y:3}, Coordinate{x: 1, y:3}), 12);
    }


    #[test]
    fn test_p2() {
        assert_eq!(part3(Coordinate{x: 3, y:3}, &[Coordinate{x: 1, y:3}],1), 47);
    }

    #[test]
    fn test_p3() {
        assert_eq!(part3(Coordinate{x: 3, y:3}, &[Coordinate{x: 1, y:3}], 3), 87);
    }

    #[test]
    fn test_p3_2() {
        let (start, bones) = parse_data(P3TESTDATA).unwrap();
        assert_eq!(part3(start,&bones, 3), 239);
    }
}