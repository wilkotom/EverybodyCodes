use std::{collections::{BinaryHeap, HashMap, HashSet}, error::Error};

use aochelpers::{Coordinate, ScoredItem, get_everybodycodes_input};

#[derive(Debug,Copy,Clone,PartialEq, Eq,Hash, PartialOrd, Ord)]
struct LoopState {
    east: bool,
    west: bool,
    south: bool,
    location: Coordinate<i32>
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_everybodycodes_input(17,2025,1)?;
    let (grid, volcano, _) = parse_data(&data);
    println!("Part 1: {}", part1(&grid, volcano.unwrap(), 10));
    let data = get_everybodycodes_input(17,2025,2)?;
    let (grid, volcano, _) = parse_data(&data);
    println!("Part 2: {}", part2(&grid, volcano.unwrap()));

    let data = get_everybodycodes_input(17,2025,3)?;
    let (grid, volcano, start) = parse_data(&data);
    println!("Part 3: {}", part3(&grid, volcano.unwrap(), start.unwrap()));
    Ok(())
}

fn parse_data(data: &str) -> (HashMap<Coordinate<i32>, i32>, Option<Coordinate<i32>>, Option<Coordinate<i32>>,) {
    let mut grid= HashMap::new();
    let mut start = None;

    let mut volcano = None;
    for (y,line) in data.lines().enumerate() {
        for(x, c) in line.chars().enumerate() {
            match c {
                '@' => {volcano = Some(Coordinate{x: x as i32,y: y as i32});}
                'S' => {
                    start = Some(Coordinate{x: x as i32,y: y as i32});
                    grid.insert(Coordinate{x: x as i32,y: y as i32},0);
                }

                c @ '0' ..= '9' =>  {
                    grid.insert(Coordinate{x: x as i32,y: y as i32}, c.to_digit(10).unwrap().try_into().unwrap());
                }
                _ => unimplemented!()
            }
        }
    }
    (grid, volcano, start)

}

fn part1(grid: &HashMap<Coordinate<i32>, i32>, volcano: Coordinate<i32>, radius: i32) -> i32 {
    let mut result = 0;
    let bounds = grid.keys().fold(Coordinate{x:0,y:0}, |c, k| Coordinate { x: c.x.max(k.x), y: c.y.max(k.y) });
    for y in 0..=bounds.y {
        for x in 0..=bounds.x {
            let hypotenuse = (volcano.x - x).pow(2) + (volcano.y - y).pow(2);
            if hypotenuse <= radius.pow(2) {
                result += grid.get(&Coordinate { x, y }).unwrap_or(&0);
            } 
        }
    }

    result
}

fn part2(grid: &HashMap<Coordinate<i32>, i32>, volcano: Coordinate<i32>) -> i32 {
    let mut result = 0;
    let bounds: Coordinate<i32> = grid.keys().fold(Coordinate{x:0,y:0}, |c, k| Coordinate { x: c.x.max(k.x), y: c.y.max(k.y) });
    let mut last_burned = 0;
    let mut winning_radius = 0;
    for radius in 0..=(bounds.x.max(bounds.y) /2) {
        let burned = part1(grid, volcano, radius);
        let newly_burned = burned - last_burned;
        if newly_burned > result {
            result = newly_burned;
            winning_radius = radius;
        }
        last_burned = burned;
    }
    result * winning_radius
}

