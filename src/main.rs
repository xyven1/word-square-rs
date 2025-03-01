use std::{collections::HashSet, env::args, fmt::Display, fs::read_to_string};

use trie_rs::{
    inc_search::{self, IncSearch, Position},
    set::Trie,
};

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
    let trie = Trie::from_iter(f.lines());

    let mut grid = Grid::new('.', w, h);

    let i = std::cell::Cell::new(0);
    let mut seen = if args_it.next().is_some() {
        Some(HashSet::with_capacity(w * h))
    } else {
        None
    };
    search(
        &mut seen,
        &trie,
        inc_search::Position::from(trie.inc_search()),
        &mut vec![inc_search::Position::from(trie.inc_search()); w],
        &mut grid,
        (0, 0),
        &|grid| {
            i.set(i.get() + 1);
            println!("{grid}");
            println!("=======");
        },
    );
    println!("Num Found: {}", i.get());
}

fn search(
    seen: &mut Option<HashSet<String>>,
    trie: &Trie<char>,
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
    for (c, _) in IncSearch::resume(&trie.0, h_pos).children() {
        let mut words = Vec::<String>::new();

        let mut horiz = IncSearch::resume(&trie.0, h_pos);
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

        let mut vert = IncSearch::resume(&trie.0, v_pos[current_idx.1]);
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
            Position::from(trie.inc_search())
        } else {
            Position::from(horiz)
        };

        grid.set(current_idx.0, current_idx.1, *c);
        let old = v_pos[current_idx.1];
        v_pos[current_idx.1] = Position::from(vert);

        search(seen, trie, h_prefix, v_pos, grid, next_idx, res);

        if let Some(s) = seen.as_mut() {
            for word in &words {
                s.remove(word);
            }
        }
        grid.set(current_idx.0, current_idx.1, '.');
        v_pos[current_idx.1] = old;
    }
}
