#[macro_use]
extern crate helix;
extern crate rand;
extern crate termion;

ruby! {
    class GameOfLife {
        struct {
            width: usize,
            height: usize,
            cells: Vec<bool>,
        }

        def random() -> GameOfLife {
            use termion::terminal_size;

            let (width, height) = terminal_size().unwrap_or((100, 100));
            let mut game = GameOfLife::new(width as usize, height as usize);

            game.randomize();

            game
        }

        def parse(width: usize, height: usize, input: String) -> GameOfLife {
            let mut game = GameOfLife::new(width, height);

            for (y, line) in input.lines().enumerate() {
                assert!(y < height, "invalid input: too many lines (the game has a height of {})", height);

                for (x, c) in line.chars().enumerate() {
                    assert!(x < width, "invalid input: line {} is too long (the game has a width of {})", y+1, width);

                    if c == '*' {
                        game.set(x, y, true);
                    }
                }
            }

            game
        }

        def initialize(helix, width: usize, height: usize) {
            assert!(width > 0, "width cannot be 0");
            assert!(height > 0, "height cannot be 0");

            let mut cells = vec![];

            cells.resize(width * height, false);

            GameOfLife { helix, width, height, cells }
        }

        #[ruby_name = "[]"]
        def get(&self, x: usize, y: usize) -> bool {
            self.cell_at(x, y).is_alive()
        }

        #[ruby_name = "[]="]
        def set(&mut self, x: usize, y: usize, is_alive: bool) {
            self.mut_cell_at(x, y).set(is_alive)
        }

        #[ruby_name = "advance!"]
        def advance(&mut self) {
            let mut next = self.cells.clone();

            for cell in self.cells() {
                let index = cell.index();
                let is_alive = cell.is_alive();
                let live_neighbors = cell.live_neighbors();

                // Any live cell with fewer than two live neighbors dies, as if
                // by under population.
                if is_alive && live_neighbors < 2 {
                    next[index] = false;
                // Any live cell with more than three live neighbors dies, as
                // if by overpopulation.
                } else if is_alive && live_neighbors > 3 {
                    next[index] = false;
                // Any dead cell with exactly three live neighbors becomes a
                // live cell, as if by reproduction.
                } else if !is_alive && live_neighbors == 3 {
                    next[index] = true;
                }
            }

            self.cells = next;
        }

        #[ruby_name = "randomize!"]
        def randomize(&mut self) {
            use rand::prelude::random;

            for cell in self.cells.iter_mut() {
                *cell = random();
            }
        }

        #[ruby_name = "play!"]
        def play(&mut self) {
            use std::io::Write;
            use std::thread::sleep;
            use std::time::Duration;

            use termion::async_stdin;
            use termion::clear;
            use termion::color::*;
            use termion::cursor::{Goto, Hide, Show};
            use termion::event::Key::Char;
            use termion::input::TermRead;
            use termion::raw::IntoRawMode;
            use termion::terminal_size;

            let mut stdin = async_stdin().keys();
            let mut stdout = std::io::stdout().into_raw_mode().unwrap();

            let (terminal_width, terminal_height) = terminal_size().unwrap();

            // ~30 FPS
            let timeout = Duration::from_millis(30);

            print!("{}", clear::All);
            print!("{}", Hide);

            loop {
                for cell in self.cells() {
                    let (x, y) = cell.coordinates();

                    if x >= terminal_width as usize || y >= terminal_height as usize {
                        continue;
                    }

                    print!("{}", Goto(x as u16 + 1, y as u16 + 1));

                    if cell.is_alive() {
                        print!("{}*{}", Fg(LightYellow), Fg(Reset));
                    } else {
                        print!(".");
                    }
                }

                self.advance();

                print!("{}", Goto(1, terminal_height));
                print!("{} Press {}r{} to randomize, {}q{} to quit. {}", Bg(Blue), Fg(LightWhite), Fg(Reset), Fg(LightWhite), Fg(Reset), Bg(Reset));

                stdout.flush().unwrap();

                match stdin.next() {
                    Some(Ok(Char('r'))) => {
                        self.randomize();
                    },
                    Some(Ok(Char('q'))) => {
                        print!("{}", clear::All);
                        print!("{}", Goto(1, 1));
                        print!("{}", Show);

                        stdout.flush().unwrap();

                        return;
                    },
                    _ => ()
                }

                sleep(timeout);
            }
        }

        def to_s(&self) -> String {
            let mut s = String::with_capacity(self.cells.len() + self.height);

            for cell in self.cells() {
                if cell.is_alive() {
                    s.push('*');
                } else {
                    s.push('.');
                }

                if cell.is_last_in_row() {
                    s.push('\n');
                }
            }

            s
        }
    }
}

