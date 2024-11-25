use std::fs::read_to_string;
fn main() {
    let nails = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q04_p1.txt")
        .unwrap()
        .split('\n')
        .map(|n| n.parse().unwrap_or(0))
        .collect::<Vec<_>>();
    let target = nails.iter().min().unwrap();
    let answer = nails.iter().map(|n| n - target).sum::<isize>();
    println!("Part 1: {}", answer);

    let nails = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q04_p2.txt")
        .unwrap()
        .split('\n')
        .map(|n| n.parse().unwrap_or(0))
        .collect::<Vec<_>>();
    let target = nails.iter().min().unwrap();
    let answer = nails.iter().map(|n| n - target).sum::<isize>();
    println!("Part 2: {}", answer);

    let nails = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q04_p3.txt")
        .unwrap()
        .split('\n')
        .map(|n| n.parse::<i64>().unwrap_or(0))
        .collect::<Vec<_>>();
        println!("Part 3: {}", part3(nails));

}

fn part3(mut nails:Vec<i64>) -> i64 {
    nails.sort();
    let mut total_passed = 0;
    let mut best = i64::MAX;
    let all_heights: i64 = nails.iter().sum();
    for (pos, nail) in nails.iter().enumerate() {

        let upward_strokes = (nail * pos as i64) - total_passed;
        let downward_strokes = (all_heights - (total_passed + nail)) - ((nails.len() - (pos+1)) as i64 * nail);
        let score = upward_strokes + downward_strokes;
        best = best.min(score);
        total_passed += nail
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part3(){
        let p3_test: Vec<i64> = vec![2,4,5,6,8];
        assert_eq!(part3(p3_test), 8);
    }
}