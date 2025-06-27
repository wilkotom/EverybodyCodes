use std::{collections::{HashMap, VecDeque}, error::Error};
use aochelpers::{get_everybodycodes_input, Coordinate, Coordinate3d, Direction};

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Airspace {
    Beacon(char),
    Cold,
    Warm,
    Empty

}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
struct GliderState {
    location: Coordinate3d<isize>,
    time: usize,
    facing: Direction,
    beacon_count: usize
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_everybodycodes_input(20,2024,1)?;
    let sky: HashMap<Coordinate<isize>, Airspace> = parse_map(&input);
    println!("Part 1: {}", fly_glider(&sky, 100));

    let input = get_everybodycodes_input(20,2024,2)?;
    let sky: HashMap<Coordinate<isize>, Airspace> = parse_map(&input);
    println!("Part 2: {}", part2(&sky, usize::MAX, 10000));

    let input = get_everybodycodes_input(20,2024,3)?;
    let sky: HashMap<Coordinate<isize>, Airspace> = parse_map(&input);
    println!("Part 3: {}", part3(&sky, 384400));
    Ok(())
}


fn parse_map(data: &str) -> HashMap<Coordinate<isize>, Airspace> {
    let mut grid = HashMap::new();
    for (y,line) in data.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            match c {
                '.' => {grid.insert(Coordinate{x: x as isize,y: y as isize}, Airspace::Empty);}
                '-' => {grid.insert(Coordinate{x: x as isize,y: y as isize}, Airspace::Cold);}
                '+' => {grid.insert(Coordinate{x: x as isize,y: y as isize}, Airspace::Warm);}
                c if c.is_ascii_uppercase() => {grid.insert(Coordinate{x: x as isize,y: y as isize}, Airspace::Beacon(c));}
                _ => {}
            }
        }        
    }

    grid
}

fn fly_glider(skyscape: &HashMap<Coordinate<isize>, Airspace>, time_limit: usize) -> isize {
    let start = skyscape.iter().find(|(_,v)| **v == Airspace::Beacon('S')).map(|(k,_)| Coordinate3d{x: k.x, y: k.y, z: 1001}).unwrap();
    let mut unvisited = VecDeque::new();
    let mut best_scores = HashMap::new();
    let initial_state = GliderState{location: start, facing: Direction::South, time: 0, beacon_count: 0};
    unvisited.push_back(initial_state);
    let mut best_height = 0;

    while let Some(mut glider) = unvisited.pop_front() {
        glider.location.z += match skyscape.get(&Coordinate{x: glider.location.x, y: glider.location.y}) {
            Some(Airspace::Cold) => -2,
            Some(Airspace::Warm) => 1,
            Some(Airspace::Empty) | Some(Airspace::Beacon(_)) => -1,
            None => unimplemented!(),
        };


        if glider.location.z <= *best_scores.get(&(Coordinate{x: glider.location.x, y: glider.location.y}, glider.facing)).unwrap_or(&0) {
            continue;
        }

        best_scores.insert((Coordinate{x: glider.location.x, y: glider.location.y}, glider.facing), glider.location.z);
        if glider.time == time_limit {
            best_height = best_height.max(glider.location.z);
            continue;
        }
        let next_directions = match glider.facing {
            Direction::North => [Direction::East, Direction::North, Direction::West],
            Direction::East => [Direction::South, Direction::East, Direction::North],
            Direction::South => [Direction::West, Direction::South, Direction::East],
            Direction::West => [Direction::North, Direction::West, Direction::South],
            _ => unimplemented!()
        };

        for direction in next_directions {
            let next_square = match direction {
                Direction::North => Coordinate3d{x:glider.location.x, y: glider.location.y -1, z: glider.location.z},
                Direction::East => Coordinate3d{x:glider.location.x +1, y: glider.location.y, z: glider.location.z},
                Direction::South => Coordinate3d{x:glider.location.x, y: glider.location.y +1, z: glider.location.z},
                Direction::West => Coordinate3d{x:glider.location.x -1, y: glider.location.y, z: glider.location.z},
                _ => unimplemented!()
            };
            if skyscape.contains_key(&Coordinate{x: next_square.x, y: next_square.y}) {
                unvisited.push_back(GliderState{location: next_square, facing: direction, time: glider.time+1, beacon_count: glider.beacon_count});
            } 
        }
    }
    *best_scores.values().max().unwrap_or(&0)
    
}



