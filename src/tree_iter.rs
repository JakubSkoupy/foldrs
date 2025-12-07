use crate::tree::Line;
use crate::tree::Tree;
use crate::tree::TreeNode;

pub struct TextLine {
    pub text: String,
    pub level: usize,
}

// Immutable iterator
pub struct TreeNodeIterator<'a> {
    stack: Vec<&'a TreeNode>, // Nothing else is needed really. At least now
}

impl<'a> Iterator for TreeNodeIterator<'a> {
    type Item = &'a TreeNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        for child in node.children.iter().rev() {
            self.stack.push(child);
        }
        Some(node)
    }
}

// Mutable iterator
pub struct TreeNodeForeachIterator<'a, F>
where
    F: FnMut(&'a mut TreeNode) -> &'a mut TreeNode,
{
    stack: Vec<&'a mut TreeNode>,
    function: F,
}

impl<'a, F> Iterator for TreeNodeForeachIterator<'a, F>
where
    F: FnMut(&'a mut TreeNode) -> &'a mut TreeNode,
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        let node = (self.function)(node);

        for child in node.children.iter_mut().rev() {
            self.stack.push(child);
        }
        Some(())
    }
}

impl Tree {
    pub fn nodes_iter(&self) -> TreeNodeIterator<'_> {
        let stack: Vec<&TreeNode> = self.roots.iter().collect();
        TreeNodeIterator { stack }
    }

    pub fn foreach_node<'a, F>(&'a mut self, f: F) -> TreeNodeForeachIterator<'a, F>
    where
        F: FnMut(&mut TreeNode) -> &'a mut TreeNode,
    {
        let stack: Vec<&'a mut TreeNode> = self.roots.iter_mut().collect();
        TreeNodeForeachIterator { stack, function: f }
    }

    pub fn lines_iter(&self) -> impl Iterator<Item = TextLine> {
        self.nodes_iter().flat_map(|node| {
            node.lines_iter().map(|x| TextLine {
                text: x.full_line.clone(),
                level: node.level,
            })
        })
    }
}
