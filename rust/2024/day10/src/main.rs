use std::{fs::read_to_string, collections::HashMap};
use aochelpers::Coordinate;

fn main() {
    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q10_p1.txt").unwrap();
    let grid: HashMap<Coordinate<usize>, char> = parse_grid(&input);
    println!("Part 1: {}", part1(&grid));
    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q10_p2.txt").unwrap();
    let grids = split_grids(&input);

    println!("Part 2: {}", grids.iter().map(part1).map(|w| calculate_power(&w)).sum::<usize>());
    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q10_p3.txt").unwrap();
    let mut grid: HashMap<Coordinate<usize>, char> = parse_grid(&input);
    let mut last = 0;
    let mut current = 1;
    while last != current {
        last = current;
        current = part3(&mut grid);
    }
    println!("Part 3: {}", current);

}

fn parse_grid(input: &str) -> HashMap<Coordinate<usize>,char> {
    let mut grid = HashMap::new();
    for (y,line) in input.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(Coordinate{x,y}, c);
        }
    }
    grid
}

fn part1(grid: &HashMap<Coordinate<usize>,char>) -> String {

    let mut output: String = "".to_string();
    for y in 2..6 {
        for x in 2..6{
            for c1 in (0..grid.len()).filter_map(|y| grid.get(&Coordinate{x,y})) {
                if c1 != &'.' && (0..grid.len()).filter_map(|x| grid.get(&Coordinate{x,y})).any(|c2| c1 == c2)   {
                    output.push(*c1);
                    continue;
                }
            }
        }
    }
    output
}

fn part3(grid: &mut HashMap<Coordinate<usize>,char>) -> usize {
    let mut x_offset = 0;
    let mut y_offset = 0;
    let mut res = 0;
    let max_x = grid.keys().map(|c| c.x).max().unwrap() +1;
    let max_y = grid.keys().map(|c| c.y).max().unwrap() +1;
    while y_offset +8  <= max_y {
        while x_offset +8  <= max_x {
            add_missing_letters(grid, x_offset, y_offset);
            resolve_question_marks(grid, x_offset,y_offset);
            x_offset +=6;
        }
        y_offset +=6;
        x_offset = 0;
    }
    x_offset = 0;
    y_offset = 0;
    while y_offset +8  <= max_y {
        while x_offset +8  <= max_x {
            let word =get_runic_word(grid, x_offset, y_offset);
            if !word.contains('.') {
                res += calculate_power(&word);
            }
            x_offset +=6;
        }
        y_offset +=6;
        x_offset = 0;
    }
    res
}

fn get_runic_word(grid: &HashMap<Coordinate<usize>,char>, x_offset: usize, y_offset: usize) -> String {
    let mut output = "".to_string();
    for y in (2..6).map(|y| y + y_offset ) {
        for x in (2..6).map(|x| x + x_offset ) {
            output.push(*grid.get(&Coordinate{x,y}).unwrap());
        }
    
    }
    output
}

fn add_missing_letters(grid: &mut HashMap<Coordinate<usize>,char>, x_offset: usize, y_offset: usize)  {
    for y in (2..6).map(|y| y+y_offset) {
        for x in (2..6).map(|x| x+x_offset){
            if grid.get(&Coordinate{x,y}) == Some(&'.') {
                let mut to_insert = None;
                for c1 in ([0,1,6,7]).iter().filter_map(|dy| grid.get(&Coordinate{x, y: dy + y_offset })).copied() {
                    if [0,1,6,7].iter().filter_map(|dx| grid.get(&Coordinate{x: x_offset + dx, y }).copied()).any(|c2| c1 == c2)  {
                        to_insert = Some(c1);
                        continue;
                    }
                }
                if let Some(c) = to_insert {
                    grid.insert(Coordinate{x,y}, c);
                }
            }
        }
    }
}

