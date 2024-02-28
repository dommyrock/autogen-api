/*
The reason we need to use Box<dyn Controller> instead of just dyn Controller is because trait objects are dynamically sized types (DSTs), and Rust requires values to have a known size at compile time. By boxing the trait object, we’re storing it on the heap, which allows us to work with it as if it were a regular, statically sized type.

In the context of the register_controllers function, we’re using Box<dyn Controller> so that we can store controllers of different types in the same vector. Each controller might be a different type, but they all implement the Controller trait, so we can treat them uniformly by boxing them and storing them as trait objects. This allows us to iterate over the vector and call methods on each controller, regardless of its concrete type. This is a form of polymorphism, a common pattern in object-oriented programming.
*/

use axum::Router;
use proc_macro::TokenStream;
use quote::quote;

// fn register_controllers(router: Router<BoxRoute>, controllers: Vec<Box<dyn Controller>>) -> Router<BoxRoute> {
//    let mut router = router;
//    for controller in controllers {
//        router = router.route(controller.path(), get(controller.get).post(controller.post));
//    }
//    router
// }

//Controllers should look somehting like this, meaning i should remove them from app/src/main.rs 
/*
pub trait Controller: DeserializeOwned + Send + 'static {}

async fn get_resource<R: Controller>(Json(_payload): Json<R>) -> impl IntoResponse {
    (StatusCode::OK, format!("Hi"))
}


//Example start
// #[derive(Deserialize)]
// struct Account {
//     email: String,
// }
// trait Resource: DeserializeOwned + Send + 'static {}

// async fn get_res<R: Resource>(Json(_payload): Json<R>) -> impl IntoResponse {
//     (StatusCode::OK, format!("Hi"))
// }

//or
// Use `impl IntoResponse` to avoid having to type the whole type
// async fn impl_trait(uri: Uri) -> impl IntoResponse {
//     (StatusCode::OK, format!("Not Found: {}", uri.path()))
// }

// fn resource_router<R: Resource>() -> Router {
//     Router::new().route("/", get(get_res::<R>))
// }
//Example ends


//I want something like this in the form of DI
    https://github.com/tokio-rs/axum/discussions/358

*/

pub(crate) fn expand(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let gen = quote! {
        pub struct #name Controller;

        impl #name Controller {
            pub async fn get() -> impl Reply {
                format!("GET endpoint for {}", stringify!(#name))
            }

            pub async fn post() -> impl Reply {
                format!("POST endpoint for {}", stringify!(#name))
            }
        }
    };

    gen.into()
}

pub fn generate_controller(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct); //should be one of my struct types Location,Job,Candidate 
    let name = &input.ident;
    let output = quote! {
        #input //Original struct definition

        impl Controller for #name { //impl Controller trait for each struct/model
            fn path() -> &'static str {
                stringify!(#name)
            }

            fn get(&self, _req: GetRequest) -> Result<GetResponse, String> {
                Ok(GetResponse {
                    message: format!("It works for {}.", stringify!(#name)),
                })
            }
        }
    };
    output.into()
}