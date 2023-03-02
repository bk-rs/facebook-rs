/*
sudo vim /etc/nginx/conf.d/xx.conf
    location ~/fb_login_deauth_callback/(\d+) {
        proxy_pass http://127.0.0.1:4001;
    }
sudo systemctl reload nginx

cargo run -p facebook-fb-login-deauth-callback-warp-demo -- 202000000000000 YOUR_APP_SECRET
*/

use std::env;

use facebook_fb_login_deauth_callback_warp::Context;
use warp::Filter as _;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "facebook-fb-login-deauth-callback=info");
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

    let path_prefix: String =
        env::var("PATH_PREFIX").unwrap_or_else(|_| "fb_login_deauth_callback".to_owned());

    let listen_port: u16 = env::var("LISTEN_PORT")
        .unwrap_or_else(|_| "4001".to_owned())
        .parse()?;

    println!(
        r#"app_id: "{}" path_prefix: "{}" listen_port: {}"#,
        app_id, path_prefix, listen_port
    );

    let ctx = MyContext {
        app_id,
        app_secret,
        db: 1,
    };
    let api = facebook_fb_login_deauth_callback_warp::handle(
        path_prefix,
        ctx,
        Box::new(move |payload, ctx| {
            Box::pin(async move {
                let _ = ctx.db;

                println!("payload: {:?}", payload);

                Ok(())
            })
        }),
    );

    let routes = api.with(warp::log("facebook-fb-login-deauth-callback"));

    warp::serve(routes).run(([127, 0, 0, 1], listen_port)).await;

    Ok(())
}

#[derive(Clone)]
struct MyContext {
    app_id: u64,
    app_secret: String,

    db: i64,
}
impl Context for MyContext {
    fn get_app_secret(
        &self,
        app_id: u64,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        if app_id == self.app_id {
            Ok(self.app_secret.to_owned())
        } else {
            Err("app_id mismatch".into())
        }
    }
}
