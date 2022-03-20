#[macro_use] extern crate rocket;

#[get("/")]
fn status() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![status])
}