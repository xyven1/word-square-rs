use argh::FromArgs;
use rayon::prelude::*;
use std::{collections::HashSet, error::Error, fmt::Display, fs::read_to_string};
mod trie;

use crate::trie::TrieNode;

#[derive(FromArgs)]
/// Find word squares.
struct Args {
    /// word list
    #[argh(positional)]
    path: String,
    /// width of word square
    #[argh(positional)]
    width: usize,
    /// height of word square
    #[argh(positional)]
    height: Option<usize>,
    /// restrict to unqique words
    #[argh(switch, short = 'u')]
    unique: bool,
    /// disable multi-threading
    #[argh(switch)]
    no_threading: bool,
}

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

fn main() -> Result<(), Box<dyn Error>> {
    let args = argh::from_env::<Args>();
    let f = read_to_string(&args.path)
        .inspect_err(|_| println!("Could not read file {}", args.path))?;
    let w = args.width;
    let h = args.height.unwrap_or(w);

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

    let unique = args.unique && w == h;

    if args.no_threading {
        search(
            trie_h,
            trie_h,
            &mut vec![trie_v; w],
            &mut Grid::new('.', w, h),
            (0, 0),
            &|grid| {
                println!("{grid}\n======");
            },
            unique,
        )
    } else {
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
                        println!("{grid}\n======");
                    },
                    unique,
                );
                println!("Finished searching with {c} as top-left")
            });
    }
    Ok(())
}

fn search(
    h_root: &TrieNode<char>,
    h_pos: &TrieNode<char>,
    v_pos: &mut [&TrieNode<char>],
    grid: &mut Grid<char>,
    pos: (usize, usize),
    res: &impl Fn(&Grid<char>),
    unique: bool,
) {
    if pos.0 == grid.h {
        res(grid);
        return;
    }

    let next_col = pos.1 + 1;
    let next_idx = if next_col == grid.w {
        (pos.0 + 1, 0)
    } else {
        (pos.0, next_col)
    };

    for (c, node) in h_pos.children() {
        if unique && grid.get(pos.1, pos.0) == c {
            continue;
        }
        let old_vert = v_pos[pos.1];
        let vert = match old_vert.get(c) {
            Some(v) => v,
            None => continue,
        };

        let old_char = *grid.get(pos.0, pos.1);
        grid.set(pos.0, pos.1, *c);
        v_pos[pos.1] = vert;
        search(
            h_root,
            if next_idx.1 == 0 { h_root } else { node },
            v_pos,
            grid,
            next_idx,
            res,
            unique,
        );
        grid.set(pos.0, pos.1, old_char);
        v_pos[pos.1] = old_vert;
    }
}
