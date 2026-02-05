use std::marker::PhantomData;

use crate::interact::Settings;

#[derive(Debug)]
pub struct Line {
    pub full_line: String,
}

impl Line {
    pub fn new(full_line: String) -> Self {
        Self { full_line }
    }
}

pub struct VecTreeNode {
    pub lines: Vec<Line>,
    pub subtree_size: usize,
    pub depth: usize,
    pub parent_index: usize, // If == this index, then root.

    // Collapse state
    pub next_node_previous_sibling_offset: usize,
    pub jump_back_stack: Vec<usize>, // TODO handle differently
    pub collapsed: bool,
}

pub struct VecTree {
    pub nodes: Vec<VecTreeNode>,
}

impl VecTreeNode {
    pub fn new(parent_index: usize, depth: usize, text: String) -> Self {
        Self {
            lines: vec![Line::new(text)],
            collapsed: false,
            subtree_size: 0, // Excluding this node
            jump_back_stack: vec![1],
            depth: depth,
            parent_index: parent_index,
            next_node_previous_sibling_offset: 1,
        }
    }

    pub fn collapse_jump(&mut self, distance: usize) {
        self.jump_back_stack.push(distance);
    }

    pub fn uncollapse_jump(&mut self) {
        // If there are jumps a, b such as a > b, a will be later in the stack.
        // let's assume a is earlier in the stack, that would mean b is added.
        // but in order for b to be added, a can't exist.
        self.jump_back_stack.pop();
    }

    pub fn push_line(&mut self, line: String) -> () {
        self.lines.push(Line::new(line));
    }

    pub fn lines_iter(&self) -> std::slice::Iter<'_, Line> {
        self.lines.iter()
    }

    fn format_text(&self, settings: &Settings, prefix: &str) -> String {
        let collapse_sign = " ▼ "; // dim grey
        let expand_sign = " ▶ "; // dim grey

        let mut string = String::new();
        string += prefix;
        string += &self.lines[0].full_line;

        string += match (self.subtree_size == 0, self.collapsed) {
            (false, true) => collapse_sign,
            (false, false) => expand_sign,
            _ => "    ",
        };
        string
    }

    pub fn print(&self, cursor: bool, index: usize, settings: &Settings) {
        let prefix = match settings.numbering {
            crate::interact::Numbering::Relative => todo!(),
            crate::interact::Numbering::Absolute => format!("{}  ", index),
            crate::interact::Numbering::Off => String::new(),
        };

        match cursor {
            true => println!("    \x1b[7m{}\x1b[0m", self.format_text(settings, &prefix)),
            false => println!(
                "    {}                 ",
                self.format_text(settings, &prefix)
            ),
        }
    }
}

// ----------------------------------------------------------------------------------------

fn get_indentation(line: &str) -> usize {
    line.chars()
        .take_while(|x| x.is_whitespace() && *x != '\n')
        .map(|_| 1)
        .sum()
}

pub fn parse_vectree<I>(input: &mut std::iter::Peekable<I>) -> VecTree
where
    I: Iterator<Item = String>,
{
    let mut tree = VecTree { nodes: vec![] };
    parse_stacktree_rec(input, &mut tree.nodes, 0, 0);
    tree
}

pub fn parse_stacktree_rec<I>(
    input: &mut std::iter::Peekable<I>,
    nodes: &mut Vec<VecTreeNode>,
    parent_index: usize,
    depth: usize,
) where
    I: Iterator<Item = String>,
{
    while let Some(line) = input.peek() {
        let line_depth = get_indentation(line);

        match line_depth.cmp(&depth) {
            // Sibling
            std::cmp::Ordering::Equal => {
                nodes.push(VecTreeNode::new(parent_index, depth, input.next().unwrap()));
            }

            // Child subtree
            std::cmp::Ordering::Greater => {
                let node_index = nodes.len() - 1;
                parse_stacktree_rec(input, nodes, node_index, depth + 1);

                let subtree_size = nodes.len() - node_index - 1;
                nodes[node_index].subtree_size = subtree_size;
                nodes
                    .last_mut()
                    .map(|x| x.next_node_previous_sibling_offset = subtree_size + 1);
            }

            // End of this subtree
            std::cmp::Ordering::Less => break,
        }
    }
}

