use flat_matrix::FlatMatrix;

pub use console;

use console::{Term, Style};

#[derive(Clone, PartialEq)]
struct Cell {
    ch: char,
    style: Style,
}

impl Cell {
    fn default() -> Self {
        return Self {
            ch: ' ',
            style: Style::new(),
        }
    }
}

struct Diff {
    cell: Cell,
    pos: (usize, usize),
}

pub struct Buffer {
    c: FlatMatrix<Cell>,
    l: FlatMatrix<Cell>,
}

impl Buffer {
    pub fn new() -> Self {
        let (width, height)= Self::get_term_size();
        return Self {
            c: FlatMatrix::new(height, width, Cell::default()),
            l: FlatMatrix::new(height, width, Cell::default()),
        };
    }
    pub fn get_term_size() -> (usize, usize) {
        let term = Term::stdout();
        let (height, width) = term.size();
        return (width as usize, height as usize);
    }
    pub fn resize(&mut self) {
        let (width, height) = Self::get_term_size();
        self.l.resize(width, height, Cell::default());
        self.c.resize(width, height, Cell::default());
    }
    pub fn put(&mut self, x: usize, y: usize, ch: char, style: Style) -> bool {
        return self.c.set(x, y, Cell {ch, style});
    }
    pub fn puts(&mut self, mut x: usize, y: usize, str: String, style: Style) -> bool {
        let chars: Vec<char> = str.chars().collect();
        for ch in chars {
            if !self.put(x, y, ch, style.clone()) {
                return false; 
            }
            x+= 1; 
        }
        return true;
    }
    fn swap(&mut self) {
        self.l.items = self.c.items.clone();     
    }
    fn get_diff(&self) -> Vec<Diff> {
        self.l.items.iter()
            .zip(self.c.items.iter()) 
            .enumerate()
            .filter(|(_, (a, b))| a != b)
            .map(|(i, (_, b))| {
                Diff {
                    pos: self.c.get_pos(i),
                    cell: b.clone(),
                }
            })
            .collect()
    }
    pub fn flush(&mut self) {
        let term = Term::stdout();
        let diffs = self.get_diff();
        for diff in diffs {
            term.move_cursor_to(diff.pos.0, diff.pos.1).unwrap();
            print!("{}", diff.cell.style.apply_to(diff.cell.ch));
        }
        self.swap();
    }
}

