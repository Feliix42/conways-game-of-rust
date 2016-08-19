extern crate pancurses;
extern crate rand;

use rand::{thread_rng, Rng};

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



// TODO Size as i32?
fn seed_field(size_y: u32, size_x: u32) -> Vec<Vec<CellState>> {
    // initialize a random number generator
    let mut rng = thread_rng();
    let mut generator = rng.gen_iter::<u32>();

    // generate a size_x * size_y field of cells
    let mut field: Vec<Vec<CellState>> = vec![vec![CellState::Dead; size_x as usize]; size_y as usize];


    for line in field.iter_mut() {
        for cell in line.iter_mut() {
            if generator.next().expect("You ran out of random numbers!") > ((u32::max_value() as f64) * 0.6) as u32 {
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
        for (pos, element) in line.iter().enumerate() {
            let symbol: char = match *element {
                CellState::Alive => 'x',
                CellState::Dead  => ' '
            };
            win.mvaddch(y, (pos * 2) as i32, symbol);
        }
        y += 1;
    }
    win.refresh();
}


fn main() {
    // initialize the ncurses window
    let win = pancurses::initscr();
    pancurses::noecho();

    // er, this is just for test purposes, ya know?
    // win.mv(4, 14);
    // win.printw("wow");

    //initially seed the field and print it
    let field = seed_field(20, 15);
    draw(&win, &field);

    let _ = win.getch();

    // delete the window and close the ncurses session
    pancurses::delwin(win);
    pancurses::endwin();
}
