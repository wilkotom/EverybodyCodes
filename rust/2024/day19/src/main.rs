use std::{collections::{HashMap, VecDeque}, error::Error};

use aochelpers::{get_everybodycodes_input, Coordinate};


enum Rotation {
    Anticlockwise,
    Clockwise
}

const UNROTATED: [Coordinate<isize>; 8] = 
                    [Coordinate{x:-1,y:-1},
                    Coordinate{x:0,y:-1},
                    Coordinate{x:1,y:-1},
                    Coordinate{x:1,y:0},
                    Coordinate{x:1,y:1},
                    Coordinate{x:0,y:1},
                    Coordinate{x:-1,y:1},
                    Coordinate{x:-1,y:0}];

fn main() -> Result<(), Box<dyn Error>> {

    let input: String = get_everybodycodes_input(19, 2024, 1)?;
    let (rotations, grid) = parse_input(&input);
    decode_display(&rotations, &grid, 1);
    println!();

    let input: String = get_everybodycodes_input(19, 2024, 2)?;
    let (rotations, grid) = parse_input(&input);
    decode_display(&rotations, &grid, 100);
    println!();


    let input: String = get_everybodycodes_input(19, 2024, 3)?;
    let (rotations, grid) = parse_input(&input);
    decode_display(&rotations, &grid, 1048576000);
    println!();
    
    Ok(())
}

fn decode_display(directions: &[Rotation], grid: &HashMap<Coordinate<isize>, char>, iterations: usize) {

    let max_x = grid.keys().map(|c| c.x).max().unwrap();
    let max_y = grid.keys().map(|c| c.y).max().unwrap();
    let mut step_came_from = grid.keys().map(|k| (*k,*k)).collect::<HashMap<Coordinate<_>,Coordinate<_>>>();
    let mut step = 0;
    for y in 1..max_y {
        for x in 1..max_x {
            let mut to_rotate = UNROTATED.iter().map(|c| *step_came_from.get(&(*c + Coordinate{x,y})).unwrap()).collect::<VecDeque<_>>();
            match directions[step % directions.len()] {
                Rotation::Clockwise => {
                    to_rotate.rotate_right(1);
                },
                Rotation::Anticlockwise => {
                    to_rotate.rotate_left(1);
                }
            }
            UNROTATED.iter().for_each(|c| { step_came_from.insert(*c + Coordinate{x,y}, to_rotate.pop_front().unwrap()); }); 
            step +=1;
        }
    }
    let step_goes_to = step_came_from.iter().map(|(k,v)| (*v,*k)).collect::<HashMap<_,_>>();
    let mut cycles = HashMap::new();
    

    for source in step_goes_to.keys() {
        let mut visited_locations: Vec<&Coordinate<isize>> = Vec::new();
        let mut next_location = step_goes_to.get(source).unwrap();
        while next_location != source {
            visited_locations.push(next_location);
            next_location = step_goes_to.get(next_location).unwrap();
        }
        visited_locations.push(source);
        cycles.insert(source, visited_locations);
    }
    let mut final_output = HashMap::new();
    for (loc, c) in grid.iter() {
        let cycle = cycles.get(&loc).unwrap();
        let final_location = cycle[(iterations-1) % cycle.len()];
        final_output.insert(*final_location, *c);
    }
    print_grid(&final_output);
}


fn parse_input(data:&str) -> (Vec<Rotation>, HashMap<Coordinate<isize>,char>) {
    let mut sections = data.split("\n\n");
    let directions = sections.next().unwrap_or("").chars().map(|c| match c { 
        'L' => Rotation::Anticlockwise,
        'R' => Rotation::Clockwise,
        _ => unimplemented!()
    }).collect::<Vec<_>>();
    
    let mut grid = HashMap::new();
    for (y, line) in sections.next().unwrap_or("").lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            grid.insert(Coordinate{x: x as isize,y: y as isize}, c);
        }
    }

    (directions,grid)
}

fn print_grid(grid: &HashMap<Coordinate<isize>, char>) {
    let max_x = grid.keys().map(|c| c.x).max().unwrap();
    let max_y = grid.keys().map(|c| c.y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x{
            print!("{}", grid.get(&Coordinate{x,y}).unwrap());
        }
        println!();
    }
}
