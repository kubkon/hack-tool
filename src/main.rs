use std::collections::HashMap;
use std::io;

type Tree<'a> = HashMap<&'a str, HashMap<usize, Vec<&'a str>>>;

fn branches<'a>(words: &Vec<&'a str>) -> Tree<'a> {
    let mut top_map = HashMap::with_capacity(words.len());
    let n = words[0].len();
    for &word in words {
        let mut map = HashMap::with_capacity(n);
        for i in 0 .. n {
            map.insert(i, Vec::new());
        }
        top_map.insert(word, map);
    }
    for (k, v) in top_map.iter_mut(){
        for &word in words {
            if word.to_string() == k.to_string() {
                continue;
            }

            let like = likeness(k, word);
            v.get_mut(&like).unwrap().push(word);
        }
    }
    top_map
}

fn recommend<'a>(tree: &Tree<'a>) -> Vec<&'a str> {
    let mut counts = Vec::new();
    for (k, v) in tree.iter() {
        let count = (k, v.iter().fold(0, |acc, (&i, ref w)| if i > 0 {acc + w.len()} else {acc} )); 
        counts.push(count);
    }
    let max = counts.iter().map(|&(_, v)| v).max().unwrap();
    counts.iter().filter(|&&(_, v)| v == max).map(|&(k, _)| k).cloned().collect()
}

fn likeness(word1: &str, word2: &str) -> usize {
    word1.chars().zip(word2.chars())
                 .filter(|&(c1, c2)| c1 == c2)
                 .count()
}

fn main() {
    let words = vec![
        "hates",
        "unite",
        "dried",
        "thief",
        "jokes",
        "dazed",
        "early",
        "laser",
        "basic",
        "grief",
        "noted",
        "harsh",
        "claim",
        "crime",
        "slide"
    ];
    // compute all branches
    let mut tree = branches(&words);
    println!("Recommended picks: {:#?}", recommend(&tree));

    loop {
        // user input
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let parsed: Vec<&str> = input.trim().split_whitespace().collect();
        let word = parsed.get(0).unwrap();
        let like = parsed.get(1).unwrap().parse::<usize>().unwrap();
        tree = branches(tree.get(word).unwrap().get(&like).unwrap());
        println!("Recommended picks: {:#?}", recommend(&tree));
    }
}

#[test]
fn test_likeness() {
    assert_eq!(2, likeness("early", "harsh"));
    assert_eq!(1, likeness("early", "laser"));
    assert_eq!(0, likeness("early", "unite"));
}
