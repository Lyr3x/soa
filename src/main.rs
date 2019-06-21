#[macro_use]
extern crate serde_derive;

mod post;
mod get;

fn main() {


let data = post::TokenIssuePost {
        client_id: String::from("admin"),
        client_secret: String::from("admin"),
        grant_type: String::from("client_credentials")
    };
    


    let access_token = post::basic_post("http://192.168.2.90:8088", data);
    get::get("http://192.168.2.90:8088", &access_token);
}