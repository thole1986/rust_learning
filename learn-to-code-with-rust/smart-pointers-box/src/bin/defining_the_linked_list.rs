#[derive(Debug)]
enum LinkedList<T> {
    Empty,
    Node { value: T, next: Box<LinkedList<T>> },
}

fn main() {
    let list = LinkedList::Node {
        value: 1,
        next: Box::new(LinkedList::Node {
            value: 2,
            next: Box::new(LinkedList::Node {
                value: 3,
                next: Box::new(LinkedList::Empty),
            }),
        }),
    };

    println!("{list:#?}");

    let im_with_you = LinkedList::Node {
        value: String::from("I'm With You"),
        next: Box::new(LinkedList::Empty),
    };
    let sk8er_boi = LinkedList::Node {
        value: String::from("Sk8er Boi"),
        next: Box::new(im_with_you),
    };
    let complicated = LinkedList::Node {
        value: String::from("Complicated"),
        next: Box::new(sk8er_boi),
    };

    println!("{complicated:#?}");
}
