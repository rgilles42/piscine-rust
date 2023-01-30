use drop_the_blog::*;
use std::rc::Rc;

fn main() {
    let blog = Blog::new();
    let (id, article) = blog.new_article(String::from("Winter is coming"));
    let (id1, article1) = blog.new_article(String::from("The story of the universe"));

    article.discard();

    println!("{:?}", (blog.is_dropped(id), id, &blog.drops));

    article1.discard();
    println!("{:?}", (blog.is_dropped(id1), id1, &blog.drops));

    let (id2, article2) = blog.new_article(String::from("How to cook 101"));
    let article2 = Rc::new(article2);
    let article2_clone = article2.clone();

    drop(article2_clone);

    println!(
        "{:?}",
        (
            blog.is_dropped(id2),
            id2,
            &blog.drops,
            Rc::strong_count(&article2)
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dropped_and_drops() {
        let blog = Blog::new();
        let (pid, article) = blog.new_article(String::from("gnome-shell"));
        let (pid0, article0) = blog.new_article(String::from("i3"));
        let (pid1, article1) = blog.new_article(String::from("shell"));
        let (pid2, article2) = blog.new_article(String::from("spotify"));

        article.discard();
        assert_eq!(blog.drops.get(), 1_usize);
        article0.discard();
        assert_eq!(blog.drops.get(), 2_usize);

        assert!(blog.is_dropped(pid), "{} should have been dropped", pid);
        assert!(blog.is_dropped(pid0), "{} should have been dropped", pid0);
        assert!(
            !blog.is_dropped(pid1),
            "{} should not have been dropped",
            pid1
        );
        assert!(
            !blog.is_dropped(pid2),
            "{} should not have been dropped",
            pid2
        );

        article1.discard();
        article2.discard();
        assert_eq!(blog.drops.get(), 4_usize);
    }

    #[test]
    fn test_using_rc() {
        // will create a new reference to the article
        // this will test the following
        // if we drop the cloned value the RC will decrease
        // but the article will not be dropped!
        let blog = Blog::new();
        let (_, article) = blog.new_article(String::from("Xorg"));
        let article = Rc::new(article);
        let article_clone = article.clone();

        assert_eq!(Rc::strong_count(&article), 2);

        drop(article_clone);

        assert_eq!(Rc::strong_count(&article), 1);
    }

    #[test]
    #[should_panic(expected = "0 is already dropped")]
    fn test_drop_same_article() {
        // test if we drop the same article after it was already been dropped
        let blog = Blog::new();
        let (_pid, article) = blog.new_article(String::from("gsd-rfkill"));
        let article_clone = article.clone();
        article.discard();
        article_clone.discard();
    }
}
