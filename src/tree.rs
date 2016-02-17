use std::collections::HashMap;

pub type Tree<'a> = HashMap<&'a str, HashMap<usize, Vec<&'a str>>>;

pub fn branches<'a>(words: &Vec<&'a str>) -> Tree<'a> {
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

pub fn recommend<'a>(tree: &Tree<'a>) -> Vec<&'a str> {
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

pub fn empty<'a>(tree: &Tree<'a>) -> bool {
    tree.values()
        .map(|v| v.iter().fold(0, |acc, (_, ref w)| acc + w.len()))
        .all(|x| x == 0)
}

fn likeness(word1: &str, word2: &str) -> usize {
    word1.chars()
         .zip(word2.chars())
         .filter(|&(c1, c2)| c1 == c2)
         .count()
}

#[test]
fn test_likeness() {
    assert_eq!(2, likeness("early", "harsh"));
    assert_eq!(1, likeness("early", "laser"));
    assert_eq!(0, likeness("early", "unite"));
}
