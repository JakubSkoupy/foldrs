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
use crate::vectree::VecTreeCursor;

struct Viewport {
    lines_visible: usize,
    max_line: usize,

    cursor: usize,
    first_line: usize,
    cursor_pad: usize,

    highlight_substring: Option<String>,
    tree_length: usize,

    settings: Settings,
}

#[derive(Debug)]
enum Action {
    ToggleCollapse,

    Up,
    Down,

    NextSibling,
    PrevSibling,
    Parent,
}

pub enum Numbering {
    Relative,
    Absolute,
    Off,
}

pub struct Settings {
    pub numbering: Numbering,
    pub debug: bool,
}

impl Viewport {
    pub fn handle_actions(
        &mut self,
        tree: &mut VecTree,
        cursor: &mut VecTreeCursor,
        action: Action,
    ) -> Option<String> {
        if let Some(node) = tree.get(cursor.index) {
            let message = match action {
                Action::ToggleCollapse => {
                    cursor.toggle_collapse(tree);
                    None
                }
                Action::Up => {
                    self.handle_scroll(-1);
                    match cursor.prev(node) {
                        Some(_) => None,
                        _ => Some("Unable to scroll"),
                    }
                }
                Action::Down => match cursor.next(node, self.tree_length) {
                    Some(_) => {
                        self.handle_scroll(1);
                        None
                    }
                    _ => Some("Unable to scroll"),
                },
                Action::Parent => {
                    let index = cursor.index;
                    let difference = (index - cursor.parent(node)) as i32;

                    self.handle_scroll(-difference);
                    None
                }
                Action::NextSibling => {
                    let index = cursor.index as i32;
                    let difference =
                        (index - cursor.next_sibling(tree, self.tree_length)? as i32) as i32;

                    self.handle_scroll(-difference);
                    None
                }
                Action::PrevSibling => {
                    let index = cursor.index;
                    let difference = (index - cursor.prev_sibling(tree)?) as i32;
                    self.handle_scroll(-difference);

                    None
                }
            };

            return match self.settings.debug {
                true => {
                    let debug_message = format!("");
                    Some(String::from(debug_message + "\n" + message.unwrap_or("")))
                }
                _ => message.map(|x| String::from(x)),
            };
        }
        Some(String::from("Fatal error: Inavlid state"))
    }

    fn new(settings: Settings, lines_visible: usize, max_line: usize, tree_length: usize) -> Self {
        Self {
            lines_visible,
            max_line,
            cursor: 0,
            first_line: 0,
            cursor_pad: lines_visible / 4,
            highlight_substring: None,
            tree_length: tree_length,
            settings: settings,
        }
    }

    pub fn draw(&self, tree: &VecTree, message: &Option<String>) {
        use crossterm::execute;
        let padding = match message {
            Some(_) => 3,
            _ => 1,
        };

        let _ = execute!(stdout(), Clear(ClearType::All));
        let _ = execute!(stdout(), MoveTo(0, 0));

        let lines_printed: usize = tree
            .nodes_iter()
            .enumerate()
            .skip(self.first_line)
            .take(self.lines_visible - padding)
            .map(|(i, (node, node_index))| {
                node.print(i == self.cursor, node_index, &self.settings);
                let _ = execute!(stdout(), MoveToColumn(0));
                1 // To count the number of printed lines
            })
            .sum();

        for _ in lines_printed..self.lines_visible - padding {
            println!("~");
            let _ = execute!(stdout(), MoveToColumn(0));
        }

        if let Some(message) = message {
            println!("{}", message);
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

    let settings = Settings {
        numbering: Numbering::Absolute,
        debug: true,
    };

    let mut viewport = Viewport::new(settings, height as usize, lines, tree.nodes.len());
    let mut cursor = VecTreeCursor::new();
    let mut message = None;

    loop {
        viewport.draw(tree, &message);

        if let Ok(read_event) = crossterm::event::read() {
            message = match read_event {
                crossterm::event::Event::Key(code) => match code.code {
                    KeyCode::Up => viewport.handle_actions(tree, &mut cursor, Action::Up),
                    KeyCode::Down => viewport.handle_actions(tree, &mut cursor, Action::Down),
                    KeyCode::Left => viewport.handle_actions(tree, &mut cursor, Action::Parent),
                    KeyCode::Char('[') => {
                        viewport.handle_actions(tree, &mut cursor, Action::PrevSibling)
                    }
                    KeyCode::Char(']') => {
                        viewport.handle_actions(tree, &mut cursor, Action::NextSibling)
                    }
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') => {
                        viewport.handle_center();
                        None
                    }
                    KeyCode::Enter => {
                        viewport.handle_actions(tree, &mut cursor, Action::ToggleCollapse)
                    }
                    _ => None,
                },
                crossterm::event::Event::Resize(_, h) => {
                    viewport.set_size(h as usize);
                    None
                }
                _ => None,
            };
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    let _ = execute!(stdout(), cursor::Show);
    Ok(())
}
