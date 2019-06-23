extern crate hyper;

use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};

use serde_xml_rs::from_reader;

// #[derive(Deserialize, Debug)]
// struct User {
//     #[serde(default)]
//     industry: String
// }

// #[derive(Deserialize, Debug)]
// struct UserList {
//     data: Vec<User>
// }

#[derive(Debug, Deserialize)]
struct Item {
    pub url: String
}


pub fn api_get() {
   rt::run(rt::lazy(|| {
       let client = Client::new();

let uri = "http://sepp-crm.inf.h-brs.de/opencrx-rest-CRX/org.opencrx.kernel.account1/provider/CRX/segment/Standard/account?userName=guest".parse().unwrap();

client
    .get(uri)
    .map(|res| {
        println!("Response: {}", res.status());
    })
    .map_err(|err| {
        println!("Error: {}", err);
    })
    }));
}


// pub fn get_user_list(url: &String) {
//     // let auth = "Z3Vlc3Q6Z3Vlc3Q=";
//     // let url = "https://sepp-crm.inf.h-brs.de/opencrx-rest-CRX/org.opencrx.kernel.account1/provider/CRX/segment/Standard/account?userName=guest";
//     // println!("{}", url);
//     // let mut client = RestClient::new(&url).unwrap();
//     // // println!("{:?}", client.get());
//     // // client.set_auth("guest", "guest");
//     // //  client
//     // //     .set_header(AUTHORIZATION.as_str(), &auth)
//     // //     .unwrap();
//     // let data: Item = client.get(()).unwrap();
//     // println!("{:#?}", data);
 
   
// }
