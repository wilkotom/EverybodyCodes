use std::{collections::{BinaryHeap, HashSet}, error::Error};

use aochelpers::{Coordinate, get_everybodycodes_input};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = get_everybodycodes_input(17, 2024, 1)?;
    let stars: Vec<Coordinate<usize>> = parse_data(&input);
    println!("Part 1 answer: {:?}", find_constellations(&stars, usize::MAX));
    let input: String = get_everybodycodes_input(17, 2024, 2)?;
    let stars = parse_data(&input);
    println!("Part 2 answer: {:?}",find_constellations(&stars, usize::MAX));
    let input: String = get_everybodycodes_input(17, 2024, 3)?;
    let stars = parse_data(&input);
    println!("Part 3 answer: {:?}", find_constellations(&stars, 6));

    Ok(())
}

fn parse_data(data: &str) -> Vec<Coordinate<usize>> {
    let  mut starfield = Vec::new();
    for (y,line) in data.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            if c == '*' {
                starfield.push(Coordinate::<usize>{x:x+1, y:y+1});
            }
        }
    }
    starfield
}


fn find_constellations(starfield: &[Coordinate<usize>], max_size: usize) -> usize {
    let mut unvisited = starfield.to_vec();
    let mut constellation_members = HashSet::new();
    let mut constellation_sizes = BinaryHeap::new();
    let mut total_distance = 0;

    //Prim's Algorithm. Find the smallest edge unconnected to the graph, and add it repeatedly.
    while !unvisited.is_empty() {
        if constellation_members.is_empty() {
            constellation_members.insert(unvisited.pop().unwrap());
            continue;
        }
        let mut closest_distance = usize::MAX;
        let mut closest_star = Coordinate{x:usize::MAX, y:usize::MAX};
        for star in constellation_members.iter() {
            for target in unvisited.iter() {
                let pairwise_distance = star.manhattan_distance(target);
                if pairwise_distance < closest_distance {
                    closest_distance = pairwise_distance;
                    closest_star = *target;
                }
            }
        }
        if closest_distance < max_size {
            constellation_members.insert(closest_star);
            total_distance += closest_distance;
            unvisited.retain(|c| *c != closest_star);
        } else {
            constellation_sizes.push(total_distance + constellation_members.len());
            total_distance = 0;
            constellation_members.drain();
        }

    }
    constellation_sizes.push(total_distance + constellation_members.len());
    
    let mut result = 1;
    for _ in 0..3 {
        result *= constellation_sizes.pop().unwrap_or(1);
    }

    result
    
}

#[cfg(test)]
mod tests {

    const P1DATA: &str = "*...*
..*..
.....
.....
*.*..";

    const P3DATA: &str = ".......................................
..*.......*...*.....*...*......**.**...
....*.................*.......*..*..*..
..*.........*.......*...*.....*.....*..
......................*........*...*...
..*.*.....*...*.....*...*........*.....
.......................................";

    use super::*;


    #[test]
    fn test_part1() {
        let stars = parse_data(P1DATA);
        assert_eq!(find_constellations(&stars, usize::MAX), 16);
    }

    #[test]
    fn test_part3() {
        let stars = parse_data(P3DATA);
        assert_eq!(find_constellations(&stars, 6), 15624);
    }
}