fn part3(grid: &HashMap<Coordinate<i32>, i32>, volcano: Coordinate<i32>, start: Coordinate<i32>) -> i32{
    let mut radius = 1;
    let bounds: Coordinate<i32> = grid.keys().fold(Coordinate{x:0,y:0}, |c, k| Coordinate { x: c.x.max(k.x), y: c.y.max(k.y) });


    while radius <  bounds.x.min(bounds.y) /2 {
        let starting_state = ScoredItem{cost:0, item: LoopState{east: false, west: false, south: false, location: start}};
        let mut next_states = BinaryHeap::new();
        next_states.push(starting_state);
        let mut visited = HashSet::new();
        while let Some(state) = next_states.pop() {
            if visited.contains(&state.item) || 
                (state.item.west && state.item.location.y < volcano.y && state.item.location.x < volcano.x ) ||
                (state.item.south && state.item.location.x < volcano.x) && state.item.location.y > volcano.y {
                // (state.item.east && state.item.location.y < volcano.y ) {
                
                continue;
            }
            visited.insert(state.item);
            if state.item.east && state.item.west && state.item.south && state.item.location == start {
                return state.cost * (radius);
            }

            for neighbour in state.item.location.neighbours().iter().filter(|c| grid.contains_key(c)) {
                let safe = (volcano.x - neighbour.x).pow(2) + (volcano.y - neighbour.y).pow(2) > (radius).pow(2);
                if safe && neighbour.x >=0 && neighbour.x <= bounds.x && neighbour.y >=0 && neighbour.y <= bounds.y {
                    let item = LoopState { 
                        east: state.item.east || (neighbour.y == volcano.y && neighbour.x > volcano.x),
                        west: state.item.west || (neighbour.y == volcano.y && neighbour.x < volcano.x),
                        south: state.item.south || (neighbour.x == volcano.x && neighbour.y > volcano.y),
                        location: *neighbour
                    };
                    let cost = state.cost + grid.get(neighbour).unwrap();
                    if !visited.contains(&item) && cost  < (radius+1) * 30{
                        let new_state = ScoredItem{ cost, item};
                        next_states.push(new_state);
                    }
                } 
            }
        }
        radius +=1
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::*;


    const P1TESTDATA: &str = "189482189843433862719
279415473483436249988
432746714658787816631
428219317375373724944
938163982835287292238
627369424372196193484
539825864246487765271
517475755641128575965
685934212385479112825
815992793826881115341
1737798467@7983146242
867597735651751839244
868364647534879928345
519348954366296559425
134425275832833829382
764324337429656245499
654662236199275446914
317179356373398118618
542673939694417586329
987342622289291613318
971977649141188759131";

    const P2TESTDATA: &str = "4547488458944
9786999467759
6969499575989
7775645848998
6659696497857
5569777444746
968586@767979
6476956899989
5659745697598
6874989897744
6479994574886
6694118785585
9568991647449";

    const P3TEST1: &str = "2645233S5466644
634566343252465
353336645243246
233343552544555
225243326235365
536334634462246
666344656233244
6426432@2366453
364346442652235
253652463426433
426666225623563
555462553462364
346225464436334
643362324542432
463332353552464";

const P3TEST2: &str = "545233443422255434324
5222533434S2322342222
523444354223232542432
553522225435232255242
232343243532432452524
245245322252324442542
252533232225244224355
523533554454232553332
522332223232242523223
524523432425432244432
3532242243@4323422334
542524223994422443222
252343244322522222332
253355425454255523242
344324325233443552555
423523225325255345522
244333345244325322335
242244352245522323422
443332352222535334325
323532222353523253542
553545434425235223552";

    const P3TEST3: &str = "5441525241225111112253553251553
133522122534119S911411222155114
3445445533355599933443455544333
3345333555434334535435433335533
5353333345335554434535533555354
3533533435355443543433453355553
3553353435335554334453355435433
5435355533533355533535335345335
4353545353545354555534334453353
4454543553533544443353355553453
5334554534533355333355543533454
4433333345445354553533554555533
5554454343455334355445533453453
4435554534445553335434455334353
3533435453433535345355533545555
534433533533535@353533355553345
4453545555435334544453344455554
4353333535535354535353353535355
4345444453554554535355345343354
3534544535533355333333445433555
3535333335335334333534553543535
5433355333553344355555344553435
5355535355535334555435534555344
3355433335553553535334544544333
3554333535553335343555345553535
3554433545353554334554345343343
5533353435533535333355343333555
5355555353355553535354333535355
4344534353535455333455353335333
5444333535533453535335454535553
3534343355355355553543545553345";

    #[test]
    fn test_p1() {
        let (grid, volcano, _) = parse_data(P1TESTDATA);
        assert_eq!(part1(&grid, volcano.unwrap(), 10), 1573);
    }

    #[test]
    fn test_p2() {
        let (grid, volcano, _) = parse_data(P2TESTDATA);
        assert_eq!(part2(&grid, volcano.unwrap()), 1090);
    }
    #[test]
    fn test_p3_1() {
        let (grid, volcano, start) = parse_data(P3TEST1);
        assert_eq!(part3(&grid, volcano.unwrap(), start.unwrap()), 592);
    }

    #[test]
    fn test_p3_2() {
        let (grid, volcano, start) = parse_data(P3TEST2);
        assert_eq!(part3(&grid, volcano.unwrap(), start.unwrap()), 330);
    }

    #[test]
    fn test_p3_3() {
        let (grid, volcano, start) = parse_data(P3TEST3);
        assert_eq!(part3(&grid, volcano.unwrap(), start.unwrap()), 3180);
    }
}