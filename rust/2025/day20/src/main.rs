use std::{collections::{BinaryHeap, HashMap, HashSet}, error::Error};

use aochelpers::{Coordinate, ScoredItem, get_everybodycodes_input};

#[derive(Debug,PartialEq,Eq,Clone, Copy)]
enum Triangle{
    Trampoline,
    Start,
    End,
    Blocked,
    Number(u8)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_everybodycodes_input(20, 2025, 1)?;
    let grid = parse_data(&data);
    println!("Part 1: {}", part1(&grid));
    let data = get_everybodycodes_input(20, 2025, 2)?;
    let grid = parse_data(&data);
    println!("Part 2: {:?}", solve(grid,1).unwrap());
    let data = get_everybodycodes_input(20, 2025, 3)?;
    let grid = parse_data(&data);
    println!("Part 3: {:?}", solve(grid, 3).unwrap());
    Ok(())
}

fn part1(grid: &HashMap<Coordinate<isize>,Triangle>) -> usize{
    let mut visited = HashSet::new();
    let mut answer = 0;
    for entry in grid.iter().filter(|(_,&t)| t == Triangle::Trampoline).map(|(k,_)| k) {
        answer += triangle_neighbours(entry ).iter().filter(|c| !visited.contains(c) && grid.get(&c) == Some(&Triangle::Trampoline)).count();
        visited.insert(entry);
    }
    answer
}

fn solve(grid: HashMap<Coordinate<isize>, Triangle>, rotations: usize) -> Option<usize> {
    let mut grids = Vec::new();
    grids.push(grid);
    while grids.len() < rotations {
        grids.push(rotate(&grids[grids.len() -1]));
    }

    let start = grids[0].iter().filter(|(_,&v)| v == Triangle::Start).map(|(&k,_)| k).next().unwrap();
    let mut unvisited = BinaryHeap::new();
    let mut visited = HashSet::new();
    let starting_state = ScoredItem{cost:0, item: start};
    unvisited.push(starting_state);
    while let Some(state) = unvisited.pop() {
        if visited.contains(&(state.item, state.cost % 3)) {
            continue;
        }
        if grids[state.cost % rotations].get(&state.item) == Some(&Triangle::End) {
            return Some(state.cost);
        }
        let next_cost = state.cost +1;
        for neighbour in triangle_neighbours(&state.item).iter().filter(|&n| grids[next_cost % rotations].contains_key(n) && grids[next_cost % rotations].get(&n) != Some(&Triangle::Blocked) )  {
            unvisited.push(ScoredItem { cost: next_cost, item: *neighbour });
        }
        if grids[next_cost % rotations].get(&state.item) != Some(&Triangle::Blocked) {
                unvisited.push(ScoredItem { cost: next_cost, item: state.item });
            } 
        visited.insert((state.item, state.cost %3));
    }
    None
}

fn rotate(grid: &HashMap<Coordinate<isize>, Triangle>) -> HashMap<Coordinate<isize>, Triangle> {
    let mut new_grid = HashMap::new();
    let start_point = grid.keys().fold(Coordinate{x:0, y:0}, |c, &k| if c.y > k.y {c} else {k});
    let mut start_x = start_point.x;
    let mut start_y = start_point.y;
    let mut level = 0;
    while start_y >= 0{
        let mut original = Coordinate{x: start_x, y: start_y};

        let mut new_coord = Coordinate{x: start_x - start_point.x, y:level};
        if let Some(entry) = grid.get(&original) {
            new_grid.insert(new_coord, *entry);
        }
        while original.y > 0 {
            original.y -= 1;
            new_coord.x +=1;
            if let Some(entry) = grid.get(&original) {
                new_grid.insert(new_coord, *entry);
            }
            original.x -= 1;
            new_coord.x +=1;
            if let Some(entry) = grid.get(&original) {
                new_grid.insert(new_coord, *entry);
            }
        }
        level +=1;
        start_x += 1;
        start_y -=1;
        }

    new_grid
}


fn triangle_neighbours(loc: &Coordinate<isize>) -> Vec<Coordinate<isize>> {
    loc.neighbours().iter()
        .filter(|c| (c.x >=0 && c.y >= 0 && c.y == loc.y) ||
             (c.y < loc.y && c.y %2 != c.x %2) || 
             (c.y> loc.y && c.x %2 == c.y%2 ) )
        .map(|c| *c).collect()
}

fn parse_data(data: &str) -> HashMap<Coordinate<isize>,Triangle> {
    let mut grid = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '#' => {grid.insert(Coordinate { x: x as isize, y: y as isize }, Triangle::Blocked);}
                'T' => {grid.insert(Coordinate { x: x as isize, y: y as isize }, Triangle::Trampoline);}
                'S' => {grid.insert(Coordinate { x: x as isize, y: y as isize }, Triangle::Start);}
                'E' => {grid.insert(Coordinate { x: x as isize, y: y as isize }, Triangle::End);}
                '1'..='9' => {grid.insert(Coordinate { x: x as isize, y: y as isize }, Triangle::Number(c.to_digit(10).unwrap() as u8));}
                _ => unimplemented!()
            }
        }
    }


    grid
}

#[cfg(test)]
mod tests {

    const P1TESTDATA: &str = "T#TTT###T##
.##TT#TT##.
..T###T#T..
...##TT#...
....T##....
.....#.....";

    const P2TESTDATA: &str = "TTTTTTTTTTTTTTTTT
.TTTT#T#T#TTTTTT.
..TT#TTTETT#TTT..
...TT#T#TTT#TT...
....TTT#T#TTT....
.....TTTTTT#.....
......TT#TT......
.......#TT.......
........S........";

const ROTATIONTEST: &str = "12345
.678.
..9..";

const P3TESTDATA: &str = "T####T#TTT##T##T#T#
.T#####TTTT##TTT##.
..TTTT#T###TTTT#T..
...T#TTT#ETTTT##...
....#TT##T#T##T....
.....#TT####T#.....
......T#TT#T#......
.......T#TTT.......
........TT#........
.........S.........";

    use super::*;
    #[test]
    fn test_triangle_neighbours() {
        assert_eq!(triangle_neighbours(&Coordinate { x: 1, y:1}), vec![ Coordinate { x: 0, y:1 }, Coordinate { x: 2, y: 1}, Coordinate { x: 1, y: 0} ]);
        assert_eq!(triangle_neighbours(&Coordinate { x: 2, y:1}), vec![  Coordinate { x: 1, y:1}, Coordinate { x: 3, y:1 }, Coordinate { x: 2, y: 2} ]);

    }

    #[test]
    fn test_p1() {
        let triangle = parse_data(P1TESTDATA);
        assert_eq!(part1(&triangle), 7);
    }

    #[test]
    fn test_p2() {
        let triangle = parse_data(P2TESTDATA);
        assert_eq!(solve(triangle,1), Some(32));
    }


    #[test]
    fn test_p3() {
        let triangle = parse_data(P3TESTDATA);
        assert_eq!(solve(triangle, 3), Some(23));
    }


    #[test]
    fn test_rotate() {
        let triangle = parse_data(ROTATIONTEST);
        let r1: HashMap<Coordinate<isize>, Triangle> = rotate(&triangle);
        let r2: HashMap<Coordinate<isize>, Triangle> = rotate(&r1);
        let r3: HashMap<Coordinate<isize>, Triangle> = rotate(&r2);
        assert_eq!(triangle, r3);
    }
}