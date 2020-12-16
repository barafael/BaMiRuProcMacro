#[pmt::id]
fn wrapped_function() {}

#[pmt::print_attr]
enum A {
    a,
    b,
    c,
}

// This ident doesn't make too much sense, anything goes
#[pmt::print_ident(for fn in struct X { fn.apply(|x| { dbg!(x); x }) })]
struct X {}

// This ident doesn't make too much sense, anything goes
#[pmt::print_ident(struct () => enum { union fn }; a = for)]
struct Y {}

#[test]
fn test_id() {
    wrapped_function();
}

