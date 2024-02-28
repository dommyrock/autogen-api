use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

//local deps
mod models;
use models::{Candidate, Job, Location, Shifts};

trait Resource: Clone + Send + Sync + 'static {}

async fn get_resource<R: Resource>() -> impl IntoResponse {
    let type_namespace_name = std::any::type_name::<R>();
    let msg = format!("Hello, World! from --> {type_namespace_name}");
    (StatusCode::OK, Json(msg))
}

fn register<T: Resource>(router: Router) -> Router {
    let type_name = std::any::type_name::<T>();
    let path = format!("/{}", type_name.split("::").last().unwrap().to_lowercase());
    router.route(
        &path,
        get(get_resource::<T>), //.post(post_resource::<T>),
    )
}

macro_rules! impl_resource {
    ($($t:ty),*) => {
        $(
            impl Resource for $t {}
        )*
    };
}

macro_rules! create_router {
    ($($model:ty),*) => {{
        let mut router = Router::new();
        $(
            router = register::<$model>(router);
        )*
        router
    }};
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    impl_resource!(Location, Candidate, Job, Shifts);
    let app = create_router!(Location, Candidate, Job, Shifts);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
//Examples Generic route registration:
//https://stackoverflow.com/questions/77851864/generic-route-for-generic-handler-with-axum
//https://github.com/tokio-rs/axum/discussions/358
//https://github.com/tokio-rs/axum/discussions/2184
//https://docs.rs/axum/latest/axum/struct.Router.html
//https://docs.rs/axum/latest/axum/response/index.html

//server config example
//https://stackoverflow.com/questions/74270324/axum-pass-parameters-to-handlers

//SQLite types -> https://docs.rs/sqlx/latest/sqlx/sqlite/types
//SQLX CLI     -> https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md

//post
// router.route(
//     &format!("/{name}", name = name),
//     get(get_resource::<T>), //.post(post_resource::<T>),
// )

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
