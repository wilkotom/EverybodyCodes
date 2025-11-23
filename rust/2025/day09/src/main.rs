use std::{collections::BTreeSet, error::Error};

use aochelpers::get_everybodycodes_input;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
    enum DNAMolecule {
    A,
    C,
    G,
    T,
}

fn main() -> Result<(), Box<dyn Error>> {
    let p1data = parse_data(&get_everybodycodes_input(9, 2025, 1)?);
    println!("Part 1: {}", part1((&p1data[0],&p1data[1]), &p1data[2]));
    let p2data = parse_data(&get_everybodycodes_input(9, 2025, 2)?);
    println!("Part 1: {}", part2(&p2data));
    let p3data = parse_data(&get_everybodycodes_input(9, 2025, 3)?);
    println!("Part 3: {}", part3(&p3data));
    Ok(())
}


fn part1(parents: (&Vec<DNAMolecule>, &Vec<DNAMolecule>), child: &Vec<DNAMolecule>) -> usize {
    parents.0.iter().zip(child.iter()).filter(|s| s.0 == s.1).count() *
    parents.1.iter().zip(child.iter()).filter(|s| s.0 == s.1).count()
}

fn valid_parents(parents: (&Vec<DNAMolecule>, &Vec<DNAMolecule>), child: &Vec<DNAMolecule>) -> bool {

    child.iter().enumerate().all(|(i,m)| parents.0[i] == *m || parents.1[i] == *m)

}

fn part2(samples: &Vec<Vec<DNAMolecule>>) -> usize {
    let mut result = 0;
    for (n, sample) in samples.iter().enumerate() {
        for i in 0..samples.len() {
            if i == n {
                continue;
            }
            for j in i+1..samples.len() {
                if j == n {
                    continue;
                }
                if valid_parents((&samples[i], &samples[j]), sample) {
                    result += part1((&samples[i], &samples[j]), sample);
                }
            }
        }
    }
    result
}

fn part3(samples: &[Vec<DNAMolecule>]) -> usize {
    // Step : identify all the parents/child triples
    let mut families = Vec::new();
        for (n, sample) in samples.iter().enumerate() {
        for i in 0..samples.len() {
            if i == n {
                continue;
            }
            for j in i+1..samples.len() {
                if j == n {
                    continue;
                }
                if valid_parents((&samples[i], &samples[j]), sample) {
                    families.push(BTreeSet::from([i+1, j+1, n+1]));
                }
            }
        }
    };

    let mut biggest = BTreeSet::new();

    'outer: while let Some(grouping) = families.pop() {
        for family in families.iter_mut() {
            if !family.is_disjoint(&grouping) {
                family.extend(grouping);
                continue 'outer;
            }
        }
        if grouping.len() > biggest.len() {
            biggest = grouping;
        }
    }
    biggest.iter().sum()
}

fn parse_data(data: &str) -> Vec<Vec<DNAMolecule>> {

    data.lines().map(|l| l.split(':').last().expect("Data did not include separator").chars().map(
        |c| match c {
            'A' => DNAMolecule::A,
            'C' => DNAMolecule::C,
            'G' => DNAMolecule::G,
            'T' => DNAMolecule::T,
            _ => unimplemented!()
        }
    ).collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1TESTDATA: &str = "1:CAAGCGCTAAGTTCGCTGGATGTGTGCCCGCG
2:CTTGAATTGGGCCGTTTACCTGGTTTAACCAT
3:CTAGCGCTGAGCTGGCTGCCTGGTTGACCGCG";

    const P2TESTDATA: &str = "1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG";

    const P3TESTDATA: &str = "1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG
8:GGCGTAAAGTATGGATGCTGGCTAGGCACCCG";

    #[test]
    fn test_part1() {
        let data = parse_data(P1TESTDATA);
        assert_eq!(part1((&data[0], &data[1]), &data[2]), 414);
    }

    #[test]
    fn test_part2() {
        let data = parse_data(P2TESTDATA);
        assert_eq!(part2(&data), 1245);
    }
    
    #[test]
    fn test_parent_validity() {
        let samples = parse_data(P1TESTDATA);
        assert!(valid_parents((&samples[0], &samples[1]), &samples[2]));
        assert!(valid_parents((&samples[1], &samples[0]), &samples[2]));
        assert!(!valid_parents((&samples[0], &samples[2]), &samples[1]));
        assert!(!valid_parents((&samples[1], &samples[2]), &samples[0]));

    }

    #[test]
    fn test_p3() {

        let data = parse_data(P3TESTDATA);
        assert_eq!(part3(&data[0..6]), 12);
        assert_eq!(part3(&data), 36);
    }
}