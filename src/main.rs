use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{
    param::Query, payload::Json, payload::PlainText, Object, OpenApi, OpenApiService,
};

static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[derive(Object)]
struct CounterResponse {
    count: u64,
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/", method = "get")]
    async fn index(&self, Query(name): Query<String>) -> PlainText<String> {
        PlainText(format!("hello: {}", name))
    }

    #[oai(path = "/counter", method = "get")]
    async fn counter(&self) -> Json<CounterResponse> {
        // Read the counter value without incrementing it.
        Json(CounterResponse {
            count: COUNTER.load(std::sync::atomic::Ordering::Acquire),
        })
    }

    #[oai(path = "/counter", method = "post")]
    async fn counter_incr(&self) -> Json<CounterResponse> {
        // Increment the counter and return the new value.
        Json(CounterResponse {
            count: COUNTER.fetch_add(1, std::sync::atomic::Ordering::AcqRel) + 1,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "Hello, world!", "0.1").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();

    let app = Route::new()
        .nest(r"/ui", ui)
        // TODO: Publish spec only in development mode
        .at(r"/<spec(?:\.yaml)?>", api_service.spec_endpoint_yaml())
        .at(r"/spec.json", api_service.spec_endpoint())
        .nest(r"/api", api_service);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
