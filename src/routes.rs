use std::env;

use json;
use reqwest::Client;
use rocket::{
    http::{Cookie, Cookies},
    request::Form,
    response::Redirect,
};
use steam_auth::{Redirector, Verifier};

use crate::rocket;

#[get("/")]
pub fn root(cookies: Cookies) -> &'static str {
    cookies.iter().for_each(|c| println!("{}", c));
    "AAUFIYHADGHUDUGHADOSUGFHOAG"
}

#[get("/login")]
pub fn login() -> Redirect {
    let redirect = Redirector::new("http://localhost:8000", "/handle-login").unwrap();
    let url = redirect.url();
    let s = url.to_owned();

    Redirect::to(s.to_string())
}

#[get("/handle-login?<contents..>")]
pub fn handle_login(contents: Form<OpenID>, mut cookies: Cookies) -> Redirect {
    let contents_clone = contents.clone();

    let steam_id =
        match Verifier::make_verify_request(&Client::new(), &contents_clone.to_querystring()) {
            Ok(steam_id) => steam_id,
            Err(_) => 0,
        };

    let client = Client::new();
    let url = format!("https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={}&format=json&steamids={}", env::var("STEAM_API_KEY").unwrap(), steam_id);
    let res = client.get(&url).send().unwrap().text().unwrap();
    let json = json::parse(&res).unwrap();
    let value = json["response"]["players"][0].clone();

    let cookie = Cookie::build("steam_user", value.to_string());

    cookies.add(cookie.finish());

    println!("{:?}", cookies.get("steam_user"));

    Redirect::to("/")
}

#[derive(Debug, Clone, FromForm)]
pub struct OpenID {
    #[form(field = "openid.ns")]
    ns: String,
    #[form(field = "openid.mode")]
    mode: String,
    #[form(field = "openid.op_endpoint")]
    op_endpoint: String,
    #[form(field = "openid.claimed_id")]
    claimed_id: String,
    #[form(field = "openid.identity")]
    identity: String,
    #[form(field = "openid.return_to")]
    return_to: String,
    #[form(field = "openid.response_nonce")]
    response: String,
    #[form(field = "openid.assoc_handle")]
    assoc_handle: String,
    #[form(field = "openid.signed")]
    signed: String,
    #[form(field = "openid.sig")]
    sig: String,
}

impl OpenID {
    pub fn to_querystring(self) -> String {
        [
            "openid.ns=".to_string(),
            self.ns,
            "&openid.mode=".to_string(),
            self.mode,
            "&openid.op_endpoint=".to_string(),
            self.op_endpoint,
            "&openid.claimed_id=".to_string(),
            self.claimed_id,
            "&openid.identity=".to_string(),
            self.identity,
            "&openid.return_to=".to_string(),
            self.return_to,
            "&openid.response_nonce=".to_string(),
            self.response,
            "&openid.assoc_handle=".to_string(),
            self.assoc_handle,
            "&openid.signed=".to_string(),
            self.signed,
            "&openid.sig=".to_string(),
            self.sig,
        ]
        .join("")
    }
}
