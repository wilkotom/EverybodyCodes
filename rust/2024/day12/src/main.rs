use std::{fs::read_to_string, isize};

use aochelpers::Coordinate;

fn main() {
    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q12_p1.txt").unwrap();
    let mut targets = Vec::new();
    let mut launchers = Vec::new();
    parse_data(&input, &mut targets, &mut launchers);
    println!("Part 1: {}", targets.iter().map(|t| part1(t, &launchers)).sum::<isize>());
    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q12_p2.txt").unwrap();
    let mut targets = Vec::new();
    let mut launchers = Vec::new();
    parse_data(&input, &mut targets, &mut launchers);
    println!("Part 2: {}", targets.iter().map(|t| part1(t, &launchers)).sum::<isize>());
    let input = read_to_string("/Users/twilkinson/Downloads/everybody_codes_e2024_q12_p3.txt").unwrap();
    let mut coords = Vec::new();
    for mut nums in input.lines().map(|l| l.split(' ')) {
        coords.push(Coordinate{x: nums.next().unwrap().parse::<isize>().unwrap(), 
                               y: nums.next().unwrap().parse::<isize>().unwrap()});
    }
    let p3result = coords.iter()
    .map(|c|  
            (0..=2)
            .filter_map(|y| find_meeting_point(*c,&Coordinate { x: 0, y})).min().unwrap_or(0))
        
        .sum::<isize>();
    println!("Part 3: {}", p3result) ;

}

fn parse_data(input: &str, targets: &mut Vec<Coordinate<isize>>, launchers: &mut Vec<Coordinate<isize>>) {

    // We subtract 1 from X and Y because in part 3 we get offsets from A, not absolute numbers.
    // Easier if A is at 0,0 to start with.
    
    for (y, line) in input.lines().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'T' {
                targets.push(Coordinate{x: x as isize -1, y: y as isize -1 });
            } else if c == 'H' {
                targets.push(Coordinate{x: x as isize -1, y: y as isize -1 });
                targets.push(Coordinate{x: x as isize -1, y: y as isize -1 });
            } else if ['A', 'B', 'C'].contains(&c) {
                launchers.push(Coordinate{x: x as isize -1, y: y as isize -1 });
            }

        }
    }

}

fn part1(target: &Coordinate<isize>, launchers: &[Coordinate<isize>]) -> isize {
    let mut best = isize::MAX;
    for launcher in launchers {
        let launcher_bonus = launcher.y +1;
        let diff = *target - *launcher;
        if diff.x <= 2* diff.y { // case where we hit it on the upswing or horizontal 
            best = best.min(diff.y * launcher_bonus);
        }
        if diff.y <= diff.x && (diff.x + diff.y) % 3 == 0 {
            best = best.min ((diff.x + diff.y) / 3 * launcher_bonus);
            
        } 
    }
    best
}

fn find_meeting_point(mut meteor: Coordinate<isize>, launcher: &Coordinate<isize>) -> Option<isize> {

    if meteor.x % 2 == 1 {
        meteor -= Coordinate{x: 1, y:1};
    }
    // meteor will be halfway between starting point and catapult when hit (as we want it to hit at the highest point possible)
    let meet_x = meteor.x /2;

    // if the meteor is on a direct path to hit catapult, power is half the distance
    if meteor.x - meteor.y + launcher.y == 0 {
        return Some(meet_x * (launcher.y +1));
    }
    // highest y for meeting point is 
    let meet_y = meteor.y - meet_x;

    // if midpoint X is in flat part of trajectory, x is in range (power <= x <= power *2)
    let power = meet_y - launcher.y;
    if meet_x >= power && meet_x <= power *2 {
        return Some(power * (launcher.y+1));
    } else {
    }
    // At this point, we're on the downstroke,
    if( meet_x + (meet_y - launcher.y)) % 3 != 0 || meet_y - launcher.y > meet_x {
        None
    } else {
        let power = (meet_x + (meet_y - launcher.y)) / 3;
        Some(power * (launcher.y+1))
    }



}


#[cfg(test)]
mod tests {
    use super::*;

    const P1TESTDATA: &str = ".............
.C...........
.B......T....
.A......T.T..
=============";

    const P3TESTDATA: &str="6 5
6 7
10 5";
    #[test]
    fn test_parser() {
        let mut targets = Vec::new();
        let mut launchers = Vec::new();
        parse_data(P1TESTDATA, &mut targets, &mut launchers);
        assert_eq!(launchers, vec![Coordinate{x:0,y:0}, Coordinate{x:0,y:1}, Coordinate{x:0,y:2}]);
        assert_eq!(targets, vec![Coordinate { x: 7, y: 0 }, Coordinate { x: 9, y: 0 }, Coordinate { x: 7, y:1 }]);

    }

    #[test]
    fn test_part1() {
        let mut targets = Vec::new();
        let mut launchers = Vec::new();
        parse_data(P1TESTDATA, &mut targets, &mut launchers);
        assert_eq!(targets.iter().map(|t| part1(t, &launchers)).sum::<isize>(), 13);
    

    }

    #[test]
    fn test_part3 () {
        let mut coords = Vec::new();
        for mut nums in P3TESTDATA.lines().map(|l| l.split(' ')) {
            coords.push(Coordinate{x:  nums.next().unwrap().parse::<isize>().unwrap(), 
                                   y: nums.next().unwrap().parse::<isize>().unwrap()});
        }
        assert_eq!(coords, vec![Coordinate{x:6, y:5},Coordinate{x:6, y:7}, Coordinate{x: 10, y:5}]);
        assert_eq!(find_meeting_point(Coordinate{x:6, y:5}, &Coordinate{x:0,y:2}), Some(3));
        assert_eq!(find_meeting_point(Coordinate{x:6, y:5}, &Coordinate{x:0,y:1}), None);
        assert_eq!(find_meeting_point(Coordinate{x:6, y:5}, &Coordinate{x:0,y:0}), Some(2));

        assert_eq!(find_meeting_point(Coordinate{x:6, y:7}, &Coordinate{x:0,y:2}), Some(6));
        assert_eq!(find_meeting_point(Coordinate{x:6, y:7}, &Coordinate{x:0,y:1}), Some(6));
        assert_eq!(find_meeting_point(Coordinate{x:6, y:7}, &Coordinate{x:0,y:0}), None);

        assert_eq!(find_meeting_point(Coordinate{x:10, y:5}, &Coordinate{x:0,y:2}), Some(3));
        assert_eq!(find_meeting_point(Coordinate{x:10, y:5}, &Coordinate{x:0,y:1}), None);
        assert_eq!(find_meeting_point(Coordinate{x:10, y:5}, &Coordinate{x:0,y:0}), None);

        assert_eq!(find_meeting_point(Coordinate{x:5, y:5}, &Coordinate{x:0,y:2}), None);

        let expected = coords.iter()
                .map(|c|  
                        (0..=2)
                        .filter_map(|y| find_meeting_point(*c,&Coordinate { x: 0, y})).min().unwrap_or(0))
                    
                    .sum::<isize>();
        assert_eq!(expected, 11);
    }

}