fn resolve_question_marks(grid: &mut HashMap<Coordinate<usize>,char>, x_offset: usize, y_offset: usize)  {
    for y in (0..8).map(|y| y + y_offset) {
        for x in (0..8).map(|x| x + x_offset) {
            if (2..6).contains(&x) && (2..6).contains(&y) {
                continue
            }
            if grid.get(&Coordinate{x,y}).unwrap_or(&'#') == &'?' {
                if (2+x_offset..6+x_offset).contains(&x) {
                    // This is a question mark at the top or bottom
                    // Check there is only one dot in the column
                    if (2..6).filter(|dy | grid.get(&Coordinate{x, y:  y_offset + dy}) == Some(&'.')).count() == 1{
                        // get the contents of the row
                        let row = (2..6).map(|y| y + y_offset).find(|y | grid.get(&Coordinate{x, y:*y }) == Some(&'.')).unwrap();
                        let mut counts: HashMap<&char, usize> = HashMap::new();
                        (0..8).filter_map(|dx| grid.get(&Coordinate{x:x_offset +dx, y: row })).for_each(|c| *counts.entry(c).or_default() += 1);
                        counts.remove(&'.');
                        if counts.values().filter(|v| **v==1).count() == 1 {
                            let missing = counts.keys().find(|k| counts[*k] == 1).unwrap();
                            grid.insert(Coordinate{x,y}, **missing);
                        }
                    }
                } else if (2+y_offset..6+y_offset).contains(&y) {
                    // This is a question mark at the side
                    // Check there is only one dot in the row
                    if (2..6).filter(|dx | grid.get(&Coordinate{x: x_offset +dx, y }) == Some(&'.')).count() == 1{
                        // get the contents of the column
                        let col = (2..6).map(|x| x + x_offset).find(|x | grid.get(&Coordinate{x: *x, y }) == Some(&'.')).unwrap();
                        let mut counts: HashMap<&char, usize> = HashMap::new();
                        (0..8).filter_map(|dy| grid.get(&Coordinate{x:col, y: y_offset + dy })).for_each(|c| *counts.entry(c).or_default() += 1);
                        counts.remove(&'.');
                        if counts.values().filter(|v| **v==1).count() == 1 {
                            let missing = counts.keys().find(|k| counts[*k] == 1).unwrap();
                            grid.insert(Coordinate{x,y}, **missing);
                        }
                    }
                }
            }
        }
    }
    add_missing_letters(grid, x_offset, y_offset);
}

fn split_grids(input: &str) -> Vec<HashMap<Coordinate<usize>,char>> {
    let mut result = Vec::new();
    let lines = input.split('\n').collect::<Vec<_>>();
    let mut y_offset = 0;
    while y_offset < lines.len() {
        let row = &lines[y_offset..y_offset+8];
        let mut x_offset = 0;
        while x_offset < lines[0].len() {
            let mut grid: String = "".to_string();
            for line in row {
                grid += &line[x_offset..x_offset+8];
                grid.push('\n');
            }
            result.push(parse_grid(&grid));
            x_offset+=9;
        }
        y_offset +=9;
    }

    result
}


fn calculate_power(runic_word: &str) -> usize {
    runic_word.chars().enumerate().map(|(n, c)| (n+1) * (c.to_digit(36).unwrap() -9) as usize ).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1DATA: &str = "**PCBS**
**RLNW**
BV....PT
CR....HZ
FL....JW
SG....MN
**FTZV**
**GMJH**
";

    const P3DATA: &str = "**XFZB**DCST**
**LWQK**GQJH**
?G....WL....DQ
BS....H?....CN
P?....KJ....TV
NM....Z?....SG
**NSHM**VKWZ**
**PJGV**XFNL**
WQ....?L....YS
FX....DJ....HV
?Y....WM....?J
TJ....YK....LP
**XRTK**BMSP**
**DWZN**GCJV**";

    #[test]
    fn test_p1() {
        let grid = parse_grid(P1DATA);
        assert_eq!(&part1(&grid), "PTBVRCZHFLJWGMNS")
    }

    #[test]
    fn test_p2() {
        assert_eq!(calculate_power("PTBVRCZHFLJWGMNS"), 1851)
    }

    #[test]
    fn test_p3() {
        let mut grid = parse_grid(P3DATA);
        assert_eq!(part3(&mut grid), 3889);
    }


}