fn part2(skyscape: &HashMap<Coordinate<isize>, Airspace>, time_limit: usize, target_altitude: isize) -> usize {
    let start: Coordinate3d<isize> = skyscape.iter().find(|(_,v)| **v == Airspace::Beacon('S')).map(|(k,_)| Coordinate3d{x: k.x, y: k.y, z: target_altitude+1}).unwrap();
    let mut unvisited = VecDeque::new();
    let mut best_scores = HashMap::new();
    let beacon_order = ['A', 'B', 'C', 'S'];
    let initial_state = GliderState{location: start, facing: Direction::South, time: 0, beacon_count: 0};
    unvisited.push_back(initial_state);
    let mut best_height = 0;

    while let Some(mut glider) = unvisited.pop_front() {
        glider.location.z += match skyscape.get(&Coordinate{x: glider.location.x, y: glider.location.y}) {
            Some(Airspace::Cold) => -2,
            Some(Airspace::Warm) => 1,
            Some(Airspace::Empty) | Some(Airspace::Beacon(_)) => -1,
            None => unimplemented!(),
        };
        if glider.location.z <= *best_scores.get(&(Coordinate{x: glider.location.x, y: glider.location.y}, glider.facing, glider.beacon_count)).unwrap_or(&0) {
            continue;
        }
        if skyscape.get(&Coordinate{x: glider.location.x, y: glider.location.y}).unwrap() == &Airspace::Beacon(beacon_order[glider.beacon_count]) {
            if beacon_order[glider.beacon_count] == 'S' {
                if glider.location.z >= target_altitude {
                    return glider.time;
                }
            } else {
                glider.beacon_count +=1;
            }
        }

        best_scores.insert((Coordinate{x: glider.location.x, y: glider.location.y}, glider.facing, glider.beacon_count), glider.location.z);
        if glider.time == time_limit {
            best_height = best_height.max(glider.location.z);
            continue;
        }
        let next_directions = match glider.facing {
            Direction::North => [Direction::East, Direction::North, Direction::West],
            Direction::East => [Direction::South, Direction::East, Direction::North],
            Direction::South => [Direction::West, Direction::South, Direction::East],
            Direction::West => [Direction::North, Direction::West, Direction::South],
            _ => unimplemented!()
        };

        for direction in next_directions {
            let next_square = match direction {
                Direction::North => Coordinate3d{x:glider.location.x, y: glider.location.y -1, z: glider.location.z},
                Direction::East => Coordinate3d{x:glider.location.x +1, y: glider.location.y, z: glider.location.z},
                Direction::South => Coordinate3d{x:glider.location.x, y: glider.location.y +1, z: glider.location.z},
                Direction::West => Coordinate3d{x:glider.location.x -1, y: glider.location.y, z: glider.location.z},
                _ => unimplemented!()
            };
            if skyscape.contains_key(&Coordinate{x: next_square.x, y: next_square.y}) {
                unvisited.push_back(GliderState{location: next_square, facing: direction, time: glider.time+1, beacon_count: glider.beacon_count});
            } 
        }
    }
   0
    
}

