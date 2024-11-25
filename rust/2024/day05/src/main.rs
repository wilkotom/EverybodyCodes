use std::{cmp, collections::{HashMap, VecDeque}, fs::read_to_string};
fn main() {
    let data: String = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q05_p1.txt").unwrap();
    let mut dancers = parse_input(&data);
    for i in 0..10 {
        dance_step(&mut dancers, i);
    }
    println!("{:?}", dancers);
    let data: String = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q05_p2.txt").unwrap();
    let mut dancers = parse_input(&data);

    println!("Part 2: {}", part2(&mut dancers));
    println!("{:?}", dancers);

    let data: String = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q05_p3.txt").unwrap();
    let mut dancers = parse_input(&data);

    println!("Part 2: {}", part3(&mut dancers));

}

fn part2(dancers: &mut [VecDeque<usize>]) -> usize {
    let mut seen: HashMap<usize, usize> = HashMap::new();
    let mut stage = 0;
    let mut maximum = 0;
    loop {
        let new_result = dance_step(dancers, stage);
        maximum = maximum.max(new_result);
        *seen.entry(new_result).or_default() +=1;
        if seen.get(&new_result) == Some(&2024) {
            println!("Result {} repeated for 2024th time at {}", new_result, stage);
            return (stage+1) * new_result;
        }
        stage +=1;
    }
    
}

fn part3(dancers: &mut [VecDeque<usize>]) -> usize {
    let mut seen: HashMap<usize, usize> = HashMap::new();
    let mut stage = 0;
    let mut maximum = 0;
    loop {
        let new_result = dance_step(dancers, stage);
        maximum = maximum.max(new_result);
        *seen.entry(new_result).or_default() +=1;
        stage +=1;
        // Brute force, horrible. 100* number of grid members is probably enough to find the highest
        if stage % (dancers.len() * dancers[0].len() * 100) == 0 {
            return maximum;
        }
    }
    
}


fn parse_input(data: &str) -> Vec<VecDeque<usize>> {    
    let mut dancers: Vec<VecDeque<usize>> = Vec::new();
    for line in data.split('\n') {
        for (col, num) in line.split(" ").map(|c| c.parse().unwrap_or(0)).enumerate() {
            if col == dancers.len() {
                dancers.push(VecDeque::new());
            }
            dancers[col].push_back(num);
        }
    }
    dancers
}

fn dance_step(dancers: &mut [VecDeque<usize>], step_number: usize) -> usize{

    let col =  step_number % dancers.len();
    let next_col = (step_number +1) % dancers.len();
    let dancer = dancers[col].pop_front().unwrap();
    let insertion_point = (dancer+ (dancers[next_col].len() *2) -1 ) % (dancers[next_col].len() *2);

    match insertion_point.cmp(&dancers[next_col].len()) {
        cmp::Ordering::Less => {
            dancers[next_col].insert(insertion_point, dancer);
        }
        cmp::Ordering::Equal => {
            dancers[next_col].push_back(insertion_point);
        },
        cmp::Ordering::Greater => {
            dancers[next_col].insert((dancers[next_col].len() *2) - insertion_point, dancer);
        },
    
    }
    let mut total = 0;
    for col in dancers.iter() {
        let mut shift = 1;
        total *= shift;
        while ((total * shift) + col[0]) % shift != col[0] {
            shift *= 10;
        }
        total *= shift;
        total += col[0]; 
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_P1: &str = "2 3 4 5
3 4 5 2
4 5 2 3
5 2 3 4";

    const DATA_P2 : &str = "2 3 4 5
6 7 8 9";

    #[test]
    fn test_parser(){
        let mut dancers = parse_input(DATA_P1);
        let expected_results = [3345,3245,3255,3252,4252,4452,4422,4423,2423,2323];
        for (step, expected) in expected_results.iter().enumerate() {
            let res = dance_step(&mut dancers, step);
            assert_eq!(res, *expected);
        }
    }

    #[test]
    fn test_part2(){
        let mut dancers = parse_input(DATA_P2);
        assert_eq!(part2(&mut dancers), 50877075);
    }
}