// These are hidden from Ruby

impl GameOfLife {
    fn index_for(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width && y < self.height, "({}, {}) is out-of-bounds", x, y);
        y * self.width + x
    }

    fn cell_at(&self, x: usize, y: usize) -> Cell {
        let index = self.index_for(x, y);
        Cell { game: self, index }
    }

    fn mut_cell_at(&mut self, x: usize, y: usize) -> MutCell {
        let index = self.index_for(x, y);
        MutCell { game: self, index }
    }

    fn cells(&self) -> Cells {
        Cells { next: Some(self.cell_at(0,0)), size: self.cells.len() }
    }
}

struct Cells<'a> {
    next: Option<Cell<'a>>,
    size: usize,
}

impl<'a> std::iter::Iterator for Cells<'a> {
    type Item = Cell<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.next {
            Some(ref cell) => cell.next(),
            None => None,
        };

        std::mem::replace(&mut self.next, next)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = match self.next {
            Some(ref cell) => self.size - cell.index(),
            None => 0,
        };

        (remaining, Some(remaining))
    }
}

impl<'a> std::iter::ExactSizeIterator for Cells<'a> {}
impl<'a> std::iter::FusedIterator for Cells<'a> {}

struct Cell<'a> {
    game: &'a GameOfLife,
    index: usize,
}

struct MutCell<'a> {
    game: &'a mut GameOfLife,
    index: usize,
}

enum Neighbor {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

trait Navigate {
    fn game(&self) -> &GameOfLife;
    fn index(&self) -> usize;

    fn coordinates(&self) -> (usize, usize) {
        (self.x(), self.y())
    }

    fn x(&self) -> usize {
        self.index() % self.game().width
    }

    fn y(&self) -> usize {
        self.index() / self.game().width
    }

    fn is_alive(&self) -> bool {
        self.game().cells[self.index()]
    }

    fn is_last_in_row(&self) -> bool {
        self.neighbor(Neighbor::Right).is_none()
    }

    fn is_last(&self) -> bool {
        self.index() == self.game().cells.len() - 1
    }

    fn live_neighbors(&self) -> usize {
        use Neighbor::*;

        let neighbors = vec![
            self.neighbor(TopLeft),
            self.neighbor(Top),
            self.neighbor(TopRight),
            self.neighbor(Left),
            self.neighbor(Right),
            self.neighbor(BottomLeft),
            self.neighbor(Bottom),
            self.neighbor(BottomRight),
        ];

        neighbors.into_iter()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .map(|cell| if cell.is_alive() { 1 } else { 0 })
            .sum()
    }

    fn neighbor(&self, position: Neighbor) -> Option<Cell> {
        use Neighbor::*;

        let (x, y) = self.coordinates();
        let game = self.game();

        let mut nx = x;
        let mut ny = y;

        match position {
            TopLeft     => { nx -= 1; ny -= 1; },
            Top         => {          ny -= 1; },
            TopRight    => { nx += 1; ny -= 1; },
            Left        => { nx -= 1;          },
            Right       => { nx += 1;          },
            BottomLeft  => { nx -= 1; ny += 1; },
            Bottom      => {          ny += 1; },
            BottomRight => { nx += 1; ny += 1; },
        }

        if nx >= game.width || ny >= game.height {
            None
        } else {
            Some(Cell { game, index: game.index_for(nx, ny) })
        }
    }
}

impl<'a> Navigate for Cell<'a> {
    fn game(&self) -> &GameOfLife {
        self.game
    }

    fn index(&self) -> usize {
        self.index
    }
}

impl<'a> Navigate for MutCell<'a> {
    fn game(&self) -> &GameOfLife {
        self.game
    }

    fn index(&self) -> usize {
        self.index
    }
}

impl<'a> Cell<'a> {
    // TODO: this should be in Navigate
    fn next(&self) -> Option<Cell<'a>> {
        if self.is_last() {
            None
        } else {
            Some(Cell { game: self.game, index: self.index + 1 })
        }
    }
}

impl<'a> MutCell<'a> {
    fn set(&mut self, value: bool) {
        self.game.cells[self.index] = value
    }
}
