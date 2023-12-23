/*
https://adventofcode.com/2023/day/16
*/

use std::collections::{VecDeque, HashMap};
use std::time::Instant;

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Beam {
    r: usize, // row
    c: usize, // col
    dir: Direction, 
}

fn step(grid: &[&[u8]], beam: &Beam) -> Vec<Beam> {    
    let tile = grid[beam.r][beam.c];
    // println!("r {}, c: {}, tile: {}", beam.r, beam.c, std::str::from_utf8(&[tile]).unwrap());
    // println!("last tile: {}",  std::str::from_utf8(&[lasttile]).unwrap());
    let new_directions = match tile {
        b'/' => {
            match beam.dir {
                Direction::Up => { vec![ Direction::Right ] },
                Direction::Down => { vec![ Direction::Left ] },
                Direction::Left => { vec![ Direction::Down ] },
                Direction::Right => { vec![ Direction::Up ] },
            }
        },
        b'\\' => {
            match beam.dir {
                Direction::Up => { vec![ Direction::Left ] },
                Direction::Down => { vec![ Direction::Right ] },
                Direction::Left => { vec![ Direction::Up ] },
                Direction::Right => { vec![ Direction::Down ] },
            }
        },
        b'-' => {
            match beam.dir {
                Direction::Up => { vec![ Direction::Left, Direction::Right ] },
                Direction::Down => { vec![ Direction::Left, Direction::Right ] },
                _ => vec![beam.dir.clone()],
            }
        },
        b'|' => {
            match beam.dir {
                Direction::Left => { vec![ Direction::Up, Direction::Down ] },
                Direction::Right => { vec![ Direction::Up, Direction::Down ] },
                _ => { vec![beam.dir.clone()] }
            }
        },
        b'.' => {
            vec![beam.dir.clone()]
        },
        _ => panic!("Unknown tile type")
    };

    let mut new_beams = Vec::<Beam>::new();
    for new_direction in  new_directions {      
        let (r, c) = match new_direction {
            Direction::Up => { (-1, 0) },
            Direction::Down => { (1, 0) },
            Direction::Left => { (0, -1) },
            Direction::Right => { (0, 1) },            
        };
        
        let (new_r, new_c) = (beam.r as isize + r, beam.c as isize + c);

        // println!("maxr {}, maxc {}", grid.len() as isize, grid[0].len()as isize);
        // println!("{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}", new_r, new_c,new_r < 0 ||new_c < 0 || new_r >= (grid.len() as isize) || new_c >= (grid[0].len()as isize) ,  new_r < 0, new_c < 0,  new_r >= (grid.len() as isize),  new_c >= (grid[0].len()as isize));
        
        // out of bound, beam stops
        if new_r < 0 || new_c < 0 || new_r >= (grid.len() as isize) || new_c >= (grid[0].len()as isize) {
            continue;
        }
        new_beams.push(Beam{r:new_r as usize, c:new_c as usize, dir:new_direction}); 
    }
    return new_beams; 
}

fn draw_beam(grid: &[&[u8]], start: &Beam)  -> HashMap<(usize, usize), Vec<Direction>> {
    let mut beams = VecDeque::<Beam>::new();
    beams.push_back(start.clone());

    // Prevent infinite looping
    let mut seen: HashMap<(usize, usize), Vec<Direction>> = Default::default();
    seen.insert((start.r, start.c), vec![start.dir.clone()]);

    while !beams.is_empty() {

        let beam = beams.pop_front().unwrap();
        let new_beams = step(grid, &beam);

        for b in new_beams {
            let k = (b.r, b.c);
            
            match seen.get_mut(&k) {
                Some(directions) =>  {
                    if directions.contains(&b.dir) {
                        continue;
                    } 
                    directions.push(b.dir.clone());
                },
                None => {
                    seen.insert(k, vec![b.dir.clone()]);
                }
            }
            beams.push_back(b.clone()); 
        }
    }
    return seen;
}

fn main() {
    // parse input
    let input = include_bytes!("input.txt");
    let grid = input
        .split(|b| b == &b'\n')
        .collect::<Vec<_>>().to_vec();
    
    
    // part 1
    let p1_start = Beam{ r : 0, c : 0, dir : Direction::Right};
    let seen = draw_beam(&grid, &p1_start);
    println!("Part 1: {}", seen.len());
    
    // part 2
    let now = Instant::now();

    let mut p2_starts = Vec::<Beam>::new();
    p2_starts.append(&mut ((0..grid.len()).map(|r| Beam{ r: r, c: 0, dir: Direction::Right}).collect::<Vec<Beam>>()));
    p2_starts.append(&mut ((0..grid.len()).map(|r| Beam{ r: r, c: grid[0].len()-1, dir: Direction::Left}).collect::<Vec<Beam>>()));
    p2_starts.append(&mut ((0..grid[0].len()).map(|c| Beam{ r: 0, c: c, dir: Direction::Down}).collect::<Vec<Beam>>()));
    p2_starts.append(&mut ((0..grid[0].len()).map(|c| Beam{ r: grid.len()-1, c: c, dir: Direction::Up}).collect::<Vec<Beam>>()));
    
    let p2_max = p2_starts.iter()
        .map(|start| {
             draw_beam(&grid, &start).len()
        }).max().unwrap();
    println!("Part 2: {}", p2_max);        
        
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
