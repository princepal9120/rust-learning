use rocket::*;

#[get("/home/<name>")]
fn index(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index] )
}




