use crate::exercises::book_manager::Book;
use crate::exercises::math::Shape;
use crate::exercises::math::fibonacci_sequence;
#[allow(unused_imports)]
use crate::exercises::unit_converter::temperature_converter;

mod exercises;

fn main() {
    let rectangle = Shape::Rectangle { width: 18, height: 32 };
    let rectangle_area = &rectangle.calculate_surface();
    println!("{}", rectangle_area);
}
