use std::fs::read_to_string;

fn main() {
    let blocks = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q08_p1.txt").unwrap().parse::<i64>().unwrap();
    println!("Part 1: {}", part1(blocks));
    let priests = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q08_p2.txt").unwrap().parse::<i64>().unwrap();
    println!("Part 3: {}", part2(priests, 1111,20240000));
    let priests = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q08_p3.txt").unwrap().parse::<i64>().unwrap();
    println!("Part 3: {}", part3(priests, 10,202400000));
}

fn part1(blocks: i64) -> i64 {
    let mut n = 1;
    while n *n < blocks {
        n +=1;
    }
    let width = (2*n) -1;
    let missing = n*n - blocks;
    width * missing
}

fn part2(priests: i64, acolytes: i64, marble: i64) -> i64 {

    let mut thickness = 1;
    let mut blocks_needed = 1;
    let mut tier = 1;
    while blocks_needed < marble {
        tier +=1;
        thickness = (thickness * priests) % acolytes;
        blocks_needed += (tier *2 -1) * thickness;
    }
    (blocks_needed - marble) * (tier *2 -1) 
}


fn part3(priests: i64, acolytes: i64, platinum: i64) -> i64 {

    let mut blocks_needed = 1;
    let mut column_height = 1;
    let mut columns = vec![1];
    
    while blocks_needed < platinum {
        column_height = (priests * column_height) % acolytes + acolytes;

        columns.iter_mut().for_each(|h| *h += column_height);

        columns.push(column_height);
        blocks_needed = columns[0] + 2* columns[1..].iter().sum::<i64>();
        let blocks_to_remove = priests * (columns.len() as i64 * 2 -1) * columns[0] % acolytes + 
            (1..(columns.len() -1)).map(|x| priests * (columns.len() as i64 * 2 -1) * columns[x] % acolytes).sum::<i64>() * 2;
            blocks_needed -= blocks_to_remove;
    }
    blocks_needed -platinum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(part1(13), 21);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(3,5,50), 27);
    }

    #[test]
    fn test_p3() {
        assert_eq!(part3(2,5,160), 2);
    }
}