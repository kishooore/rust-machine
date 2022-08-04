
trait Shape {
  fn calculate_area(&self) -> usize;
}

#[derive(Debug)]
struct Square {
  _width: usize,
  _height: usize
}

impl Square {
    fn new(width: usize, height: usize) -> Self { 
      Self { _width: width, _height: height }
    }
}

impl Shape for Square {
  fn calculate_area(&self) -> usize {
    &self._width * &self._height
 }
}

struct Circle {
  radis: usize
}

impl Circle {
    fn new(radis: usize) -> Self { Self { radis } }
}

impl Shape for Circle {
  fn calculate_area(&self) -> usize {
    let area = std::f32::consts::PI * *&self.radis as f32 * *&self.radis as f32;
    area as usize
  }
}

fn area_printer(shape: &dyn Shape) {
  println!("{}", &shape.calculate_area());
}

fn main() {
  let square = Square::new(10, 20);
  area_printer(&square);
  let circle = Circle::new(100);
  area_printer(&circle);
}