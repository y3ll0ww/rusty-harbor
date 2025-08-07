use reqwest::{Method, Response};

use crate::{
    client::{HarborClient, error::ClientError},
    request::ToUrl,
};

macro_rules! http_method_fn {
    ($name:ident, $method:expr) => {
        pub async fn $name<R: ToUrl>(&self, request: R) -> Result<Response, ClientError> {
            self.dispatch($method, request).await
        }
    };
}

impl HarborClient {
    async fn dispatch<R: ToUrl>(
        &self,
        method: Method,
        request: R,
    ) -> Result<Response, ClientError> {
        let url = format!("{}/api/v2.0/{}", self.base_url, request.to_url());

        self.client
            .request(method, url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .map_err(ClientError::from)
    }

    http_method_fn!(delete, Method::DELETE);
    http_method_fn!(get, Method::GET);
    http_method_fn!(head, Method::HEAD);
    http_method_fn!(patch, Method::PATCH);
    http_method_fn!(post, Method::POST);
    http_method_fn!(put, Method::PUT);
}
