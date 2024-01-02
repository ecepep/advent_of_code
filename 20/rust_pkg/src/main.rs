/*
https://adventofcode.com/2023/day/20
*/

// \todo better use reference
// RefCell is unecessary here, but I use it for the sake of learning

use std::{collections::{HashMap,hash_map::Entry, VecDeque}, str::FromStr, cell::RefCell};
use gcd::Gcd;

#[derive(Clone)]
enum Logic {
    Broadcaster,
    FlipFlop(RefCell<bool>), // false => off
    Conjunction(RefCell<HashMap<String, bool>>), // false => low
}

#[derive(Clone)]
struct Node {
    key: String,
    logic: Logic,
    output: Vec<String> 
}

struct Signal {
    ori: String,
    out: String,
    pulse: bool, // false is low
}

fn parse_input() ->  HashMap::<String, Node> {
    let input = include_str!("input.txt");

    let mut nodes = HashMap::<String, Node>::new();

    for l in input.split('\n') {
        let (src, output) = l.split_once(" -> ").unwrap();
        let (key, logic) = match src.as_bytes()[0] {
            b'b' => (src, Logic::Broadcaster),
            b'%' => (&src[1..], Logic::FlipFlop(RefCell::new(false))),
            b'&' => (&src[1..], Logic::Conjunction(RefCell::new(HashMap::new()))),
            _ => panic!("Unknown logic"),
        };

        let key = String::from_str(key).unwrap();
        let output = output
            .split(", ")
            .map(|s| String::from_str(s).unwrap() )
            .collect::<Vec<_>>();
        let node = Node {key: key.clone(), logic: logic, output: output};
        nodes.insert(key, node);
    }

    // Bind conjunction to incoming node
    for (key, node) in nodes.iter() {
        for out in node.output.iter() {
            if let Some(o_node) = nodes.get(out) {
                match &o_node.logic {
                    Logic::Conjunction(m) => { 
                        m.borrow_mut().insert(key.clone(), false);
                    },
                    _ => {},
                }
              }
        }
    }

    return nodes;
}

fn cycle_through(nodes: &HashMap<String, Node>, low: &mut usize, high: &mut usize){    
    *low += 1; // part 1
    let mut q = nodes.get("broadcaster")
        .unwrap()
        .output
        .iter()
        .map(|x| Signal{ ori: String::from_str("broadcaster").unwrap(), out: x.clone(), pulse: false })
        .collect::<VecDeque::<Signal>>();

    while !q.is_empty() {
        let s = q.pop_front().unwrap();

        if s.pulse {
            *high += 1;
        } else {
            *low += 1;
        }

        if !nodes.contains_key(&s.out) {
            continue;
        }

        let node = nodes.get(&s.out).unwrap();
        match &node.logic {
            Logic::FlipFlop(b) => { 
                if !s.pulse { // low
                    let mut b = b.borrow_mut();
                    *b = !(*b);  
                    for n_out in &node.output {
                        q.push_back(Signal{ori: node.key.clone(), out: n_out.clone(), pulse: *b}) // high if just turned on and vice versa
                    }
                }
            },
            Logic::Conjunction(m) => { 
                match m.borrow_mut().entry(s.ori) {
                    Entry::Occupied(mut o) =>  { *o.get_mut() = s.pulse; },
                    Entry::Vacant(v) => { v.insert(s.pulse); },
                };
                let n_pulse = m.borrow().iter().any(|(_k, v)| { !v });
                for n_out in &node.output {
                    q.push_back(Signal{ori: node.key.clone(), out: n_out.clone(), pulse: n_pulse });
                }
            },
            _ => panic!("Unexptected logic")
        }
    }
}

fn find_cycle(nodes: &HashMap<String, Node>) -> usize{

    let rx = String::from_str("rx").unwrap();
    let feed = nodes
        .iter()
        .filter(|(_key, node)| { node.output.contains(&rx) })
        .map(|(key, _node)| { key.clone() })
        .collect::<Vec::<String>>();
    
    let mut seen = nodes
        .iter()
        .filter(|(_key, node)|{ 
            node.output.iter().any(|out| { feed.contains(out) })
        })
        .map(|(key, _node)|{ (key.clone(), 0) })
        .collect::<HashMap::<String, usize>>();

    let mut cycle_length = HashMap::<String, usize>::new();

    let mut presses: usize = 0;
    loop {
        presses+=1;

        let mut q = nodes.get("broadcaster")
        .unwrap()
        .output
        .iter()
        .map(|x| Signal{ ori: String::from_str("broadcaster").unwrap(), out: x.clone(), pulse: false })
        .collect::<VecDeque::<Signal>>();

        while !q.is_empty() {
            let s = q.pop_front().unwrap();
            
            if !nodes.contains_key(&s.out) {
                continue;
            }

            let node = nodes.get(&s.out).unwrap();

            if feed.contains(&node.key) && s.pulse{
                *seen.get_mut(&s.ori).unwrap() += 1;

                if !cycle_length.contains_key(&s.ori) {
                    cycle_length.insert(s.ori.clone(), presses);
                } else {
                    assert!(presses == (seen.get(&s.ori).unwrap() * cycle_length.get(&s.ori).unwrap()))
                }

                let all_seen = seen
                    .iter()
                    .all(|(_k, v)| { *v > 0 } );

                if all_seen {
                    let mut x = 1;
                    for (_k, cl) in cycle_length.iter() {
                        x = x * cl / x.gcd(*cl);
                    }
                    return x;
                }
            }

            match &node.logic {
                Logic::FlipFlop(b) => { 
                    if !s.pulse { // low
                        let mut b = b.borrow_mut();
                        *b = !(*b);  
                        for n_out in &node.output {
                            q.push_back(Signal{ori: node.key.clone(), out: n_out.clone(), pulse: *b}) // high if just turned on and vice versa
                        }
                    }
                },
                Logic::Conjunction(m) => { 
                    match m.borrow_mut().entry(s.ori) {
                        Entry::Occupied(mut o) =>  { *o.get_mut() = s.pulse; },
                        Entry::Vacant(v) => { v.insert(s.pulse); },
                    };
                    let n_pulse = m.borrow().iter().any(|(_k, v)| { !v });
                    for n_out in &node.output {
                        q.push_back(Signal{ori: node.key.clone(), out: n_out.clone(), pulse: n_pulse });
                    }
                },
                _ => panic!("Unexptected logic")
            }
        }
    }
}

fn main() {
    let nodes_p1 = parse_input();
    let nodes_p2 = nodes_p1.clone();
    
    let mut low: usize = 0;
    let mut high: usize= 0;
    for _ in 0..1000 {
        cycle_through(&nodes_p1, &mut low, &mut high);
    }
    let result = high*low;
    println!("Part1: {}", result);
    
    println!("Part2: {}", find_cycle(&nodes_p2));
}
