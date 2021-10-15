#[derive(Copy, Clone)]
struct Label { number: u32 }

fn print(l : Label) {
    println!("{}", l.number);
}

fn main() {
    let l: Label = Label {number: 23};
    print(l);
    println!("{}", l.number);
}