use salvo::prelude::*;
use serde::Serialize;

#[derive(Serialize,Debug)]
struct User {
    name: String,
    age: i32,
}

#[handler]
async fn hello_world(res: &mut Response) {
    res.render(Text::Plain("Hello World"));
}

#[handler]
async fn vjson(res: &mut Response) {
    let user = User {
        name: "vilay".to_string(),
        age: 18,
    };
    res.render(Json(user));
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let router = Router::new().get(hello_world).push(Router::with_path("vjson").get(vjson));
    let accepter = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(accepter).serve(router).await;
}
