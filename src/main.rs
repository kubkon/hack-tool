use std::collections::HashMap;
use std::io;

fn routes<'a>(words: &Vec<&'a str>) -> HashMap<&'a str, HashMap<usize, Vec<&'a str>>> {
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
    // compute all routes
    let mut r = routes(&words);
    println!("Pick from: {:#?}", r.keys().collect::<Vec<&&str>>());

    loop {
        // user input
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let input: Vec<&str> = input.trim().split_whitespace().collect();
        let word = input.get(0).unwrap();
        let like = input.get(1).unwrap().parse::<usize>().unwrap();
        // user selected "hates" and received likeness 2
        r = routes(r.get(word).unwrap().get(&like).unwrap());
        println!("Pick from: {:#?}", r.keys().collect::<Vec<&&str>>());
    }
}

#[test]
fn test_likeness() {
    assert_eq!(2, likeness("early", "harsh"));
    assert_eq!(1, likeness("early", "laser"));
    assert_eq!(0, likeness("early", "unite"));
}
