use std::{collections::HashSet, error::Error};
use aochelpers::{get_everybodycodes_input, Coordinate};

fn main() -> Result<(), Box<dyn Error>>{
    let p1data = get_everybodycodes_input(2, 2024, 1)?;

    let (words, inscription) = parse_data(&p1data);
    println!("Part 1: {}", find_runic_words(&words[..], inscription.first().unwrap()));

    let p2data = get_everybodycodes_input(2, 2024, 2)?;
    let (words, inscription) = parse_data(&p2data);
    println!("Part 2: {}", part2(&words[..], &inscription));
    
    let p3data = get_everybodycodes_input(2, 2024, 3)?;
    let (words, inscription) = parse_data(&p3data);
    println!("Part 3: {}", find_runic_symbols_horizontally_with_wraparound(&words[..], &inscription));

    Ok(())
}

fn part2(words: &[String], inscriptions: &[String]) -> usize {
    inscriptions.iter().map(|w| find_runic_symbols_p2(words, w)).sum()
}

fn find_runic_symbols_horizontally_with_wraparound(words: &[String], inscription: &[String]) -> usize {
    let mut matched_squares = HashSet::new();

    let reversed = words.iter().map(|w| w.chars().rev().collect::<String>()).collect::<Vec<_>>();

    for (y, line) in inscription.iter().enumerate() {
        for word in words {
            for x in 0..line.len() {
                let comparator = (0..word.len()).map(|n| line.chars().nth( (n +x) % line.len()).unwrap()).collect::<String>();
                if word == &comparator {
                    for n in 0..word.len() {
                        matched_squares.insert(Coordinate { x: (n+x) % line.len(), y });
                    }
                }
            }
        }
        for word in &reversed[..] {
            for x in 0..line.len() {
                let comparator = (0..word.len()).map(|n| line.chars().nth( (n +x) % line.len()).unwrap()).collect::<String>();
                if word == &comparator {
                    for n in 0..word.len() {
                        matched_squares.insert(Coordinate { x: (n+x) % line.len(), y });
                    }
                }
            }
        }
    }


    let mut columns = vec!["".to_string(); inscription.first().unwrap().len()];
    for line in inscription.iter() {
        for (x,c) in line.chars().enumerate() {
            columns[x].push(c);
        }
    }

    for (x, col) in columns.iter().enumerate() {
        for (y, _) in col.chars().enumerate() {
            for word in words {
                if (*word).chars().enumerate().all(|(i, wc)| Some(wc) == col.chars().nth(y +i)) {
                    for square in (0..word.len()).map(|n| Coordinate{x, y: y+n}) {
                        matched_squares.insert(square);
                    }
                }
            }

            for word in &reversed[..] {
                if (*word).chars().enumerate().all(|(i, wc)| Some(wc) == col.chars().nth(y +i)) {
                    for square in (0..word.len()).map(|n| Coordinate{x, y: y+n}) {
                        matched_squares.insert(square);
                    }

                }
            }
        }
    }


    matched_squares.len()
}


fn find_runic_words(words: &[String], inscription: &str) -> usize {
    let mut score= 0;

    for i in 0..inscription.len() {
        for word in words {
            if i + word.len() > inscription.len() {
                continue;
            }
            if inscription[i..i+word.len()] == *word {
                score +=1;
            }
        }
    }
    score
}

fn find_runic_symbols_p2(words: &[String], inscription: &str) -> usize {
    let mut runes = HashSet::new();

    let reversed = words.iter().map(|w| w.chars().rev().collect::<String>()).collect::<Vec<_>>();
    for i in 0..inscription.len() {
        for word in words {
            if i + word.len() > inscription.len() {
                continue;
            }
            if inscription[i..i+word.len()] == *word {
                for n in i..i+word.len() {
                    runes.insert(n);
                }
            }
        }
        for rev_word in reversed.iter() {
            if i + rev_word.len() > inscription.len() {
                continue;
            }
            if inscription[i..i+rev_word.len()] == *rev_word {
                for n in i..i+rev_word.len() {
                    runes.insert(n);
                }
            }

        }
    }
    runes.len()
}

fn parse_data(data: &str) -> (Vec<String>, Vec<String>) {
    let mut lines =  data.split('\n');
    let words = lines.next().unwrap().split(":").nth(1).unwrap().split(',').map(|s| s.to_string()).collect::<Vec<_>>();
    lines.next();
    let inscriptions = lines.map(|s| s.to_string()).collect::<Vec<_>>();
    (words, inscriptions)
    }

#[cfg(test)]
mod tests {
    use super::*;

    const P1DATA: &str = "WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE";

    const P2DATA: &str = "WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END";

    const P3DATA: &str = "WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL";

    #[test]
    fn test_parse(){
    let (words, inscription) = parse_data(P1DATA);
    assert_eq!(words, vec!["THE","OWE","MES","ROD","HER"]);
    assert_eq!(inscription, vec!["AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE"]);
    }

    #[test]
    fn test_part1() {
        let (words, inscription) = parse_data(P1DATA);
        assert_eq!(find_runic_words(&words,inscription.first().unwrap()), 4)
    }

    #[test]
    fn test_part2() {
        let (words, inscriptions) = parse_data(P2DATA);
        assert_eq!(find_runic_symbols_p2(&words,inscriptions.first().unwrap()), 15);
        let total_r = part2(&words, &inscriptions);
        assert_eq!(total_r, 37);
    }
    #[test]
    fn test_part3() {
        let (words, inscriptions) = parse_data(P3DATA);
        assert_eq!(find_runic_symbols_horizontally_with_wraparound(&words,&inscriptions), 10);
    }
}