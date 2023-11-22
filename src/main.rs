extern crate ncurses;

use ncurses::*;
use std::cmp::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

type Id = usize;

#[derive(Default)]
struct Ui{
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}
impl Ui {
    fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    fn begin_list(&mut self, id: Id) {
        assert!(self.list_curr.is_none(), "Nested list are not allowed.");
        self.list_curr = Some(id);
    }

    fn label(&mut self, text: &str, pair: i16) {
        mv(self.row, self.col);
        attron(pair);
        addstr(text);
        attroff(pair);
        self.row += 1;
    }

    fn list_element(&mut self, label: &str, id: Id) {
        let pair = {
            if todo_curr == index {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            }
        };

        attron(COLOR_PAIR(pair));
        mv(index as i32, 1);
        addstr(*todo);
        attroff(COLOR_PAIR(pair));
    }

    fn end_list(&mut self) {
        todo!()
    }

    fn end(&mut self) {
        todo!()
    }
}
fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let todos: Vec<String> = vec![
        "Buy bread.".to_string(),
        "And millk".to_string(),
        "Make more tea".to_string()
    ];

    let mut todo_curr: usize = 1;
    let dones = Vec::<String>::new();
    let mut done_curr: usize = 0;

    let mut ui = Ui::default();

    while !quit {
        ui.begin();
        {
            ui.begin_list(todo_curr);
        
            for (index, todo) in todos.iter().enumerate() {
                ui.list_element(todo, index);

            }
            
            ui.end_list();

            ui.label("------------------");

            ui.begin_list(done_curr);
            for (index, done) in dones.iter().enumerate() {
                ui.list_element(done, index);
            }
            ui.end_list();
        }
        
        ui.end();

        refresh();
        
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'w' => if todo_curr > 0 {
                todo_curr -= 1;
            },
            's' => todo_curr = min(todo_curr + 1, todos.len() -1),
            _=> {}
        }
    }

    endwin();
}
