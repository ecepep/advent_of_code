/*
https://adventofcode.com/2023/day/17

OC, djikstra: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
*/

// use std::cmp;
use std::{/* cmp::Reverse,  */cmp::Ordering,  collections::HashMap, collections::BinaryHeap};
use std::time::{Instant, Duration};

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

    let mut seen = HashMap::<Path, usize>::new();
    // let mut seen_2 = HashMap::<(usize, usize), usize>::new();
    
    // let now = Instant::now();
    
    // let mut seen_skip = 0;
    // let mut seen_pos_skip = 0;
    // let mut iter: i32 = 0;
    
    'step_path:  while !pq.is_empty() /* && iter <5 */ {
        
        // iter+=1;
        let p = pq.pop().unwrap();

        // if iter % 10000 == 0 {
        //     println!("p.len() {} , iter {}", pq.len(), iter);
        // }
        // let elapsed = now.elapsed();
        // if elapsed.as_secs() > 60*20 {
        //     panic!("Too slow compute")
        // }       
        
        // we reach the end of the path
        if p.r == grid.len()-1 && p.c == grid[0].len()-1 {    
            // let elapsed = now.elapsed();
            // println!("Elapsed: {:.2?}", elapsed);
            // println!("seen_skip) {} , seen_pos_skip {}", seen_skip, seen_pos_skip);
            println!("Part1: {}", p.hl);
        }
        // skip lesser optimal path
        let p_0hl = Path{ hl: 0, ..p };
        if let Some(_prev) = seen.get_mut(&p_0hl) {
            let (key, val) = seen.get_key_value(&p_0hl).unwrap();
            // seen_skip +=1;
            continue 'step_path;
        } else{
            seen.insert(p_0hl, p.hl);
        }
           
        // let p_pos = (p.r, p.c);
        // match seen_2.get(&p_pos) {
        //     Some(prev) => {
        //         if p.hl - prev > 20  {
        //             seen_pos_skip += 1;
        //             continue 'step_path;
        //         }
        //     },
        //     None => {
        //         seen_2.insert(p_pos, p.hl);
        //     }
        // }
                
        // try to keep straight
        if p.cnt < 3 && p.dir != (0,0) {
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
