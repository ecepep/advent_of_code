/*
https://adventofcode.com/2023/day/11/
My first ever Rust code ;) 
*/

use core::cmp::max as max;
// use std::fs;
// use regex::Regex;

// Find 2D coordinates of '#' in the INPUT
fn find_galaxies (input : &[u8]) -> Vec::<[u32; 2]> {
    let mut galaxies = Vec::<[u32; 2]>::new();
    let mut row = 0;
    let mut col = 0;
    for c in input {
        match c {
            b'\n' => {
                row += 1;
                col = 0;
            },
            b'#' => {
                galaxies.push([row, col])
            },
            _ => {}
        }

        col += 1;
    }
    return galaxies;
}

// Correct galaxy coordinate for universe expansion
// \param dim: dimension (row or col)
fn expand_universe(mut galaxies: Vec::<[u32; 2]>, dim : usize) -> Vec::<[u32; 2]> {
    let universe_dimension = galaxies.first().expect("No galaxies in universe :/").len();
    assert!(dim < universe_dimension); // 2D

    galaxies.sort_by_key(|g| g[dim]);

    let mut last = galaxies[0][dim];
    let mut expansion = 0;

    for (_index, g) in galaxies.iter_mut().enumerate() {
        expansion += max(g[dim] - last, 1) - 1;
        last = g[dim];
        g[dim] += expansion;
    }
    return galaxies;
}

fn main() {
    // const FILENAME = String::from();
    // const INPUT : &[u8; 118] = include_bytes!("input_test.txt");
    let input = include_bytes!("input.txt");

    let mut galaxies =  find_galaxies(input);
    
    // println!("___________________________");
    // {
    //     for g in &galaxies {
    //         println!("({}, {})", g[0], g[1]);
    //     }
    // }
    
    galaxies = expand_universe(galaxies, 0);
    galaxies = expand_universe(galaxies, 1);

    let distances = galaxies.iter().enumerate().map(|(i, g)|{
        galaxies.iter().skip(i).map(|g2| {
            (g[0].abs_diff(g2[0]) + g[1].abs_diff(g2[1])) as u64
        }).sum::<u64>()
    }).sum::<u64>();

    println!("part1: {}", distances);

    // std::thread::sleep(std::time::Duration::from_millis(4000));
}
  