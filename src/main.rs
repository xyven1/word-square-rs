use rayon::prelude::*;
use std::{collections::HashSet, env::args, fmt::Display, fs::read_to_string};
mod trie;

use crate::trie::TrieNode;

#[derive(Clone)]
struct Grid<T> {
    mem: Vec<T>,
    w: usize,
    h: usize,
}

impl<T: Clone> Grid<T> {
    fn new(default: T, w: usize, h: usize) -> Self {
        Self {
            mem: vec![default; w * h],
            w,
            h,
        }
    }
}

impl<T> Grid<T> {
    fn get(&self, i: usize, j: usize) -> &T {
        &self.mem[i * self.w + j]
    }
    fn set(&mut self, i: usize, j: usize, v: T) {
        self.mem[i * self.w + j] = v;
    }
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.h {
            for j in 0..self.w {
                write!(f, "{} ", *self.get(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut args_it = args();
    let command_name = args_it.next().unwrap();
    let (p, w, h) = match (
        args_it.next(),
        args_it.next().map(|v| v.parse::<usize>()),
        args_it.next().map(|v| v.parse::<usize>()),
    ) {
        (Some(p), Some(Ok(w)), Some(Ok(h))) => (p, w, h),
        _ => {
            println!("Usage: ./{command_name} <path> <width> <height> <?unique>");
            return;
        }
    };
    let f = read_to_string(&p).unwrap_or_else(|e| panic!("{e}: Could not read file {p}"));

    let mut count = 0;
    let trie_h = &TrieNode::from_iter(f.lines().filter(|v| v.len() == w).inspect(|_| count += 1));
    println!("Loaded horizontal dictionary: {} words", count);
    let mut count = 0;
    let trie_v = if h == w {
        println!("Using horizontal dictionary for vertical");
        trie_h
    } else {
        &TrieNode::from_iter(f.lines().filter(|v| v.len() == h).inspect(|_| count += 1))
    };
    if count > 0 {
        println!("Loaded vertical dictionary: {} words", count);
    }

    let unique = args_it.next().is_some() && w == h;

    trie_h
        .children()
        .map(|v| v.0)
        .collect::<HashSet<_>>()
        .intersection(&trie_v.children().map(|v| v.0).collect())
        .par_bridge()
        .for_each(|&c| {
            let mut grid = Grid::new('.', w, h);
            grid.set(0, 0, *c);
            let h_pos = trie_h.get(c).unwrap();
            let mut v_pos = vec![trie_v; w];
            v_pos[0] = trie_v.get(c).unwrap();
            search(
                trie_h,
                h_pos,
                &mut v_pos,
                &mut grid,
                (0, 1),
                &|grid| {
                    println!("{grid}");
                    println!("=======");
                },
                unique,
            );
            println!("Finished searching with {c} as top-left")
        });
}

fn search(
    h_root: &TrieNode<char>,
    h_pos: &TrieNode<char>,
    v_pos: &mut [&TrieNode<char>],
    grid: &mut Grid<char>,
    current_idx: (usize, usize),
    res: &impl Fn(&Grid<char>),
    unique: bool,
) {
    if current_idx.0 == grid.h {
        res(grid);
        return;
    }

    let next_j = current_idx.1 + 1;
    let next_idx = if next_j == grid.w {
        (current_idx.0 + 1, 0)
    } else {
        (current_idx.0, next_j)
    };

    for (c, node) in h_pos.children() {
        if unique && grid.get(current_idx.1, current_idx.0) == c {
            continue;
        }
        let old_vert = v_pos[current_idx.1];
        let vert = match old_vert.get(c) {
            Some(v) => v,
            None => continue,
        };

        let old_char = *grid.get(current_idx.0, current_idx.1);
        grid.set(current_idx.0, current_idx.1, *c);
        v_pos[current_idx.1] = vert;
        search(
            h_root,
            if next_idx.1 == 0 { h_root } else { node },
            v_pos,
            grid,
            next_idx,
            res,
            unique,
        );
        grid.set(current_idx.0, current_idx.1, old_char);
        v_pos[current_idx.1] = old_vert;
    }
}
