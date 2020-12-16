#[pmt::id]
fn wrapped_function() {}

#[pmt::print_attr]
enum A {
    a,
    b,
    c,
}

// This attr doesn't make too much sense, anything goes
#[pmt::print_attr]
for fn in struct X { fn.apply(|x| { dbg!(x); x }) }

// This ident doesn't make too much sense, anything goes
#[pmt::print_ident(struct () => enum { union fn }; a = for)]
struct X {}

#[test]
fn test_id() {
    wrapped_function();
}

