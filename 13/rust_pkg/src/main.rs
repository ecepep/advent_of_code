/*
https://adventofcode.com/2023/day/13/input
*/

use transpose;
use core::cmp::min;

const IS_PART_1 : bool = false;

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    let notes = input.split("\r\n\r\n");
    
    let grids: Vec<Vec<Vec<char>>> = notes.map(|n| {
        n.split("\r\n")
            .map(|line| {
                line.chars().collect::<Vec<_>>()
            }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    return grids;
}


fn find_reflect(g: &Vec<Vec<char>>) -> usize {
    for row in 1..g.len() {
        let above = g.get(..row).unwrap().into_iter().rev().cloned().collect::<Vec<Vec<char>>>();
        let below = g.get(row..).unwrap().to_vec();
        
        // Count nb of mismatch above and below the potential reflection
        let window = min(above.len(), below.len());
        let smudge_cnt = (0..window).map(|r|{
            let a = above.get(r).unwrap();
            let b = below.get(r).unwrap();
            a.iter().zip(b).filter(|&(a, b)| a != b).count()
        }).sum::<usize>();
        
        if smudge_cnt == (if IS_PART_1 {0} else {1}) {
            return row;
        }
    }
    
    return 0;
}

fn transpose_grid(g: &Vec<Vec<char>>) -> Vec<Vec<char>>{
    let flatten= g.into_iter().flatten().copied().collect::<Vec<_>>();
    let g_width = g.get(0).expect("empty note").len();
    let g_height = g.len();
    
    let mut output_array: Vec<char> = vec!['a'; flatten.len()];
    transpose::transpose(&flatten, &mut output_array, g_width, g_height);
    
    let transposed = output_array.chunks(g_height)
    .map(|x|{ x.to_vec() }).collect::<Vec<_>>();
return transposed;
}

fn score_grid(g: &Vec<Vec<char>>) -> usize {
    let row = find_reflect(g);
    
    let t = transpose_grid(g);
    let col = find_reflect(&t);
    
    return row*100 + col;
}

fn main() {    
    // @note from now on, we use include_str (char) instead of include_bytes (u8), to simplify
    let input = include_str!("input.txt");
    let grids = parse_input(input);
    
    let result =  grids.get(..).expect("Missing notes").iter().map(|g| {
        score_grid(g)
    }).sum::<usize>();

    println!("Part {}: {}", if IS_PART_1 {1} else {2}, result);
}
