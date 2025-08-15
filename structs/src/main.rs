struct User {
    active: bool,
    username: String,
    email: String,
    signincount: u64,


}
// fn main() {
//     // let user: User= User{
//     //     email: String::from("princepal@gmail.com"),
//     //     username: String::from("prince"),
//     //     active: true,
//     //     signincount: 4,

//     // };

//     // println!("user name is {} ", user.username);

// }



//more on structs

struct Rectangle {
    width: u32,
    height: u32,
}
// strunct on ownership

impl Rectangle {
    // method to calculate area
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // let rect2: Rectangle = Rectangle {
    //     width: 10,
    //     height: 40,
    // };
    let rect3 = Rectangle::new(20, 60);

    println!("The area of the rectangle1 is {} square pixels.", area(&Rectangle::new(30, 50)));
    println!("The area of the rectangle2 is {} square pixels.", Rectangle::new(10, 40).area());
    println!("The area of the rectangle3 is {} square pixels.", rect3.area());

}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

