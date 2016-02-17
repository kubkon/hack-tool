#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;

use std::collections::HashMap;
use std::io;

docopt!(Args derive Debug, "
HACK

Usage:
    hack <words>...
    hack (-h | --help)
    hack --version
");

type Tree<'a> = HashMap<&'a str, HashMap<usize, Vec<&'a str>>>;

fn branches<'a>(words: &Vec<&'a str>) -> Tree<'a> {
    let mut tree = HashMap::with_capacity(words.len());
    let n = words[0].len();
    for &word in words {
        let mut branch = HashMap::with_capacity(n);
        for i in 0 .. n {
            branch.insert(i, Vec::new());
        }
        tree.insert(word, branch);
    }
    for (k, v) in tree.iter_mut(){
        for &word in words {
            if word.to_string() == k.to_string() {
                continue;
            }

            let like = likeness(k, word);
            v.get_mut(&like).unwrap().push(word);
        }
    }
    tree
}

fn recommend<'a>(tree: &Tree<'a>) -> Vec<&'a str> {
    let mut counts = Vec::new();
    for (k, v) in tree.iter() {
        let count = (k, v.iter().fold(0, |acc, (&i, ref w)| {
            if i > 0 {
                acc + w.len()
            } else {
                acc
            }
        })); 
        counts.push(count);
    }
    let max = counts.iter().map(|&(_, v)| v).max().unwrap();
    counts.iter()
          .filter(|&&(_, v)| v == max)
          .map(|&(k, _)| k)
          .cloned()
          .collect()
}

fn likeness(word1: &str, word2: &str) -> usize {
    word1.chars().zip(word2.chars())
                 .filter(|&(c1, c2)| c1 == c2)
                 .count()
}

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let words: Vec<&str> = args.arg_words.iter().map(|s| s.as_str()).collect();
    // compute all branches
    let mut tree = branches(&words);

    loop {
        println!("Recommended picks: {:#?}", recommend(&tree));
        println!("{:#?}", tree);
        // user input
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let parsed: Vec<&str> = input.trim().split_whitespace().collect();
        let (word, like) = match (parsed.get(0), parsed.get(1)) {
            (None, _) | (_, None) => {
                println!("You need to specify the word followed by likeness");
                continue;
            },
            (Some(w), Some(l)) => (w, l.parse::<usize>().unwrap()),
        };
        // recompute branches based on user input
        tree = branches(tree.get(word).unwrap().get(&like).unwrap());
    }
}

#[test]
fn test_likeness() {
    assert_eq!(2, likeness("early", "harsh"));
    assert_eq!(1, likeness("early", "laser"));
    assert_eq!(0, likeness("early", "unite"));
}
