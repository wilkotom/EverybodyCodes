use std::{collections::{HashMap, HashSet}, error::Error};

use aochelpers::get_everybodycodes_input;

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = get_everybodycodes_input(7, 2025, 1)?;
    let (names, rules) = parse_data(&input_data);
    println!("Part 1: {}", part1(&names, &rules));
    let input_data = get_everybodycodes_input(7, 2025, 2)?;
    let (names, rules) = parse_data(&input_data);
    println!("Part 2: {}", part2(&names, &rules));
    let input_data = get_everybodycodes_input(7, 2025, 3)?;

    let (names, rules) = parse_data(&input_data);
    println!("Part 3: {}", part3(&names, &rules));
    Ok(())
}

fn parse_data <'a> (data:&'a str) -> (Vec<&'a str>, HashMap<char, Vec<char>>) {
    let mut lines = data.lines();
    let names = lines.next().unwrap_or_default().split(',').collect();
    let mut rules: HashMap<char, Vec<char>> = HashMap::new();
    lines.next();
    lines.for_each(|l| { rules.insert(l.chars().next().unwrap(), l[4..].chars().filter(|c| c != &',').collect()); });
    (names,rules)
}

fn part1<'a>(names: &Vec<&'a str>, rules: &HashMap<char, Vec<char>>) -> &'a str {
    *names.iter().filter(
        |name| (
            0..name.len()-1).all(
                |i| rules.get(&name.chars().nth(i).unwrap_or_default()).unwrap().contains(&name.chars().nth(i+1).unwrap()))
        
        ).next().unwrap_or(&"")

}

fn part2<'a>(names: &Vec<&'a str>, rules: &HashMap<char, Vec<char>>) -> usize {
    names.iter().enumerate().filter(
        |(_,name)| (
            0..name.len()-1).all(
                |i| rules.get(&name.chars().nth(i).unwrap_or_default()).unwrap().contains(&name.chars().nth(i+1).unwrap()))
        ).map(|(n,_) | n+1).sum::<usize>()
}

fn part3<'a>(names: &Vec<&'a str>, rules: &HashMap<char, Vec<char>>) -> usize {

    let mut valid_names = names.iter().filter(
        |name| (
            0..name.len()-1).all(
                |i| rules.get(&name.chars().nth(i).unwrap_or_default()).unwrap().contains(&name.chars().nth(i+1).unwrap()))
        ).map(|x| x.to_string()).collect::<Vec<_>>();
    
    let mut combos: HashSet<String> = HashSet::new();
    while let Some(name) = valid_names.pop() {
        if name.len() > 11 || combos.contains(&name){
            continue;
        }
        let last = name.chars().last().unwrap();
        for new_last in rules.get(&last).unwrap_or(&vec![]) {
            let mut new_name= name.clone();
            new_name.push(*new_last);
            valid_names.push(new_name)
        }
        if name.len() >= 7 {
            combos.insert(name);
        }
    }
    combos.len()
}



#[cfg(test)]

mod test {

    use super::*;

    const P1TESTDATA: &str = "Oronris,Urakris,Oroneth,Uraketh

r > a,i,o
i > p,w
n > e,r
o > n,m
k > f,r
a > k
U > r
e > t
O > r
t > h";

    const P2TESTDATA: &str = "Xanverax,Khargyth,Nexzeth,Helther,Braerex,Tirgryph,Kharverax

r > v,e,a,g,y
a > e,v,x,r
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i";

    const P3TESTDATA: &str = "Xaryt

X > a,o
a > r,t
r > y,e,a
h > a,e,v
t > h
v > e
y > p,t";

    const P3TESTDATA2: &str = "Khara,Xaryt,Noxer,Kharax

r > v,e,a,g,y
a > e,v,x,r,g
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i";

    #[test]
    fn test_parser() {

        assert_eq!(parse_data(P1TESTDATA), 
                    (vec!["Oronris","Urakris","Oroneth","Uraketh"], HashMap::from([
                        ('r', vec!['a','i','o']),
                        ('i', vec!['p','w']),
                        ('n', vec!['e','r']),
                        ('o', vec!['n','m']),
                        ('k', vec!['f','r']),
                        ('a', vec!['k']),
                        ('U', vec!['r']),
                        ('e', vec!['t']),
                        ('O', vec!['r']),
                        ('t', vec!['h'])
                        ])));

    }
    #[test]
    fn test_p1() {
        let (names, rules) = parse_data(P1TESTDATA);
        assert_eq!(part1(&names, &rules), "Oroneth")
    }

    #[test]
    fn test_p2() {
        let (names, rules) = parse_data(P2TESTDATA);
        assert_eq!(part2(&names, &rules), 23)
    }

    #[test]
    fn test_p3() {
        let (names, rules) = parse_data(P3TESTDATA);
        assert_eq!(part3(&names, &rules), 25)
    }

    #[test]
    fn test_p3_2() {
        let (names, rules) = parse_data(P3TESTDATA2);
        assert_eq!(part3(&names, &rules), 1154)
    }

}