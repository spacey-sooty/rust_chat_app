#[macro_use]
extern crate rocket;

#[get("/")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![hello_world])
}
