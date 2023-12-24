use core::cmp::min as min;
use core::cmp::max as max;

fn get_points(plan: &Vec<(u8, isize)>) -> (isize, Vec<(isize, isize)>) {
    let n_sum: isize = plan.iter().map(|(d,n)| {n}).sum();
    let mut curr_pos = (0 as isize, 0 as isize);
    let mut points = vec![curr_pos];
    for (d, n) in plan {
        match d {
            b'U' => { curr_pos.0 -= n; },
            b'D' => { curr_pos.0 += n; },
            b'L' => { curr_pos.1 -= n; },
            b'R' => { curr_pos.1 += n; },
            _ => panic!("Not existing")
        }
        points.push(curr_pos);
    }

    return (n_sum, points);
}

fn get_area(plan: Vec<(u8, isize)>) -> isize {    
    let (n_sum, points) = get_points(&plan);
    let area = (0..points.len()).map(|i|{
        points[i].0 * (points[(if i == 0 {points.len()} else {i})-1].1 - points[(i+1) % points.len()].1)
    }).sum::<isize>().abs();
    let area = area / 2 - n_sum / 2 + 1 + n_sum;
    return area;
}

fn main() {
    let input: &[u8] = include_bytes!("input.txt");

    let plan_p1: Vec<(u8, isize)> = input
    .split(|b| b == &b'\n')
    .map(|line| {
        let n = line[2..].split(|b| b == &b' ').next().unwrap();
        let n = std::str::from_utf8(n).unwrap().parse::<isize>().unwrap();
        let d = line[0];
        (d, n)
    }).collect::<_>();

    let area_p1 = get_area(plan_p1);
    println!("Part 1: {}", area_p1);
    
    let plan_p2: Vec<(u8, isize)> = input
        .split(|b| b == &b'\n')
        .map(|line| {
            let color = line.split(|b| b == &b'#').skip(1).next().unwrap();
            let str_col = String::from_utf8(color.to_vec()).unwrap();
            let n = isize::from_str_radix(&str_col[..5], 16).unwrap();
            let d = match str_col.chars().nth(5).unwrap() {
                '0' => { b'R' },
                '1' => { b'D' },
                '2' => { b'L' },
                '3' => { b'U' },
                _ => { panic!("Unexpected char.")}
            };
            (d, n)
        }).collect::<_>();

    let area_p2 = get_area(plan_p2);
    println!("Part 2: {}", area_p2);
}
