/*
https://adventofcode.com/2023/day/12/input

Ain't no time on finding it out. I focus on learning rust.
Translate from: https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day12p1.py

\todo HashMap key clone instead of using references (fix dependency on the lifetime)
\todo HashMap should be a static variable -> (must it use lazy_static?)
\todo Could the memoizing be handled by memoize::memoize or fn_cache::{FnCache, HashCache}?
*/

use std::collections::HashMap;
use std::str;

fn parse_input(input: &[u8], is_part_1: bool) -> Vec::<(&[u8], Vec::<usize>)> {
    // split lines
    let lines = input.split(|b| b == &b'\n');

    // split cfg and nums
    let mut splits = lines.map(|line| {
        let mut spl = line.split(|b| b == &b' ');
        assert_eq!(spl.clone().count(), 2);
        
        let cfg = spl.next().unwrap();
        let nums = spl.next().unwrap(); // nums as u8 string
        
        // Repeat input 5* for part 2
        // if !is_part_1 {
        //     let cfg = [cfg; 5].join(&b'?');
        // }

        (cfg, nums)
    });

    // parse the list of nums from u8 str to vec of int
    splits.map(|(cfg, nums)| {
        let nums = nums.split(|b| b == &b',')
                    .map(|n| {
                        str::from_utf8(n)
                            .unwrap()
                            .trim()
                            .parse::<usize>()
                            .expect("Wrong input")
                    }).collect::<Vec::<usize>>();
        (cfg, nums)
    }).collect::<Vec::<(&[u8], Vec::<usize> )>>()
}

fn possibilities<'a>(cfg: &'a [u8], nums: &'a[usize], mut cache : &mut HashMap::<(&'a [u8], &'a[usize]), usize>) -> usize {
    if cfg.len() == 0 {
        return if nums.len() == 0 {1} else {0};
    }
    
    if  nums.len() == 0 {
        return if cfg.contains(&b'#') {0} else {1};
    }

    // search in cache 
    let key =  (cfg, nums);
    if cache.contains_key(&key) {
        return cache.get(&key).unwrap().clone();
    }

    let mut result = 0;

    if (cfg[0] == b'.') || (cfg[0] == b'?') {
        result += possibilities(&cfg[1..], nums, &mut cache);
    }  

    if (cfg[0] == b'#') || (cfg[0] == b'?') {
        if (nums[0] <= cfg.len()) && !(cfg[..nums[0]].contains(&b'.')) && 
            ((nums[0] == cfg.len()) || (cfg[nums[0]] != b'#')){
            let cfg = if cfg.len() > nums[0] {&cfg[(nums[0] + 1)..]} else {&[]};
            result += possibilities(cfg, &nums[1..], &mut cache);
        }
    }  

    if !cache.contains_key(&key) {
        cache.insert(key.clone(), result);
        
        if cache.len() % 2500 == 0 {
            println!("cache size: {}", cache.len());    
        }

    }
    return result
}

fn main() {
    let input = include_bytes!("input.txt");
    // println!("input {}", String::from(str::from_utf8(input).unwrap()));

    const IS_PART_1 : bool = true;

    let cfg_nums_parsed =  parse_input(input, IS_PART_1);

    let mut cache : HashMap::<(&[u8], &[usize]), usize> = HashMap::new(); // cache previous fn output #memoizing

    let total = cfg_nums_parsed.iter()
        .map(|(cfg, nums)| {
            possibilities(cfg, &nums[..], &mut cache)
        }).sum::<usize>();

    println!("Part1: {}", total);
}