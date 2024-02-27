use axum::body::Body;
use axum::handler::Handler;
use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::{de::DeserializeOwned, Deserialize};
 

//local
mod models;
use models::{Candidate, Job, Location, Shifts};
//Breaks because models.rs is broken at the moment  (Resolve macro  registration first)

//Examples Generic route registration:
//https://stackoverflow.com/questions/77851864/generic-route-for-generic-handler-with-axum
//https://github.com/tokio-rs/axum/discussions/358
//https://github.com/tokio-rs/axum/discussions/2184

pub trait Controller: DeserializeOwned + Send + 'static {}

async fn get_resource<R: Controller>(Json(_payload): Json<R>) -> impl IntoResponse {
    (StatusCode::OK, format!("Hi"))
}
// async fn post_resource<R: Controller>(Json(_payload): Json<R>) -> impl IntoResponse {
//     (StatusCode::OK, format!("Hi"))
// }

fn register<T: Controller>(router: Router) -> Router {
    router.route(
        &format!("/{name}", name = stringify!(T).to_lowercase()),
        get(get_resource::<T>)//.post(post_resource::<T>),
    )
}

macro_rules! register_all {
    ($router:expr, $($model:ty),*) => {{
        let mut router = $router;
        $(
            router = register::<$model>(router);
        )*
        router
    }};
}
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
macro_rules! impl_handler {
    ($($t:ty),*) => {
        $(
            impl Controller for $t {

                // async fn get_resource<R: Controller>(Json(_payload): Json<R>) -> impl IntoResponse {
                //     (StatusCode::OK, format!("Hi"))
                // }

                // fn get(&self, _req: GetRequest) -> Result<GetResponse, String> {
                //     Ok(GetResponse {
                //         message: format!("It works for {}.", stringify!($t)),
                //     })
                // }
            }
        )*
    };
}


// Use `impl IntoResponse` to avoid having to type the whole type
// async fn impl_trait(uri: Uri) -> impl IntoResponse {
//     (StatusCode::OK, format!("Not Found: {}", uri.path()))
// }

//Example start
// #[derive(Deserialize)]
// struct Account {
//     email: String,
// }
// trait Resource: DeserializeOwned + Send + 'static {}

// async fn get_res<R: Resource>(Json(_payload): Json<R>) -> impl IntoResponse {
//     (StatusCode::OK, format!("Hi"))
// }

// fn resource_router<R: Resource>() -> Router {
//     Router::new().route("/", get(get_res::<R>))
// }
//Example ends

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // let controllers: Vec<Box<dyn Controller>> = vec![
    //     Box::new(MyModel1Controller),
    //     Box::new(MyModel2Controller),
    //     // Add more controllers here...
    // ];

    // let app = register_controllers(Router::new(), controllers);

    //Router
    //https://docs.rs/axum/latest/axum/struct.Router.html

    // async fn handler() {}

    let mut app = Router::new();
    
    impl_handler!(Location, Candidate, Job, Shifts);
    app = register_all!(app, Location, Candidate, Job, Shifts);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
