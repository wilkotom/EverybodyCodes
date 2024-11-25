use {aochelpers::{Coordinate,Label, Direction}, std::{fs::read_to_string,collections::{HashSet,HashMap}}, itertools::Itertools};

fn main() {
    let data: String = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q07_p1.txt").unwrap();
    let racers = parse_input(&data);
    let p1_res = part1(&racers, 10);
    println!("Part 1: {}", p1_res);

    let data: String = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q07_p2.txt").unwrap();
    let racers = parse_input(&data);

    let part2_track = parse_racetrack("S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--
-                                                                     -
=                                                                     =
+                                                                     +
=                                                                     +
+                                                                     =
=                                                                     =
-                                                                     -
--==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-");
    let p2_res = part2(&racers, &part2_track, 10);
    println!("Part 2: {}", p2_res);

    let part3_track = parse_racetrack("S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-");
    let data: String = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q07_p3.txt").unwrap();
    let racers = parse_input(&data);

    let to_beat = part3(&racers.values().next().unwrap().clone(), &part3_track);

    let mut possible = 0;
    for perm in permutations(5,3,3) {

        let score = part3(&perm, &part3_track);
        if score > to_beat {
            possible +=1;
        }
    }
    println!("Part 3: {}", possible);

    // let data: String = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q07_p2.txt").unwrap();
    // let tree = parse_input(&data);
    // println!("Part 2: {}", part1(&tree, true));

    // let data: String = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q07_p3.txt").unwrap();
    // let tree = parse_input(&data);
    // println!("Part 3: {}", part1(&tree, true));
}

fn part1(riders: &HashMap<Label, Vec<i64>>, rounds: usize) -> String{
    let mut power: HashMap<Label, i64> = HashMap::new();
    let mut accumulator: HashMap<Label, i64> = HashMap::new();
    for round in 0..rounds {
        for rider in riders.keys() {

            *power.entry(*rider).or_insert(10) += riders[rider][round % riders[rider].len()];
            if *power.get(rider).unwrap() < 0 {
                power.insert(*rider, 0);
            }
            *accumulator.entry(*rider).or_default() += *power.get(rider).unwrap();
        }
    }

    accumulator.keys().sorted_by(|a, b| accumulator.get(b).cmp(&accumulator.get(a))).map(|x| x.to_string()).collect::<String>().to_ascii_uppercase()
}

fn part2(riders: &HashMap<Label, Vec<i64>>, track: &[i64], rounds: usize) -> String{
    let mut power: HashMap<Label, i64> = HashMap::new();
    let mut accumulator: HashMap<Label, i64> = HashMap::new();

    for step in 0..(rounds* track.len()) {
        for rider in riders.keys() {
            let track_power = track.get(step % track.len()).unwrap();
            if *track_power != 0 {
                *power.entry(*rider).or_insert(10) += track_power;
            } else {
                *power.entry(*rider).or_insert(10) += riders[rider][step % riders[rider].len()];
            }

            if *power.get(rider).unwrap() < 0 {
                power.insert(*rider, 0);
            }
            *accumulator.entry(*rider).or_default() += *power.get(rider).unwrap();
        }
    }

    accumulator.keys().sorted_by(|a, b| accumulator.get(b).cmp(&accumulator.get(a))).map(|x| x.to_string()).collect::<String>().to_ascii_uppercase()
}



fn part3(rider:&[i64], track: &[i64]) -> i64{
    let mut power = 0;
    let mut accumulator =0;

    for step in 0..(2024* track.len()) {
        
            let track_power = track.get(step % track.len()).unwrap();
            if *track_power != 0 {
                power += track_power;
            } else {
                power += rider[step % rider.len()];
            }

            if power < 0 {
                power = 0;
            }
            accumulator += power;
        
    }

    accumulator
}

fn permutations(up:i64, down:i64, zero: i64) -> Vec<Vec<i64>>{
    let mut perms = Vec::new();
    if up > 0 {
        for mut subperm in permutations(up -1, down, zero) {
            subperm.push(1);
            perms.push(subperm)
        }
    }
    if down > 0 {
        for mut subperm in permutations(up, down -1, zero) {
            subperm.push(-1);
            perms.push(subperm)
        }
    }
    if zero > 0 {
        for mut subperm in permutations(up, down, zero -1) {
            subperm.push(0);
            perms.push(subperm)
        }
    }
    if perms.is_empty() {
        perms.push(Vec::new());
    }
    perms
}

fn parse_input(input: &str) -> HashMap<Label, Vec<i64>> {
    let mut chariots = HashMap::new();
    for line in input.split("\n") {
        let mut sections = line.split(":");
        let chariot = sections.next().unwrap().parse::<Label>().unwrap();
        let variation = sections.next()
            .unwrap()
            .split(",")
            .map(|x| match x {
                "+" => 1,
                "-" => -1,
                "=" => 0,
                _ => unimplemented!()
            }).collect::<Vec<_>>();
        chariots.insert(chariot, variation);
    }
    chariots
}

fn parse_racetrack(racetrack: &str) -> Vec<i64> {
    let mut map = HashMap::new();
    for (y, line) in racetrack.split('\n').enumerate() {
        for (x,c ) in line.chars().enumerate() {
            if ['+', '-', '=', 'S'].contains(&c) {
                map.insert(Coordinate{x: x as i64,y: y as i64}, match c {
                    '+' => 1,
                    '-' => -1,
                    '=' | 'S' => 0,
                    _ => unimplemented!()});
                }
            }
    }
    let mut output: Vec<i64> = Vec::new();
    let mut position = Coordinate{x: 1, y:0};
    let mut direction = Direction::North;
    let mut visited = HashSet::new();
    while !visited.contains(&position) {
        visited.insert(position);
        output.push(*map.get(&position).unwrap());
        if !map.contains_key(&(position.neighbour(direction))) {
            direction = match direction {
                Direction::North => {
                    if map.contains_key(&(position.neighbour(Direction::East))) {
                        Direction::East
                    } else {
                        Direction::West
                    }
                }
                Direction::South => {
                    if map.contains_key(&(position.neighbour(Direction::West))) {
                        Direction::West
                    } else {
                        Direction::East
                    }

                }
                Direction::East => {
                    if map.contains_key(&(position.neighbour(Direction::South))) {
                        Direction::South
                    } else {
                        Direction::North
                    }
                }
                Direction::West => {
                    if map.contains_key(&(position.neighbour(Direction::North))) {
                        Direction::North
                    } else {
                        Direction::South
                    }

                }
                _ => unimplemented!()
            }
        }
        position = position.neighbour(direction);
    }

    output
}


#[cfg(test)]
mod tests {
    use super::*;

    const DATA_P1: &str = "A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+";

    const DATA_P2: &str = "A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+";

    const TRACK_TEST_P2: &str = "S+===
-   +
=+=-+";
    #[test]
    fn test_parser(){
        let riders = parse_input(DATA_P1);
        assert_eq!(riders, HashMap::from([
            ("A".parse::<Label>().unwrap(), vec![1,-1,0,0]),
            ("B".parse::<Label>().unwrap(), vec![1,0,-1,1]),
            ("C".parse::<Label>().unwrap(), vec![0,-1,1,1]),
            ("D".parse::<Label>().unwrap(), vec![0,0,0,1])
        ]))
    }
    
    #[test]
    fn test_parse_racetrack(){
        let track = parse_racetrack(TRACK_TEST_P2);
        assert_eq!(track, vec![1, 0, 0, 0, 1, 1, -1, 0, 1, 0, -1, 0]);
    }

    #[test]
    fn test_part1(){
        let riders = parse_input(DATA_P1);

        assert_eq!(part1(&riders, 10), "BDCA")
    }

    #[test]
    fn test_part2(){
        let riders = parse_input(DATA_P2);
        let track = parse_racetrack(TRACK_TEST_P2);
        assert_eq!(part2(&riders, &track, 10), "DCBA");
    }

}