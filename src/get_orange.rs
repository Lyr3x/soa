extern crate hyper;
extern crate restson;

use restson::{Error, RestClient, RestPath};
use std::time::Duration;
use hyper::header::*;


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
struct UserList {
    data: Vec<User>
}


impl RestPath<()> for UserList {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("/symfony/web/api/v1/user"))
    }
}

pub fn getUserList(url: &str, access_token: &str) {
    let mut client = RestClient::new(&url).unwrap();
    let mut bearer = "Bearer ".to_string();
    bearer.push_str(&access_token);

    client
        .set_header(AUTHORIZATION.as_str(), &bearer)
        .unwrap();

    let data: UserList = client.get(()).unwrap();
    println!("{:#?}", data.data);
}
