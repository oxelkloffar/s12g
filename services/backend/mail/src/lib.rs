use std::env;

pub fn send_login_email(email: &str, login_code: &str) {
    println!("Time to send to {} with code {}", email, login_code);
    let dev = match env::var("DEV") {
        Ok(value) => Some(value),
        Err(e) => None,
    };

    match dev {
        Some(_) => {
            println!("DEV mode, will not send email")
        }
        _ => {
            let api_key = match env::var("MAILGUN_API_KEY") {
                Ok(key) => key,
                Err(e) => panic!("Missing env \"MAILGUN_API_KEY\""),
            };

            // form parameters
            let params = [
                ("from", "s12g <noreply@mg.richodemus.com>"),
                ("to", email),
                ("subject", "Welcome to s12g"),
                ("text", "Hi2u! signup link etc etc"),
            ];


            let client = reqwest::Client::new();
            let res = client.post("https://api.eu.mailgun.net/v3/mg.richodemus.com/messages")
                .basic_auth("api", Some(api_key))
                .form(&params)
                .send()
                .unwrap();
        }
    };
}
