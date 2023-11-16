extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    addstr("Hello, world!");

    refresh();

    let mut quit = false;
    let mut todos = vec![
        "Buy bread.",
        "And millk",
        "Make more tea",
    ]; 


    while !quit {
        let key = getch();

        for (row, todo) in todos.iter().enumerate() {
            mv(row as i32, 0);
            addstr(*todo);
        }

        match key as u8 as char {
            'q' => quit = true,
            _=> {}
        }
    }

    endwin();
}
