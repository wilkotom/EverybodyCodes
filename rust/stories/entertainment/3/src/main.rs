use std::{collections::{HashMap, HashSet}, error::Error, num::ParseIntError, str::FromStr};

use aochelpers::{get_everybodycodes_input, parse_number_grid, Coordinate};

#[derive(PartialEq, Debug, Eq, Clone)]
struct Die {
    id: usize,
    faces: Vec<i32>,
    seed: usize,
    pulse: usize,
    roll_count: usize,
    current_face: usize
}
impl FromStr for Die {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split_ascii_whitespace();
        let id = sections.next().unwrap_or_default().trim_end_matches(|c| c == ':').parse::<usize>()?;
        let faces = sections.next().unwrap_or_default().strip_prefix("faces=[").unwrap_or_default().strip_suffix("]").unwrap_or_default()
            .split(',').map(|c| c.parse().unwrap_or_default()).collect();
        let seed = sections.next().unwrap_or_default().strip_prefix("seed=").unwrap_or_default().parse()?;

        Ok(Self {
            id, 
            faces,
            seed, 
            pulse: seed,
            roll_count: 0,
            current_face: 0
        })
    }
}

impl Die {
    fn roll(self: &mut Self) -> i32 {
        self.roll_count +=1;
        let spin = self.roll_count * self.pulse;
        self.current_face += spin;
        self.current_face %= self.faces.len();
        self.pulse += spin;
        self.pulse %= self.seed;
        self.pulse += 1 + self.roll_count + self.seed;
        self.faces[self.current_face]
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let part1_input: String = get_everybodycodes_input(3, 2, 1)?;
    let mut dice = part1_input.lines().map(|l| l.parse::<Die>()).flatten().collect::<Vec<_>>();
    println!("Part 1: {}", part1(&mut dice));

    let part2_input: String = get_everybodycodes_input(3, 2, 2)?;
    let mut sections: std::str::Split<'_, &'static str> = part2_input.split("\n\n");
    let mut dice = sections.next().unwrap_or_default().lines().map(|l| l.parse::<Die>()).flatten().collect::<Vec<_>>();
    let squares = sections.next().unwrap_or_default().chars().map(|c| c.to_digit(10)).flatten().map(|c| c as i32).collect::<Vec<_>>();
    println!("Part 2: {:?}", part2(&mut dice, &squares));

    let part3_input: String = get_everybodycodes_input(3, 2, 3)?;
    let mut sections: std::str::Split<'_, &'static str> = part3_input.split("\n\n");
    let mut dice = sections.next().unwrap_or_default().lines().map(|l| l.parse::<Die>()).flatten().collect::<Vec<_>>();
    let board = parse_number_grid::<i32,i32>(sections.next().unwrap_or_default());
    println!("Part 3: {:?}", part3(&mut dice, &board));

    Ok(())
}

fn part1(dice: &mut Vec<Die>) -> usize {
    let mut score = 0;
    let mut rolls = 0;
    while score< 10000 {
        for die in dice.iter_mut() {
            score += die.roll()
        }
        rolls +=1;
    }
    rolls
}

fn part2(dice: &mut Vec<Die>, squares: &[i32]) -> Vec<usize> {
    let mut finishes = dice.iter_mut()
        .enumerate()
        .map(|(i, d)| (i+1, time_to_traverse(d, squares)))
        .collect::<Vec<_>>();
    finishes.sort_by(|a,b| a.1.cmp(&b.1));
    finishes.iter().map(|(i, _)| *i ).collect()
}

fn part3(dice: &mut Vec<Die>, board: &HashMap<Coordinate<i32>,i32>) -> usize{
    let mut final_squares = HashSet::new();
    for die in dice {
        let squares = fill_board(die, board);
        final_squares.extend(squares);
    }
    final_squares.len()
}

fn time_to_traverse(die: &mut Die, squares: &[i32]) -> usize {
    for square in squares {
        while die.roll() != *square {
        }
    }
    die.roll_count
}

fn fill_board(die: &mut Die, board: &HashMap<Coordinate<i32>,i32>) -> HashSet<Coordinate<i32>> {
    let mut claimed_squares = HashSet::new();
    let previous: i32 = die.roll();
    let mut current_squares = board.iter()
        .filter(|(_,v)| **v == previous)
        .map(|(k,_)| *k)
        .collect::<HashSet<_>>();
    while !current_squares.is_empty() {
        let mut next_squares = HashSet::new();
        let face = die.roll();
        for square in current_squares.iter() {
            for neighbour in square.neighbours() {
                if let Some(board_square) = board.get(&neighbour) {
                    if board_square == &face {
                        next_squares.insert(neighbour);
                    }
                }
            }
            if board.get(square).unwrap() == &face {
                next_squares.insert(*square);
            }
        }
        claimed_squares.extend(&current_squares);
        current_squares = next_squares;
    }

    claimed_squares
}

#[cfg(test)]
mod test {
    use super::*;

    const P1DATA: &str = "1: faces=[1,2,3,4,5,6] seed=7
2: faces=[-1,1,-1,1,-1] seed=13
3: faces=[9,8,7,8,9] seed=17";

    const P2DATA: &str = "1: faces=[1,2,3,4,5,6,7,8,9] seed=13
2: faces=[1,2,3,4,5,6,7,8,9] seed=29
3: faces=[1,2,3,4,5,6,7,8,9] seed=37
4: faces=[1,2,3,4,5,6,7,8,9] seed=43

51257284";

    const P3EXAMPLE1: &str = "1: faces=[1,2,3,4,5,6,7,8,9] seed=13

1523758297
4822941583
7627997892
4397697132
1799773472";

