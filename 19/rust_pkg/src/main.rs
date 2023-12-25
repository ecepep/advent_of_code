
/*
https://adventofcode.com/2023/day/19
*/

use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Default)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn acceptable(workflows: &HashMap<String, String>, part: &Part) -> bool {
    // println!("part: {:?}", part);
    let mut label = String::from_str("in").unwrap();
    loop {
        let flow = workflows.get(&label).expect("Unknwon label");
        for rule in flow.split(',') {           
            let rule_split = rule.split_once(':');
            let (condition, result) =
                match rule_split {
                    Some((condition, result)) => { 
                        (Some(String::from_str(condition).unwrap()), String::from_str(result).unwrap())
                    },
                    None => { 
                        (None, String::from_str(rule).unwrap() )
                    },
                };

            // println!("label: {:?}, condition: {:?}, result: {:?}", label, condition,result);
            if let Some(condition) = condition {
                let p: &str = &(condition.to_owned())[..2];
                let num: usize = (&(condition.to_owned())[2..]).parse().unwrap();
                let fullfilled = match p {
                    "x<" => { part.x < num },
                    "x>" => { part.x > num },
                    "m<" => { part.m < num },
                    "m>" => { part.m > num },
                    "a<" => { part.a < num },
                    "a>" => { part.a > num },
                    "s<" => { part.s < num },
                    "s>" => { part.s > num },
                    _ => { panic!("Unexpected condition") }
                };

                if !fullfilled {
                    continue;
                }
            }

            match &result[..] {
                "R" => { return false; },
                "A" => { return true; },
                _ => { label = result },
            }
            break;
        }
    }
}

fn count_accepted(workflows: &HashMap<String, String>, label: &String, mut ranges: [Vec<usize>; 4]) -> usize {
    if label == "A" {
        return ranges.iter().map(|v| v.len()).product();
    }
    if label == "R" {
        return 0;
    }

    let mut ans = 0;
    let flow = workflows.get(label).expect("Unknwon label");
    for rule in flow.split(',') {           
        let rule_split = rule.split_once(':');
        let (condition, result) =
            match rule_split {
                Some((condition, result)) => { 
                    (Some(String::from_str(condition).unwrap()), String::from_str(result).unwrap())
                },
                None => { 
                    (None, String::from_str(rule).unwrap() )
                },
            };

            // (newranges[i], ranges[i]) = ranges[i].iter().partition(|&&val| if lt {val < n} else {val > n});
        // println!("label: {:?}, condition: {:?}, result: {:?}", label, condition,result);
        if let Some(condition) = condition {
            let mut newranges = ranges.clone();

            let p: &str = &(condition.to_owned())[..2];
            let num: usize = (&(condition.to_owned())[2..]).parse().unwrap();
            let (index, lt) = match p {
                "x<" => { (0, true) },
                "x>" => { (0, false) },
                "m<" => { (1, true) },
                "m>" => { (1, false) },
                "a<" => { (2, true) },
                "a>" => { (2, false) },
                "s<" => { (3, true) },
                "s>" => { (3, false) },
                _ => { panic!("Unexpected condition") }
            };
            (newranges[index], ranges[index]) = ranges[index].iter().partition(|&&val| if lt {val < num} else {val > num});
            ans += count_accepted( workflows, &result, newranges);
        } else {
            ans += count_accepted( workflows, &result, ranges.clone());
        }
    }
    return ans;
}

fn parse_input() -> (HashMap<String, String>, Vec<Part>) {
    let (workflows, parts) = include_str!("input.txt").split_once("\n\n").unwrap();

    let workflows = workflows.split("\n")
        .map(|line|{ 
            let mut line = line.split(['{', '}']);
            let key = line.next().expect("No label");
            let key = String::from_str(key).unwrap();
            let rules = line.next().expect("No rules");
            let rules = String::from_str(rules).unwrap();

            (key, rules)
        }).collect::<HashMap<String, String>>();

    let parts = parts.split("\n")
        .map(|line|{ 
            let mut part: Part = Default::default();
            let part_str = &line[1..(line.len()-1)]; // remove '{' '}'
            for p in part_str.split(',') {
                let (p, num) = p.split_once('=').expect(" = not found");
                match p {
                    "x" => { part.x = num.parse().unwrap(); },
                    "m" => { part.m = num.parse().unwrap(); },
                    "a" => { part.a = num.parse().unwrap(); },
                    "s" => { part.s = num.parse().unwrap(); },
                    _ => { panic!("Unknown"); },
                }
            }
            return part;
        }).collect::<Vec<Part>>();
    
    return (workflows, parts);
}

fn main() {
    let (workflows, parts) = parse_input();

    // part 1
    let p1 = parts.iter().map(|part| { 
        if acceptable(&workflows, part){
            return part.x + part.m + part.a + part.s;
        }
        return 0;
    }).sum::<usize>();
    println!("Part 1: {}", p1);

    // part 2
    let p2 = count_accepted( &workflows, &String::from_str("in").unwrap(), std::array::from_fn(|_| (1..=4000).collect::<Vec<_>>()));
    println!("Part 2: {}", p2);
}
