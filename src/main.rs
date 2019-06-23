mod orangeHrm;
mod openCRX;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;
extern crate hyper;
extern crate xmlJSON;

use serde_xml_rs::from_reader;

fn main() {

    let orangeHrm = "https://sepp-hrm.inf.h-brs.de";
    let orangeHrmDocker = "https://orangehrm.ironmanserver.de";

    let data = orangeHrm::post::TokenIssuePost {
            client_id: String::from("admin"),
            client_secret: String::from("admin"),
            grant_type: String::from("client_credentials")
        };
    

    let access_token = orangeHrm::post::basic_post(orangeHrmDocker, data);
    orangeHrm::get::getUserList(orangeHrmDocker, &access_token);
    let username = "guest";
    let password = "guest";
    let openCRX = format!("http://{}:{}@sepp-crm.inf.h-brs.de/opencrx-core-CRX",username,password);

    openCRX::get::get();
}