    const P3EXAMPLE2: &str = "1: faces=[1,2,3,4,5,6,7,8,9] seed=339211
2: faces=[1,2,3,4,5,6,7,8,9] seed=339517
3: faces=[1,2,3,4,5,6,7,8,9] seed=339769
4: faces=[1,2,3,4,5,6,7,8,9] seed=339049
5: faces=[1,2,3,4,5,6,7,8,9] seed=338959
6: faces=[1,2,3,4,5,6,7,8,9] seed=340111
7: faces=[1,2,3,4,5,6,7,8,9] seed=339679
8: faces=[1,2,3,4,5,6,7,8,9] seed=339121
9: faces=[1,2,3,4,5,6,7,8,9] seed=338851

94129478611916584144567479397512595367821487689499329543245932151
45326719759656232865938673559697851227323497148536117267854241288
44425936468288462848395149959678842215853561564389485413422813386
64558359733811767982282485122488769592428259771817485135798694145
17145764554656647599363636643624443394141749674594439266267914738
89687344812176758317288229174788352467288242171125512646356965953
72436836424726621961424876248346712363842529736689287535527512173
18295771348356417112646514812963612341591986162693455745689374361
56445661964557624561727322332461348422854112571195242864151143533
77537797151985578367895335725777225518396231453691496787716283477
37666899356978497489345173784484282858559847597424967325966961183
26423131974661694562195955939964966722352323745667498767153191712
99821139398463125478734415536932821142852955688669975837535594682
17768265895455681847771319336534851247125295119363323122744953158
25655579913247189643736314385964221584784477663153155222414634387
62881693835262899543396571369125158422922821541597516885389448546
71751114798332662666694134456689735288947441583123159231519473489
94932859392146885633942828174712588132581248183339538341386944937
53828883514868969493559487848248847169557825166338328352792866332
54329673374115668178556175692459528276819221245996289611868492731
97799599164121988455613343238811122469229423272696867686953891233
56249752581283778997317243845187615584225693829653495119532543712
39171354221177772498317826968247939792845866251456175433557619425
56425749216121421458547849142439211299266255482219915528173596421
48679971256541851497913572722857258171788611888347747362797259539
32676924489943265499379145361515824954991343541956993467914114579
45733396847369746189956225365375253819969643711633873473662833395
42291594527499443926636288241672629499242134451937866578992236427
47615394883193571183931424851238451485822477158595936634849167455
16742896921499963113544858716552428241241973653655714294517865841
57496921774277833341488566199458567884285639693339942468585269698
22734249697451127789698862596688824444191118289959746248348491792
28575193613471799766369217455617858422158428235521423695479745656
74234343226976999161289522983885254212712515669681365845434541257
43457237419516813368452247532764649744546181229533942414983335895";

    #[test]
    fn test_parser() {
        let parsed: Result<Die, ParseIntError> = "1: faces=[1,2,3,4,5,6] seed=7".parse::<Die>();
        assert_eq!(parsed, Ok(Die{id: 1, faces: vec![1,2,3,4,5,6], seed: 7, pulse: 7, roll_count:0, current_face: 0}));
        let parsed: Result<Die, ParseIntError> = "2: faces=[-1,1,-1,1,-1] seed=13".parse::<Die>();
        assert_eq!(parsed, Ok(Die{id: 2, faces: vec![-1,1,-1,1,-1] , seed: 13, pulse: 13, roll_count:0, current_face: 0}));
        let parsed: Result<Die, ParseIntError> = "3: faces=[9,8,7,8,9] seed=17".parse::<Die>();
        assert_eq!(parsed, Ok(Die{id: 3, faces: vec![9,8,7,8,9], seed: 17, pulse: 17, roll_count:0, current_face: 0}));
    }

    #[test]
    fn test_die_behaviour() {
        if let Ok(mut die) = "1: faces=[1,2,4,-1,5,7,9] seed=3 ".parse::<Die>() {
            for result in [-1, 9, -1, -1, 5, 4, 4, 2, 5, 2] {
                assert_eq!(die.roll(), result);
            }
        } 
    }

    #[test]
    fn test_part1() {
        let mut dice = P1DATA.lines().map(|l| l.parse::<Die>().unwrap()).collect::<Vec<_>>();
        assert_eq!(part1(&mut dice), 844);
    }

    #[test]
    fn test_part2() {
        let mut sections: std::str::Split<'_, &'static str> = P2DATA.split("\n\n");
        let mut dice = sections.next().unwrap_or_default().lines().map(|l| l.parse::<Die>()).flatten().collect::<Vec<_>>();
        let squares = sections.next().unwrap_or_default().chars().map(|c| c.to_digit(10)).flatten().map(|c| c as i32).collect::<Vec<_>>();
        assert_eq!(part2(&mut dice,&squares), vec![1,3,4,2]);
    }

    #[test]
    fn test_fill_p3_simple() {
        let mut sections: std::str::Split<'_, &'static str> = P3EXAMPLE1.split("\n\n");
        let mut dice = sections.next().unwrap_or_default().lines().map(|l| l.parse::<Die>()).flatten().collect::<Vec<_>>();
        let board = parse_number_grid::<i32,i32>(sections.next().unwrap_or_default());
        let results = fill_board(&mut dice[0], &board);
        assert_eq!(results.len(), 33);
    }

    #[test]
    fn test_test_p3() {
        let mut sections: std::str::Split<'_, &'static str> = P3EXAMPLE2.split("\n\n");
        let mut dice = sections.next().unwrap_or_default().lines().map(|l| l.parse::<Die>()).flatten().collect::<Vec<_>>();
        let board: HashMap<Coordinate<i32>, i32> = parse_number_grid::<i32,i32>(sections.next().unwrap_or_default());
        assert_eq!(part3(&mut dice, &board), 1125);
    }

}

