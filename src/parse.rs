/* So far let's just parse indent of size 4 */

use crate::tree::Tree;
use crate::tree::TreeNode;

fn get_indentation(line: &str) -> usize {
    line.chars()
        .take_while(|x| x.is_whitespace() && *x != '\n')
        .map(|_| 1)
        .sum()
}

pub fn parse<'a, I>(input: &mut std::iter::Peekable<I>) -> Tree
where
    I: Iterator<Item = String>,
{
    Tree::from_roots(parse_rec(input, 0))
}

pub fn parse_rec<I>(input: &mut std::iter::Peekable<I>, level: usize) -> Vec<TreeNode>
where
    I: Iterator<Item = String>,
{
    let mut result: Vec<TreeNode> = vec![];
    let mut next = true;

    let multiline = false; // TODO make it an argument

    while let Some(line) = input.peek() {
        let indentation = get_indentation(&line);

        match result.last_mut() {
            Some(node) if !next => {
                match indentation.cmp(&node.indentation) {
                    // Expand current node.
                    std::cmp::Ordering::Equal => match multiline {
                        true => {
                            node.push_line(&input.next().unwrap());
                        }
                        false => {
                            result.push(TreeNode::new(level, indentation));
                            let node = result.last_mut().unwrap();
                            node.push_line(&input.next().unwrap());
                        }
                    },

                    // Make a new node, and go deeper.
                    std::cmp::Ordering::Greater => {
                        node.children = parse_rec(input, level + 1);
                        next = true;
                    }

                    // Finish up current node
                    std::cmp::Ordering::Less => return result,
                }
            }
            // New node (Either first, or after returning from deeper recursion)
            _ => {
                result.push(TreeNode::new(level, indentation));
                let node = result.last_mut().unwrap();
                node.push_line(&input.next().unwrap());
                next = false;
            }
        }
    }
    result
}
