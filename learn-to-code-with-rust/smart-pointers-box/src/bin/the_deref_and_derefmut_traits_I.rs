use std::ops::{Deref, DerefMut};

struct CustomBox<T> {
    data: T,
}

impl<T> CustomBox<T> {
    fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Deref for CustomBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for CustomBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

fn main() {
    let mut boxy: Box<f64> = Box::new(3.14);
    *boxy = 6.28;
    println!("{}", *boxy); // 25

    let mut custom_boxy = CustomBox::new(4);
    *custom_boxy = 25;
    println!("{}", *custom_boxy);
}
