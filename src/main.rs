#[macro_use]
extern crate rouille;

use clap::Clap;

use crate::core::Core;

mod core;
mod oauth;
mod servo_controller;

#[derive(Clap)]
// #[clap]
struct Opts {
    #[clap(short, long)]
    token: Option<String>,
    #[clap(short, long)]
    client_id: Option<String>,
    #[clap(short = "s", long)]
    client_secret: Option<String>,
    #[clap(short, long, default_value = "U02HL73AQ")]
    user_id: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut core = match (opts.token, opts.client_id, opts.client_secret) {
        (Some(token), _, _) => Core::from_token(token, opts.user_id),
        (_, Some(client_id), Some(client_secret)) => {
            Core::from_credentials(client_id, client_secret)
        }
        _ => panic!("Need either token or client id and client_secret"),
    };

    core.run().expect("run failed");
}