fn part3(skyscape: &HashMap<Coordinate<isize>, Airspace>, starting_altitude: isize) -> isize {
    // This solution makes assumptions:
    // That the altitude is many times the length of the map
    // There are no cold spots in the top row
    // The way through the map which involves the least loss is a straight line

    // Find the column which has the lowest altitude reduction starting at y=0
    let max_x = skyscape.keys().map(|c| c.x).max().unwrap();
    let max_y = skyscape.keys().map(|c| c.y).max().unwrap();
    let mut best_reduction = isize::MAX;
    let start: Coordinate3d<isize> = skyscape.iter().find(|(_,v)| **v == Airspace::Beacon('S')).map(|(k,_)| Coordinate3d{x: k.x, y: k.y, z: starting_altitude+1}).unwrap();
    let mut starting_column  = max_x +1;
    'outer: for x in 1..=max_x {

        let mut reduction = 0;

        for y in 0..=max_y {
            match skyscape.get(&Coordinate{x, y}) {
                None => {
                    continue 'outer;
                }
                Some(Airspace::Cold) => {reduction +=2},
                Some(Airspace::Warm) => {reduction -=1},
                Some(Airspace::Empty) | Some(Airspace::Beacon(_)) => {reduction +=1},
            };
        }
        if reduction < best_reduction {
            best_reduction = reduction;
            starting_column = x;
        } else if reduction == best_reduction && (starting_column - start.x).abs() > (x - start.x).abs() {
            starting_column = x;
        }
    }
    let mut location = Coordinate3d{x:starting_column, y: 0, z:  starting_altitude - (start.x - starting_column).abs()};

    //TODO: there's a cycle here. refactor for a detected cycle length
    while location.z > 0 {
        location.y +=1;
        match skyscape.get(&Coordinate{x: location.x, y: location.y % (max_y +1)}) {
            Some(Airspace::Cold) => {
                location.z -=2; 
            },
            Some(Airspace::Warm) => {
                location.z +=1;
            },
            Some(Airspace::Empty) | Some(Airspace::Beacon(_)) =>{
                location.z -=1;
            },      
            None => todo!(),
        }
    }
    location.y
        
}

#[cfg(test)]
mod tests {

    use super::*;

    const P1TEST: &str = "#....S....#
#.........#
#---------#
#.........#
#..+.+.+..#
#.+-.+.++.#
#.........#";

    const P2EXAMPLE1: &str = "####S####
#-.+++.-#
#.+.+.+.#
#-.+.+.-#
#A+.-.+C#
#.+-.-+.#
#.+.B.+.#
#########";

const P2EXAMPLE2: &str = "###############S###############
#+#..-.+.-++.-.+.--+.#+.#++..+#
#-+-.+-..--..-+++.+-+.#+.-+.+.#
#---.--+.--..++++++..+.-.#.-..#
#+-+.#+-.#-..+#.--.--.....-..##
#..+..-+-.-+.++..-+..+#-.--..-#
#.--.A.-#-+-.-++++....+..C-...#
#++...-..+-.+-..+#--..-.-+..-.#
#..-#-#---..+....#+#-.-.-.-+.-#
#.-+.#+++.-...+.+-.-..+-++..-.#
##-+.+--.#.++--...-+.+-#-+---.#
#.-.#+...#----...+-.++-+-.+#..#
#.---#--++#.++.+-+.#.--..-.+#+#
#+.+.+.+.#.---#+..+-..#-...---#
#-#.-+##+-#.--#-.-......-#..-##
#...+.-+..##+..+B.+.#-+-++..--#
###############################";

const P2EXAMPLE3: &str = "###############S###############
#-----------------------------#
#-------------+++-------------#
#-------------+++-------------#
#-------------+++-------------#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#--A-----------------------C--#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#--------------B--------------#
#-----------------------------#
#-----------------------------#
###############################";

const P3EXAMPLE: &str = "#......S......#
#-...+...-...+#
#.............#
#..+...-...+..#
#.............#
#-...-...+...-#
#.............#
#..#...+...+..#";

    #[test]
    fn test_part1() {
        let grid = parse_map(P1TEST);

        assert_eq!(fly_glider(&grid,100),1045);
    }


    #[test]
    fn test_part2() {
        let grid = parse_map(P2EXAMPLE1);
        assert_eq!(part2(&grid,usize::MAX, 10000),24);

        let grid = parse_map(P2EXAMPLE2);
        assert_eq!(part2(&grid,usize::MAX, 10000),78);

        let grid = parse_map(P2EXAMPLE3);
        assert_eq!(part2(&grid,usize::MAX, 10000),206);
    }

    #[test]
    fn test_part3() {
        let grid = parse_map(P3EXAMPLE);
         assert_eq!(part3(&grid,100),190);
       assert_eq!(part3(&grid,1000),1990);

       assert_eq!(part3(&grid,384400),768790);
    }
}
