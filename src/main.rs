mod orangeHrm;
mod openCRX;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;
extern crate hyper;
extern crate xmlJSON;

// use serde_xml_rs::from_reader;

fn main() {

    let orangeHrm = "https://sepp-hrm.inf.h-brs.de";
    let orangeHrmDocker = "https://orangehrm.ironmanserver.de";
    let open_crx ="https://sepp-crm.inf.h-brs.de/";

    let data = orangeHrm::post::TokenIssuePost {
            client_id: String::from("tom"),
            client_secret: String::from("tom123"),
            grant_type: String::from("client_credentials")
        };

    let access_token = orangeHrm::post::basic_post(orangeHrm, data);
    orangeHrm::get::getUserList(orangeHrm, &access_token);
    openCRX::get_json::getUserList(open_crx);

    // openCRX::get::get();
}
