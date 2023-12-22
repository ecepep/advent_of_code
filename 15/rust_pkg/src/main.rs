/*
https://adventofcode.com/2023/day/15/input
*/

use std::{collections::LinkedList, str::FromStr};

fn hash(seq: &String) -> usize {
    let mut hash = 0;
    for c in seq.chars(){
        hash += c as usize; 
        hash *= 17;
        hash %= 256;
    }
    return hash
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal: usize,
}

// https://stackoverflow.com/a/72992169/6193177
fn remove_from_list(index_to_remove: usize, original_list: &mut LinkedList::<Lens>) {
    let mut split_list = original_list.split_off(index_to_remove);
    split_list.pop_front();
    original_list.append(&mut split_list);
}

fn fill_boxes(seq: String, mut boxes: [LinkedList::<Lens>; 256]) -> [LinkedList<Lens>; 256] {
    let sub_str = |s: &String, start: usize, end : usize| {
         s.chars().skip(start).take(end).collect::<String>()
    };
    let sep = seq.split_once(|c| { c == '-' || c == '=' })
        .expect("Operation undefined")
        .0.len();

    let label = sub_str(&seq, 0, sep);
    let op = sub_str(&seq, sep, 1); 
    let op = op.chars().next().expect("missing operator");
    let focal = sub_str(&seq, sep+1, 1);
    let focal = focal.parse().unwrap_or(0 as usize); 
    let curr_lens = Lens{ label: label.clone(), focal: focal };

    let hashed_label = hash(&label);
    let box_list = &mut boxes[hashed_label];

    let res: Option<(usize, &mut Lens)> = box_list.iter_mut().enumerate()
        .find(|(_i, lens)| { 
            lens.label == curr_lens.label 
        } );

    match res {
        // Label already exist
        Some((i, found_lens)) => {
            match op {
                '=' =>  { // add
                    found_lens.focal = curr_lens.focal;
                },
                '-' =>  { // remove
                    remove_from_list(i, box_list);
                },
                _ => panic!("Unknown operation")         
            }
        },
        // Label is yet missing
        None => {
            match op {
                '=' =>  { // add
                    box_list.push_back(curr_lens);
                },
                '-' =>  { }, // nothing happens (remove)
                _ => panic!("Unknown operation")         
            }
        }
    }

    return boxes;
}

fn main() {
    let input = include_str!("input.txt");
    let sequences = input.split(',');

    let p1 = sequences.clone().map(|seq| { 
        hash(&String::from_str(seq).unwrap() )
    }).sum::<usize>();
    println!("P1: {}", p1);
    
    let mut boxes: [LinkedList<Lens>; 256] = core::array::from_fn(|_| LinkedList::<Lens>::new());
    for seq in sequences {
        let seq = String::from_str(seq).unwrap();
        boxes = fill_boxes(seq, boxes);
    }

    let p2 = boxes.iter().enumerate()
        .map(|(box_i, boxe)| {
            boxe.iter().enumerate()
                .map(|(slot_i, lens)|{
                    (box_i+1) * (slot_i+1) * lens.focal
                }).sum::<usize>()
        }).sum::<usize>();

    // println!("boxes\n{:?}", boxes[6]);
    println!("p2: {}", p2);
}
