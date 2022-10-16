use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::{Status, ContentType};

#[test]
fn home_page() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(super::home)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type().unwrap(), ContentType::HTML);
}

#[test]
fn impressum_page() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(super::impressum)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type().unwrap(), ContentType::HTML);
}

#[test]
fn not_found_page() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!("/THIS-PAGE-DOES-NOT-EXIST")).dispatch();
    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type().unwrap(), ContentType::HTML);
}

#[test]
fn static_file() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!("/css/main.css")).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type().unwrap(), ContentType::CSS);
}