use aochelpers::{Coordinate, Direction, Rectangle, ScoredItem, get_everybodycodes_input};
use std::{collections::{BinaryHeap, HashMap, HashSet}, error::Error};


fn main() -> Result<(), Box<dyn Error>> {
    let data = get_everybodycodes_input(15, 2025, 1)?;
    let (arena, goal) = parse_data(&data);
    println!("Part 2: {}", solve(arena, goal).unwrap_or(0));
    let data = get_everybodycodes_input(15, 2025, 2)?;
    let (arena, goal) = parse_data(&data);
    println!("Part 2: {}", solve(arena, goal).unwrap_or(0));
    let data = get_everybodycodes_input(15, 2025, 3)?;
    let (arena, goal) = parse_data(&data);
    println!("Part 3: {}", solve(arena, goal).unwrap_or(0));

    Ok(())
}

fn parse_data(data: &str) -> (Vec<Rectangle<i32>>, Coordinate<i32>) {
    let mut walls = Vec::new();
    let mut current_location = Coordinate{x:0,y:0};
    let mut facing = Direction::North;
    for instruction in data.split(",") {
        facing = match (instruction.chars().next(), facing) {
            (Some('R'), Direction::North) => Direction::East,
            (Some('L'), Direction::North) => Direction::West,
            (Some('R'), Direction::East) => Direction::South,
            (Some('L'), Direction::East) => Direction::North,
            (Some('R'), Direction::South) => Direction::West,
            (Some('L'), Direction::South) => Direction::East,
            (Some('R'), Direction::West) => Direction::North,
            (Some('L'), Direction::West) => Direction::South,
            (_,_) => unimplemented!()
        };
        let length = instruction[1..].parse().unwrap();
        let wall_end = current_location + (match facing {
            Direction::North => Coordinate { x: 0, y: -1 },
            Direction::East => Coordinate { x: 1, y: 0 },
            Direction::South => Coordinate { x: 0, y: 1 },
            Direction::West => Coordinate { x: -1, y: 0 },
            _ => unimplemented!()
        } * length);
        walls.push(Rectangle::new(current_location, wall_end));
        
        current_location = wall_end;
    }

    (walls, current_location)
}



fn solve(walls: Vec<Rectangle<i32>>, goal: Coordinate<i32>) -> Option<i32> {
    let mut x_values = HashSet::new();
    let mut y_values = HashSet::new();
    for wall in &walls {
        for i in -1..=1 {
            x_values.insert(wall.top_left.x +i);
            x_values.insert(wall.bottom_right.x +i);

            y_values.insert(wall.top_left.y +i);
            y_values.insert(wall.bottom_right.y +i);
        }
    }
    let mut x_values = x_values.into_iter().collect::<Vec<_>>();
    x_values.sort();
    let mut y_values = y_values.into_iter().collect::<Vec<_>>();
    y_values.sort();

    let x_lookup = x_values.iter().enumerate().map(|(i,v)| (*v,i as i32)).collect::<HashMap<_,_>>();
    let y_lookup = y_values.iter().enumerate().map(|(i,v)| (*v,i as i32)).collect::<HashMap<_,_>>();
    let mut grid= HashSet::new();
    for wall in &walls {
        let start_x = *x_lookup.get(&wall.top_left.x).unwrap();
        let end_x = *x_lookup.get(&wall.bottom_right.x).unwrap();
        let start_y = *y_lookup.get(&wall.top_left.y).unwrap();
        let end_y = *y_lookup.get(&wall.bottom_right.y).unwrap();
        for y in start_y..=end_y {
            for x in start_x..=end_x {
                grid.insert(Coordinate{x: x as i32,y: y as i32});
            }
        }
    }
    let start_x = x_values.iter().position(|&n| n == 0).unwrap() as i32;
    let start_y = y_values.iter().position(|&n| n == 0).unwrap() as i32;
    let starting_state = ScoredItem{cost:0, item: Coordinate{x: start_x, y: start_y}};
    let new_goal = Coordinate{x: *x_lookup.get(&goal.x).unwrap(), y: *y_lookup.get(&goal.y).unwrap()};
    grid.remove(&new_goal);

    let mut seen = HashSet::new();
    let mut unvisited = BinaryHeap::new();
    unvisited.push(starting_state);

    while let Some(state) = unvisited.pop() {
        if state.item == new_goal {
            return Some(state.cost);
        }
        if seen.contains(&state.item) {
            continue;
        }
        seen.insert(state.item);

        for neighbour in state.item.neighbours() {
            if neighbour.x >=0 && neighbour.x < x_values.len().try_into().unwrap() && neighbour.y >=0 && neighbour.y < y_values.len().try_into().unwrap() && !(grid.contains(&neighbour) && !seen.contains(&neighbour)) {
                let new_cost = state.cost + (x_values[state.item.x as usize] - x_values[neighbour.x as usize]).abs() + (y_values[state.item.y as usize] - y_values[neighbour.y as usize]).abs();
                unvisited.push(ScoredItem { cost: new_cost, item: neighbour });
            }
        }
    }
    None

}


#[cfg(test)]
mod tests {
    use super::*;

    const P1TEST1: &str = "R3,R4,L3,L4,R3,R6,R9";
    const P1TEST2: &str = "L6,L3,L6,R3,L6,L3,L3,R6,L6,R6,L6,L6,R3,L3,L3,R3,R3,L6,L6,L3";

    #[test]
    fn test_p1() {

        let (arena, goal)= parse_data(P1TEST1);
        assert_eq!(solve(arena, goal), Some(6));
        let (arena, goal)= parse_data(P1TEST2);
        assert_eq!(solve(arena, goal), Some(16));
    }

}