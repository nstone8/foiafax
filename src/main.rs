#![feature(custom_derive)] //Remove in future
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate foiafax;
extern crate rocket;
extern crate regex;

use foiafax::FormLetter;

use rocket::request::Form;
use rocket::Response;
use rocket::http::ContentType;
use rocket::State;

use std::io::BufReader;
use std::fs::File;
use std::io::Cursor;

#[derive(FromForm)]
struct FormResponse {
    response: String,
}

#[get("/favicon.ico")]
fn stream_icon() -> Response<'static>{
    let favicon=BufReader::new(File::open("static/favicon.ico").expect("Failed to open favicon"));
    let mut resp:Response = Response::new();
    resp.set_header(ContentType::GIF);
    resp.set_sized_body(favicon);
    resp
}

#[post("/", data="<form>")]
fn post_form(form:Result<Form<FormResponse>, Option<String>>) -> String{
    let this_form_post=form.expect("SCREEEEE");
    let this_form=this_form_post.get();
    println!("Response was: {}",this_form.response);
    format!("Response was: {}",this_form.response).to_string()
}
#[get("/")]
fn get_form(f: State<FormLetter>) -> Response<'static> {
    let mut resp:Response = Response::new();
    resp.set_header(ContentType::HTML);
    resp.set_sized_body(Cursor::new(f.format_form()));
    resp
}

fn main() {
    let my_form=FormLetter::build_form(File::open("forms/kal").expect("where are you kal, boy?")).expect("Form build error");
    rocket::ignite().manage(my_form).mount("/", routes![get_form,post_form,stream_icon]).launch();
}
