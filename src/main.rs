use reqwest::{
    cookie::{CookieStore, Jar},
    header::HeaderValue,
    Client,
};

use std::sync::Arc;

use std::io;
use std::io::Write;

fn input(prompt: &str, expect: &str) -> String {
    let mut input = String::new(); 
    print!("{prompt}");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect(&expect);
    if input.ends_with('\n') || input.ends_with('\r') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    return input;
}

fn cookie_jar() {
    let jar = Arc::new(Jar::default());

    let client = Client::builder()
        .cookie_provider(Arc::clone(&jar))
        .cookie_store(true)
        .build()
        .unwrap();

    // let cookie = HeaderValue::from_static(
    //     "refresh-token=example; Expires=Fri, 21 Oct 2022 02:48:00 GMT; Secure; HttpOnly",
    //     "access-token=example; Expires=Fri, 21 Oct 2022 02:48:00 GMT; Secure; HttpOnly",
    // );
    // let url = "https://mewe.com/i/wwwxkz".parse().unwrap();
    // jar.set_cookies(&mut [cookie].iter(), &url);
}

fn credentials_client() {
    // let client = reqwest::blocking::Client::builder().cookie_store(true).build()?;
    // client.post("https://mewe.com/login")
    //     .form(&[
    //         ("action", ""),
    //         ("method", ""),
    //         ("email", ""),
    //         ("password", "")
    //     ])
    //     .send()?;
}

fn main() {
    let mut method = String::new();
    while method != '1'.to_string() && 
          method != '2'.to_string() {
        method = input("Methods:\n\n1 - Cookie\n2 - Credentials\n\n=> ", "One method is expected");
    }
    
    if method == '1'.to_string() {
        let refresh_token = input("refresh-token: ", "Both cookies are necessary");
        let access_token = input("access-token: ", "Both cookies are necessary");
        println!("refresh_token: {refresh_token}");
        println!("access_token: {access_token}");
        // cookie_jar();
    }

    if method == '2'.to_string() {
        let email_telephone = input("email-telephone: ", "Either the email or the telephone is required");
        let password = input("password: ", "Password is required");
        println!("email-telephone: {email_telephone}");
        println!("password: {password}");
        // credentials_client();
    }
}