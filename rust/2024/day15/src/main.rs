use std::{collections::{HashMap, HashSet, VecDeque}, error::Error};

use aochelpers::{Coordinate, ScoredItem, get_everybodycodes_input};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum GardenArea {
    Empty,
    Herb(char),
    Entrance
}

#[derive(Debug,Copy,Clone,PartialEq,Eq, PartialOrd, Ord, Hash)]
struct GardenerState {
    position: Coordinate<usize>,
    seeds_collected: i32,

}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_everybodycodes_input(15, 2024, 1)?;
    let grid = parse_grid(&input);
    println!("Part 1 answer: {:?}", walk_garden(&grid).unwrap_or(0));
    
    let input = get_everybodycodes_input(15, 2024, 2)?;
    let grid = parse_grid(&input);
    println!("Part 2 answer: {:?}", walk_garden(&grid).unwrap_or(0));

    // This worked, but took 10 minutes to give an answer on an M2 Macbook Pro. Ripe for a real solution.
    let input = get_everybodycodes_input(15, 2024, 3)?;
    let grid = parse_grid(&input);
    println!("Part 3 answer: {:?}", walk_garden(&grid).unwrap_or(0));

    Ok(())
}

fn walk_garden(grid: &HashMap<Coordinate<usize>, GardenArea>) -> Option<usize> {
    let entrance = grid.keys().find(|k| k.y ==1).unwrap();
    let mut visited_states = HashSet::new();
    // Why a ScoredItem? Because I'd thought to use A* here. Just need to decide on the right heuristic...
    let starting_state = ScoredItem{cost: 0, item:GardenerState{ position: *entrance, seeds_collected: 1}};
    let mut desired_mask = 1;  // Encoding letters as a single bit in an i32
    for v in grid.values() {
        if let GardenArea::Herb(c) = v {
            desired_mask |= 2_i32.pow(c.to_digit(36).unwrap() - 9);
        }
    }
    let mut unvisited = VecDeque::new();
    unvisited.push_back(starting_state);
    while let Some(state) = unvisited.pop_front() {
        if state.item.position == *entrance && state.item.seeds_collected == desired_mask {
            return Some(state.cost);
        }
        visited_states.insert(state.item);
        let new_seeds = if let Some(GardenArea::Herb(c)) = grid.get(&state.item.position) {
            state.item.seeds_collected | 2_i32.pow(c.to_digit(36).unwrap() - 9) 
        } else {
            state.item.seeds_collected
        };
        for neighbour in state.item.position.neighbours() {
            if grid.contains_key(&neighbour) {
                let next_state = ScoredItem{cost: state.cost +1, item: GardenerState{position: neighbour, seeds_collected: new_seeds}};
                if visited_states.insert(next_state.item) {
                    unvisited.push_back(next_state);
                }
            }

            
        }
        
    }

    None
}

fn parse_grid(data: &str) -> HashMap<Coordinate<usize>, GardenArea>{

    let mut grid = HashMap::new();
    let mut herbs =Vec::new();
    for (y,line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    if y == 0 {
                        grid.insert(Coordinate{x:x+1, y: y+1}, GardenArea::Entrance);
                    } else {
                        grid.insert(Coordinate{x:x+1, y: y+1}, GardenArea::Empty);
                    }
                }
                '#' | '~' => {}
                c if c.is_ascii_uppercase() => {
                    grid.insert(Coordinate{x:x+1, y: y+1}, GardenArea::Herb(c));
                    herbs.push(Coordinate{x,y});
                }
                _ => {

                }
            }
        }
    }
    grid
}


#[cfg(test)]
mod tests {
    use super::*;

    const P1TEST: &str = "#####.#####
#.........#
#.######.##
#.........#
###.#.#####
#H.......H#
###########";

const P2TEST: &str = "##########.##########
#...................#
#.###.##.###.##.#.#.#
#..A#.#..~~~....#A#.#
#.#...#.~~~~~...#.#.#
#.#.#.#.~~~~~.#.#.#.#
#...#.#.B~~~B.#.#...#
#...#....BBB..#....##
#C............#....C#
#####################";

    #[test]
    fn test_p1() {
        let grid = parse_grid(P1TEST);
        assert_eq!(walk_garden(&grid), Some(26));

    }

    #[test]
    fn test_p2() {
        let grid = parse_grid(P2TEST);
        assert_eq!(walk_garden(&grid), Some(38));

    }
}