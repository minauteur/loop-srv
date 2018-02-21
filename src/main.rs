extern crate mount;
extern crate staticfile;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate mustache;
// extern crate rustc_serialize;
extern crate router;
extern crate reqwest;
extern crate iron;
#[macro_use]
extern crate rustc_serialize;
// use std::fs::*;
use iron::headers::AccessControlAllowOrigin;
use serde_json::Value;
use iron::prelude::*;
use iron::request::Request;
use iron::middleware::Handler;
use iron::IronResult;
use iron::response::Response;
use iron::status::Status;
// use iron::*;
use iron::Iron;
use std::path::Path;
use std::fs;
use iron::mime;
// use iron::Handler;
use mount::Mount;
use staticfile::Static;
use std::path::PathBuf;
use router::Router;
use rustc_serialize::json;
// use reqwest;

const search_base: &'static str = "https://www.googleapis.com/customsearch/v1?key=AIzaSyA-dQNa_5lpjHIsCzx1kbWSgG1XkoWhfyU&cx=015835535942221852645:udi9r5mhipg&q=";
const search_post: &'static str = "&searchType=image&fileType=jpg&imgSize=small&alt=json";

impl Default for SrvResp {
    fn default() -> Self {
        let mut d_vec: Vec<PathBuf> = Vec::new();
        d_vec.push(PathBuf::new());
        let default: SrvResp = SrvResp {
            file_list: d_vec
        };
        return default;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, RustcEncodable)]
pub struct LoopItem {
    active: bool,
    path: String,
    name: String,
    url: String,
    img: String
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, RustcEncodable)]
pub struct LVec {
    stored: Vec<LoopItem>
}
fn main() {
    let mut fs = Mount::new();
    let mut router = Router::new();
    router.get("/", SrvResp::new(), "srvresp");
    println!("listening at localhost:5000!");
    Iron::new(router).http("localhost:5000").unwrap();
    
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ImageItem {
    link: String
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SrvResp {
    file_list: Vec<PathBuf>,
}
impl SrvResp {
    pub fn new() -> SrvResp {
        let s_r: SrvResp = SrvResp {
            file_list: Vec::new()
        };
        return s_r;
    }
}
impl Handler for SrvResp {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        // let dir =  fs::read_dir("/var/www/microwavemansion.com/loops").expect("couldn't read directory!");
        let win_dir = fs::read_dir("C:\\Users\\Minauteur\\Desktop\\tools\\caddy_v0.10.10_windows_amd64_custom_personal\\loops").expect("couldn't read directory!");
        let mut p_vec: Vec<PathBuf> = Vec::new();
        // for path in dir {
        for path in win_dir {
            let p_b = PathBuf::from(&path.unwrap().path().display().to_string());
            p_vec.push(p_b);
        }
        let mut loops: Vec<LoopItem> = Vec::new();        
        for l_path in p_vec {
            let l_name = Path::new(&l_path.clone()).file_stem().unwrap().to_owned();
            let mut new_loop: LoopItem = LoopItem {
                active: false,
                path: String::from(l_path.clone().to_str().unwrap()),
                name: l_name.into_string().unwrap(),
                url: format!("https://microwavemansion.com/loops/{}", l_path.file_name().unwrap().to_owned().into_string().unwrap()),
                img: String::new()
            };
            println!("image searching: {}", &new_loop.name);
            let word: &str = &new_loop.name.as_str().split_whitespace().next().unwrap();
            let img_url = format!("{}{}{}", search_base, &word, search_post);
            let img_req: Value = reqwest::get(&img_url).unwrap().json().unwrap();
            let items: Value = img_req.get("items").expect("couldn't find any 'items'!").clone();
            let result: ImageItem = serde_json::from_value(items[0].clone()).expect("couldn't extract link from search results!");
            println!("image link: {}", &result.link);            
            new_loop.img = result.link;
            println!("url: {}", &new_loop.url);
            println!("local path: {}", &l_path.to_str().unwrap());
            loops.push(new_loop.to_owned());
        }
        let json = json::encode(&loops).unwrap();
        // let content_type = mime::Mime(mime::TopLevel::Text, mime::SubLevel::Html, vec![]);        
        let mut res = Response::with((Status::Ok, json));
        res.headers.set( 
            AccessControlAllowOrigin::Any
            );
        return Ok(res);
    }
}