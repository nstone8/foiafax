#![feature(custom_derive)] //Remove in future
#![feature(plugin)]
#![plugin(rocket_codegen)]


extern crate rocket;
extern crate regex;
use rocket::request::Form;
use rocket::Response;
use rocket::http::ContentType;

use std::io::BufReader;
use std::fs::File;
use std::vec::Vec;
use std::io::Read;
use regex::Regex;
use regex::CaptureMatches;

#[derive(FromForm)]
struct FormResponse {
    response: String,
}

#[get("/favicon.ico")]
fn stream_icon() -> Response<'static>{
    let favicon=BufReader::new(File::open("static/kal.ico").expect("Failed to open favicon"));
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
fn get_form() -> Response<'static> {
    let index=BufReader::new(File::open("static/index.html").expect("Failed to open index HTML document"));
    let mut resp:Response = Response::new();
    resp.set_header(ContentType::HTML);
    resp.set_sized_body(index);
    resp
}

fn main() {
    rocket::ignite().mount("/", routes![get_form,post_form,stream_icon]).launch();
}
struct FormField{
    prompt: &str,
    height: i32,
}

struct FormLetter{
    title: &str,
    width: i32,
    entries: Vec<FormField>,
    const_sec: Vec<&str>,
} impl FormLetter {
    pub fn new() -> FormLetter{
        FormLetter{entries: Vec::new(),
                   const_sec: Vec::new(),
        }
    }
    pub fn format(&mut self)-> &str{
        
    }
}

fn build_form(letter_file:Read, html_path:&str) -> Result<FormLetter,&str>{
    //build an html form using the form letter stored in the BufReader. Store resulting document at html_path. Returns Ok(html_path) if everything goes well, Err(why) otherwise
    
    let mut letter=String::new();
    let read_result=letter_file.read_to_string(&mut letter);
    match read_result{
        Err(_) => return "Failed to read letter buffer",
        Ok(_) => {
            //Assume first line of a file is a FORM block
            let reg=Regex::new(r"[[.*(.*)]]");
            let tag_reg=Regex::new(r".*=.*(|||)?");
            let tag_blocks=reg.captures_iter(&letter);
            let mut f = FormLetter::new();
            for tag in tag_blocks{
                match tag[1].trim(){ //this should be the block name
                    "FORM" => {for keyPair in tag_reg.captures(&letter){
                        match keyPair[1].trim(){
                            "width" => f.width=keyPair[2].trim().parse::<i32>().expect("not an int!"),
                            "title" => keyPair[2].trim(),
                        }
                        
                    }
                    },
                    "ENTRY" => {
                        let mut ent = FormField{};
                        for keyPair in tag_reg.captures(&letter){
                            match keyPair[1].trim(){
                                "prompt" => ent.prompt=keyPair[2].trim(),
                                "height" => ent.height=keyPair[2].trim().parse::<i32>().expect("height not an int!"),
                            }
                            
                        }},
                    
                }
            }
            //Add extraction of constant form letter sections here
            
        },
    }
    
}

