use edit_distance::*;

fn main() {
    let source = "alignment";
    let target = "assignment";

    println!(
        "It's necessary to make {} change(s) to {:?} to get {:?}",
        edit_distance(source, target),
        source,
        target
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(edit_distance("gumbo", "gambol"), 2);
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("rosettacode", "raisethysword"), 8);
    }
}
