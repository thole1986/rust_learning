use std::ops::{Deref, DerefMut};

struct CustomBox<T, U> {
    data: T,
    more_data: U,
}

impl<T, U> CustomBox<T, U> {
    fn new(data: T, more_data: U) -> Self {
        Self { data, more_data }
    }
}

impl<T, U> Deref for CustomBox<T, U> {
    type Target = U;

    fn deref(&self) -> &Self::Target {
        &self.more_data
    }
}

impl<T, U> DerefMut for CustomBox<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.more_data
    }
}
impl<T, U> Drop for CustomBox<T, U> {
    fn drop(&mut self) {
        println!("I'm removing the CustomBox from memory!");
    }
}

fn main() {
    let mut boxy: Box<f64> = Box::new(3.14);
    *boxy = 6.28;
    println!("{}", *boxy); // 25

    let mut custom_boxy = CustomBox::new(3.14, "Hello");
    *custom_boxy = "Goodbye";
    println!("{}", *custom_boxy);
}
