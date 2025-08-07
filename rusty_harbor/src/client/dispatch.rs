use reqwest::Method;
use serde::de::DeserializeOwned;

use crate::{
    client::{HarborClient, error::ClientError},
    request::ToUrl,
};

/// This macro allows for custom implementations that call [`dispatch`](HarborClient::dispatch)
/// with a different [`Method`].
macro_rules! http_method_fn {
    ($name:ident, $method:expr) => {
        pub async fn $name<R, T>(&self, request: R) -> Result<T, ClientError>
        where
            R: ToUrl,
            T: DeserializeOwned,
        {
            self.dispatch($method, request).await
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

    /// Dispatch an HTTP request to the Harbor API.
    ///
    /// It will form the API url using the [`base_url`](HarborClient::base_url) and the
    /// [`to_url`](ToUrl::to_url) function of the request of type `R`.
    /// 
    /// Then it sends the request using basic authorization with
    /// [`username`](HarborClient::username) and [`password`](HarborClient::password), check if the
    /// response is OK and (if so) deserialize it into type `T`.
    async fn dispatch<R, T>(&self, method: Method, request: R) -> Result<T, ClientError>
    where
        R: ToUrl,
        T: DeserializeOwned,
    {
        // Define the API url with the url encoded request
        let url = format!("{}/api/v2.0/{}", self.base_url, request.to_url());

        // Send the request and wait for the response
        let response = self
            .client
            .request(method, url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .map_err(ClientError::from)?;

        // Get the status of the response
        let status = response.status();

        // Check if the response is OK
        if !status.is_success() {
            let message: String = response.text().await?;
            return Err(ClientError::Response { status, message });
        }

        // Deserialize the response in the expected type
        serde_json::from_str::<T>(&response.text().await?).map_err(ClientError::from)
    }
}
