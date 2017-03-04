#![feature(custom_derive)] //Remove in future
#![feature(plugin)]
#![plugin(rocket_codegen)]


extern crate rocket;
extern crate regex;

use rocket::request::Form;
use rocket::Response;
use rocket::http::ContentType;
use rocket::State;

use std::io::BufReader;
use std::fs::File;
use std::vec::Vec;
use std::io::Read;
use std::io::Cursor;

use regex::Regex;
use regex::CaptureMatches;

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
    let mut my_form=build_form(File::open("forms/kal").expect("where are you kal, boy?")).expect("Form build error");
    rocket::ignite().manage(my_form).mount("/", routes![get_form,post_form,stream_icon]).launch();
}
struct FormField{
    height: i32,
    prompt: [u8;1000],
}impl Clone for FormField{
    fn clone(&self) -> FormField{
        FormField{height:self.height,prompt:self.prompt}
    }
}impl Copy for FormField{}
impl FormField{
    fn blank() -> FormField{
        FormField{height:0, prompt: [0;1000]}
    }
    fn get_prompt(&self)->String{
        let output=String::from_utf8(self.prompt.to_vec()).expect("Not a string!!!!");
        println!("got prompt: {}",output);
        output
    }
    fn set_prompt(&mut self,prompt: &[u8]){
        if prompt.len() > self.prompt.len(){
            panic!("prompt is too long");
        }else{
            let mut my_prompt=[0;1000];
            for j in 0..prompt.len(){
                my_prompt[j]=prompt[j];
            }
            println!("Set prompt to: {}", String::from_utf8(my_prompt.to_vec()).expect("hiiii"));
            self.prompt=my_prompt;
        }
    }
}

struct FormLetter{
    title: [u8;100],
    width: i32,
    entries: [FormField;100],
    num_entries: usize,
    const_sec: Vec<[u8;10000]>,
} impl FormLetter {
    fn new() -> FormLetter{
        FormLetter{entries: [FormField::blank();100],
                   num_entries:0,
                   const_sec: Vec::new(),
                   title: [0;100],
                   width: 0,
        }
    }
    fn get_title(&self)->String{
        String::from_utf8(self.title.to_vec()).expect("Not a string!!!!")
    }
    fn set_title(&mut self,title: &[u8]){
        if title.len() > self.title.len(){
            panic!("title is too long");
        }else{
            let mut mytitle=[0;100];
            for j in 0..title.len(){
                mytitle[j]=title[j];
            }
            self.title=mytitle
        }
    }
    fn get_width(&self)->i32{
        self.width
    }
    fn get_const_sec(&self) -> Vec<String>{
        let map_fn=|x:&[u8;10000]| -> String{
            String::from_utf8(x.to_vec()).expect("Not a string!")
        };
        self.const_sec.iter().map(map_fn).collect()
    }
    fn set_const_sec(&mut self,const_sec: &[u8]){
        if const_sec.len() > 10000{
            panic!("constant section is too long");
        }else{
            let mut my_const_sec=[0;10000];
            for j in 0..const_sec.len(){
                my_const_sec[j]=const_sec[j];
            }
            self.const_sec.push(my_const_sec);
        }
    }
    pub fn format_form(&self)-> String{
        println!("title={}",self.get_title());
        println!("width={}",self.get_width());        
        let entries=self.entries;
        let mut out=String::new()+"<h1>"+&self.get_title()+"</h1>\n";
        let mut index=0;
        for k in 0..self.num_entries{
            let entry=self.entries[k];
            out=String::new()+&out+"<p>"+&entry.get_prompt()+"</p>\n";
            out=String::new()+&out+"<textarea name=\"response"+&index.to_string()+"\" rows=\""+&entry.height.to_string()+"\""+"cols=\""+&self.width.to_string()+"\"></textarea>\n";
            index=index+1;
        }
        println!("output HTML:");
        println!("{}",out);
        out
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
            let reg=Regex::new(r"\[\[.*\(.*\)\]\]").expect("Error compiling regex");
//            let tag_reg=Regex::new(r".*=.*(\|\|\|)?").expect("Error compiling regex");
            let block_reg=Regex::new(r"\[\[.+\(").expect("Error compiling regex");
            let pair_reg=Regex::new(r"\(.*\)").expect("Error compiling regex");
            let tag_blocks=reg.captures_iter(&letter);
            for tag in tag_blocks{
                println!("Found block:");
                println!("{}",&tag[0]);
                let mut block_name=block_reg.captures(&tag[0]).unwrap().get(0).unwrap().as_str();
                block_name=&block_name[2..(block_name.len()-1)].trim();
                println!("block name: {}",block_name);
                let arg_str=pair_reg.captures(&tag[0]).unwrap().get(0).unwrap().as_str();
                let key_pair_vec=arg_str[1..(arg_str.len()-1)].split("|||");
                match block_name{ //this is the whole block
                    "FORM" => {for key_pair in key_pair_vec{
                        println!("key pair: {}",key_pair);
                        let mut this_key_pair=key_pair.split("=");
                        match this_key_pair.next().unwrap().trim(){
                            "width" => f.width=this_key_pair.next().unwrap().trim().parse::<i32>().expect("not an int!"),
                            "title" => f.set_title(this_key_pair.next().unwrap().trim().as_bytes()),
                            _ => panic!("bad tag"),
                        }
                        
                    }
                    },
                    "ENTRY" => {
                        let mut ent=FormField::blank();
                        for key_pair in key_pair_vec{
                            println!("key pair: {}",key_pair);
                            let mut this_key_pair=key_pair.split("=");
                            match this_key_pair.next().unwrap().trim(){
                                "prompt" => ent.set_prompt(this_key_pair.next().unwrap().trim().as_bytes()),
                                "height" => ent.height=this_key_pair.next().unwrap().trim().parse::<i32>().expect("height not an int!"),
                                _ => panic!("bad tag"),
                            }
                            
                        }
                        f.entries[f.num_entries]=ent;
                        f.num_entries=f.num_entries+1;
                    },
                    _ => panic!("bad tag"),
                    
                }
            }
            //Add extraction of constant form letter sections here
            let reg_consts=Regex::new(r"\]\].*\[\[").expect("Error compiling regex");
            let consts=reg_consts.captures_iter(&letter);
            for constant in consts{
                f.set_const_sec(constant[0].trim().as_bytes())
            }
        },
    }
    Ok(f)    
}
