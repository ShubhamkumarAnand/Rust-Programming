fn main() {
    let mut x: i32 = 6; // Mutable Variable 
    print!("{x}");
    while x != 1 {
        if x % 2 == 0 {
            x /= 2;
        } else {
            x = 3 * x + 1;
        }
        print!("->{x}");
    }
    println!();
}
