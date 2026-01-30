mod interact;
mod parse;
mod parse_rules;
mod tree;
mod tree_iter;
mod vectree;

use anyhow::Error;
use anyhow::Result;

use std::io;
use std::io::BufRead;

use crate::interact::main_loop;
use crate::vectree::parse_stacktree_rec;
use crate::vectree::VecTree;

fn main() -> Result<(), Error> {
    let stdin = io::stdin();

    let mut input_iter = stdin.lock().lines().map(|l| l.unwrap()).peekable();
    let mut tree = VecTree { nodes: vec![] };
    let _ = parse_stacktree_rec(&mut input_iter, &mut tree.nodes, 0);

    main_loop(&mut tree, 100)
}
