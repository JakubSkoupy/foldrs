#[derive(Debug)]
pub struct Line {
    pub full_line: String,
}

impl Line {
    fn new(full_line: String) -> Self {
        Self { full_line }
    }

    /// Returns a string slice of the [`Line`] content without
    /// indentation.
    fn line(&self, indentation: usize) -> &str {
        &self.full_line[indentation..]
    }
}

/* Represents one tree node. So probably one line from the input */
#[derive(Debug)]
pub struct TreeNode {
    pub(crate) indentation: usize,
    pub(crate) level: usize,
    lines: Vec<Line>, // Represents the string content.

    pub(crate) children: Vec<TreeNode>,
    pub(crate) collapsed: bool,
}

impl TreeNode {
    pub fn new(level: usize, indentation: usize) -> Self {
        Self {
            lines: vec![],
            children: vec![],
            collapsed: false,
            level,
            indentation,
        }
    }

    pub fn lines_iter(&self) -> std::slice::Iter<'_, Line> {
        self.lines.iter()
    }

    pub fn lines_iter_mut(&mut self) -> std::slice::IterMut<'_, Line> {
        self.lines.iter_mut()
    }

    pub fn foreach_line_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut Line),
    {
        self.lines.iter_mut().for_each(f)
    }

    /// Adds a line of text to the node.
    pub fn push_line(&mut self, line: &str) -> () {
        self.lines.push(Line::new(line.to_string()));
    }

    pub fn toggle_collapse(&mut self) {
        self.collapsed = !self.collapsed;
    }

    pub fn print_rec(&self) -> () {
        // println!("{} Level: {}", "-".repeat(self.indentation), &self.level);
        for line in &self.lines {
            println!(
                "{}",
                " ".repeat(self.indentation) + " " + line.line(self.indentation)
            );
        }

        for child in &self.children {
            child.print_rec();
        }
    }
}

#[derive(Debug)]
pub struct Tree {
    pub roots: Vec<TreeNode>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { roots: vec![] }
    }

    pub fn from_roots(roots: Vec<TreeNode>) -> Self {
        Tree { roots }
    }

    pub fn print(&self) -> () {
        for root in &self.roots {
            root.print_rec();
        }
    }
}
