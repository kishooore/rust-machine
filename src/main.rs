mod my_math;

fn main() {
 let square = my_math::geometry::Square::new(10, 20);
 my_math::geometry::area_printer(&square);
 let circle = my_math::geometry::Circle::new(10);
 my_math::geometry::area_printer(&circle);
}