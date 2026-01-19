use crate::tree::Line;

pub struct StackTreeNode {
    lines: Vec<Line>,
    pub(crate) collapsed: bool,

    next_sibling_offset: Option<usize>,
}

impl StackTreeNode {
    pub fn is_terminal(&self) -> bool {
        self.next_sibling_offset.is_none() || self.next_sibling_offset == Some(0)
    }
}

pub struct StackTree {
    pub nodes: Vec<StackTreeNode>,
}

pub struct StackTreeIterator<'a> {
    index: usize,
    depth: usize,

    tree: &'a StackTree,
    last_sibling_stack: Vec<usize>,
}

impl<'a> Iterator for StackTreeIterator<'a> {
    type Item = &'a StackTreeNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.tree.nodes[self.index];
        self.index += 1;

        match node.next_sibling_offset {
            None /* Last node */ => { None}
            Some(offset) if offset == 1 => /* Sibling or less deep */ {}
            Some(offset) => {}
        }
    }
}
