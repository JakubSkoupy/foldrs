mod interactive_print;
mod parse;
mod parse_rules;
pub(crate) mod tree;
mod tree_iter;

use anyhow::Error;
use anyhow::Result;
use parse::parse;

use std::io::{self, BufRead};

use crate::interactive_print::main_loop;

fn main() -> Result<(), Error> {
    let stdin = io::stdin();

    let mut input_iter = stdin.lock().lines().map(|l| l.unwrap()).peekable();
    let tree = parse(&mut input_iter);

    main_loop(&tree, 100)
}
