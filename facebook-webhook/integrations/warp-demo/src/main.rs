/*
cargo run -p facebook-webhook-warp-demo -- 202000000000000 YOUR_APP_SECRET
*/

use std::{env, error};

use facebook_webhook_warp::Context;
use passwords::PasswordGenerator;
use warp::Filter as _;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    run().await
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "facebook-webhook-warp-demo=info");
    }
    pretty_env_logger::init();

    let app_id: u64 = env::args()
        .nth(1)
        .or_else(|| env::var("APP_ID").ok())
        .ok_or("app_id missing")?
        .parse()?;
    let app_secret = env::args()
        .nth(2)
        .or_else(|| env::var("APP_SECRET").ok())
        .ok_or("app_secret missing")?;

    let path_prefix: String = env::var("PATH_PREFIX").unwrap_or_else(|_| "fb_webhooks".to_owned());
    let verify_token: String = env::var("VERIFY_TOKEN").unwrap_or_else(|_| {
        PasswordGenerator {
            length: 32,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: true,
            spaces: false,
            exclude_similar_characters: true,
            strict: true,
        }
        .generate_one()
        .unwrap()
    });

    let listen_port: u16 = env::var("LISTEN_PORT")
        .unwrap_or_else(|_| "4001".to_owned())
        .parse()?;

    println!(
        r#"facebook-webhook-warp-demo app_id: "{}" path_prefix: "{}" verify_token: "{}" listen_port: {}"#,
        app_id, path_prefix, verify_token, listen_port
    );

    let ctx = MyContext {
        app_id,
        app_secret,
        verify_token,
        db: 1,
    };
    let api = facebook_webhook_warp::handle(
        path_prefix,
        ctx,
        Box::new(move |payload, ctx| {
            Box::pin(async move {
                println!("payload: {:?} db: {:?}", payload, ctx.db);

                Ok(())
            })
        }),
    );

    let routes = api.with(warp::log("facebook-webhook-warp-demo"));

    warp::serve(routes).run(([127, 0, 0, 1], listen_port)).await;

    Ok(())
}

#[derive(Clone)]
struct MyContext {
    app_id: u64,
    app_secret: String,
    verify_token: String,
    db: i64,
}
impl Context for MyContext {
    fn get_verify_token(&self, app_id: u64) -> Result<String, Box<dyn error::Error + Send + Sync>> {
        if app_id == self.app_id {
            Ok(self.verify_token.to_owned())
        } else {
            Err("app_id mismatch".into())
        }
    }

    fn get_app_secret(&self, app_id: u64) -> Result<String, Box<dyn error::Error + Send + Sync>> {
        if app_id == self.app_id {
            Ok(self.app_secret.to_owned())
        } else {
            Err("app_id mismatch".into())
        }
    }
}
