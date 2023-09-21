use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
static ENDPOINT: &str = "https://jsonplaceholder.typicode.com";

#[derive(Deserialize, Debug, Serialize)]
pub struct User {
    id: u32,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company,
}

#[derive(serde::Deserialize, Debug, Serialize)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(serde::Deserialize, Debug, Serialize)]
struct Geo {
    lat: String,
    lng: String,
}

#[derive(serde::Deserialize, Debug, Serialize)]
struct Company {
    name: String,
    catchPhrase: String,
    bs: String,
}

#[derive(serde::Deserialize, Debug, Serialize)]
pub struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

pub async fn get_many() -> Result<Vec<User>, Error> {
    let client = Client::new();
    let res = client.get(format!("{}/users", ENDPOINT)).send().await?;
    let body = res.json::<Vec<User>>().await?;
    Ok(body)
}

pub async fn get_by_id(id: String) -> Result<User, Error> {
    let client = Client::new();
    let res = client
        .get(format!("{}/users/{}", ENDPOINT, id))
        .send()
        .await?;
    let body = res.json::<User>().await?;
    Ok(body)
}

pub async fn get_posts(user_id: String) -> Result<Vec<Post>, Error> {
    let client = Client::new();
    let res = client
        .get(format!("{}/users/{}/posts", ENDPOINT, user_id))
        .send()
        .await?;
    let body = res.json::<Vec<Post>>().await?;
    Ok(body)
}
