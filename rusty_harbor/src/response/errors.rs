use serde::Deserialize;

#[derive(Deserialize)]
pub struct ErrorReponse {
    errors: Vec<HarborError>,
}

#[derive(Deserialize)]
pub struct HarborError {
    code: String,
    message: String,
}
