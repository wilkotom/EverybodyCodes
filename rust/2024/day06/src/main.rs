use std::{collections::{HashMap, VecDeque},error::Error};
use aochelpers::{Label, get_everybodycodes_input};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum TreeNode {
    Fruit,
    Branch(Label)
}

fn main() -> Result<(), Box<dyn Error>>{

    for i in [1,2,3] {
        let data: String = get_everybodycodes_input(6, 2024, i)?;
        let tree = parse_input(&data);
        println!("Part {}: {}", i, part1(&tree, i >1));
    }

    Ok(())
}

fn parse_input(input: &str) -> HashMap<Label, Vec<TreeNode>> {
    let mut tree = HashMap::new();
    for line in input.split("\n") {
        let mut sections = line.split(":");
        let start = sections.next().unwrap().parse::<Label>().unwrap();
        let ends = sections.next()
            .unwrap()
            .split(",")
            .map(|x| if let Ok(l) = x.parse::<Label>() {
                TreeNode::Branch(l)
            } else {
                TreeNode::Fruit
            }).collect::<Vec<_>>();
        tree.insert(start, ends);
    }
    tree
}


fn part1(tree: &HashMap<Label,Vec<TreeNode>>, part2: bool) -> String {
    let mut queue = VecDeque::new();
    queue.push_back((0, vec![TreeNode::Branch("RR".parse::<Label>().unwrap())]));
    let mut paths: HashMap<usize, Vec<Vec<TreeNode>>> = HashMap::new();
    while let Some((steps, path)) = queue.pop_front() {
        let current_node = path.iter().last().unwrap();
        if current_node == &TreeNode::Fruit {
            paths.entry(path.len()).or_default().push(path);
        } else if let TreeNode::Branch(label) = current_node{
            if *label == "BUG".parse::<Label>().unwrap() || *label == "ANT".parse::<Label>().unwrap() {
                continue;
            }
            if tree.get(label).is_some() {
                for next_step in tree.get(label).unwrap() {
                    let mut next_path = path.clone();
                    next_path.push(*next_step);
                    queue.push_back((steps +1, next_path));
                }
            }
        }
    }
    let mut output = "".to_string();
    for leaf in paths.values() {
        if leaf.len() == 1 {
            for entry in leaf[0].iter() {
                if let TreeNode::Branch(label) = entry {
                    let formatted = format!("{}", label);
                    if part2 {
                        output.push(formatted.chars().next().unwrap());
                    } else {
                        output.push_str(&formatted);
                    }
                } else {
                    output.push('@');
                }
            }
        }
    }

    output.to_ascii_uppercase()
}



#[cfg(test)]
mod tests {
    use super::*;

    const DATA_P1: &str = "RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@";

    #[test]
    fn test_parser(){
        let tree = parse_input(DATA_P1);
        let expected = HashMap::from([
            ("RR".parse::<Label>().unwrap(),vec![TreeNode::Branch("A".parse::<Label>().unwrap()), TreeNode::Branch("B".parse::<Label>().unwrap()),TreeNode::Branch( "C".parse::<Label>().unwrap())]),
            ("A".parse::<Label>().unwrap(), vec![TreeNode::Branch("D".parse::<Label>().unwrap()), TreeNode::Branch("E".parse::<Label>().unwrap())]),
            ("B".parse::<Label>().unwrap(), vec![TreeNode::Branch("F".parse::<Label>().unwrap()), TreeNode::Fruit]),
            ("C".parse::<Label>().unwrap(), vec![TreeNode::Branch("G".parse::<Label>().unwrap()), TreeNode::Branch("H".parse::<Label>().unwrap())]),
            ("D".parse::<Label>().unwrap(), vec![TreeNode::Fruit]),
            ("E".parse::<Label>().unwrap(), vec![TreeNode::Fruit]),
            ("F".parse::<Label>().unwrap(), vec![TreeNode::Fruit]),
            ("G".parse::<Label>().unwrap(), vec![TreeNode::Fruit]),
            ("H".parse::<Label>().unwrap(), vec![TreeNode::Fruit]),
        ]);
        assert_eq!(tree, expected);
    }

    #[test]
    fn test_part1(){
        let tree = parse_input(DATA_P1);

        assert_eq!(part1(&tree, false), "RRB@")
    }

}