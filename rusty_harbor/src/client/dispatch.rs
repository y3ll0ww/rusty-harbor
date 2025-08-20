use reqwest::{Method, RequestBuilder, Response};
use serde::de::DeserializeOwned;

use crate::{
    client::{HarborClient, error::ClientError},
    request::HarborRequest,
};

/// This macro allows for custom implementations that call [`dispatch`](HarborClient::dispatch)
/// with a different [`Method`].
macro_rules! http_method_fn {
    ($name:ident, $method:expr) => {
        pub async fn $name<R: HarborRequest>(
            &self,
            request: R,
        ) -> Result<R::Response, ClientError> {
            let request_builder = self.request($method, request)?;
            let response = self.dispatch::<R>(request_builder).await?;
            deserialize_response($method, response).await
        }
    };
}

impl HarborClient {
    // Implement dispatchers for various HTTP request methods.
    http_method_fn!(delete, Method::DELETE);
    http_method_fn!(get, Method::GET);
    http_method_fn!(head, Method::HEAD);
    http_method_fn!(patch, Method::PATCH);
    http_method_fn!(post, Method::POST);
    http_method_fn!(put, Method::PUT);

    fn request<R: HarborRequest>(
        &self,
        method: Method,
        request: R,
    ) -> Result<RequestBuilder, ClientError> {
        // Define the API url with the url encoded request
        let url = format!("{}/api/v2.0/{}", self.base_url, request.to_url());

        // Create the request
        let request = self
            .client
            .request(method.clone(), url)
            .headers(request.headers().map_err(ClientError::Header)?)
            .basic_auth(&self.username, Some(&self.password));

        if let Some(cloned) = request.try_clone() {
            let req = cloned.build()?;
            println!("{:#?}", req);
        }

        Ok(request)
    }

    /// Dispatch an HTTP request to the Harbor API.
    ///
    /// It will form the API url using the [`base_url`](HarborClient::base_url) and the
    /// [`to_url`](ToUrl::to_url) function of the request of type `R`.
    ///
    /// Then it sends the request using basic authorization with
    /// [`username`](HarborClient::username) and [`password`](HarborClient::password), check if the
    /// response is OK and (if so) deserialize it into type `T`.
    async fn dispatch<R: HarborRequest>(
        &self,
        request: RequestBuilder,
    ) -> Result<Response, ClientError> {
        // Send the request and wait for the response
        let response = request.send().await?;

        // Get the status of the response
        let status = response.status();

        // Check if the response is OK
        if !status.is_success() {
            let message: String = response.text().await?;
            return Err(ClientError::Response { status, message });
        }

        Ok(response)
    }
}

async fn deserialize_response<R: DeserializeOwned>(
    method: Method,
    response: Response,
) -> Result<R, ClientError> {
    // Deserialize the response in the expected type
    serde_json::from_str::<R>(&{
        match method {
            // Special case for HEAD since it won't return any body
            Method::HEAD => String::from("null"),
            _ => response.text().await?,
        }
    })
    .map_err(ClientError::from)
}
