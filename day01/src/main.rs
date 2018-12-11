extern crate input;

use input::stdlines;
use std::collections::HashSet;
use std::ops::Add;
use std::hash::Hash;

fn main() {
    let inp = stdlines!().into_iter().map(|x| x.parse::<i64>().unwrap());
    println!("Part 1: {}", solve_first(inp.clone()));
    println!("Part 2: {}", solve_second(inp));
}

fn solve_first<N: Add<Output = N> + Default + Hash + Eq + Copy, I: IntoIterator<Item = N>>(inp: I) -> N where
    I::IntoIter: Clone {
    inp.into_iter().fold(N::default(), |acc, x| acc + x)
}

fn solve_second<N: Add<Output = N> + Default + Hash + Eq + Copy, I: IntoIterator<Item = N>>(inp: I) -> N where
    I::IntoIter: Clone {
    let mut inp = inp.into_iter().cycle();
    let mut seen_vals = HashSet::new();
    let mut sum = N::default();
    while !seen_vals.contains(&sum) {
        seen_vals.insert(sum);
        sum = sum + inp.next().unwrap();
    }
    sum
}
