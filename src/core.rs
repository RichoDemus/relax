use std::thread::sleep;
use std::time::Duration;

use reqwest::blocking::Client;
use serde_json::Value;

use crate::oauth;
use crate::servo_controller::ServoController;

pub(crate) struct Core {
    client_id: Option<String>,
    client_secret: Option<String>,
    token: Option<String>,
    user_id: Option<String>,
    client: Client,
    servo_controller: ServoController,
}

impl Core {
    pub(crate) fn from_token(token: String, user_id: String) -> Core {
        Core {
            client_id: None,
            client_secret: None,
            token: Some(token),
            user_id: Some(user_id),
            client: reqwest::blocking::Client::new(),
            servo_controller: ServoController::default(),
        }
    }
    pub(crate) fn from_credentials(client_id: String, client_secret: String) -> Core {
        Core {
            client_id: Some(client_id),
            client_secret: Some(client_secret),
            token: None,
            user_id: None,
            client: reqwest::blocking::Client::new(),
            servo_controller: ServoController::default(),
        }
    }

    pub(crate) fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.get_token_if_missing()?;

        loop {
            let unread_items = self.get_unread_items()?;
            println!("{} unread items", unread_items);
            if unread_items > 0 {
                self.servo_controller.set_enabled();
            } else {
                self.servo_controller.set_disabled();
            }
            sleep(Duration::from_secs(10));
        }
    }

    fn get_token_if_missing(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.token.is_some() {
            return Ok(());
        }

        let (token, user_id) = oauth::get_token(
            &self.client,
            self.client_id.as_ref().expect("expected a client id"),
            self.client_secret
                .as_ref()
                .expect("expected a client secret"),
        )?;
        self.token = Some(token);
        self.user_id = Some(user_id);
        println!("Token is now: {:?}", self.token);
        println!("user id is now: {:?}", self.user_id);
        Ok(())
    }

    fn get_unread_items(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let token = self.token.as_ref().expect("expected token");
        let user_id = self.user_id.as_ref().expect("expected user_id");

        let response = self
            .client
            .get("https://slack.com/api/users.conversations")
            .query(&[
                ("token", token),
                ("types", &String::from("im")),
                ("user", user_id),
            ])
            .send()?
            .text()?;

        let v: Value = serde_json::from_str(response.as_str())?;
        let channels = v["channels"].clone();
        let channels = match channels {
            Value::Array(channels) => channels,
            _ => panic!("channels are not an array :o"),
        };
        let ids = channels
            .iter()
            .map(|channel| channel["id"].clone())
            .map(|id| match id {
                Value::String(str) => str,
                _ => panic!("id is not string"),
            })
            .collect::<Vec<_>>();
        println!("found {} channels", ids.len());

        let unreads = ids
            .into_iter()
            .map(|id| {
                let response = self
                    .client
                    .get("https://slack.com/api/conversations.info")
                    .query(&[("token", token), ("channel", &id)])
                    .send()
                    .expect("failed to get channel info")
                    .text()
                    .expect("failed to get text body");

                let v: Value =
                    serde_json::from_str(response.as_str()).expect("failed to parse json");
                let v = v["channel"].clone();
                v["unread_count"].clone()
                // v
            })
            .map(|count| match count {
                Value::Number(asd) => match asd.as_i64() {
                    Some(c) => c,
                    None => panic!("count is not an integer!"),
                },
                _ => panic!("count is not a number!"),
            })
            .sum::<i64>();
        Ok(unreads)
    }
}
