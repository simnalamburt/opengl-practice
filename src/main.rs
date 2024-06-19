use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/", method = "get")]
    async fn index(&self, Query(name): Query<String>) -> PlainText<String> {
        PlainText(format!("hello: {}", name))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "Hello, world!", "0.1").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();

    let app = Route::new().nest("/api", api_service).nest("/ui", ui);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
