use std::{collections::{BinaryHeap, HashMap, HashSet, VecDeque}, error::Error, os::macos::raw::stat};

use aochelpers::{Coordinate, ScoredItem, get_everybodycodes_input};

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_everybodycodes_input(19, 2025, 1)?;
    let walls= parse_data(&data);

    println!("{:?}", solve(&walls));
    let data = get_everybodycodes_input(19, 2025, 2)?;
    let walls= parse_data(&data);

    println!("{:?}", solve(&walls));

    let data = get_everybodycodes_input(19, 2025, 3)?;
    let walls= parse_data(&data);

    println!("{:?}", solve(&walls));


    Ok(())
}


fn parse_data(data: &str) -> HashMap<isize, Vec<(isize, isize)>>{
    let mut walls: HashMap<isize, Vec<(isize, isize)>> = HashMap::new();
    for line in data.lines(){
        let mut nums = line.split(',').map(|v| v.parse::<isize>().unwrap());

        walls.entry(nums.next().unwrap()).or_insert(Vec::new()).push((nums.next().unwrap(), nums.next().unwrap()));

    }

    walls
}
fn solve(walls: &HashMap<isize, Vec<(isize, isize)>>) -> Option<isize>{
    let &target = walls.keys().max().unwrap();
    let start_position = ScoredItem{cost: 0, item: Coordinate{x:0, y:0}};
    let mut unseen = BinaryHeap::new();
    let mut visited = HashSet::new();
    unseen.push(start_position);
    while let Some(state) = unseen.pop() {
        if visited.contains(&state.item) {
            continue;
        }
        visited.insert(state.item);
        if let Some(_) = walls.get(&state.item.x) {
            if state.item.x == target {
                return Some(state.cost);
            } 
        }
        let next_gap_x = walls.keys().filter(|&&x| x > state.item.x).min().unwrap();
        for &(gap_start, height) in walls.get(next_gap_x).unwrap() {
            for next_gap_y in gap_start..gap_start+height {
                let mut current_location = state.item;
                let vertical_distance = (next_gap_y - state.item.y).abs();
                if  next_gap_x - state.item.x < vertical_distance {
                    continue;
                }
                let mut jumps_to_point = 0;
                if next_gap_y > current_location.y {
                    jumps_to_point += next_gap_y - current_location.y;

                    current_location.x += jumps_to_point;
                    current_location.y += jumps_to_point;
                } else if next_gap_y < current_location.y {
                    current_location.x += current_location.y - next_gap_y;
                    current_location.y -= current_location.y - next_gap_y;
                }
                jumps_to_point += (next_gap_x - current_location.x) /2;

                if (next_gap_x + next_gap_y) % 2 == 0 {
                    unseen.push(ScoredItem { cost: state.cost + jumps_to_point, item: Coordinate { x: *next_gap_x, y: next_gap_y } });
                }
            }
        }
        visited.insert(state.item);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1TESTDATA: &str = "7,7,2
12,0,4
15,5,3
24,1,6
28,5,5
40,8,2";

const P2TESTDATA: &str = "7,7,2
7,1,3
12,0,4
15,5,3
24,1,6
28,5,5
40,3,3
40,8,2";

    #[test]
    fn test_parser(){
        assert_eq!(parse_data(P1TESTDATA), HashMap::from([(7,vec![(7,2)]), (12,vec![(0,4)]), (15,vec![(5,3)]), (24,vec![(1,6)]), (28,vec![(5,5)]), (40,vec![(8,2)])]))
    }


    #[test]
    fn test_part1(){
        let walls= parse_data(P1TESTDATA);
        assert_eq!(solve(&walls),Some(24));
    }


    #[test]
    fn test_part2(){
        let walls= parse_data(P2TESTDATA);
        assert_eq!(solve(&walls),Some(22));
    }
}