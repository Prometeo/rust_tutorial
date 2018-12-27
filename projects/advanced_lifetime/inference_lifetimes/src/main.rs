trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

fn main() {
    let num = 5;

    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}


// This code compiles without any errors, even though we haven’t explicitly
// annotated the lifetimes involved in obj. This code works because there
// are rules for working with lifetimes and trait objects:
// * The default lifetime of a trait object is 'static.
// * With &'a Trait or &'a mut Trait, the default lifetime of the trait object is 'a.
// * With a single T: 'a clause, the default lifetime of the trait object is 'a.
// * With multiple clauses like T: 'a, there is no default lifetime; we must be explicit.