// Iter -------------------------------------------------------------------------------------

pub struct VecTreeNodeIterator<'a> {
    index: usize,
    tree: &'a VecTree,
}

impl<'a> Iterator for VecTreeNodeIterator<'a> {
    type Item = (&'a VecTreeNode, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let current_node = &self.tree.nodes.get(self.index)?;
        let index = self.index;

        self.index += 1 + match current_node.collapsed {
            true => current_node.subtree_size,
            _ => 0,
        };

        Some((current_node, index))
    }
}

pub struct VecTreeCursor {
    pub index: usize,
}

impl VecTreeCursor {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn collapse(&mut self, tree: &mut VecTree) {
        let subtree_size = match tree.get_mut(self.index) {
            Some(node) => {
                node.collapsed = true;
                Some(node.subtree_size)
            }
            _ => None,
        };

        if let Some(jump) = subtree_size {
            if let Some(node) = tree.get_mut(self.index + jump + 1) {
                node.collapse_jump(jump + 1);
            }
        }
    }

    pub fn uncollapse(&mut self, tree: &mut VecTree) {
        let subtree_size = match tree.get_mut(self.index) {
            Some(node) => {
                node.collapsed = false;
                Some(node.subtree_size)
            }
            _ => None,
        };

        if let Some(jump) = subtree_size {
            if let Some(node) = tree.get_mut(self.index + jump + 1) {
                node.uncollapse_jump();
            }
        }
    }

    pub fn collapse_subtree(&mut self, tree: &mut VecTree) {
        todo!()
    }

    pub fn uncollapse_subtree(&mut self, tree: &mut VecTree) {
        todo!()
    }

    pub fn toggle_collapse(&mut self, tree: &mut VecTree) {
        if let Some(node) = tree.get(self.index) {
            match node.collapsed {
                true => self.uncollapse(tree),
                _ => self.collapse(tree),
            }
        }
    }

    pub fn next_sibling(&mut self, tree: &VecTree, length: usize) -> Option<usize> {
        let node = tree.get(self.index)?;
        match self.index + 1 + node.subtree_size {
            index if index < length => {
                let next_node = tree.get(index)?;

                if next_node.depth != node.depth {
                    return None;
                }
                self.index = index
            }
            _ => return None,
        }

        Some(self.index)
    }

    pub fn prev_sibling(&mut self, tree: &VecTree) -> Option<usize> {
        let prev_node = tree.get(self.index - 1)?;
        let node = tree.get(self.index)?;

        let index = self
            .index
            .checked_sub(prev_node.next_node_previous_sibling_offset)?;

        match tree.get(index) {
            Some(n) if n.depth == node.depth => {
                self.index = index;
                Some(index)
            }
            _ => None,
        }
    }

    pub fn parent(&mut self, node: &VecTreeNode) -> usize {
        self.index = node.parent_index;
        self.index
    }

    pub fn prev(&mut self, node: &VecTreeNode) -> Option<usize> {
        match node.jump_back_stack.last() {
            Some(jump) => {
                self.index = self.index.checked_sub(*jump)?;
            }
            _ => {
                self.index = self.index.checked_sub(1)?;
            }
        }
        Some(self.index)
    }

    pub fn next(&mut self, node: &VecTreeNode, length: usize) -> Option<usize> {
        let index = self.index;

        let increment = match node.collapsed {
            true => node.subtree_size + 1,
            _ => 1,
        };

        // Safe add
        match index + increment {
            index if index < length => self.index = index,
            _ => return None,
        }

        Some(self.index)
    }
}

impl VecTree {
    pub fn get(&self, index: usize) -> Option<&VecTreeNode> {
        self.nodes.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut VecTreeNode> {
        self.nodes.get_mut(index)
    }

    pub fn nodes_iter(&self) -> VecTreeNodeIterator<'_> {
        VecTreeNodeIterator {
            index: 0,
            tree: self,
        }
    }
}
