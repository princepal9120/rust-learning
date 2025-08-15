#[derive(Debug)]

enum Color {
    Red,
    Green,
    Blue,
}
#[derive(Debug)]
enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // width, height

}
const PI: f64=3.14;

impl Shape {
    fn new_circle(radius: f64) -> Self {
        Self::Circle(radius)
    }
    fn new_rectangle(width: f64, height: f64) -> Self {
        Self::Rectangle(width, height)
    }
    fn area(&self) {
        match self {
            Shape::Circle(radius) => println!("Area of Circle: {}", PI * radius * radius),
            Shape::Rectangle(width, height) => println!("Area of Rectangle: {}", width * height),
        }
    }
}

fn main() {
    // let color = Color::Red;
    // match color {
    //     Color::Red => println!("The color is red."),
    //     Color::Green => println!("The color is green."),
    //     Color::Blue => println!("The color is blue."),
    // }
    let circle = Shape::new_circle(5.0);
    // println!("circle ar: {:?}", circle);
    let rectangle=Shape::new_rectangle(5.0,6.0);
    circle.area();
    rectangle.area();


}
