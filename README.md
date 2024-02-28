



### Helpfull links

Axum and Server Config Links:

[generic-route-for-generic-handler-with-axum](https://stackoverflow.com/questions/77851864/generic-route-for-generic-handler-with-axum)

[Generic route example1](https://github.com/tokio-rs/axum/discussions/358)

[Generic route example2](https://github.com/tokio-rs/axum/discussions/2184)

[Axum Router](https://docs.rs/axum/latest/axum/struct.Router.html)

[Axum response](https://docs.rs/axum/latest/axum/response/index.html)

[Server Config Example](https://stackoverflow.com/questions/74270324/axum-pass-parameters-to-handlers)


Sqlx Links:

[SQLite Types](https://docs.rs/sqlx/latest/sqlx/sqlite/types)

[SQLX CLI](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)


### Macro expansion

Current macro that handles code gen that connects model + Routes

```rust
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
```

Expands to

```rust
    let body = async {
        let app = {
            impl Resource for Location {}
            impl Resource for Candidate {}
            impl Resource for Job {}
            impl Resource for Shifts {}
            let mut router = Router::new();
            router = register::<Location>(router);
            router = register::<Candidate>(router);
            router = register::<Job>(router);
            router = register::<Shifts>(router);
            router
        };
        ...
    };
```

### Notes 

>Unfortunately, procedural macros in Rust don’t have the ability to see the entire crate at once. 
They are **invoked on a per-item basis**, meaning they only see the specific item they are applied to. 
This is a limitation of the current design of procedural macros in Rust.

There are workarounds to this limitation. 
Another approach is to use a single macro invocation that takes the names of all the structs as arguments. This way, you can generate the code for all the structs at once.
This approach I used here.

Other approach is to use a separate build script (a build.rs file in your crate root) that scans your crate for items annotated with a specific attribute and generates code based on those items. This approach gives you more control and flexibility, but it also requires more setup and complexity.

something like this 
```rust
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() {
    // Get the directory of the crate
    let out_dir = env::var("OUT_DIR").unwrap();
    let crate_dir = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();

    // Read the contents of the crate
    let mut file = BufReader::new(File::open(crate_dir.join("src/lib.rs")).unwrap());
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse the contents of the crate
    let syntax = syn::parse_file(&contents).unwrap();

    // Iterate over the items in the crate
    for item in syntax.items {
        match item {
            // If the item is a struct
            syn::Item::Struct(item_struct) => {
                // If the struct is annotated with the `#[generate_controller]` attribute
                if item_struct.attrs.iter().any(|attr| attr.path.is_ident("generate_controller")) {
                    // Generate the controller code for the struct
                    let controller_code = generate_controller_code(&item_struct.ident);
                    println!("{}", controller_code);
                }
            }
            _ => {}
        }
    }
}

fn generate_controller_code(name: &syn::Ident) -> String {
    // Generate the controller code for a struct with the given name
    format!("impl Resource for {} {{}}", name)
}

```