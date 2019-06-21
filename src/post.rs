extern crate restson;

use restson::{Error, RestClient, RestPath};

impl RestPath<()> for TokenIssuePost {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("/symfony/web/index.php/oauth/issueToken"))
    }
}
#[derive(Deserialize)]
struct IssueTokenPostResp {
    access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenIssuePost {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
}

pub fn basic_post(url: &str, data: TokenIssuePost) -> String {
    let mut client = RestClient::new(&url).unwrap();

    let serialized = serde_json::to_string(&data).unwrap();
    println!("{}",serialized );
    println!("{}", url);
    let resp: IssueTokenPostResp = client.post_capture((), &data).unwrap();
    println!("{}", resp.access_token);
    return resp.access_token;
}

