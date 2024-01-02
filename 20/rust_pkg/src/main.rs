/*
https://adventofcode.com/2023/day/20
*/

// \todo better use reference
// RefCell is unecessary here, but I use it for the sake of learning

use std::{collections::{self, HashMap,hash_map::Entry, VecDeque}, str::FromStr, cell::RefCell};


enum Logic {
    Broadcaster,
    FlipFlop(RefCell<bool>), // false => off
    Conjunction(RefCell<HashMap<String, bool>>), // false => low
}

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
    *low += 1;
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

fn main() {
    println!("Hello, world!");

    let nodes_p1 = parse_input();
    // let nodes_P2 = nodes_P1.clone();
    
    let mut low: usize = 0;
    let mut high: usize= 0;
    for _ in 0..1000 {
        cycle_through(&nodes_p1, &mut low, &mut high);
    }
    let result = high*low;
    print!("Part1: {}", result);
}
