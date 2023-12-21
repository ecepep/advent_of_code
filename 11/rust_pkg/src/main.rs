/*
https://adventofcode.com/2023/day/11/
My first ever Rust code ;) 
*/

use core::cmp::max as max;

const IS_PART1 : bool = false;

// Find 2D coordinates of '#' in the INPUT
fn find_galaxies (input : &[u8]) -> Vec::<[u64; 2]> {
    let mut galaxies = Vec::<[u64; 2]>::new();
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

// Correct galaxy coordinate for universe expansion in 1D
// \param dim: dimension (row or col)
fn expand_universe(mut galaxies: Vec::<[u64; 2]>, dim : usize) -> Vec::<[u64; 2]> {
    let expansion_size = if IS_PART1 {1} else {1000000-1};

    galaxies.sort_by_key(|g| g[dim]);

    let mut last = galaxies[0][dim];
    let mut expansion = 0;
    for (_index, g) in galaxies.iter_mut().enumerate() {
        expansion += max(g[dim] - last, 1) - 1;
        last = g[dim];
        g[dim] += expansion * expansion_size;
    }
    return galaxies;
}

fn main() {
    let input = include_bytes!("input.txt");

    let mut galaxies =  find_galaxies(input);
    
    galaxies = expand_universe(galaxies, 0);
    galaxies = expand_universe(galaxies, 1);

    let distances = galaxies.iter().enumerate().map(|(i, g)|{
        galaxies.iter().skip(i).map(|g2| {
            (g[0].abs_diff(g2[0]) + g[1].abs_diff(g2[1])) as u64
        }).sum::<u64>()
    }).sum::<u64>();

    println!("part {}: {}", if IS_PART1 {"1"} else {"2"}, distances);
}
  