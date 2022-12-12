#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unconditional_recursion)]
fn main() {
    fn a() {
        fn a(i: i32) {
            a(1);
        }

        fn b() {
            a(1);
            c()
        }
    }

    fn c() {}
}
