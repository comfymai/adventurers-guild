#[macro_use] extern crate rocket;

#[get("/meow")]
fn meow() -> String {
    String::from("Nyan")
}

#[launch]
fn app() -> _ {
    rocket::build().mount("/", routes![meow])
}
