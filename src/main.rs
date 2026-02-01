mod interact;
mod parse_rules;
mod vectree;

use anyhow::Error;
use anyhow::Result;

use std::io;
use std::io::BufRead;

use crate::interact::main_loop;
use crate::vectree::parse_vectree;

fn main() -> Result<(), Error> {
    let stdin = io::stdin();

    let mut input_iter = stdin.lock().lines().map(|l| l.unwrap()).peekable();
    let mut tree = parse_vectree(&mut input_iter);

    main_loop(&mut tree, 100)
}
