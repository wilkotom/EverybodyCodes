use std::{collections::{HashMap, HashSet, VecDeque}, error::Error};

use aochelpers::{Coordinate, get_everybodycodes_input};

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
    steps_taken: usize
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_everybodycodes_input(15, 2024, 1)?;
    let grid = parse_grid(&input);
    println!("Part 1 answer: {:?}", walk_garden(&grid).unwrap_or(0));
    
    let input = get_everybodycodes_input(15, 2024, 2)?;
    let grid = parse_grid(&input);
    println!("Part 2 answer: {:?}", walk_garden(&grid).unwrap_or(0));

    let input = get_everybodycodes_input(15, 2024, 3)?;
    let grid = parse_grid(&input);
    println!("Part 3 answer: {:?}", walk_garden(&grid).unwrap_or(0));

    Ok(())
}

fn walk_garden(grid: &HashMap<Coordinate<usize>, GardenArea>) -> Option<usize> {
    let entrance = grid.keys().find(|k| k.y ==1).unwrap();
    let mut visited_states = HashSet::new();
    let starting_state = GardenerState{ position: *entrance, seeds_collected: 1, steps_taken: 0};
    let mut desired_mask = 1;  // Encoding letters as a single bit in an i32
    let mut most_herbs = 0;
    for v in grid.values() {
        if let GardenArea::Herb(c) = v {
            desired_mask |= 2_i32.pow(c.to_digit(36).unwrap() - 9);
        }
    }
    let mut unvisited = VecDeque::new();
    unvisited.push_back(starting_state);
    while let Some(state) = unvisited.pop_front() {
        if state.position == *entrance && state.seeds_collected == desired_mask {
            return Some(state.steps_taken);
        }
        let new_seeds = if let Some(GardenArea::Herb(c)) = grid.get(&state.position) {
            state.seeds_collected | 2_i32.pow(c.to_digit(36).unwrap() - 9) 
        } else {
            state.seeds_collected
        };
        let herbs_collected = i32::count_ones(new_seeds);
        // prune the search space; disregard any paths which have collected two fewer herbs for the same distance travelled
        if herbs_collected + 2 < most_herbs {
            continue;
        }
        most_herbs = most_herbs.max(herbs_collected);
        for neighbour in state.position.neighbours() {
            if grid.contains_key(&neighbour) {
                let mut next_state =  GardenerState{position: neighbour, seeds_collected: new_seeds, steps_taken: 0};
                if visited_states.insert(next_state) {
                    next_state.steps_taken = state.steps_taken+1;
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