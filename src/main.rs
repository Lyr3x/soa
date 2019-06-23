#[macro_use]
extern crate serde_derive;

mod post_orange;
mod get_orange;

fn main() {

let orangeHrm = "https://sepp-hrm.inf.h-brs.de";
let orangeHrmDocker = "http://192.168.2.90:8088";

let openCrx = "https://sepp-crm.inf.h-brs.de/opencrx-core-CRX/";

let data = post_orange::TokenIssuePost {
        client_id: String::from("admin"),
        client_secret: String::from("admin"),
        grant_type: String::from("client_credentials")
    };
    

    let access_token = post_orange::basic_post(orangeHrm, data);
    get_orange::getUserList(orangeHrm, &access_token);
}