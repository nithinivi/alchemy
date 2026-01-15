#[derive(Debug, Clone)]
struct A {
    x: i32,
}

fn print_a(a: &A) {
    println!("A is {a:?}")
}
fn main() {
    let a = A { x: 32 };
    // print_a(a.clone())
    print_a(&a);
    println!("{a:?}")
}
