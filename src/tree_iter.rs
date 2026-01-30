/* DEPRECATED */
use std::marker::PhantomData;

use crate::tree::Tree;
use crate::tree::TreeNode;

pub struct TextLine {
    pub text: String,
    pub level: usize,

    leaf: bool,
    collapse: bool,
}

impl TextLine {
    fn print_inner(&self) -> String {
        match self.leaf {
            true => format!("    { }", self.text),
            false => match self.collapse {
                true => format!("    { } >>> ", self.text),
                false => format!("    { } vvv", self.text),
            },
        }
    }

    pub fn print(&self, cursor: bool) {
        match cursor {
            true => println!("==> {} ================", self.print_inner()),
            false => println!("    {}                 ", self.print_inner()),
        }
    }
}

// Immutable iterator
pub struct TreeNodeIterator<'a> {
    stack: Vec<&'a TreeNode>, // Nothing else is needed really. At least now
}

impl<'a> Iterator for TreeNodeIterator<'a> {
    type Item = &'a TreeNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        if !node.collapsed {
            for child in node.children.iter().rev() {
                self.stack.push(child);
            }
        }
        Some(node)
    }
}

// Mutable iterator
pub struct TreeNodeMutIterator<'a> {
    stack: Vec<*mut TreeNode>, // Nothing else is needed really. At least now
    _marker: PhantomData<&'a mut TreeNode>,
}

impl<'a> Iterator for TreeNodeMutIterator<'a> {
    type Item = &'a mut TreeNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node_ptr = self.stack.pop()?;

        unsafe {
            // TODO use lib or implement safe iterator
            let node = &mut *node_ptr;

            if !node.collapsed {
                for child in node.children.iter_mut().rev() {
                    self.stack.push(child as *mut _);
                }
            }

            Some(node)
        }
    }
}

impl Tree {
    pub fn nodes_iter(&self) -> TreeNodeIterator<'_> {
        let stack: Vec<&TreeNode> = self.roots.iter().collect();
        TreeNodeIterator { stack }
    }

    pub fn nodes_iter_mut<'a>(&mut self) -> TreeNodeMutIterator<'a> {
        let stack: Vec<*mut TreeNode> = self.roots.iter_mut().map(|n| n as *mut _).collect();
        TreeNodeMutIterator {
            stack,
            _marker: PhantomData,
        }
    }

    pub fn lines_iter(&self) -> impl Iterator<Item = TextLine> {
        self.nodes_iter().flat_map(|node| {
            node.lines_iter().map(|x| TextLine {
                text: x.full_line.clone(),
                level: node.level,
                collapse: node.collapsed,
                leaf: node.is_leaf(),
            })
        })
    }
}
