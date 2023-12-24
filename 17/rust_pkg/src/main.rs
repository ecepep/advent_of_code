/*
https://adventofcode.com/2023/day/17

OC, djikstra: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
*/

use std::{ cmp::Ordering,  collections::HashSet, collections::BinaryHeap };

const IS_PART_1: bool = false;

#[derive(Eq, Hash, Clone, Debug)]
struct Path {
    hl : usize, // heat level
    r: usize, // row
    c: usize, // col
    dir: (isize, isize), // current direction displacement (row, col)
    cnt: usize, // count the number of straight move (same direction)
}

// Heat level is the score for Djikstra path search
//https://stackoverflow.com/questions/54489368/how-do-i-create-a-binaryheap-that-pops-the-smallest-value-not-the-largest
// \todo cleanup
impl Ord for Path {
    // Used by binary heap (pq)
    fn cmp(&self, other: &Self) -> Ordering {
        self.hl.cmp(&other.hl).reverse() 
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Path {
    // Use by hashmap (seen) 
    fn eq(&self, other: &Self) -> bool {
        self.hl == other.hl && self.r == other.r && self.c == other.c && self.dir.0 == other.dir.0
        && self.dir.1 == other.dir.1 && self.cnt == other.cnt   
    }
}

// \todo cleanup
fn to_int(ascii: u8) -> usize {
    return std::str::from_utf8(&[ascii]).unwrap().parse().unwrap();
}

fn main() {

    let input = include_bytes!("input.txt");
    let grid = input
        .split(|b| b == &b'\n')
        .collect::<Vec<_>>().to_vec();

    // priority queue of paths
    let mut pq = BinaryHeap::<Path>::new();
    let entrance = Path{ hl: 0, r: 0, c: 0, dir: (0,0), cnt: 0};
    pq.push(entrance);

    let mut seen = HashSet::<Path>::new();
    
    'step_path:  while !pq.is_empty() {
        let p = pq.pop().unwrap();

        // we reach the end of the path
        if p.r == grid.len()-1 && p.c == grid[0].len()-1 {
            if IS_PART_1 || p.cnt >= 4 {
                println!("Part1: {}", p.hl);
                break;
            }    
        }

        // skip lesser optimal path
        let p_0hl = Path{ hl: 0, ..p };
        if seen.contains(&p_0hl) {
            continue 'step_path;
        } 
        seen.insert(p_0hl);
                           
        // try to keep straight
        if p.cnt < (if IS_PART_1 {3} else {10}) && p.dir != (0,0) {
            let n_r = p.r as isize + p.dir.0;
            let n_c = p.c as isize + p.dir.1;

            if n_r >= 0 && (n_r as usize) < grid.len() 
            && n_c >= 0 && (n_c as usize) < grid[0].len() {
                let n_r = n_r as usize;
                let n_c = n_c as usize;
               
               pq.push(Path{
                    hl: p.hl + to_int(grid[n_r][n_c]),
                    r: n_r,
                    c: n_c,
                    dir: p.dir,
                    cnt: p.cnt + 1
                })
            }  
        }

        // try to explore
        for n_dir in [(0, 1), (1, 0), (0, -1), (-1, 0)]{
            if !IS_PART_1 && p.cnt < 4 && p.dir != (0, 0) {
                continue;
            }

            if n_dir != p.dir && n_dir != (-p.dir.0, -p.dir.1) {
                let n_r = p.r as isize + n_dir.0;
                let n_c = p.c as isize + n_dir.1;        

                if n_r >= 0 && (n_r as usize) < grid.len() 
                && n_c >= 0 && (n_c as usize) < grid[0].len() {
                    
                    let n_r = n_r as usize;
                    let n_c = n_c as usize;
                    
                    pq.push(Path{
                        hl: p.hl + to_int(grid[n_r][n_c]),
                        r: n_r,
                        c: n_c,
                        dir: n_dir,
                        cnt: 1,
                    })
                }
            }            
        }
    }
}
