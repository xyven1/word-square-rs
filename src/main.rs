use rayon::prelude::*;
use std::{
    collections::{BTreeSet, HashSet},
    env::args,
    fmt::Display,
    fs::read_to_string,
};

use trie_rs::{
    inc_search::{self, IncSearch, Position},
    set::Trie,
};

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
    fn get(&self, i: usize, j: usize) -> Option<&T> {
        if i >= self.h || j >= self.w {
            return None;
        }
        Some(&self.mem[i * self.w + j])
    }
    fn set(&mut self, i: usize, j: usize, v: T) {
        self.mem[i * self.w + j] = v;
    }
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.h {
            for j in 0..self.w {
                write!(f, "{} ", *self.get(i, j).unwrap())?;
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
    let trie_h = &Trie::from_iter(f.lines().filter(|v| v.len() == w).inspect(|_| count += 1));
    println!("Loaded dictionary: {} words", count);
    let trie_v = if h == w {
        trie_h
    } else {
        let mut count = 0;
        &Trie::from_iter(f.lines().filter(|v| v.len() == h).inspect(|_| count += 1))
        // println!("Loaded vertical dictionary: {} words", count);
        // a
    };

    let grid = Grid::new('.', w, h);

    let seen = if args_it.next().is_some() {
        Some(HashSet::with_capacity(w * h))
    } else {
        None
    };
    trie_h
        .inc_search()
        .children()
        .chain(trie_v.inc_search().children())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .for_each(|(&c, _)| {
            let mut seen = seen.clone();
            let mut grid = grid.clone();
            grid.set(0, 0, c);
            let trie_h = &trie_h;
            let trie_v = &trie_v;
            let mut inc_h = trie_h.inc_search();
            inc_h.next(&c);
            let h_pos = Position::from(inc_h);
            let mut v_pos = vec![Position::from(trie_v.inc_search()); w];
            let mut inc_v = trie_v.inc_search();
            inc_v.next(&c);
            v_pos[0] = Position::from(inc_v);
            search(
                &mut seen,
                trie_h,
                trie_v,
                h_pos,
                &mut v_pos,
                &mut grid,
                (0, 1),
                &|grid| {
                    println!("{grid}");
                    println!("=======");
                },
            );
            println!("Finished searching with {c} as top-left")
        });
}

#[allow(clippy::too_many_arguments)]
fn search(
    seen: &mut Option<HashSet<String>>,
    trie_h: &Trie<char>,
    trie_v: &Trie<char>,
    h_pos: inc_search::Position,
    v_pos: &mut [inc_search::Position],
    grid: &mut Grid<char>,
    current_idx: (usize, usize),
    res: &impl Fn(&Grid<char>),
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

    let last_row = current_idx.0 == grid.h - 1;
    let end_of_row = current_idx.1 == grid.w - 1;
    for (c, _) in IncSearch::resume(&trie_h.0, h_pos).children() {
        let mut words = Vec::<String>::new();

        let mut horiz = IncSearch::resume(&trie_h.0, h_pos);
        let Some(k) = horiz.next_kind(c) else {
            continue;
        };
        if end_of_row {
            if !k.is_exact() {
                continue;
            }
            let word = horiz.prefix();
            if let Some(s) = seen.as_ref() {
                if s.contains(&word) {
                    continue;
                }
            }
            words.push(word);
        } else if !k.is_prefix() {
            continue; // Simple optimization which cuts off an unnecessary recursion
        }

        let mut vert = IncSearch::resume(&trie_v.0, v_pos[current_idx.1]);
        let Some(k) = vert.next_kind(c) else { continue };
        if last_row {
            if !k.is_exact() {
                continue;
            }
            let word = vert.prefix();
            if let Some(s) = seen.as_ref() {
                if s.contains(&word) {
                    continue;
                }
            }
            words.push(word);
        } else if !k.is_prefix() {
            continue; // Simple optimization which cuts off an unnecessary recursion
        }

        if let Some(s) = seen.as_mut() {
            // This will only happen on the last letter, but must be checked
            if let [w1, w2] = &words[..] {
                if w1 == w2 {
                    continue;
                }
            }
            for word in &words {
                (*s).insert(word.clone());
            }
        }

        let h_prefix = if next_idx.1 == 0 {
            Position::from(trie_h.inc_search())
        } else {
            Position::from(horiz)
        };

        grid.set(current_idx.0, current_idx.1, *c);
        let old = v_pos[current_idx.1];
        v_pos[current_idx.1] = Position::from(vert);

        search(seen, trie_h, trie_v, h_prefix, v_pos, grid, next_idx, res);

        if let Some(s) = seen.as_mut() {
            for word in &words {
                s.remove(word);
            }
        }
        grid.set(current_idx.0, current_idx.1, '.');
        v_pos[current_idx.1] = old;
    }
}
