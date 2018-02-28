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
// extern crate reqwest;
extern crate iron;
extern crate regex;
#[macro_use]
extern crate rustc_serialize;
// use std::fs::*;
use iron::headers::AccessControlAllowOrigin;
use serde_json::Value;
use iron::prelude::*;
use rustc_serialize::base64::{ToBase64, MIME};
use rustc_serialize::hex::{ToHex};
use iron::request::Request;
use iron::middleware::Handler;
use iron::IronResult;
use iron::response::Response;
use iron::status::Status;
// use iron::*;
use iron::Iron;
use std::path::Path;
use std::fs;
use std::io::{Read, BufReader};
// use iron::mime;
// use iron::Handler;
// use mount::Mount;
// use staticfile::Static;
use std::path::PathBuf;
use router::Router;
use rustc_serialize::json;
use std::ffi::OsStr;
use regex::Regex;
// use reqwest;

// const SEARCH_BASE: &'static str = "https://www.googleapis.com/customsearch/v1?key=AIzaSyA-dQNa_5lpjHIsCzx1kbWSgG1XkoWhfyU&cx=015835535942221852645:udi9r5mhipg&q=";
// const SEARCH_POST: &'static str = "&searchType=image&fileType=jpg&imgSize=small&alt=json";

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
    img: String,
    uri: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, RustcEncodable)]
pub struct LVec {
    stored: Vec<LoopItem>
}
fn main() {
    // let mut fs = Mount::new();
    let mut router = Router::new();
    router.get("/", SrvResp::new(), "srvresp");
    println!("listening at localhost:5000!");
    Iron::new(router).http("localhost:5000").unwrap();
    
}
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
// struct ImageItem {
//     link: String
// }
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
        let dir = fs::read_dir("C:\\Users\\Minauteur\\Desktop\\tools\\caddy_v0.10.10_windows_amd64_custom_personal\\loops").expect("couldn't read directory!");
        let mut p_vec: Vec<PathBuf> = Vec::new();
        let mut i_vec: Vec<PathBuf> = Vec::new();
        for entry in dir {
        // for path in win_dir {
            if entry.is_ok() {
                let item = entry.unwrap();
                if let Some(ext) = item.path().clone().extension() {
                    match ext.to_str() {
                        Some("wav") | Some("mp3") => p_vec.push(item.path().clone()),
                        Some("png") => i_vec.push(item.path().clone()),
                        _=> (),
                    }
                }
            }
            // {
            //     if let Ok(p_b) = PathBuf::from(&path) {
            //         p_vec.push(p_b)
            //     }
            // }
        }
        let mut loops: Vec<LoopItem> = Vec::new();        
        for l_path in p_vec {
            let mut new_loop: LoopItem = LoopItem {
                active: false,
                path: format!("{}", l_path.to_str().unwrap()),
                name: String::new(),
                url: String::new(),
                img: String::new(),
                uri: String::new()
            };
            if let Some(l_name) = l_path.clone().file_stem().unwrap().to_str() {
                new_loop.name = l_name.to_string();
                new_loop.img = format!("https://microwavemansion.com/loops/{}.png", l_name);
            }
            if let Some(l_string) = l_path.clone().file_name().unwrap().to_str() {
                new_loop.url = format!("https://microwavemansion.com/loops/{}", &l_string);
            }
            let img_path = PathBuf::from(&new_loop.path).with_extension("png");
            if check_img(&img_path) {
                new_loop.uri = format!("{}", {uri(&img_path)});
            }
            println!("image link: {}", &new_loop.img);            
            // new_loop.img = result.link;
            println!("url: {}", &new_loop.url);
            println!("local path: {}", &l_path.to_str().unwrap());
            loops.push(new_loop.to_owned());
        }
        
        
        // for mut l in &mut loops {
        //     if let Ok(mut loc_file) = fs::File::open(PathBuf::from(&l.path).with_extension("png")) {
        //         l.uri = format!("{}", uri(&mut loc_file));
        //     }
        // }
        let json = json::encode(&loops).unwrap();
        // let content_type = mime::Mime(mime::TopLevel::Text, mime::SubLevel::Html, vec![]);        
        let mut res = Response::with((Status::Ok, json));
        res.headers.set( 
            AccessControlAllowOrigin::Any
            );
        return Ok(res);
    }
}
fn check_img(p_buf: &PathBuf) -> bool {
    return p_buf.exists()
}
fn uri(p_buf: &PathBuf) -> String {

    let mut buffer = Vec::new();
    let mut local_file = fs::File::open(p_buf).unwrap();
    let reader = BufReader::new(local_file).read_to_end(&mut buffer).unwrap();
    // let _out = local_file.read_to_end(&mut buffer).unwrap();

    let b64 = buffer.to_base64(MIME);
    // let hex = buffer.to_hex();

    let uri = format!("data:image/png;base64,{}", b64);
    println!("{}", &uri);
    return uri
}