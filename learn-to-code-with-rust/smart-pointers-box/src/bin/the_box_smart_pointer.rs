fn main() {
    let my_box = Box::new(100);
    println!("{}", *my_box);
    println!("{my_box}");
    println!("{}", my_box);
    println!("{:?}", *my_box);

    let your_box = my_box;
    println!("{your_box}");
}
