use crate::authentication::scope;

pub async fn health() {}

pub async fn example_public() -> String {
    "".to_string()
}

pub async fn example_private(_: scope::Superuser) -> String {
    "".to_string()
}

pub async fn create_user(_: scope::Superuser) -> String {
    "".to_string()
}
