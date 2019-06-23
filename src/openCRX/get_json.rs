extern crate hyper;
extern crate restson;

use restson::{Error, RestClient, RestPath};
// use std::time::Duration;
use hyper::header::*;
// use std::io::prelude::*;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

#[derive(Deserialize, Debug)]
struct User {
    #[serde(default)]
    userName: String,
    userRole: String,
    status: String,
    employeeName: String,
    employeeId: String
}

#[derive(Deserialize, Debug)]
#[serde(rename(deserialize = "@"))]
struct UserList {
    // data: Vec<User>
    r#type: String
}

// https://sepp-crm.inf.h-brs.de/opencrx-rest-CRX/org.opencrx.kernel.account1/provider/CRX/segment/Standard/account
impl RestPath<()> for UserList {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("opencrx-rest-CRX/org.opencrx.kernel.account1/provider/CRX/segment/Standard/account"))
    }
}
fn write_to_file(body : &Vec<User>) -> std::io::Result<()>{
    let write_file = File::create("orange_hrm_get.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);
    write!(&mut writer, "{:#?}", &body);
    Ok(())
}

pub fn getUserList(url: &str) {
    let mut client = RestClient::new(&url).unwrap();
    // let mut bearer = "Bearer ".to_string();
    // bearer.push_str(&access_token);
    // client.
    client
        .set_header(ACCEPT.as_str(), "application/json")
        .unwrap();
    client
        .set_header(AUTHORIZATION.as_str(), "Basic Z3Vlc3Q6Z3Vlc3QK")
        .unwrap();

    let data: UserList = client.get(()).unwrap();
    println!("{:#?}", data.r#type);
    // println!("{:?}", write_to_file(&data.data));
}
