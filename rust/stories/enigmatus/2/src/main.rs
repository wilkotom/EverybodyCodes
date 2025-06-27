use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc};
use aochelpers::get_everybodycodes_input;

#[derive(Debug,Clone, PartialEq, Eq)]
struct Tree {
    label: String,
    value: i64,
    left:  Option<Rc<RefCell<Tree>>>,
    right: Option<Rc<RefCell<Tree>>>,
}

impl Tree {
    fn add_leaf(&mut self, leaf: Rc<RefCell<Tree>>) {
        if leaf.borrow().value <= self.value {
            if let Some(left) = self.left.as_mut() {
                left.borrow_mut().add_leaf(leaf)
            } else {
                self.left = Some(leaf);
            }
        } else if let Some(right) = self.right.as_mut() {
            right.borrow_mut().add_leaf(leaf)
        } else {
            self.right = Some(leaf);
        }
    }
    fn level_readout(&self, level: usize) -> Option<String> {
        if level == 0 {
            Some(self.label.to_string())
        } else {
            match (self.left.as_ref(), self.right.as_ref()) {
                (None, None) => None,
                (Some(left), None) => left.borrow().level_readout(level-1),
                (None, Some(right)) => right.borrow().level_readout(level-1),
                (Some(left), Some(right)) => 
                if let Some(left_label) = left.borrow().level_readout(level-1) {
                    Some(left_label + &right.borrow().level_readout(level-1).unwrap_or_default())
                } else {
                    right.borrow().level_readout(level-1)
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let data: String = get_everybodycodes_input(2, 1, 1)?;
    println!("Part 1: {}", answer(&data, false));
    let data: String = get_everybodycodes_input(2, 1, 2)?;
    println!("Part 2: {}", answer(&data, false));
    let data: String = get_everybodycodes_input(2, 1, 3)?;
    println!("Part 3: {}", answer(&data, true));

    Ok(())
}

fn answer(data: &str, part3: bool) -> String {
    let (left, right) = build_trees(data, part3);
    longest_readout(&left) + &longest_readout(&right)
}

fn longest_readout(tree: &Rc<RefCell<Tree>>) -> String {
    let mut level = 0;
    let mut maybe_readout = tree.borrow().level_readout(level);
    let mut longest = String::from("");
    while let Some(readout) = maybe_readout {
        if readout.len() > longest.len() {
            longest = readout;
        }
        level += 1;
        maybe_readout = tree.borrow().level_readout(level);
    }
    longest

}

fn parse_add_line(line: &mut dyn Iterator<Item = &str>) -> (i64, Rc<RefCell<Tree>>, Rc<RefCell<Tree>>) {
    let id: i64 = line.next().unwrap_or_default().split("=").nth(1).unwrap_or("0").parse().unwrap_or_default();
    let mut left_branch_vals = line.next()
        .unwrap_or("left=[0,]")
        .split("[")
        .skip(1)
        .map(|v| v.trim_end_matches("]").split(","))
        .next()
        .unwrap();
    let left_val = left_branch_vals.next().unwrap_or("0").parse::<i64>().unwrap();
    let left_label = left_branch_vals.next().unwrap_or_default();
    let left = Tree{label: String::from(left_label), value: left_val, left: None, right: None};

    let mut right_branch_vals = line.next()
        .unwrap_or("right=[0,]")
        .split("[")
        .skip(1)
        .map(|v| v.trim_end_matches("]").split(","))
        .next()
        .unwrap();
    let right_val: i64 = right_branch_vals.next().unwrap_or("0").parse::<i64>().unwrap();
    let right_label = right_branch_vals.next().unwrap_or_default();
    let right: Tree = Tree{label: String::from(right_label), value: right_val, left: None, right: None};

    (id, Rc::new(RefCell::new(left)), Rc::new(RefCell::new(right)))

}
fn build_trees(data: &str, part3: bool) -> (Rc<RefCell<Tree>>, Rc<RefCell<Tree>>) {
    let mut left_id_lookups: HashMap<i64, Rc<RefCell<Tree>>> = HashMap::new();
    let mut right_id_lookups: HashMap<i64, Rc<RefCell<Tree>>> = HashMap::new();
    let mut lines = data.lines();
    let first_line = lines.next().unwrap_or_default().split_ascii_whitespace().skip(1);
    let (id, left, right) = parse_add_line(&mut first_line.into_iter());
    left_id_lookups.insert(id, Rc::clone(&left));
    right_id_lookups.insert(id, Rc::clone(&right));
    for mut line in lines.map(|x| x.split_ascii_whitespace()) {
        let verb = line.next().unwrap_or_default();
        match verb {
            "ADD" => {
                let (id, new_left, new_right) = parse_add_line(&mut line);
                left.borrow_mut().add_leaf(Rc::clone(&new_left));
                left_id_lookups.insert(id, new_left);
                right.borrow_mut().add_leaf(Rc::clone(&new_right));
                right_id_lookups.insert(id, new_right);
            },
            "SWAP" => {
                let id: i64 = line.next().unwrap_or("0").parse().unwrap_or_default();
                let mut left_swap = left_id_lookups.get_mut(&id).unwrap().borrow_mut();
                let mut right_swap = right_id_lookups.get_mut(&id).unwrap().borrow_mut();
                (left_swap.label, right_swap.label) = (right_swap.label.to_owned(), left_swap.label.to_owned());
                (left_swap.value, right_swap.value) = (right_swap.value, left_swap.value);
                if part3 {
                    match (&left_swap.left, &right_swap.left) {
                        (None, None) => {},
                        (None, Some(new_left)) => {
                            (left_swap.left, right_swap.left) = (Some(Rc::clone(new_left)), None)
                        },
                        (Some(new_right), None) => {
                            (left_swap.left, right_swap.left) = (None, Some(Rc::clone(new_right)))
                        },
                        (Some(new_right), Some(new_left)) => {
                            (left_swap.left, right_swap.left) = (Some(Rc::clone(new_left)), Some(Rc::clone(new_right)))
                        }
                    }

                    match (&left_swap.right, &right_swap.right) {
                        (None, None) => {},
                        (None, Some(new_left)) => {
                            (left_swap.right, right_swap.right) = (Some(Rc::clone(new_left)), None)
                        },
                        (Some(new_right), None) => {
                            (left_swap.right, right_swap.right) = (None, Some(Rc::clone(new_right)))
                        },
                        (Some(new_right), Some(new_left)) => {
                            (left_swap.right, right_swap.right) = (Some(Rc::clone(new_left)), Some(Rc::clone(new_right)))
                        }
                    }

                }
            }
            _ => unimplemented!()
        }
    }
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1DATA: &str = "ADD id=1 left=[160,E] right=[175,S]
ADD id=2 left=[140,W] right=[224,D]
ADD id=3 left=[122,U] right=[203,F]
ADD id=4 left=[204,N] right=[114,G]
ADD id=5 left=[136,V] right=[256,H]
ADD id=6 left=[147,G] right=[192,O]
ADD id=7 left=[232,I] right=[154,K]
ADD id=8 left=[118,E] right=[125,Y]
ADD id=9 left=[102,A] right=[210,D]
ADD id=10 left=[183,Q] right=[254,E]
ADD id=11 left=[146,E] right=[148,C]
ADD id=12 left=[173,Y] right=[299,S]
ADD id=13 left=[190,B] right=[277,B]
ADD id=14 left=[124,T] right=[142,N]
ADD id=15 left=[153,R] right=[133,M]
ADD id=16 left=[252,D] right=[276,M]
ADD id=17 left=[258,I] right=[245,P]
ADD id=18 left=[117,O] right=[283,!]
ADD id=19 left=[212,O] right=[127,R]
ADD id=20 left=[278,A] right=[169,C]";

    const P2DATA: &str = "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]";

    const P3DATA: &str = "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]
SWAP 2
SWAP 5";
    #[test]
    fn test_tree_left() {
        let mut tree = Tree {label: String::from("A"), value: 10, left: None, right: None};
        tree.add_leaf(Rc::new(RefCell::new(Tree {label: String::from("B"), value: 5, left: None, right: None})));
        assert_eq!(tree.left, Some(Rc::new(RefCell::new(Tree {label: String::from("B"), value: 5, left: None, right: None}))));
        assert_eq!(tree.right, None);
    }

    #[test]
    fn test_tree_right() {
        let mut tree = Tree {label: String::from("A"), value: 10, left: None, right: None};
        tree.add_leaf(Rc::new(RefCell::new(Tree {label: String::from("B"), value: 15, left: None, right: None})));
        assert_eq!(tree.left, None);
        assert_eq!(tree.right,Some(Rc::new(RefCell::new(Tree {label: String::from("B"), value: 15, left: None, right: None}))));
    }

    #[test]
    fn test_tree_read() {
        let mut tree = Tree {label: String::from("A"), value: 10, left: None, right: None};
        assert_eq!(tree.level_readout(0), Some(String::from("A")));
        tree.add_leaf(Rc::new(RefCell::new(Tree {label: String::from("B"), value: 5, left: None, right: None})));
        assert_eq!(tree.level_readout(1), Some("B".to_string()));
        tree.add_leaf(Rc::new(RefCell::new(Tree {label: String::from("C"), value: 15, left: None, right: None})));
        assert_eq!(tree.level_readout(1), Some(String::from("BC")));
        assert_eq!(tree.level_readout(2), None);
        tree.add_leaf(Rc::new(RefCell::new(Tree {label: String::from("D"), value: 1, left: None, right: None})));
        assert_eq!(tree.level_readout(2), Some(String::from("D")));
        assert_eq!(tree.left.unwrap().borrow().level_readout(1), Some(String::from("D")));
        assert_eq!(tree.right.unwrap().borrow().level_readout(1), None);
    }

    #[test]
    fn test_part_1(){
        assert_eq!(answer(P1DATA, false), String::from("EVERYBODYCODES"))
    }
    #[test]
    fn test_part_2(){
        assert_eq!(answer(P2DATA, false), String::from("MGFLNK"))
    }
    #[test]
    fn test_part_3(){
        assert_eq!(answer(P3DATA, true), String::from("DJCGL"))
    }


}