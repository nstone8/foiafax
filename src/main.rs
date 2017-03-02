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
use std::iter::Map;

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
    let mut my_form=build_form(File::open("forms/kal").expect("where are you kal, boy?")).expect("Form build error");
    my_form.format();
    rocket::ignite().mount("/", routes![get_form,post_form,stream_icon]).launch();
}
struct FormField{
    height: i32,
    prompt: [u8;1000],
}impl FormField{
    fn blank() -> FormField{
        FormField{height:0, prompt: [0;1000]}
    }
    fn set_prompt(&self,prompt: &[u8]){
        if prompt.len() > self.prompt.len(){
            panic!("prompt is too long");
        }else{
            self.prompt[1:self.prompt.len()-1]=prompt[1:self.prompt.len()-1]
        }
    }
}

struct FormLetter{
    title: [u8;100],
    width: i32,
    entries: [FormField;100],
    const_sec: Vec<[u8;10000]>,
} impl FormLetter {
    fn new() -> FormLetter{
        FormLetter{entries: [FormField::blank();100],
                   const_sec: Vec::new(),
                   title: [0;100],
                   width: 0,
        }
    }
    fn get_title(&self)->String{
        String::from_utf8(self.title.to_vec()).expect("Not a string!!!!")
    }
    fn set_title(&self,title: &[u8]){
        if title.len() > self.title.len(){
            panic!("title is too long");
        }else{
            self.title[1:self.title.len()-1]=title[1:self.title.len()-1]
        }
    }
    fn get_width(&self)->i32{
        self.width
    }
    fn get_const_sec(&self) -> Vec<String>{
        map_fn=|x:[u8;10000]|{
            String::from_utf8(x.to_vec()).expect("Not a string!");
        }
        let str_map=self.const_sec.iter().map(map_fn);
        str_map.collect()
    }

    pub fn format(&mut self)-> &str{
        println!("title={}",self.get_title());
        println!("width={}",self.get_width());
        "Formatted"
    }
    
}

fn build_form<T:Read>(mut letter_file:T) -> Result<FormLetter,&'static str>{
    //build an html form using the form letter stored in the BufReader. Store resulting document at html_path. Returns Ok(html_path) if everything goes well, Err(why) otherwise
    
    let mut letter=String::new();
    let read_result=letter_file.read_to_string(&mut letter);
    let mut f = FormLetter::new();
    match read_result{
        Err(_) => return Err("Failed to read letter buffer"),
        Ok(_) => {
            //Assume first line of a file is a FORM block
            let reg=Regex::new(r"[[.*(.*)]]").expect("Error compiling regex");
            let tag_reg=Regex::new(r".*=.*(|||)?").expect("Error compiling regex");
            let tag_blocks=reg.captures_iter(&letter);
            for tag in tag_blocks{
                match tag[1].trim(){ //this should be the block name
                    "FORM" => {for keyPair in tag_reg.captures(&letter){
                        match keyPair[1].trim(){
                            "width" => f.width=keyPair[2].trim().parse::<i32>().expect("not an int!"),
                            "title" => f.set_title(keyPair[2].trim().as_bytes()),
                            _ => panic!("bad tag"),
                        }
                        
                    }
                    },
                    "ENTRY" => {
                        let mut ent=FormField::blank();
                        for keyPair in tag_reg.captures(&letter){
                            match keyPair[1].trim(){
                                "prompt" => ent.set_prompt(keyPair[2].trim().as_bytes()),
                                "height" => ent.height=keyPair[2].trim().parse::<i32>().expect("height not an int!"),
                                _ => panic!("bad tag"),
                            }
                            
                        }},
                    _ => panic!("bad tag"),
                    
                }
            }
            //Add extraction of constant form letter sections here
            let reg_consts=Regex::new(r"]].*[[").expect("Error compiling regex");
            let consts=reg_consts.captures_iter(&letter);
            for constant in consts{
                f.const_sec.push(constant[1].trim().as_bytes())
            }
        },
    }
    Ok(f)    
}

