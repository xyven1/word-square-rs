use std::{env::args, fmt::Display, fs::read_to_string};

use trie_rs::{
    inc_search::{self, Position},
    Trie,
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
    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        if i >= self.h || j >= self.w {
            return None;
        }
        Some(&mut self.mem[i * self.w + j])
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.h {
            for j in 0..self.w {
                write!(f, "{} ", *self.get(i, j).unwrap() as char)?;
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
            println!("Usage: ./{command_name} <path> <width> <height>");
            return;
        }
    };
    let f = read_to_string(&p).unwrap_or_else(|e| panic!("{e}: Could not read file {p}"));
    let trie = Trie::from_iter(f.lines());

    let mut grid = Grid::new(b'.', w, h);

    search(
        &trie,
        inc_search::Position::from(trie.inc_search()),
        &mut vec![inc_search::Position::from(trie.inc_search()); w],
        &mut grid,
        (0, 0),
        &|grid| {
            println!("{grid}");
            println!("=======");
        },
    );
}

const LETTERS: &[u8] = "abcdefghijklmnopqrstuvwxyz".as_bytes();
// void BoxSearch(Trie* trie, Trie* vtries[VTRIE_SIZE], int pos) {
//   //Reset when coming back to first letter
//   const int v_ix = pos % SIZE_W;
//   //Check if this is the beginning of a row
//   if (v_ix == 0) {
//     //If the entire grid is filled, we're done, print the solution
//     if (pos == SIZE_H * SIZE_W) {
//       PrintBox(g_words);
//       return;
//     }
//     //Reset the horizontal trie position to the beginning
//     trie = &g_trie_w;
//   }
//   Trie::Iter iter = trie->iter();
//   while (iter.next()) {
//     //Try next letter if vertical trie fails
//     if (!vtries[v_ix]->hasIx(iter.getIx())) { continue; }
//     //Show progress bar
//     if (pos == 0) { std::cout << "=== [" << iter.getLetter() << "] ===" << std::endl; }
//     //Letter is valid, update the solution
//     g_words[pos] = iter.getLetter();
//     //Make a backup of the vertical trie position in the stack for backtracking
//     Trie* backup_vtrie = vtries[v_ix];
//     //Update the vertical trie position
//     vtries[v_ix] = vtries[v_ix]->decend(iter.getIx());
//     //Make the recursive call
//     BoxSearch(iter.get(), vtries, pos + 1);
//     //After returning, restore the vertical trie position from the stack
//     vtries[v_ix] = backup_vtrie;
//   }
// }

fn search(
    trie: &Trie<u8>,
    h_pos: inc_search::Position,
    v_pos: &mut [inc_search::Position],
    grid: &mut Grid<u8>,
    current_idx: (usize, usize),
    res: &impl Fn(&Grid<u8>),
) {
    // println!("{current_idx:?}\n{grid}");
    // println!(
    //     "hroiz: {}",
    //     inc_search::IncSearch::resume(&trie.0, h_trie).prefix::<String, _>()
    // );
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

    for letter in LETTERS {
        let mut horiz = inc_search::IncSearch::resume(&trie.0, h_pos);
        let last_letter = current_idx.1 == grid.w - 1;
        match horiz.query(letter) {
            Some(inc_search::Answer::Prefix) if !last_letter => {}
            Some(inc_search::Answer::Match) if last_letter => {}
            Some(inc_search::Answer::PrefixAndMatch) => {}
            _ => continue,
        }
        let mut vert = inc_search::IncSearch::resume(&trie.0, v_pos[current_idx.1]);
        let last_letter = current_idx.0 == grid.h - 1;
        match vert.query(letter) {
            Some(inc_search::Answer::Prefix) if !last_letter => {}
            Some(inc_search::Answer::Match) if last_letter => {}
            Some(inc_search::Answer::PrefixAndMatch) => {}
            _ => continue,
        }
        *grid.get_mut(current_idx.0, current_idx.1).unwrap() = *letter;
        let old = v_pos[current_idx.1];
        v_pos[current_idx.1] = Position::from(vert);
        let h_prefix = if next_idx.1 == 0 {
            Position::from(trie.inc_search())
        } else {
            Position::from(horiz)
        };
        search(trie, h_prefix, v_pos, grid, next_idx, res);
        v_pos[current_idx.1] = old;
        *grid.get_mut(current_idx.0, current_idx.1).unwrap() = b'.';
    }
}
