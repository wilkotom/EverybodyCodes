use std::{collections::{BinaryHeap, HashMap, HashSet}, error::Error, usize};

use aochelpers::{Coordinate, ScoredItem, get_everybodycodes_input};

fn main()  -> Result<(), Box<dyn Error>> {
    for part in [1,2,3] {
    let input: String = get_everybodycodes_input(13, 2024, part)?;
    let mut grid: HashMap<Coordinate<usize>, usize> = HashMap::new();
    let (start_positions, end_pos) = populate_grid(&input, &mut grid);
    
    let res = part1(&start_positions, &end_pos, &grid);

    println!("Part {}: {}", part, res);

    }

    Ok(())
}

fn part1(start_pos: &[Coordinate<usize>], end_pos: &Coordinate<usize>, grid: &HashMap::<Coordinate<usize>, usize>) -> usize {

    let mut unvisited = BinaryHeap::new();
    let mut seen_squares = HashSet::new();
    for position in start_pos {
        unvisited.push( ScoredItem{cost: 0, item: *position});
    }


    while let Some(current_square) = unvisited.pop() {
        if seen_squares.contains(&current_square.item){
            continue;
        }
        seen_squares.insert(current_square.item);
        if current_square.item == *end_pos {
            return current_square.cost
        }
        for next_square in current_square.item.neighbours() {

            if let Some(step_height) = grid.get(&next_square) {
                let current_height = grid.get(&current_square.item).unwrap();
                let mut next_cost = if current_height <= step_height {
                   step_height - current_height
                } else {
                    current_height - step_height
                };
                if next_cost > 5 {
                    next_cost = 10-next_cost;
                }
                unvisited.push(ScoredItem { cost: current_square.cost + next_cost +1, item: next_square });
            }
            
        }
    }
    
    0
}


fn populate_grid(input: &str, grid: &mut HashMap::<Coordinate<usize>, usize>) -> (Vec<Coordinate<usize>>, Coordinate<usize>) {
    let mut start_positions = Vec::new();
    let mut end_pos = Coordinate{x: usize::MAX, y: usize::MAX};
    for (y, line )in input.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start_positions.push(Coordinate{x: x +1 ,y: y +1});
                    grid.insert(Coordinate{x: x +1 ,y: y +1}, 0);
                }
                'E' => {
                    end_pos = Coordinate{x: x +1 ,y: y +1};
                    grid.insert(Coordinate{x: x +1 ,y: y +1}, 0);
                }
                '#' | ' ' => {}
                c if c.is_ascii_digit() => {
                    grid.insert(Coordinate{x: x +1 ,y: y +1}, c.to_digit(10).unwrap().try_into().unwrap());
                }
                _ => unimplemented!()
            }
        }
    }
    (start_positions,end_pos)
}


#[cfg(test)]
mod tests {
    use super::*;

    const P1DATA: &str = "#######
#6769##
S50505E
#97434#
#######";

    const P3DATA: &str = "SSSSSSSSSSS
S674345621S
S###6#4#18S
S53#6#4532S
S5450E0485S
S##7154532S
S2##314#18S
S971595#34S
SSSSSSSSSSS";

    #[test]
    fn test_parser() {
        let mut grid: HashMap<Coordinate<usize>, usize> = HashMap::new();
        let (start_positions, end_pos) = populate_grid(P1DATA, &mut grid);
        assert_eq!(start_positions, vec![Coordinate{x:1, y:3}]);
        assert_eq!(end_pos, Coordinate{x:7, y:3});

    }

    #[test]
    fn test_part1() {
        let mut grid: HashMap<Coordinate<usize>, usize> = HashMap::new();
        let (start_positions, end_pos) = populate_grid(P1DATA, &mut grid);
        let res = part1(&start_positions, &end_pos, &grid);
        assert_eq!(res, 28);
    }
    #[test]
    fn test_part3() {
        let mut grid: HashMap<Coordinate<usize>, usize> = HashMap::new();
        let (start_positions, end_pos) = populate_grid(P3DATA, &mut grid);
        let res = part1(&start_positions, &end_pos, &grid);
        assert_eq!(res, 14);
        // for pos in start_positions {
        //     println!("{} {}", pos, part1(&pos, &end_pos, &grid));
        // }
    }


}