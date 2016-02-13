fn likeness(word1: &str, word2: &str) -> usize {
    word1.chars().zip(word2.chars())
                 .filter(|&(c1, c2)| c1 == c2)
                 .count()
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_likeness() {
    assert_eq!(2, likeness("early", "harsh"));
    assert_eq!(1, likeness("early", "laser"));
    assert_eq!(0, likeness("early", "unite"));
}
