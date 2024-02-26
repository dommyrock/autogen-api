use axum::Router;

//local
mod models;

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

    async fn handler() {}

    let app = Router::new().fallback(handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // let x = models::Location {id :12,state: "Arcansas".to_string()};

    Ok(())
}

struct MyModel1Controller {}
struct MyModel2Controller {}
