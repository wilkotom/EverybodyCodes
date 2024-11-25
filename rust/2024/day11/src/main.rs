use std::{fs::read_to_string, collections::HashMap};
use aochelpers::Label;

fn main() {
    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q11_p1.txt").unwrap();
    let data = parse_data(&input);
    println!("{}", part1("A".parse().unwrap(),4, &data));
    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q11_p2.txt").unwrap();
    let data = parse_data(&input);
    println!("{}", part1("Z".parse().unwrap(),10, &data));

    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q11_p3.txt").unwrap();
    let data = parse_data(&input);
    let results = data.keys().map(|k| part1(*k, 20, &data)).collect::<Vec<_>>();
    println!("Part 3: {}", results.iter().max().unwrap() - results.iter().min().unwrap());

}

fn parse_data(input: &str) -> HashMap<Label,HashMap<Label,usize>> {
    let mut results = HashMap::new();
    for line in input.split('\n') {
        let mut sections = line.split(':');
        let source = sections.next().unwrap_or("").parse::<Label>().unwrap();
        let mut destinations:HashMap<Label, usize> = HashMap::new();
        sections.next()
            .unwrap_or("")
            .split(',')
            .map(|v| v.parse().unwrap())
            .for_each(|v| *destinations.entry(v).or_default() +=1);
        results.insert(source, destinations);
    }

    results
}

fn part1(start_termite : Label, rounds: usize, termites: &HashMap<Label,HashMap<Label,usize>>) -> usize {
    let mut termite_counts = HashMap::from([(start_termite, 1)]);
    for _ in 0..rounds {
        let mut next_counts: HashMap<Label, usize> = HashMap::new();
        for (termite, count) in termite_counts.into_iter() {
            for (destination, multiplier) in termites.get(&termite).unwrap() {
                *next_counts.entry(*destination).or_default() += count* multiplier;
            }
        }
        termite_counts = next_counts;
    }
    termite_counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1TESTDATA: &str = "A:B,C
B:C,A
C:A";

    #[test]
    fn test_parser() {
        let a = "a".parse::<Label>().unwrap();
        let b = "b".parse::<Label>().unwrap();
        let c = "c".parse::<Label>().unwrap();
        let expected = HashMap::from([
            (a, HashMap::from([(b,1),(c,1)])),
            (b, HashMap::from([(c,1),(a,1)])),
            (c, HashMap::from([(a,1)]))
        ]);

        assert_eq!(parse_data(P1TESTDATA), expected);
    }

    #[test]
    fn test_p1() {
        let data = parse_data(P1TESTDATA);
        assert_eq!(part1("A".parse().unwrap(), 4, &data), 8);
    }
}