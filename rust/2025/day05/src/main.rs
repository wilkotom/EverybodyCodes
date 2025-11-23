use std::{cell::RefCell, error::Error, rc::Rc};
use aochelpers::get_everybodycodes_input;


#[derive(Debug, Clone, PartialEq, Eq)]
struct Sword {
    id: i64,
    head: FishBoneNode
}

impl Sword {

    fn new(id: i64) -> Self {
        Self { id, head: FishBoneNode{left: None, right: None, centre: None, next_node: None}}
    }

    fn quality(&self) -> i64 {
        self.head.quality().parse().unwrap()
    }
}

impl PartialOrd for Sword {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }

}

impl Ord for Sword {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.head.cmp(&other.head) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                self.id.cmp(&other.id)
            }
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct FishBoneNode {
    left: Option<i64>,
    centre: Option<i64>,
    right: Option<i64>,
    next_node: Option<Rc<RefCell<FishBoneNode>>>
}

impl FishBoneNode {
    fn add_value(&mut self, val: i64) {
        if let Some(centre) = self.centre {
            if val < centre && self.left.is_none() {
                self.left = Some(val)
            } else if val > centre && self.right.is_none() {
                self.right = Some(val)
            } else if let Some(next_node) = &self.next_node {
                next_node.borrow_mut().add_value(val);
            } else {
                self.next_node = Some(Rc::new(RefCell::new(
                    FishBoneNode{left: None, right: None, centre: Some(val), next_node: None}
                )));
            }
        }
        else {
            self.centre = Some(val)
        }
    }

    fn quality(&self) -> String {
        if self.centre.is_none() {
            String::new()
        } else if let Some(next_node) = &self.next_node {
               format!("{}", self.centre.unwrap()) + &next_node.borrow().quality()
        } else {
            format!("{}", self.centre.unwrap())
        }
    }

    fn node_values(&self) -> Vec<i64> {
        if self.centre.is_some() {
            if let Some(next_node) = &self.next_node {
                let mut result = vec![self.node_value()];
                result.append(&mut next_node.borrow_mut().node_values());
                result
            } else {
                vec![self.node_value()]
            }

        } else {
            Vec::new()
        }
    }

    fn node_value(&self) -> i64 {

        format!("{}{}{}", if let Some(v) = self.left {format!("{}", v)} else {"".to_string()}, 
                if let Some(v) = self.centre {format!("{}", v)} else {"".to_string()},
                if let Some(v) = self.right {format!("{}", v)} else {"".to_string()})
            .parse().unwrap()
        


    }
}

impl PartialOrd for FishBoneNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FishBoneNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.quality().parse::<i64>().unwrap().cmp(&other.quality().parse().unwrap()) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                self.node_values().cmp(&other.node_values())
            }
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let values = parse_data(&get_everybodycodes_input(5, 2025, 1)?);
    println!("Part 1: {}", values[0].quality());

    let values = parse_data(&get_everybodycodes_input(5, 2025, 2)?);
    println!("Part 2: {}", part2(&values));

    let values = parse_data(&get_everybodycodes_input(5, 2025, 3)?);
    println!("Part 3: {}", part3(values));
    Ok(())
}

fn part2(swords: &[Sword]) -> i64 {
    let sword_vals = swords.iter().map(|s| s.clone().quality().to_owned()).collect::<Vec<_>>();
    sword_vals.iter().max().unwrap_or(&0) - sword_vals.iter().min().unwrap_or(&0)
}

fn part3(mut swords: Vec<Sword>) -> i64 {
    swords.sort_by(|a,b| b.cmp(&a));
    swords.iter().enumerate().map(|(n,s)| s.id * (n as i64 +1)).sum()
    
}



fn parse_data(data:&str) -> Vec<Sword> {
    let mut swords = Vec::new();
    for line in data.lines() {
        let mut sections = line.split(":");
        let id = sections.next().unwrap_or_default().parse().unwrap_or_default();
        let values = sections.next().expect("No Second field found")
            .split(',')
            .map(|v| v.parse::<i64>().expect("Cannot parse value"));
        let mut next_sword = Sword::new(id);
        for value in values {
            next_sword.head.add_value(value);
        }
        swords.push(next_sword);
    }
    swords
}

#[cfg(test)]
mod tests{
    use super::*;

    const P2DATA: &str = "1:2,4,1,1,8,2,7,9,8,6
2:7,9,9,3,8,3,8,8,6,8
3:4,7,6,9,1,8,3,7,2,2
4:6,4,2,1,7,4,5,5,5,8
5:2,9,3,8,3,9,5,2,1,4
6:2,4,9,6,7,4,1,7,6,8
7:2,3,7,6,2,2,4,1,4,2
8:5,1,5,6,8,3,1,8,3,9
9:5,7,7,3,7,2,3,8,6,7
10:4,1,9,3,8,5,4,3,5,5";

    const P3DATA: &str = "1:7,1,9,1,6,9,8,3,7,2
2:6,1,9,2,9,8,8,4,3,1
3:7,1,9,1,6,9,8,3,8,3
4:6,1,9,2,8,8,8,4,3,1
5:7,1,9,1,6,9,8,3,7,3
6:6,1,9,2,8,8,8,4,3,5
7:3,7,2,2,7,4,4,6,3,1
8:3,7,2,2,7,4,4,6,3,7
9:3,7,2,2,7,4,1,6,3,7";

    #[test]
    fn test_p1() {
        let mut sword= Sword::new(58);
        for val in [5,3,7,8,9,10,4,5,7,8,8] {
            sword.head.add_value(val);
        }
        assert_eq!(sword.quality(),581078)
    }

    #[test]
    fn test_p2() {
        let swords = parse_data(P2DATA);
        assert_eq!(part2(&swords), 77053);
    }

    #[test]
    fn test_p3() {
        let swords = parse_data(P3DATA);
        assert_eq!(part3(swords), 260);
    }
}