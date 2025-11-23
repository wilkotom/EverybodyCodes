use aochelpers::{Coordinate, get_everybodycodes_input};
use std::{collections::{HashMap, HashSet}, error::Error};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd,Ord)]
struct GameState {
    sheep: Vec<Coordinate<i8>>,
    dragon: Coordinate<i8>
}

fn main()-> Result<(), Box<dyn Error>> {
    let p1data: String = get_everybodycodes_input(10, 2025, 1)?;
    let (sheep, _, dragon, _) = parse_data(&p1data);
    println!("Part 1: {}", part1(&sheep.iter().map(|s| *s).collect::<HashSet<_>>(), dragon, 4));

    let p2data: String = get_everybodycodes_input(10, 2025, 2)?;
    let (sheep, hideouts, dragon, _) = parse_data(&p2data);
    println!("Part 1: {}", part2(sheep.iter().map(|s| *s).collect::<HashSet<_>>(), hideouts, dragon, 20));


    let p3data: String = get_everybodycodes_input(10, 2025, 3)?;
    let (sheep, hideouts, dragon, bounds) = parse_data(&p3data);
    println!("Part 3: {}", part3(sheep, &hideouts, dragon, bounds));
    Ok(())
}

fn parse_data(data: &str) -> (Vec<Coordinate<i8>>, HashSet<Coordinate<i8>>, Coordinate<i8>, Coordinate<i8>) {
    let mut dragon = Coordinate{x: 0, y: 0};
    let mut sheep = Vec::new();
    let mut hideouts = HashSet::new();
    let mut arena_size = Coordinate{x:0 as i8, y:0 as i8};
    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {},
                'S' => {sheep.push(Coordinate{x: x as i8,y: y as i8});},
                '#' => {hideouts.insert(Coordinate{x: x as i8, y: y as i8});}
                'D' => {dragon = Coordinate { x: x as i8, y: y as i8 };},
                _ => unimplemented!()
            };
            arena_size.x = arena_size.x.max(x as i8)
        }
        arena_size.y = arena_size.y.max(y as i8)
    }

    (sheep, hideouts, dragon, arena_size)
}

fn dragon_moves(loc: Coordinate<i8>) -> Vec<Coordinate<i8>> {

    vec![
        Coordinate{x: loc.x -2, y: loc.y -1},
        Coordinate{x: loc.x -2, y: loc.y +1},
        Coordinate{x: loc.x -1, y: loc.y -2},
        Coordinate{x: loc.x +1, y: loc.y -2},
        Coordinate{x: loc.x +2, y: loc.y -1},
        Coordinate{x: loc.x +2, y: loc.y +1},
        Coordinate{x: loc.x -1, y: loc.y +2},
        Coordinate{x: loc.x +1, y: loc.y +2}
    ]
}

fn part1(sheep: &HashSet<Coordinate<i8>>, dragon: Coordinate<i8>, repetitions: usize) -> usize {
    let max_x = sheep.iter().map(|c| c.x).max().unwrap_or_default();
    let max_y = sheep.iter().map(|c| c.y).max().unwrap_or_default();
    let mut dragon_squares = HashSet::from([dragon]);
    for _ in 0..repetitions {
        let mut new_squares = HashSet::new();
        for square in dragon_squares.iter() {
            for c in dragon_moves(*square) {
                if  c.x >=0 && c.x <= max_x && c.y >=0 && c.y <= max_y {
                    new_squares.insert(c);
                }
            }
        }
        dragon_squares.extend(new_squares);
    }
    sheep.intersection(&dragon_squares).collect::<Vec<_>>().len()
}

fn part2(mut sheep: HashSet<Coordinate<i8>>, hideouts: HashSet<Coordinate<i8>>, dragon: Coordinate<i8>, repetitions: usize) -> usize {
    let max_x = sheep.iter().map(|c| c.x).max().unwrap_or_default();
    let max_y = sheep.iter().map(|c| c.y).max().unwrap_or_default();
    let mut dragon_squares: HashSet<Coordinate<i8>> = HashSet::from([dragon]);
    let starting_sheep_count = sheep.len();
    for _ in 0..repetitions {
        let mut new_sheep = HashSet::new();
        let mut new_dragons = HashSet::new();
        for square in dragon_squares.iter() {
            for c in dragon_moves(*square) {
                if  c.x >=0 && c.x <= max_x && c.y >=0 && c.y <= max_y {
                    new_dragons.insert(c);
                    if sheep.contains(&c) && !hideouts.contains(&c){
                        sheep.remove(&c);
                    }
                }
            }
        }
        dragon_squares = new_dragons;
        for s in sheep.iter() {
            let s = Coordinate{x: s.x, y: s.y +1};
            if (dragon_squares.contains(&s) && hideouts.contains(&s)) || ! dragon_squares.contains(&s){
                 new_sheep.insert(s);
            }
        }
        sheep = new_sheep;
    }
    starting_sheep_count - sheep.len()
}

