#![allow(unused_variables)]
struct StrWrap<'a>(&'a str);

fn main() {
    struct StrWrap<'a>(&'a str);
    fn foo<'a>(string: &'a str) -> StrWrap<'a> {
        StrWrap(string)
    }
}

// But that’s a lot of 'as! To cut down on some of this noise,
// we can use the anonymous lifetime, '_, like this:

fn foo(string: &str) -> StrWrap<'_> {
    StrWrap(string)
}

// The '_ says “use the elidied lifetime here.” This means that we
// can still see that StrWrap contains a reference, but we don’t need
// all of the lifetime annotations to make sense of it.

// It works in impl headers too; for example:

// verbose
impl<'a> fmt::Debug for StrWrap<'a> {

// elided
impl fmt::Debug for StrWrap<'_> {
