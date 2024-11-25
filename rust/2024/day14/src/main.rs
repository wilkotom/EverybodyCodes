use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string};
use aochelpers::Coordinate3d;

fn main() {
    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q14_p1.txt").unwrap();
    let directions = parse_data(&input);
    println!("Part 1: {:?}", part2(&directions).0);
    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q14_p2.txt").unwrap();
    let directions = parse_data(&input);
    println!("Part 2: {:?}", part2(&directions).1.len());

    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q14_p3.txt").unwrap();
    let directions = parse_data(&input);
    println!("Part 3: {:?}", part3(&directions));



}

fn part2(directions: &[Vec<Coordinate3d<isize>>]) -> (isize, HashSet<Coordinate3d<isize>>){
    let mut tree: HashSet<Coordinate3d<isize>> = HashSet::new();
    let mut best_y = 0;
    for branch in directions {
        let mut current_pos = Coordinate3d{x:0, y:0, z:0};
        for step in branch {
            for _ in 0..step.x.abs() {
                current_pos.x += step.x.abs() / step.x ;
                tree.insert(current_pos);
            }
            for _ in 0..step.y.abs() {
                current_pos.y += step.y.abs() / step.y;
                tree.insert(current_pos);
            }
            for _ in 0..step.z.abs() {
                current_pos.z += step.z.abs() / step.z;
                tree.insert(current_pos);
            }
            best_y = best_y.max(current_pos.y)
        } 

    }
        (best_y, tree)
}

fn part3(branches: &[Vec<Coordinate3d<isize>>]) -> usize {
    let leaves = branches.iter().map(|branch |branch.iter().fold(Coordinate3d{x:0, y:0, z:0}, |acc, d| acc+*d)).collect::<Vec<_>>();
    let (_, tree) = part2(branches);
    let mut best = usize::MAX;
    for starting_point in tree.iter().filter(|p| p.x == 0 && p.z == 0 && tree.contains(p)) {
        let mut distances = HashMap::new();
        let mut unvisited = VecDeque::new();
        unvisited.push_back((0,*starting_point));

        while let Some((steps, loc)) = unvisited.pop_front() {
            if distances.contains_key(&loc) {
                continue;
            }
            distances.insert(loc, steps);
            for neighbour in loc.neighbours() {
                if tree.contains(&neighbour) {
                   unvisited.push_back((steps+1, neighbour)); 
                }
            }
        }
        best = best.min(leaves.iter().map(|l| distances.get(l).unwrap_or(&usize::MAX)).sum());
    }
    best
}


fn parse_data(data: &str) -> Vec<Vec<Coordinate3d<isize>>> {
    let mut tree = Vec::new();
    for line in data.lines() {
        let mut branch = Vec::new();
        for entry in line.split(',') {
            let distance = entry[1..].parse::<isize>();
            branch.push(match (entry.chars().next(), distance.clone())  {
                (Some('U'), Ok(d)) => Coordinate3d{x:0, y: d, z:0},
                (Some('D'), Ok(d)) => Coordinate3d{x:0, y: -d, z:0},
                (Some('L'), Ok(d)) => Coordinate3d{x: -d, y:0, z:0},
                (Some('R'), Ok(d)) => Coordinate3d{x: d, y:0, z:0},
                (Some('F'), Ok(d)) => Coordinate3d{x: 0, y:0, z:-d},
                (Some('B'), Ok(d)) => Coordinate3d{x:0, y:0, z: d},
                (_, Err(_)) | (None, _) | (Some(_), _) => unimplemented!()
            });
        }
        tree.push(branch);
    }
    tree
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1TEST: &str = "U5,R3,D2,L5,U4,R5,D2";
    const P2TEST: &str = "U5,R3,D2,L5,U4,R5,D2
U6,L1,D2,R3,U2,L1";
    const P2TEST2: &str = "U5,B3,D2,F5,U4,B5,D2
U6,F1,D2,B3,U2,F1";
    const P3TEST: &str = "U20,L1,B1,L2,B1,R2,L1,F1,U1
U10,F1,B1,R1,L1,B1,L1,F1,R2,U1
U30,L2,F1,R1,B1,R1,F2,U1,F1
U25,R1,L2,B1,U1,R2,F1,L2
U16,L1,B1,L1,B3,L1,B1,F1";

    #[test]
    fn test_parser() {
        assert_eq!(parse_data(P1TEST), vec![vec![
            Coordinate3d{x:0,y:5, z:0},
            Coordinate3d{x:3,y:0, z:0},
            Coordinate3d{x:0,y:-2, z:0},
            Coordinate3d{x:-5,y:0, z:0},
            Coordinate3d{x:0,y:4, z:0},
            Coordinate3d{x:5,y:0, z:0},
            Coordinate3d{x:0,y:-2, z:0}]])
    }
    #[test]
    fn test_part1(){
        let directions = parse_data(P1TEST);
        assert_eq!(part2(&directions).0, 7);
    }
    #[test]
    fn test_part2(){
        let directions = parse_data(P2TEST);
        assert_eq!(part2(&directions).1.len(), 32);
        let directions = parse_data(P2TEST2);
        assert_eq!(part2(&directions).1.len(), 32);
    }

    #[test]
    fn test_part3(){
        let directions = parse_data(P2TEST);
        assert_eq!(part3(&directions), 5);

        let directions = parse_data(P3TEST);
        assert_eq!(part3(&directions), 46);

    }
}