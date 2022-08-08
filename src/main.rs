mod geometry;

fn main() {
 let square = geometry::Square::new(10, 20);
 geometry::area_printer(&square);
 let circle = geometry::Circle::new(100);
 geometry::area_printer(&circle);
}