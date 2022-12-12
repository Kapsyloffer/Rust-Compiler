fn main() {
    let mut a = 6;
    let _b = {
        a = a + 1;
        a
    };
}
