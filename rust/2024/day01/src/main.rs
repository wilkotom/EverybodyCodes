use aochelpers::get_everybodycodes_input;
use itertools::Itertools;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>>{
    let p1data = get_everybodycodes_input(1, 2024, 1)?;
    println!("Part 1: {}", combat(&p1data,1));
    let p2data = get_everybodycodes_input(1, 2024, 3)?;
    println!("Part 2: {}", combat(&p2data,2));
    let p3data = get_everybodycodes_input(1, 2024, 2)?;
    println!("Part 2: {}", combat(&p3data,3));

    Ok(())
}

fn combat(creatures: &str, group_count: usize) -> usize {
    let mut score= 0;
    for pair in &creatures.chars().chunks(group_count) {
        let mut creature_count = 0;
        for c in pair {
            score += match c {
                'A' => {creature_count +=1; 0}
                'B' => {creature_count +=1; 1},
                'C' => {creature_count +=1; 3},
                'D' => {creature_count +=1; 5},
                _ => 0,
            };
        }
        score += creature_count * (creature_count -1)
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1(){
        assert_eq!(combat("A",1),0);
        assert_eq!(combat("B",1),1);
        assert_eq!(combat("C",1),3);
        assert_eq!(combat("A",1),0);
        assert_eq!(combat("ABBAC",1), 5);
    }
    
    #[test]
    fn test_part2(){
        assert_eq!(combat("Ax",2),0);
        assert_eq!(combat("Bx",2),1);
        assert_eq!(combat("Cx",2),3);
        assert_eq!(combat("AA",2),2);
    }

        
    #[test]
    fn test_part3(){
        assert_eq!(combat("xBx",3),1);
        assert_eq!(combat("BCD",3),15);
        assert_eq!(combat("AAA",3),6);
        assert_eq!(combat("xCC",3),8);
    }

}