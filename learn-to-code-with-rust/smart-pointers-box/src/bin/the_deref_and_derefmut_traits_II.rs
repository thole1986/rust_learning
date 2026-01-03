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

fn main() {
    let boxy = CustomBox::new("Ham and Eggs", 15);
    println!("{}", *boxy); // 15
}
