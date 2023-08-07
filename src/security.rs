use actix_web::{Error, HttpRequest};
use std::{
    fs::{File, OpenOptions},
    io::{prelude::*, BufReader},
    path::Path,
};
use uuid::Uuid;

use crate::config;

pub fn get_keys() -> Vec<String> {
    let file = match File::open("keys") {
        Ok(data) => data,
        Err(_) => File::create("keys").expect("FAILED_TO_CREATE_KEYS"),
    };

    BufReader::new(file)
        .lines()
        .map(|l| l.expect("ERROR_PARSING_KEYS"))
        .collect()
}

pub fn add_key() -> Result<String, Error> {
    let id = Uuid::new_v4().to_string();

    if !Path::new("keys").exists() {
        File::create("keys").expect("FAILED_TO_CREATE_KEYS");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("keys")
        .expect("FAILED_TO_OPEN_KEYS");

    writeln!(file, "{id}")?;

    Ok(id)
}

pub fn is_logged_in(request: HttpRequest) -> bool {
    match request.cookie("_session") {
        Some(c) => get_keys().contains(&c.value().to_string()),
        None => false,
    }
}

pub fn is_valid_token(token: String) -> bool {
    let conf = config::get();

    token == conf.authorization
}
