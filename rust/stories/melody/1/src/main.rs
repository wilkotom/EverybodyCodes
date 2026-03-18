use std::collections::HashMap;
use aochelpers::get_everybodycodes_input;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Scale {
    identifier: i32,
    red: i32,
    green: i32,
    blue: i32,
    shine: i32
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let data: String = get_everybodycodes_input(1, 3, 1)?;
    let scales = parse_data(&data);
    println!("Part 1: {}", part1(&scales));

    let data: String = get_everybodycodes_input(1, 3, 2)?;
    let scales = parse_data(&data);
    println!("Part 2: {}", part2(scales));


    let data: String = get_everybodycodes_input(1, 3, 3)?;
    let scales = parse_data(&data);
    println!("Part 3: {}", part3(&scales));
    Ok(())
}

fn part1(scales: &[Scale]) -> i32 {

    scales.iter().filter(|s| s.green > s.blue && s.green > s.red).map(|s| s.identifier).sum()
}

fn part2(mut scales: Vec<Scale>) -> i32 {

    scales.sort_unstable_by(|a, b| (0- a.shine, a.red + a.green + a.blue).cmp(&(0- b.shine, b.red+ b.green + b.blue)));
    scales.iter().next().unwrap().identifier
}

fn part3(scales: &[Scale]) -> i32 {
    let mut groups: HashMap<&str, i32> = HashMap::new();

    for scale in scales.iter() {
        let shine_description = match scale {
            s if (s.red == s.blue && s.green <= s.red) || 
                (s.red == s.green && s.blue <= s.red) ||
                (s.blue == s.green && s.red <= s.blue) ||
                (s.shine > 30 && s.shine < 33) => "ignored",
            s if s.shine <= 30  && s.red > s.green && s.red > s.blue =>  "red-matte",
            s if s.shine <= 30  && s.green > s.blue =>  "green-matte",
            s if s.shine <= 30 =>  "blue-matte",
            s if s.red > s.green && s.red > s.blue =>  "red-shiny",
            s if  s.green > s.blue =>  "green-shiny",
            _ =>  "blue-shiny",
        };
        *groups.entry(shine_description).or_default() += scale.identifier;
    }
    *groups.values().max().unwrap_or(&0)
}

fn parse_data(data: &str) -> Vec<Scale> {
    let mut scales = Vec::new();
    for line in data.lines() {
        let mut fields = line.split(|c| c == ':' || c == ' ');
        let identifier = fields.next().unwrap_or_default().parse::<i32>().unwrap_or_default();
        let red = scale_to_value(fields.next().unwrap_or_default());
        let green = scale_to_value(fields.next().unwrap_or_default());
        let blue = scale_to_value(fields.next().unwrap_or_default());
        let shine = scale_to_value(fields.next().unwrap_or_default());
        scales.push(Scale { identifier, red, green, blue, shine });
    }

    scales
}


fn scale_to_value(scale: &str) -> i32 {
    let mut val = 0;
    for c in scale.chars() {
        val *= 2;
        if c.is_ascii_uppercase() {
            val +=1;
        }
    }
    val
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1TESTDATA: &str = "2456:rrrrrr ggGgGG bbbbBB sSsSsS
7689:rrRrrr ggGggg bbbBBB ssSSss
3145:rrRrRr gggGgg bbbbBB sSsSsS
6710:rrrRRr ggGGGg bbBBbB ssSSss";

    const P3TESTDATA: &str = "15437:rRrrRR gGGGGG BBBBBB sSSSSS
94682:RrRrrR gGGggG bBBBBB ssSSSs
56513:RRRrrr ggGGgG bbbBbb ssSsSS
76346:rRRrrR GGgggg bbbBBB ssssSs
87569:rrRRrR gGGGGg BbbbbB SssSss
44191:rrrrrr gGgGGG bBBbbB sSssSS
49176:rRRrRr GggggG BbBbbb sSSssS
85071:RRrrrr GgGGgg BBbbbb SSsSss
44303:rRRrrR gGggGg bBbBBB SsSSSs
94978:rrRrRR ggGggG BBbBBb SSSSSS
26325:rrRRrr gGGGgg BBbBbb SssssS
43463:rrrrRR gGgGgg bBBbBB sSssSs
15059:RRrrrR GGgggG bbBBbb sSSsSS
85004:RRRrrR GgGgGG bbbBBB sSssss
56121:RRrRrr gGgGgg BbbbBB sSsSSs
80219:rRRrRR GGGggg BBbbbb SssSSs";

    #[test]
    fn test_parser() {
        assert_eq!(parse_data(P1TESTDATA), vec![
            Scale {identifier: 2456, red:0, green: 11, blue: 3, shine: 21},
            Scale {identifier: 7689, red:8, green: 8, blue: 7, shine: 12},
            Scale {identifier: 3145, red:10, green: 4, blue: 3, shine: 21},
            Scale {identifier: 6710, red:6, green: 14, blue: 13, shine: 12},

        ])
    }

    #[test]
    fn test_p1() {
        let scales = parse_data(P1TESTDATA);
        assert_eq!(part1(&scales), 9166)
    }


    #[test]
    fn test_p2() {
        let scales = parse_data(P1TESTDATA);
        assert_eq!(part2(scales), 2456)
    }


    #[test]
    fn test_p3() {
        let scales = parse_data(P3TESTDATA);
        assert_eq!(part3(&scales), 292320)
    }
}
