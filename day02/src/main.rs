extern crate input;

use input::stdlines;
use std::collections::HashMap;

fn main() {
    let inp = stdlines!();
    println!("Part 1: {}", solve_first(inp.clone()));
    println!("Part 2: {}", solve_second(inp));
}

fn solve_first<I: IntoIterator<Item = String>>(inp: I) -> u32 {
    let mut tws = 0;
    let mut ths = 0;
    for line in inp {
        let mut map: HashMap<char, u32> = HashMap::new();
        for ch in line.chars() {
            let counter = map.entry(ch).or_insert(0);
            *counter += 1;
        }
        let mut tw = false;
        let mut th = false;
        for val in map.values() {
            if *val == 2 { tw = true; }
            if *val == 3 { th = true; }
        }
        if tw { tws += 1; }
        if th { ths += 1; }
    }
    tws * ths
}

fn solve_second<I: IntoIterator<Item = String> + Clone>(inp: I) -> String {
    let mut common: String = String::new();
    for val1 in inp.clone() {
        for val2 in inp.clone() {
            if val1 == val2 { continue; }
            let s = find_common_letters(&val1, &val2);
            if s.len() > common.len() { common = s; }
        }
    }

    common
}

fn find_common_letters(s1: &str, s2: &str) -> String {
    s1.chars().zip(s2.chars()).fold(String::new(), |mut acc, (a, b)| { if a == b { acc.push(a); }; acc })
}
