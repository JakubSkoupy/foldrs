use std::cmp::max;
use std::io::Stdout;

use crossterm::cursor::{MoveTo, MoveToColumn};
use crossterm::event::KeyCode;
use crossterm::terminal;
use crossterm::terminal::{Clear, ClearType, size};

use anyhow::Error;
use anyhow::Result;
use std::io::stdout;

use crate::tree::Tree;

struct Viewport {
    lines_visible: usize,
    max_line: usize,

    cursor: usize,
    first_line: usize,
    cursor_pad: usize,

    highlight_substring: Option<String>,
}

impl Viewport {
    fn new(lines_visible: usize, max_line: usize) -> Self {
        Self {
            lines_visible,
            max_line,
            cursor: 0,
            first_line: 0,
            cursor_pad: lines_visible / 4,
            highlight_substring: None,
        }
    }

    pub fn draw(&self, tree: &Tree) {
        use crossterm::execute;

        let _ = execute!(stdout(), Clear(ClearType::All));
        let _ = execute!(stdout(), MoveTo(0, 0));

        tree.lines_iter()
            .enumerate()
            .skip(self.first_line)
            .take(self.lines_visible)
            .for_each(|(i, line)| {
                // Print line
                match i == self.cursor {
                    true => {
                        println!("==> {} ================", line.text);
                    }
                    false => println!("    {}", line.text),
                }

                let _ = execute!(stdout(), MoveToColumn(0));
            });
    }

    pub fn handle_scroll(&mut self, lines: i32) {
        let total_lines = self.max_line; // implement this to return buffer length
        let new_cursor =
            (self.cursor as i32 + lines).clamp(0, (total_lines.saturating_sub(1)) as i32) as usize;
        self.cursor = new_cursor;

        let padding_up = self.cursor.saturating_sub(self.first_line);
        let padding_down = self.first_line + self.lines_visible - 1 - self.cursor;

        if padding_up < self.cursor_pad {
            self.first_line = self.cursor.saturating_sub(self.cursor_pad);
        } else if padding_down < self.cursor_pad {
            self.first_line = self
                .cursor
                .saturating_sub(self.lines_visible.saturating_sub(self.cursor_pad + 1));
        }

        if total_lines > self.lines_visible {
            self.first_line = self.first_line.min(total_lines - self.lines_visible);
        } else {
            self.first_line = 0;
        }
    }

    // pub fn handle_scroll(&mut self, lines: i32) -> () {
    //     let new_line = self.cursor as i64 + lines as i64;
    //
    //     let padding_up = self.cursor - self.first_line;
    //     let padding_down = self.lines_visible - self.cursor;
    //
    //     let move_viewport: i64 = {
    //         if padding_up < self.cursor_pad {
    //             -1
    //         } else if padding_down < self.cursor_pad {
    //             0
    //         } else {
    //             1
    //         }
    //     };
    //     if new_line >= 0 {
    //         self.cursor = new_line as usize;
    //         self.first_line = (self.first_line as i64 + move_viewport) as usize; // TODO not like this
    //     }
    // }

    pub fn handle_center(&mut self) {
        self.first_line = max(0, self.cursor as i64 - self.lines_visible as i64 / 2) as usize;
    }

    pub fn set_size(&mut self, height: usize) {
        self.lines_visible = height;
    }
}

pub fn main_loop(tree: &Tree, lines: usize) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;

    let (_width, height) = crossterm::terminal::size()?;
    let mut viewport = Viewport::new(height as usize, lines);

    loop {
        viewport.draw(tree);

        if let Ok(read_event) = crossterm::event::read() {
            match read_event {
                crossterm::event::Event::Key(code) => match code.code {
                    KeyCode::Up => viewport.handle_scroll(-1),
                    KeyCode::Down => viewport.handle_scroll(1),
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') => viewport.handle_center(),
                    _ => {}
                },
                crossterm::event::Event::Resize(_, h) => viewport.lines_visible = h as usize,
                _ => {}
            }
        }
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
