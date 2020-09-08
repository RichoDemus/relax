use std::sync::mpsc;
use std::thread;

use reqwest::blocking::Client;
use serde_json::Value;

pub(crate) fn get_token(
    client: &Client,
    client_id: &str,
    client_secret: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let scopes = ["mpim:read", "groups:read", "channels:read", "im:read"].join(",");
    let user_url = format!("https://slack.com/oauth/authorize?scope={}&client_id={}&redirect_uri=http://localhost:8080", scopes, client_id);

    let (tx, rx) = mpsc::sync_channel(1);
    // I couldn't find a good way to start and stop a webserver
    // so lets just spawn a thread and forget about it ^^
    thread::spawn(move || {
        println!("Starting server, please visit http://localhost:8080");
        rouille::start_server("localhost:8080", move |request| {
            match request.get_param("code") {
                Some(code) => {
                    tx.send(code.clone())
                        .expect("failed to send code to main thread");
                    rouille::Response::html(format!("<h1>thanks 4 code: {:?}</h1>", code))
                }
                None => rouille::Response::html(format!("<a href=\"{}\">Click me!</a>", user_url)),
            }
        });
    });

    let code = rx.recv()?;

    let params = [
        ("code", code.as_str()),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];

    println!("params: {:?}", params);
    // let mut map = HashMap::new();
    let resp = client
        .post("https://slack.com/api/oauth.access")
        .form(&params)
        .send()?;

    // let map = resp.json::<HashMap<String, String>>();
    let map: serde_json::Value = resp.json()?;
    println!("map: {:?}", map);
    match (map.get("access_token"), map.get("user_id")) {
        (Some(Value::String(token)), Some(Value::String(user_id))) => {
            Ok((token.clone(), user_id.clone()))
        }
        _ => panic!("Missing access token or user id"),
    }
}
