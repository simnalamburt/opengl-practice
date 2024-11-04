use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{
    param::Query, payload::Json, payload::PlainText, ApiResponse, Object, OpenApi, OpenApiService,
    Union,
};

static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[derive(ApiResponse)]
enum CounterResponse {
    #[oai(status = 200)]
    Ok(Json<CounterPayload>),

    #[oai(status = 400)]
    BadRequest(Json<InvalidArguments>),

    #[oai(status = 500)]
    InternalServerError(Json<InternalServerError>),
}

#[derive(Union)]
#[oai(discriminator_name = "type")]
enum CounterPayload {
    Welcome(Welcome),
    Count(Count),
}

#[derive(Object)]
struct Welcome {
    message: String,
}

#[derive(Object)]
struct Count {
    count: u64,
}

#[derive(Object)]
struct InvalidArguments {
    message: String,
    amount: i64,
}

#[derive(Object)]
struct InternalServerError {
    message: String,
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/", method = "get")]
    async fn index(&self, Query(name): Query<String>) -> PlainText<String> {
        PlainText(format!("hello: {}", name))
    }

    #[oai(path = "/counter", method = "get")]
    async fn counter(&self) -> CounterResponse {
        let count = COUNTER.load(std::sync::atomic::Ordering::Acquire);

        if count == 0 {
            return CounterResponse::Ok(Json(CounterPayload::Welcome(Welcome {
                message: "Welcome to the counter API!".to_string(),
            })));
        }

        // Read the counter value without incrementing it.
        CounterResponse::Ok(Json(CounterPayload::Count(Count { count })))
    }

    #[oai(path = "/counter", method = "post")]
    async fn counter_incr(&self, amount: Query<Option<i64>>) -> CounterResponse {
        // Increment the counter by the specified amount, or by 1 if no amount is provided.
        let amount = amount.unwrap_or(1);
        if amount < 0 {
            return CounterResponse::BadRequest(Json(InvalidArguments {
                message: format!("amount must be non-negative, got {}", amount),
                amount,
            }));
        }

        if amount > 100 {
            return CounterResponse::InternalServerError(Json(InternalServerError {
                message: format!("omg such a big number ({})", amount),
            }));
        }

        let amount = amount as u64;

        CounterResponse::Ok(Json(CounterPayload::Count(Count {
            count: COUNTER.fetch_add(amount, std::sync::atomic::Ordering::AcqRel) + amount,
        })))
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
