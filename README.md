



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