fn part3(sheep: Vec<Coordinate<i8>>, hideouts: &HashSet<Coordinate<i8>>, dragon: Coordinate<i8>, bounds: Coordinate<i8>) -> usize {

    let mut cache = HashMap::new();
    let starting_state = GameState{ sheep, dragon};
    let mut safe_squares = HashSet::new();
    (0..=bounds.x).map(|x| Coordinate{x, y: bounds.y+1}).for_each(|c| {safe_squares.insert(c);});
    for y in (0..=bounds.y).rev() {
        for x in 0..=bounds.x {
            if hideouts.contains(&Coordinate { x, y}) && safe_squares.contains(&Coordinate { x, y: y+1 }) {
                safe_squares.insert(Coordinate { x, y });
            }
        }
    }
    wins_from_state(starting_state, hideouts, bounds, &safe_squares, &mut cache) 
}

fn wins_from_state(state:GameState, hideouts: &HashSet<Coordinate<i8>>, bounds: Coordinate<i8>, safe_squares: &HashSet<Coordinate<i8>>, cache: &mut HashMap<GameState, usize>) -> usize{
    if let Some(precomputed) = cache.get(&state) {
        return *precomputed
    }
    if state.sheep.is_empty() {
        1
    } else if state.sheep.iter().any(|s| safe_squares.contains(s)){
        0
    } else {
        let mut next_sheep = Vec::new();
        for i in 0..state.sheep.len() {
            let candidate = state.sheep[i] + Coordinate{x:0, y:1};
            if candidate != state.dragon || hideouts.contains(&candidate){
                let mut next_configuration = state.sheep.clone();
                next_configuration[i] += Coordinate{x:0, y:1};
                next_sheep.push(next_configuration);
            }
        }
        if next_sheep.is_empty() {
            next_sheep.push(state.sheep.clone());
        }
        let mut res = 0;
        for dragon in dragon_moves(state.dragon).into_iter().filter(|c| c.x >=0 && c.x <= bounds.x && c.y >=0 && c.y <= bounds.y ) {
            for sheep in next_sheep.clone().into_iter() {
                let sheep = sheep.iter().filter(|&&s| s != dragon || hideouts.contains(&s)).map(|&s| s).collect::<Vec<_>>();
                if sheep.is_empty() {
                    res +=1 
                } else {
                    res += wins_from_state(GameState { sheep, dragon}, hideouts, bounds, safe_squares, cache)
                }
            }
        }
        cache.insert(state,res);
        res
    }
    

}

#[cfg(test)]
mod tests {

    use super::*;
    
    const P1TESTDATA: &str = "...SSS.......
.S......S.SS.
..S....S...S.
..........SS.
..SSSS...S...
.....SS..S..S
SS....D.S....
S.S..S..S....
....S.......S
.SSS..SS.....
.........S...
.......S....S
SS.....S..S..";

    const P2TESTDATA: &str = "...SSS##.....
.S#.##..S#SS.
..S.##.S#..S.
.#..#S##..SS.
..SSSS.#.S.#.
.##..SS.#S.#S
SS##.#D.S.#..
S.S..S..S###.
.##.S#.#....S
.SSS.#SS..##.
..#.##...S##.
.#...#.S#...S
SS...#.S.#S..";

    const P3TEST1: &str = "SSS
..#
#.#
#D.";

    const P3TEST2: &str = "SSS
..#
..#
.##
.D#";

    const P3TEST5: &str = "SSS.S
.....
#.#.#
.#.#.
#.D.#";


    #[test]
    fn test_p1() {
        let (sheep, _, dragon,_) = parse_data(P1TESTDATA);
        assert_eq!(part1(&sheep.iter().map(|s| *s).collect::<HashSet<_>>(), dragon, 3), 27)
    }

    #[test]
    fn test_p2() {
        let (sheep, hideouts, dragon, _) = parse_data(P2TESTDATA);
        assert_eq!(part2(sheep.iter().map(|s| *s).collect::<HashSet<_>>(), hideouts, dragon, 3), 27)
    }

    #[test]
    fn test_p3() {
        let (sheep, hideouts, dragon, bounds) = parse_data(P3TEST1);
        assert_eq!(part3(sheep, &hideouts, dragon, bounds), 15)
    }

    #[test]
    fn test_p3_ex2() {
        let (sheep, hideouts, dragon, bounds) = parse_data(P3TEST2);
        assert_eq!(part3(sheep, &hideouts, dragon, bounds), 8)
    }


    #[test]
    fn test_p3_ex5() {
        let (sheep, hideouts, dragon, bounds) = parse_data(P3TEST5);
        assert_eq!(part3(sheep, &hideouts, dragon, bounds), 13033988838)
    }
}
