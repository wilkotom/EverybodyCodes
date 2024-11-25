use aochelpers::Coordinate;
use std::{collections::{HashMap, HashSet}, fs::read_to_string};


fn main() {
    let p1data = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q03_p1.txt").unwrap();
    let mut p1_map = create_map(&p1data);
    while dig(&mut p1_map) {
    }
    println!("Part 1: {}",p1_map.values().sum::<isize>());

    let p2data = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q03_p2.txt").unwrap();
    let mut p2_map = create_map(&p2data);
    while dig(&mut p2_map) {
    }
    println!("Part 2: {}",p2_map.values().sum::<isize>());

    let p3data = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q03_p3.txt").unwrap();
    let mut p3_map = create_map(&p3data);
    while dig_p3(&mut p3_map) {
        print_map(&p3_map);
    }
    println!("Part 3: {}",p3_map.values().sum::<isize>());
}

fn dig(excavation: &mut HashMap<Coordinate<isize>, isize>) -> bool {
    let mut modified = false;
    let mut to_dig = HashSet::new();
    for loc in excavation.keys().copied() {
        let height = excavation.get(&loc).unwrap();
        if loc.neighbours().iter().all(|n| excavation.get(n).unwrap_or(&0) == height) {
            to_dig.insert(loc);
            modified = true;
        }
    }

    for dig in to_dig {
        *excavation.get_mut(&dig).unwrap_or(&mut 0) += 1;
    }

    modified
}

fn dig_p3(excavation: &mut HashMap<Coordinate<isize>, isize>) -> bool {
    let mut modified = false;
    let mut to_dig = HashSet::new();
    for loc in excavation.keys().copied() {
        let height = excavation.get(&loc).unwrap();
        if loc.extended_neighbours().iter().all(|n| excavation.get(n).unwrap_or(&0) == height) {
            to_dig.insert(loc);
            modified = true;
        }
    }

    for dig in to_dig {
        *excavation.get_mut(&dig).unwrap_or(&mut 0) += 1;
    }

    modified
}

fn create_map(input_data: &str) -> HashMap<Coordinate<isize>, isize> {
    let mut dig_map = HashMap::new();
    for (y, line) in input_data.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                dig_map.insert(Coordinate{x: x as isize,y: y as isize}, 1);
            }
        }
    }

    dig_map
}

fn print_map(map: &HashMap<Coordinate<isize>, isize>) {
    let max_x = map.keys().map(|c| c.x).max().unwrap_or(0);
    let max_y = map.keys().map(|c| c.y).max().unwrap_or(0);

    for y in 0..=max_y {
        for x in 0..=max_x {
            match map.get(&Coordinate{x,y}) {
                None => {print!(".");}
                Some(n) => {print!("{}", n);}
            }
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "..........
..###.##..
...####...
..######..
..######..
...####...
..........
";

    #[test]
    fn test_createmap(){
        let data = "...\n.#.\n...";
        let map = create_map(data);
        assert_eq!(map, HashMap::from([(Coordinate{x:1,y:1}, 1)]));

    }

    #[test]
    fn test_part1(){
        let mut map = create_map(DATA);
        while dig(&mut map) {}
        assert_eq!(map.values().sum::<isize>(), 35);
    
    }

    #[test]
    fn test_part3(){
        let mut map = create_map(DATA);
        while dig_p3(&mut map) {}
        assert_eq!(map.values().sum::<isize>(), 29);
    
    }

}