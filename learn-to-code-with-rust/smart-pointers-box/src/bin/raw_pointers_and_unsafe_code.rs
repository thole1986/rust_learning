fn main() {
    let mut sushi = String::from("Yellowtail");

    let sushi_raw_pointer_1 = &raw const sushi;
    let sushi_raw_pointer_2: *const String = &sushi;
    let sushi_raw_mutable_pointer_1 = &raw mut sushi;
    let sushi_raw_mutable_pointer_2 = &raw mut sushi;

    unsafe {
        println!("{}", *sushi_raw_pointer_1, *sushi_raw_pointer_2);
    }
}
