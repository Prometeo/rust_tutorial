struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

// Now we get to the point of this section: the Rust feature lifetime
// subtyping specifies that one lifetime parameter lives at least as long as another one.
// we can declare a lifetime 'a as usual and declare a lifetime 'b that lives at least
// as long as 'a by declaring 'b using the syntax 'b: 'a.
