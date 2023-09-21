use salvo::prelude::TcpListener;
use salvo::writing::Text;
use salvo::{handler, Listener, Response, Router, Server};

use trying_rust::routes::user_router;

#[handler]
async fn index(res: &mut Response) {
    res.render(Text::Html("<a href=\"/api/v1/users\">GET users</a>"))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let user_router = user_router::new();

    let router = Router::new()
        .get(index)
        .push(Router::with_path("/api/v1").push(user_router));

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    Server::new(acceptor).serve(router).await;
}
