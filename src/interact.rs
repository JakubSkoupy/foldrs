use std::cmp::max;

use crossterm::cursor;
use crossterm::cursor::MoveTo;
use crossterm::cursor::MoveToColumn;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;

use anyhow::Result;
use std::io::stdout;

use crate::vectree::VecTree;

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

    pub fn handle_events(&self, tree: &mut VecTree, collapse: bool) {
        use crossterm::execute;

        let _ = execute!(stdout(), Clear(ClearType::All));
        let _ = execute!(stdout(), MoveTo(0, 0));

        let mut line_index = self.first_line;
        tree.nodes_iter_mut()
            .skip(self.first_line)
            .take(self.lines_visible)
            .for_each(|(node, deph)| {
                for _ in 0..node.lines.len() {
                    if line_index == self.cursor && collapse {
                        node.toggle_collapse();
                    }
                    line_index += 1;
                }
            });
    }

    pub fn draw(&self, tree: &VecTree) {
        use crossterm::execute;

        let _ = execute!(stdout(), Clear(ClearType::All));
        let _ = execute!(stdout(), MoveTo(0, 0));

        let lines_printed: usize = tree
            .lines_iter()
            .enumerate()
            .skip(self.first_line)
            .take(self.lines_visible)
            .map(|(i, line)| {
                line.print(i == self.cursor);
                let _ = execute!(stdout(), MoveToColumn(0));
                1 // To count the number of printed lines
            })
            .sum();

        for i in lines_printed..self.lines_visible - 1 {
            match i == self.cursor {
                true => println!("~ ======================="),
                false => println!("~"),
            }

            let _ = execute!(stdout(), MoveToColumn(0));
        }
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

    /// Scrolls in the viewport so that the cursor is in the center.
    pub fn handle_center(&mut self) {
        self.first_line = max(0, self.cursor as i64 - self.lines_visible as i64 / 2) as usize;
    }

    /// Sets the vertical size of the viewport
    pub fn set_size(&mut self, height: usize) {
        self.lines_visible = height;
    }
}

pub fn main_loop(tree: &mut VecTree, lines: usize) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    let _ = execute!(stdout(), cursor::Hide);

    let (_width, height) = crossterm::terminal::size()?;
    let mut viewport = Viewport::new(height as usize, lines);

    let mut collapse_cmd = false;

    loop {
        viewport.handle_events(tree, collapse_cmd);
        collapse_cmd = false;
        viewport.draw(tree);

        if let Ok(read_event) = crossterm::event::read() {
            match read_event {
                crossterm::event::Event::Key(code) => match code.code {
                    KeyCode::Up => viewport.handle_scroll(-1),
                    KeyCode::Down => viewport.handle_scroll(1),
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') => viewport.handle_center(),
                    KeyCode::Enter => collapse_cmd = true,
                    _ => {}
                },
                crossterm::event::Event::Resize(_, h) => viewport.set_size(h as usize),
                _ => {}
            }
        }
    }
    crossterm::terminal::disable_raw_mode()?;
    let _ = execute!(stdout(), cursor::Show);
    Ok(())
}
