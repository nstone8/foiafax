#![feature(custom_derive)] //Remove in future
#![feature(plugin)]
#![plugin(rocket_codegen)]


extern crate rocket;
use rocket::request::Form;
use rocket::Response;
use rocket::http::ContentType;
use std::io::BufReader;
use std::fs::File;

#[derive(FromForm)]
struct FormLetter {
    response: String,
}

#[post("/", data="<form>")]
fn post_form(form:Result<Form<FormLetter>, Option<String>>) -> String{
    let this_form_post=form.expect("SCREEEEE");
    let this_form=this_form_post.get();
    println!("Response was: {}",this_form.response);
    format!("Response was: {}",this_form.response).to_string()
}
#[get("/")]
fn get_form() -> Response<'static> {
    let index=BufReader::new(File::open("static/index.html").expect("Failed to open index HTML document"));
    let mut resp:Response = Response::new();
    resp.set_header(ContentType::HTML);
    resp.set_sized_body(index);
    resp
}

fn main() {
    rocket::ignite().mount("/", routes![get_form,post_form]).launch();
}
