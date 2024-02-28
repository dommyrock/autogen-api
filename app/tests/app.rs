#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    #[allow(non_snake_case)]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Location {
        pub id: i32,
        pub state: String,
    }
    trait Resource: Clone + Send + Sync + 'static {}
    impl Resource for Location {}

    #[test]
    fn type_extract_stringify() {
        assert_eq!(get_type_1::<Location>(), "t");
    }

    #[test]
    fn type_extract_typename() {
        assert_eq!(get_type_2::<Location>(), "/app::tests::Location");
    }

    #[test]
    fn type_extract_typename_split() {
        assert_eq!(gettype_3::<Location>(), "/location");
    }

    fn get_type_1<T: Resource>() -> String {
        stringify!(T).to_lowercase()
    }

    fn get_type_2<R: Resource>() -> String {
        format!("/{}", std::any::type_name::<R>())
    }

    fn gettype_3<T: Resource>() -> String {
        let type_name = std::any::type_name::<T>();
        let path = format!("/{}", type_name.split("::").last().unwrap().to_lowercase());
        path
    }
}
