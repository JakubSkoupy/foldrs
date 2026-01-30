use crate::tree::Line;
use std::marker::PhantomData;

pub struct VecTreeNode {
    pub lines: Vec<Line>,
    pub(crate) collapsed: bool,

    pub subtree_size: usize,
}

pub struct VecTree {
    pub nodes: Vec<VecTreeNode>,
}

impl VecTreeNode {
    pub fn new() -> Self {
        Self {
            lines: vec![],
            collapsed: false,
            subtree_size: 0, // Excluding this node
        }
    }

    pub fn push_line(&mut self, line: &str) -> () {
        self.lines.push(Line::new(line.to_string()));
    }
}

// ----------------------------------------------------------------------------------------

fn get_indentation(line: &str) -> usize {
    line.chars()
        .take_while(|x| x.is_whitespace() && *x != '\n')
        .map(|_| 1)
        .sum()
}

pub fn parse_stacktree_rec<I>(
    input: &mut std::iter::Peekable<I>,
    nodes: &mut Vec<VecTreeNode>,
    depth: usize,
) -> usize
where
    I: Iterator<Item = String>,
{
    let mut size = 0; // Return size to parent node

    while let Some(line) = input.peek() {
        let line_depth = get_indentation(line);

        match line_depth.cmp(&depth) {
            // Sibling
            std::cmp::Ordering::Equal => {
                nodes.push(VecTreeNode::new());
                nodes.last_mut().unwrap().push_line(&input.next().unwrap());
                size += 1;
            }

            // Child subtree
            std::cmp::Ordering::Greater => {
                let node_index = nodes.len() - 1;

                let last_node_subtree_size = parse_stacktree_rec(input, nodes, depth + 1);

                nodes[node_index].subtree_size = last_node_subtree_size;
                size += last_node_subtree_size;
            }

            // End of this subtree
            std::cmp::Ordering::Less => break,
        }
    }

    size
}

// Iter -------------------------------------------------------------------------------------

pub struct StackTreeIterator<'a> {
    index: usize,
    depth: usize,

    stack: Vec<usize>, // keeps the remaining nodes in subtrees

    tree: &'a VecTree,
}

impl<'a> Iterator for StackTreeIterator<'a> {
    type Item = (&'a VecTreeNode, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let current_node = &self.tree.nodes.get(self.index)?;
        let ret_depth = self.depth;

        self.index += 1 + match current_node.collapsed {
            true => current_node.subtree_size,
            _ => 0,
        };

        self.depth = match current_node.collapsed {
            // Go deeper
            false if current_node.subtree_size > 1 => {
                self.stack.push(current_node.subtree_size);
                self.depth + 1
            }
            // Sibling or less deep
            is_collapsed => {
                let pop_count = 1 + if is_collapsed {
                    current_node.subtree_size
                } else {
                    0
                };

                let mut depth = self.depth;
                while let Some(last) = self.stack.last_mut() {
                    *last -= pop_count;

                    if *last > 0 {
                        break;
                    }

                    self.stack.pop();
                    depth -= 1;
                }
                depth
            }
        };

        Some((current_node, ret_depth))
    }
}

pub struct StackTreeMutIterator<'a> {
    index: usize,
    depth: usize,

    stack: Vec<usize>, // keeps the remaining nodes in subtrees

    tree: *mut VecTree,
    _marker: PhantomData<&'a mut VecTreeNode>,
}

impl<'a> Iterator for StackTreeMutIterator<'a> {
    type Item = (&'a mut VecTreeNode, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        unsafe {
            let current_node = (*self.tree).nodes.get_mut(index)?;

            let ret_depth = self.depth;

            self.index += 1 + match current_node.collapsed {
                true => current_node.subtree_size,
                _ => 0,
            };

            self.depth = match current_node.collapsed {
                // Go deeper
                false if current_node.subtree_size > 1 => {
                    self.stack.push(current_node.subtree_size);
                    self.depth + 1
                }
                // Sibling or less deep
                is_collapsed => {
                    let pop_count = 1 + if is_collapsed {
                        current_node.subtree_size
                    } else {
                        0
                    };

                    let mut depth = self.depth;
                    while let Some(last) = self.stack.last_mut() {
                        *last -= pop_count;

                        if *last > 0 {
                            break;
                        }

                        self.stack.pop();
                        depth -= 1;
                    }
                    depth
                }
            };
            Some((current_node, ret_depth)) // Return mutable reference to the node
        }
    }
}

impl VecTree {
    pub fn nodes_iter(&self) -> StackTreeIterator<'_> {
        StackTreeIterator {
            index: 0,
            depth: 0,
            stack: vec![],
            tree: self,
        }
    }

    pub fn nodes_iter_mut(&mut self) -> StackTreeMutIterator<'_> {
        StackTreeMutIterator {
            index: 0,
            depth: 0,
            stack: vec![],
            tree: self,
            _marker: PhantomData,
        }
    }
}
