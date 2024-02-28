use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

mod models;
use models::{Candidate, Job, Location, Shifts};

trait Resource: Clone + Send + Sync + 'static {}

async fn get_resource<R: Resource>() -> impl IntoResponse {
    let type_namespace_name = std::any::type_name::<R>();
    let msg = format!("Hello, World! from --> {type_namespace_name}");
    (StatusCode::OK, Json(msg))
}
async fn post<R: Resource>() -> impl IntoResponse {
    (StatusCode::OK, "Entity updated")
}
async fn fb<R: Resource>(uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        format!("Route '{uri}' doesn't exist."),
    )
}

fn register<T: Resource>(router: Router) -> Router {
    let type_name = std::any::type_name::<T>();
    let path = format!("/{}", type_name.split("::").last().unwrap().to_lowercase());
    router
        .route(&path, get(get_resource::<T>).post(post::<T>))
        .fallback(fb::<T>)
}

macro_rules! impl_resource_create_router {
    ($($model:ty),*) => {{
        $(
            impl Resource for $model {}
        )*

        let mut router = Router::new();
        $(
            router = register::<$model>(router);
        )*
        router
    }};
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let app = impl_resource_create_router![Location, Candidate, Job, Shifts];
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    
    axum::serve(listener, app)
        .await
        .expect("Failed to bind sever to localhost:3000");
    Ok(())
}

//macro examples

//option 1
// macro_rules! impl_handler {
//     ($($t:ty),*) => {
//         $(
//             impl Handler for $t {
//                 fn get(&self, _req: GetRequest) -> Result<GetResponse, String> {
//                     Ok(GetResponse {
//                         message: format!("It works for {}.", stringify!($t)),
//                     })
//                 }
//             }
//         )*
//     };
// }
//option 2
// macro_rules! impl_handler {
//     ($($t:ty),*) => {
//         $(
//             impl Controller for $t {

//                 async fn get_resource<R: Controller>(Json(_payload): Json<R>) -> impl IntoResponse {
//                     (StatusCode::OK, format!("Hi"))
//                 }

//                 fn get(&self, _req: GetRequest) -> Result<GetResponse, String> {
//                     Ok(GetResponse {
//                         message: format!("It works for {}.", stringify!($t)),
//                     })
//                 }
//             }
//         )*
//     };
// }
