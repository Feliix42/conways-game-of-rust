extern crate pancurses;
extern crate rand;

use rand::{thread_rng, Rng};
use std::{thread, time};

#[derive(Debug)]
enum CellState {
    Alive,
    Dead
}

impl Clone for CellState {
    fn clone(&self) -> CellState {
        match *self {
            CellState::Alive => CellState::Alive,
            CellState::Dead  => CellState::Dead
        }
    }
}

impl PartialEq for CellState {
    fn eq(&self, other: &CellState) -> bool {
        let this = match *self {
            CellState::Alive => true,
            CellState::Dead  => false
        };

        let the_other = match *other {
            CellState::Alive => true,
            CellState::Dead  => false
        };

        if this && the_other || (!this && !the_other) {
            true
        } else {
            false
        }
    }
}



// TODO Size as i32?
fn seed_field(size_y: u32, size_x: u32) -> Vec<Vec<CellState>> {
    let size_y = if size_y >= 3 { size_y } else { 4 };
    let size_x = if size_x >= 3 { size_x } else { 4 };

    // initialize a random number generator
    let mut rng = thread_rng();
    let mut generator = rng.gen_iter::<u32>();

    // generate a size_x * size_y field of cells
    let mut field: Vec<Vec<CellState>> = vec![vec![CellState::Dead; size_x as usize]; size_y as usize];


    for line in field.iter_mut() {
        for cell in line.iter_mut() {
            if generator.next().expect("You ran out of random numbers!") > ((u32::max_value() as f64) * 0.85) as u32 {
                *cell = CellState::Alive;
            }
        }
    }
    field
}


fn draw(win: &pancurses::Window, field: &Vec<Vec<CellState>>) {
    // TODO Check size, better adjustment of the Cells
    let mut y: i32 = 0;

    for line in field.iter() {
        win.mv(y, 0);
        // TODO Might want to use zip over enumerate to get an i32
        for cell in 0..line.len() {
            let symbol: char = match line[cell] {
                CellState::Alive => 'x',
                CellState::Dead  => ' '
            };
            win.mvaddch(y, (cell * 2) as i32, symbol);
        }
        y += 1;
    }
    win.refresh();
}


fn count_living_neighbours(field: &Vec<Vec<CellState>>, y: usize, x: usize) -> u8 {
    let mut neighbours: u8 = 0;

    let lower_x_ok = x > 0;
    let upper_x_ok = (x+1) < field[0].len();  // that's ok since the field size is guaranteed to be >= 3x3
    let lower_y_ok = y > 0;
    let upper_y_ok = (y+1) < field.len();

    if lower_y_ok {
        if field[y-1][x] == CellState::Alive { neighbours += 1; }

        if upper_x_ok {
            if field[y-1][x+1] == CellState::Alive { neighbours += 1; }
        }

        if lower_x_ok {
            if field[y-1][x-1] == CellState::Alive { neighbours += 1; }
        }
    }

    if upper_y_ok {
        if field[y+1][x] == CellState::Alive { neighbours += 1; }

        if upper_x_ok {
            if field[y+1][x+1] == CellState::Alive { neighbours += 1; }
        }

        if lower_x_ok {
            if field[y+1][x-1] == CellState::Alive { neighbours += 1; }
        }
    }

    if upper_x_ok {
        if field[y][x+1] == CellState::Alive { neighbours += 1; }
    }

    if lower_x_ok {
        if field[y][x-1] == CellState::Alive { neighbours += 1; }
    }

    neighbours
}



fn evolve(field: Vec<Vec<CellState>>) -> Vec<Vec<CellState>> {
    let mut next_gen = field.clone();

    for line in 0..next_gen.len() {
        for cell in 0..next_gen[line].len() {
            if field[line][cell] == CellState::Dead {
                next_gen[line][cell] = match count_living_neighbours(&field, line, cell) {
                    3 => CellState::Alive,
                    _ => CellState::Dead
                };
            } else {
                next_gen[line][cell] = match count_living_neighbours(&field, line, cell) {
                    0 | 1 => CellState::Alive,
                    2 | 3 => CellState::Alive,
                    _     => CellState::Dead
                };
            }
        }
    }
    next_gen
}


fn main() {
    // initialize the ncurses window
    let win = pancurses::initscr();
    pancurses::noecho();

    // er, this is just for test purposes, ya know?
    // win.mv(4, 14);
    // win.printw("wow");

    //initially seed the field and print it
    let mut field = seed_field(20, 15);
    loop {
        draw(&win, &field);
        field = evolve(field);
        thread::sleep(time::Duration::from_millis(500));
    }

    let _ = win.getch();

    // delete the window and close the ncurses session
    pancurses::delwin(win);
    pancurses::endwin();
}
