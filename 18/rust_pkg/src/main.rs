
use core::cmp::min as min;
use core::cmp::max as max;


fn draw_contour(input: &[u8]) -> Vec<Vec<usize>> {
    let plan: Vec<(u8, usize)> = input
        .split(|b| b == &b'\n')
        .map(|line| {
            let n = line[2..].split(|b| b == &b' ').next().unwrap();
            let n = std::str::from_utf8(n).unwrap().parse::<usize>().unwrap();
            let d = line[0];
            (d, n)
        }).collect::<_>();

    // We must provision a vector of the good size because AOC was to mean to start at (0, 0) :/
    let mut max_r: isize = 0;
    let mut min_r: isize = 0;
    let mut max_c: isize = 0;
    let mut min_c: isize = 0;
    let mut r: isize = 0;
    let mut c: isize = 0;
    for (d, n) in plan.clone().iter(){
        match d {
            b'U' => { r -= *n as isize; },
            b'D' => { r += *n as isize; },
            b'L' => { c -= *n as isize; },
            b'R' => { c += *n as isize; },
            _ => {}
        }
        min_r = min(min_r, r);
        max_r = max(max_r, r);
        min_c = min(min_c, c);
        max_c = max(max_c, c);
    }        
    
    let mut pos = ((-min_r) as usize, (-min_c) as usize);
    let mut rows: Vec<Vec<usize>> = vec![vec![]; (max_r - min_r) as usize+1];
    for (d, n) in plan {
        let mut n_pos = pos;
        match d {
            b'U' => { n_pos.0 -= n; },
            b'D' => { n_pos.0 += n; },
            b'L' => { n_pos.1 -= n; },
            b'R' => { n_pos.1 += n; },
            _ => panic!("Not existing")
        }
        match d {
            b'U' | b'D' => { 
                for r in min(pos.0, n_pos.0)..max(pos.0, n_pos.0)+1 {
                    rows.get_mut(r).unwrap().push(pos.1);
                }
            },
            b'L' | b'R' => { 
                rows.get_mut(pos.0).unwrap().push(pos.1);
                rows.get_mut(pos.0).unwrap().push(n_pos.1);
            },
            _ => panic!("Not existing")
        }
        pos = n_pos;
    }

    // Some duplicate were added for simplicity, they must be removed. Later we need a sorted vec anyway
    for row in &mut rows {
        row.sort();
        row.dedup();
    }

    return rows;
}

fn main() {
    let input: &[u8] = include_bytes!("input.txt");
    let contour = draw_contour(&input);

    // Some smart people would use the formula for a polygon area
    let p1 = contour.iter()
        .map(|row|{
            
        @todo
            // row.iter().max().unwrap() - row.iter().min().unwrap() + 1
        }).sum::<usize>();
        
    println!("Part1: {}", p1);

}
