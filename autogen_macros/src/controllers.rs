use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

use proc_macro::TokenStream;
use quote::quote;

//Turns out this is not so efficient , so for now im going in favour of declarative macro_rules

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
trait Resource: Clone + Send + Sync + 'static {}

pub fn generate_controller(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct); //should be one of my struct types Location,Job,Candidate 
    let name = &input.ident;
    let output = quote! {
        #input //Original struct definition
        //integrate "impl_resource_create_router" macro here 
        
    };
    output.into()
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

fn register<T: Resource>(router: Router) -> Router {
    let type_name = std::any::type_name::<T>();
    let path = format!("/{}", type_name.split("::").last().unwrap().to_lowercase());
    router
        .route(&path, get(get_resource::<T>).post(post::<T>))
        .fallback(fb::<T>)
}
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
