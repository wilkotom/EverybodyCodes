use std::{collections::{HashSet, VecDeque}, error::Error};

use aochelpers::{Coordinate, Grid, get_everybodycodes_input};


fn main() -> Result<(), Box<dyn Error>> {
    let input_data = get_everybodycodes_input(12, 2025, 1)?;
    let grid: Grid<u8> = parse_data(&input_data);
    println!("Part 1: {}", solve(&grid, Coordinate{x:0, y:0}, None, &HashSet::new()).len());
    let input_data = get_everybodycodes_input(12, 2025, 2)?;
    let grid = parse_data(&input_data);
    let bounds = grid.keys().fold(Coordinate{x:0, y:0}, |m: Coordinate<usize>,c: Coordinate<usize>| Coordinate{x: c.x.max(m.x), y: c.y.max(m.y)});
    println!("Part 2: {}", solve(&grid, Coordinate{x:0, y:0}, Some(bounds), &HashSet::new()).len());
    let input_data = get_everybodycodes_input(12, 2025, 3)?;
    let grid: Grid<u8> = parse_data(&input_data);
    println!("Part 3: {}", part3(&grid));

    Ok(())
}

fn parse_data(data: &str) -> Grid<u8> {
    let mut grid= Grid::new();

    for (y, line) in data.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate(){
            grid.insert( Coordinate{x: usize::try_from(x).ok().unwrap(),  y: usize::try_from(y).ok().unwrap()}, 
                        String::from(c).parse::<u8>().unwrap());
        }
    }
    grid
}

fn part3(grid: &Grid<u8>) -> usize {
    let mut candidates = grid.keys::<usize>().collect::<Vec<_>>();
    candidates.sort_by(|a,b| grid.get(&b).cmp(&grid.get(&a)));

    let mut already_burned: HashSet<Coordinate<usize>> = HashSet::new();
    let mut already_seen: HashSet<Coordinate<usize>> = HashSet::new();
    for _ in 0..3 {
        let mut best = HashSet::new();
        for candidate in candidates.iter() {
            if already_seen.contains(candidate) {
                continue;
            }
            let burned = solve(grid, *candidate, None, &already_burned);
            if burned.len() > best.len() {
                best = burned.clone()
            }
            already_seen.extend(burned);
        }
        already_burned.extend(best);
        already_seen.drain();
    }
    already_burned.len()
}

fn solve(grid: &Grid<u8>, start: Coordinate<usize>, second: Option<Coordinate<usize>>, already_burned: &HashSet<Coordinate<usize>>) -> HashSet<Coordinate<usize>> {
    let mut burned = HashSet::new();
    let mut to_burn = VecDeque::new();
    to_burn.push_back((start, grid.get(&start).unwrap()));
    if let Some (additional) = second{
        to_burn.push_back((additional, grid.get(&additional).unwrap()));
    }
    while let Some((barrel, size)) = to_burn.pop_front() {
        if burned.contains(&barrel) || already_burned.contains(&barrel){
            continue;
        }
        for neighbour in barrel.checked_neighbours().filter(|n| !burned.contains(n)) {

            if let Some(n_size) = grid.get(&neighbour) { 
                if  n_size <= size && ! burned.contains(&neighbour){
                    to_burn.push_back((neighbour, n_size));
            }}
        }
        burned.insert(barrel);

    }

    burned
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1TESTDATA: &str = "989601
857782
746543
766789";

    const P2TESTDATA: &str = "9589233445
9679121695
8469121876
8352919876
7342914327
7234193437
6789193538
6781219648
5691219769
5443329859";

    const P3TEST2 : &str = "41951111131882511179
32112222211518122215
31223333322115122219
31234444432147511128
91223333322176121892
61112222211166431583
14661111166111111746
11111119142122222177
41222118881233333219
71222127839122222196
56111126279711111517";
    #[test]
    fn test_p1() {
        let grid = parse_data(P1TESTDATA);
        assert_eq!(solve(&grid,Coordinate { x: 0, y: 0 }, None, &HashSet::new()).len(), 16);
    }

#[test]

    fn test_p2() {
        let grid = parse_data(P2TESTDATA);
        assert_eq!(solve(&grid, Coordinate { x: 0, y: 0 }, Some(Coordinate { x: 9, y: 9 }), &HashSet::new()).len(), 58);
    }

    #[test]
    fn test_p3() {
        let grid = parse_data(P3TEST2);
        assert_eq!(part3(&grid), 136);

    }
}