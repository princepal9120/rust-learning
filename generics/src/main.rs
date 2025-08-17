use std::fmt::Display;
 fn print_data<T: Display>(data: T ) {
    println!("This is a generic function {}", data);
 }

fn main() {
    let x = "Hello, world!";
    let y=43;
    let z=true;
    print_data(x);
    print_data(y);
    print_data(z);
}
