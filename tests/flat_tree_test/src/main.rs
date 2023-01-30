// convert a binary search tree into a sorted array

use flat_tree::*;
use std::collections::BTreeSet;

fn main() {
    let mut tree = BTreeSet::new();
    tree.insert(34);
    tree.insert(0);
    tree.insert(9);
    tree.insert(30);
    println!("{:?}", flatten_tree(&tree));

    let mut tree = BTreeSet::new();
    tree.insert("Slow");
    tree.insert("kill");
    tree.insert("will");
    tree.insert("Horses");
    println!("{:?}", flatten_tree(&tree));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = BTreeSet::new();
        tree.insert(3);
        tree.insert(0);
        tree.insert(9);
        tree.insert(10);
        assert_eq!(flatten_tree(&tree), &[0, 3, 9, 10]);
    }

    #[test]
    fn test_with_str() {
        let mut tree = BTreeSet::new();
        tree.insert("Slow");
        tree.insert("kill");
        tree.insert("will");
        tree.insert("Horses");
        assert_eq!(flatten_tree(&tree), &["Horses", "Slow", "kill", "will"]);
    }
}
