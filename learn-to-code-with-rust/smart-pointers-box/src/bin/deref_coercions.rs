fn main() {
    let text = String::from("Hello");
    let my_box = Box::new(text);
    let value = &(*my_box)[..];
    output_text(value);
}

fn output_text(text: &str) {
    println!("{}", text);
}
