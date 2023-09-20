use salvo::prelude::TcpListener;
use salvo::writing::{Json, Text};
use salvo::{handler, Listener, Request, Response, Result, Router, Server};

use trying_rust::services::user_service::user_service;

#[handler]
async fn get_users(res: &mut Response) -> Result<()> {
    let users = user_service::get_many()
        .await
        .map(|users| users)
        .expect("Failed to get users");

    res.render(Json(users));
    Ok(())
}

#[handler]
async fn get_user_by_id(res: &mut Response, req: &mut Request) -> Result<()> {
    let id = req.param::<String>("id").map(|id| id).unwrap();

    let user = user_service::get_by_id(id)
        .await
        .map(|user| user)
        .expect("Failed to get user");

    res.render(Json(user));
    Ok(())
}

#[handler]
async fn get_user_posts(res: &mut Response, req: &mut Request) -> Result<()> {
    let id = req.param::<String>("id").map(|id| id).unwrap();

    let posts = user_service::get_posts(id)
        .await
        .map(|posts| posts)
        .expect("Failed to get posts");

    res.render(Json(posts));
    Ok(())
}

#[handler]
async fn index(res: &mut Response) {
    res.render(Text::Html("<a href=\"/api/v1/users\">GET users</a>"))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().get(index).push(
        Router::with_path("/api/v1").push(
            Router::with_path("/users").get(get_users).push(
                Router::with_path("<id>")
                    .get(get_user_by_id)
                    .push(Router::with_path("/posts").get(get_user_posts)),
            ),
        ),
    );

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    Server::new(acceptor).serve(router).await;
}
