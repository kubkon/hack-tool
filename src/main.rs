#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;

mod tree;

use std::io;
use tree::Tree;

docopt!(Args derive Debug, "
HACK is a simple utility that helps you solve those 'difficult' hacking
mini-games in Fallout 4.

Usage:
    hack <words>...
    hack --help
");


fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let words: Vec<&str> = args.arg_words.iter().map(|s| s.as_str()).collect();
    // compute all branches
    let mut tree = Tree::new(&words);

    loop {
        println!("Recommended picks: {:#?}", tree.recommend());
        if tree.empty() {
            break;
        };
        // user input
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let parsed: Vec<&str> = input.trim().split_whitespace().collect();
        let (word, like) = match (parsed.get(0), parsed.get(1)) {
            (None, _) | (_, None) => {
                println!("You need to specify the word followed by likeness");
                continue;
            },
            (Some(w), Some(l)) => (w, l.parse::<usize>().unwrap()),
        };
        // recompute branches based on user input
        tree = Tree::new(tree.branches.get(word).unwrap().get(&like).unwrap());
    }
}

