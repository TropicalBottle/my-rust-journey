use crate::exercises::book_manager::Book;
use crate::exercises::math::fibonacci_sequence;
#[allow(unused_imports)]
use crate::exercises::unit_converter::temperature_converter;

mod exercises;

fn main() {
    let harry_potter = Book::new(
        String::from("J.K. Rowling"),
        String::from("Harry Potter"),
        1997,
        String::from("0-7475-3269-9"),
    );

    println!("{}", harry_potter.get_title());
}
