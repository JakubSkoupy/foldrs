mod interact;
mod parse;
mod parse_rules;
pub(crate) mod tree;
mod tree_iter;
mod vectree;

use anyhow::Error;
use anyhow::Result;

use std::io;
use std::io::BufRead;

use crate::vectree::parse_stacktree_rec;
use crate::vectree::VecTree;

// fn main() -> Result<(), Error> {
//     let stdin = io::stdin();
//
//     let mut input_iter = stdin.lock().lines().map(|l| l.unwrap()).peekable();
//     let mut tree = parse(&mut input_iter);
//
//     main_loop(&mut tree, 100)
// }

fn main() -> Result<(), Error> {
    let stdin = io::stdin();

    let mut input_iter = stdin.lock().lines().map(|l| l.unwrap()).peekable();

    let mut tree = VecTree { nodes: vec![] };
    let _ = parse_stacktree_rec(&mut input_iter, &mut tree.nodes, 0);

    // tree.nodes[3].collapsed = true;

    for (node, depth) in tree.nodes_iter_mut() {
        node.lines[0].full_line += "Visited";
        println!(
            "NODE: {} | depth: {} | st: {}",
            node.lines[0].full_line, depth, node.subtree_size
        );

        // println!(
        //     "NODE: {} |  st: {}",
        //     node.lines[0].full_line, node.subtree_size
        // );
    }
    Ok(())
}
