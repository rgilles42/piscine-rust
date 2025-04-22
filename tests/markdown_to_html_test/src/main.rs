use markdown_to_html::*;

const EXAMPLE: &str = "# This is a nice converter

 ## It handle *all* titles

## From # to ### with no problems

### With attention to details ;)
###With attention to details ;)

> Blockquotes are handled too
>if well formatted of course

And **bold** and *italics* of course work as well.

****

**bold on
multiple
lines
also**";

fn main() {
    println!("{}", markdown_to_html(EXAMPLE));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_HTML: &str = "<h1>This is a nice converter</h1>

 <h2>It handle <em>all</em> titles</h2>

<h2>From # to ### with no problems</h2>

<h3>With attention to details ;)</h3>
###With attention to details ;)

<blockquote>Blockquotes are handled too</blockquote>
>if well formatted of course

And <strong>bold</strong> and <em>italics</em> of course work as well.

<strong></strong>

<strong>bold on
multiple
lines
also</strong>";

    #[test]
    fn test_subject_example() {
        assert_eq!(markdown_to_html(EXAMPLE), EXAMPLE_HTML);
    }

    const TITLES: &str = "# first title
## second title
### third title

#not valid title
 # valid title with no new line";

    const TITLES_HTML: &str = "<h1>first title</h1>
<h2>second title</h2>
<h3>third title</h3>

#not valid title
 <h1>valid title with no new line</h1>
";

    #[test]
    fn test_titles() {
        assert_eq!(markdown_to_html(TITLES), TITLES_HTML);
    }

    const STRONG_ITALIC: &str = "**bold** and *italic* text
## Nested in ** a title ** element *do work*";

    const STRONG_ITALIC_HTML: &str = "<strong>bold</strong> and <em>italic</em> text
<h2>Nested in <strong> a title </strong> element <em>do work</em></h2>
";

    #[test]
    fn test_strong_italic() {
        assert_eq!(markdown_to_html(STRONG_ITALIC), STRONG_ITALIC_HTML);
    }
}
