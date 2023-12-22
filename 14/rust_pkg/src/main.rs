/*
https://adventofcode.com/2023/day/14/input
\todo resirculate a lot from 13 desember, I could make a "Utils" file. 
*/

use transpose;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = input.split("\r\n")
            .map(|line| {
                line.chars().collect::<Vec<_>>()
            }).collect::<Vec<_>>();

    return grid;
}

// copy paste from 13 desember
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

fn print_grid(name: &str, g: &Vec<Vec<char>>){
    let sep = std::str::from_utf8(&[b'\r', b'\n']).unwrap();
    let as_str = g.iter()
        .map(|row| {  
            row.iter().collect::<String>()
         }).collect::<Vec<String>>().join(sep);
    println!("{}:____\n{}\n",name, as_str);
}

enum Direction {
    north,
    south,
    east,
    west,
}

// Move the grid O to the specify directions
fn move_grid(g: &Vec<Vec<char>>, direction: Direction) -> Vec<Vec<char>>{
    let vertical = match direction {
        Direction::north => true,
        Direction::south => true,
        _ => false,
    };
    // sort depending on diretion, should O go left or right (asc: '0' > '.')
    let sort_asc = match direction {
        Direction::south => true,
        Direction::east => true,
        _ => false,
    };

    // transpose when moving along north-south axis, to avoid dealing with column-wise operation and for genericity
    let t = if vertical {transpose_grid(g)} else {g.clone()};

    let moved_transposed = t.iter().map(|row| {

        let sorted_segs = &row[..].split(|c| c == &'#').map(|segment|{
            let mut sorted_seg = segment.to_vec();
            sorted_seg.sort_by(|a, b| { if sort_asc {a.cmp(b)} else {b.cmp(a)} });  
            sorted_seg
        }).collect::<Vec<_>>();
        
        let sorted_row = &sorted_segs[..].join(&'#');
        sorted_row.to_vec()
    }).collect::<Vec<Vec<_>>>();

    // transpose back to normal if necessary
    let moved = if vertical { transpose_grid(&moved_transposed)} else { moved_transposed };
    return moved;
}

// cycle once
fn cycle_directions(g: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let g = move_grid(g, Direction::north);
    let g = move_grid(&g, Direction::west);
    let g = move_grid(&g, Direction::south);
    let g = move_grid(&g, Direction::east);
    return g;
}

fn north_load(g: &Vec<Vec<char>>) -> usize {
    g.iter().enumerate()
        .map(|(i, row)|{
            row.iter().filter(|r| r == &&'O').count() * (g.len() - i)
        }).sum::<usize>()
}

// We must memoize again
fn a_million_cycle(g: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut seen = [g.clone()].to_vec();
    let mut iter: usize = 0;
    let mut first: usize = 0;
    let mut g = g.clone(); // grid at each cycle

    'cycle_through: loop {
        iter += 1;
        g = cycle_directions(&g); // cycle once N, W, S, E

        for (i, s) in seen.iter().enumerate() {
            if s == &g {
                first = i;
                break 'cycle_through;
            }
        }
        seen.push(g.clone());
    }

    return seen.get((1000000000 - first) % (iter - first) + first).unwrap().clone();
}

fn main() {
    let input = include_str!("input.txt");
    let grid = parse_input(input);

    let moved_north = move_grid(&grid, Direction::north);
    println!("part 1 : {}", north_load(&moved_north));

    let cycled = a_million_cycle(&grid);
    println!("part 2 : {}", north_load(&cycled));
}
