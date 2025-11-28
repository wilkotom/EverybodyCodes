use std::{error::Error, usize};

use aochelpers::get_everybodycodes_input;

#[derive(Debug,Clone, Copy)]
struct Branch{
    thickness: isize,
    connection: Option<usize>
}

#[derive(Debug,Clone)]
struct Plant {
    thickness: isize,
    branches: Vec<Branch>
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_everybodycodes_input(18,2025,1)?;
    let (plants, _) = parse_data(&data);
    println!("Part 1: {}", part1(&plants, &Vec::new()));

    let data = get_everybodycodes_input(18,2025,2)?;
    let (plants, masks) = parse_data(&data);
    println!("Part 2: {}", part2(&plants, &masks));

    let data = get_everybodycodes_input(18,2025,3)?;
    let (plants, masks) = parse_data(&data);
    println!("Part 3: {}", part3(&plants, &masks));

    Ok(())
}

fn part1(plants: &[Plant], mask: &[isize]) -> isize {
    let mut precalculated = vec![0];
    for (i,plant) in plants.iter().enumerate() {
        let mut total = 0;
        for branch in &plant.branches {
            if let Some(connection) = branch.connection {
                total += branch.thickness * precalculated[connection]
            } else {
                total = if mask.len() > 0 {mask[i]} else {1};
            }
        }
        precalculated.push( if total >= plant.thickness {total} else {0})
        
    }
    *precalculated.iter().last().unwrap()
}

fn part2(plants: &[Plant], masks: &[Vec<isize>]) -> isize {
    masks.iter().map(|mask| part1(plants, mask)).sum()
}


fn part3(plants: &[Plant], masks: &[Vec<isize>]) -> isize {
    let mut precalculated = vec![0];
    for plant in plants {
        let mut total = 0;
        for branch in &plant.branches {
            if let Some(connection) = branch.connection {
                if branch.thickness > 0 {
                    total += branch.thickness * precalculated[connection]
                }
            } else {
                total = 1
            }
        }
        precalculated.push( if total >= plant.thickness {total} else {0})
    }
    let best = *precalculated.iter().last().unwrap();
    masks.iter().map(|mask| part1(plants, mask)).filter(|v| v > &0).map(|v| best - v).sum()

}


fn parse_data(data: &str) -> (Vec<Plant>, Vec<Vec<isize>>) {
    let mut plants = Vec::new();
    let mut sections = data.split("\n\n\n");
    let records = sections.next().unwrap().split("\n\n");

    for record in records {
        let mut lines= record.lines();
        let header = lines.next().unwrap().trim_end_matches(|c| c == ':').split_ascii_whitespace();
        let thickness = header.last().unwrap().parse().unwrap();
        let mut branches = Vec::new();
        for branch in lines {
            if branch.contains("free") {
                branches.push(Branch{connection: None, thickness: 1});
            } else {
                let mut fields = branch.split_ascii_whitespace().skip(4);
                let connection = Some(fields.next().unwrap().parse().unwrap());
                let thickness = fields.last().unwrap().parse().unwrap();
                branches.push(Branch { thickness, connection});
            }
        }
        plants.push(Plant { thickness, branches });
    }

    let mut masks = Vec::new();
    for mask_line in sections.next().unwrap_or_default().lines() {
        let mut mask = Vec::new();
        for c in mask_line.split_ascii_whitespace() {
            mask.push(c.parse().unwrap());
        }
        masks.push(mask);
    }
    (plants, masks)
}

#[cfg(test)]
mod tests {

    use super::*;

    const P1TESTDATA: &str = "Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 17:
- branch to Plant 1 with thickness 15
- branch to Plant 2 with thickness 3

Plant 5 with thickness 24:
- branch to Plant 2 with thickness 11
- branch to Plant 3 with thickness 13

Plant 6 with thickness 15:
- branch to Plant 3 with thickness 14

Plant 7 with thickness 10:
- branch to Plant 4 with thickness 15
- branch to Plant 5 with thickness 21
- branch to Plant 6 with thickness 34";

const P2TESTDATA: &str = "Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 10:
- branch to Plant 1 with thickness -25
- branch to Plant 2 with thickness 17
- branch to Plant 3 with thickness 12

Plant 5 with thickness 14:
- branch to Plant 1 with thickness 14
- branch to Plant 2 with thickness -26
- branch to Plant 3 with thickness 15

Plant 6 with thickness 150:
- branch to Plant 4 with thickness 5
- branch to Plant 5 with thickness 6


1 0 1
0 0 1
0 1 1";

    #[test]
    fn test_p1() {
        let (data, _) = parse_data(P1TESTDATA);
        assert_eq!(part1(&data, &Vec::new()), 774);
    }

    #[test]
    fn test_p2() {
        let (plants, masks) = parse_data(P2TESTDATA);
        assert_eq!(part2(&plants, &masks), 324);
    }

}