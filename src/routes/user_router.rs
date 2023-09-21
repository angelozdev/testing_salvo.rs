use salvo::writing::Json;
use salvo::{handler, Request, Response, Result, Router};

use crate::services::user_service;

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

pub fn new() -> Router {
    Router::with_path("/users").get(get_users).push(
        Router::with_path("<id>")
            .get(get_user_by_id)
            .push(Router::with_path("/posts").get(get_user_posts)),
    )
}
