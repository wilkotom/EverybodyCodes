use itertools::Itertools;
fn main() {
    let p1data = std::fs::read_to_string("./part1.txt").unwrap();
    println!("Part 1: {}", combat(&p1data,1));
    let p2data = std::fs::read_to_string("./part2.txt").unwrap();
    println!("Part 2: {}", combat(&p2data,2));
    let p3data = std::fs::read_to_string("./part3.txt").unwrap();
    println!("Part 2: {}", combat(&p3data,3));

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