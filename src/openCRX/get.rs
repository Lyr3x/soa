// use std::io::{self, Write};
// use hyper::Client;
// use hyper::rt::{self, Future, Stream};
extern crate reqwest;
use reqwest::header;
use std::io::Read;
// use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;


fn write_to_file(body : &String) -> std::io::Result<()>{
    let mut file = File::create("opencrx.txt")?;
    // let len = body.len();
    let slice = &body[..];
    file.write_all(slice.as_bytes())?;
    Ok(())
}
pub fn build_header() -> hyper::HeaderMap{
    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static("Basic Z3Vlc3Q6Z3Vlc3QK"));
    return headers;
}

pub fn get() -> Result<(), Box<std::error::Error>> {
    let headers = build_header();
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let mut res = client.get("https://sepp-crm.inf.h-brs.de/opencrx-rest-CRX/org.opencrx.kernel.account1/provider/CRX/segment/Standard/account").send()?;
    // println!("{:?}", res.text());
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    println!("{}", body);
    write_to_file(&body);
    Ok(())
 }


pub fn get_json(){
    
}

