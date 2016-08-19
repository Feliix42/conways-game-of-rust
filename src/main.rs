extern crate pancurses;

fn main() {
    // initialize the ncurses window
    let win = pancurses::initscr();
    pancurses::noecho();

    // er, this is just for test purposes, ya know?
    win.mv(4, 14);
    win.printw("wow");


    win.refresh();
    let _ = win.getch();

    // delete the window and close the ncurses session
    pancurses::delwin(win);
    pancurses::endwin();
}
