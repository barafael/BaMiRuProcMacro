#[pmt::id]
fn wrapped_function() {}

#[pmt::print_attr]
enum A {
    a,
    b,
    c,
}

#[pmt::print_ident(struct () => enum { union fn }; a = for)]
struct X {}

#[test]
fn test_id() {
    wrapped_function();
}

