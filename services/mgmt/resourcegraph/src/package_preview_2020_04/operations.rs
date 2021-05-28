#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
pub async fn resources(
    operation_config: &crate::OperationConfig,
    query: &QueryRequest,
) -> std::result::Result<QueryResponse, resources::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!("{}/providers/Microsoft.ResourceGraph/resources", operation_config.base_path(),);
    let mut url = url::Url::parse(url_str).map_err(resources::Error::ParseUrlError)?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::POST);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .map_err(resources::Error::GetTokenError)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
    let req_body = azure_core::to_json(query).map_err(resources::Error::SerializeError)?;
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder.body(req_body).map_err(resources::Error::BuildRequestError)?;
    let rsp = http_client
        .execute_request(req)
        .await
        .map_err(resources::Error::ExecuteRequestError)?;
    match rsp.status() {
        http::StatusCode::OK => {
            let rsp_body = rsp.body();
            let rsp_value: QueryResponse =
                serde_json::from_slice(rsp_body).map_err(|source| resources::Error::DeserializeError(source, rsp_body.clone()))?;
            Ok(rsp_value)
        }
        status_code => {
            let rsp_body = rsp.body();
            let rsp_value: ErrorResponse =
                serde_json::from_slice(rsp_body).map_err(|source| resources::Error::DeserializeError(source, rsp_body.clone()))?;
            Err(resources::Error::DefaultResponse {
                status_code,
                value: rsp_value,
            })
        }
    }
}
pub mod resources {
    use crate::{models, models::*};
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL: {0}")]
        ParseUrlError(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequestError(http::Error),
        #[error("Failed to execute request: {0}")]
        ExecuteRequestError(azure_core::HttpError),
        #[error("Failed to serialize request body: {0}")]
        SerializeError(serde_json::Error),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        DeserializeError(serde_json::Error, bytes::Bytes),
        #[error("Failed to get access token: {0}")]
        GetTokenError(azure_core::Error),
    }
}
pub async fn resource_changes(
    operation_config: &crate::OperationConfig,
    parameters: &ResourceChangesRequestParameters,
) -> std::result::Result<ResourceChangeList, resource_changes::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!("{}/providers/Microsoft.ResourceGraph/resourceChanges", operation_config.base_path(),);
    let mut url = url::Url::parse(url_str).map_err(resource_changes::Error::ParseUrlError)?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::POST);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .map_err(resource_changes::Error::GetTokenError)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
    let req_body = azure_core::to_json(parameters).map_err(resource_changes::Error::SerializeError)?;
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder.body(req_body).map_err(resource_changes::Error::BuildRequestError)?;
    let rsp = http_client
        .execute_request(req)
        .await
        .map_err(resource_changes::Error::ExecuteRequestError)?;
    match rsp.status() {
        http::StatusCode::OK => {
            let rsp_body = rsp.body();
            let rsp_value: ResourceChangeList =
                serde_json::from_slice(rsp_body).map_err(|source| resource_changes::Error::DeserializeError(source, rsp_body.clone()))?;
            Ok(rsp_value)
        }
        status_code => {
            let rsp_body = rsp.body();
            let rsp_value: ErrorResponse =
                serde_json::from_slice(rsp_body).map_err(|source| resource_changes::Error::DeserializeError(source, rsp_body.clone()))?;
            Err(resource_changes::Error::DefaultResponse {
                status_code,
                value: rsp_value,
            })
        }
    }
}
pub mod resource_changes {
    use crate::{models, models::*};
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL: {0}")]
        ParseUrlError(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequestError(http::Error),
        #[error("Failed to execute request: {0}")]
        ExecuteRequestError(azure_core::HttpError),
        #[error("Failed to serialize request body: {0}")]
        SerializeError(serde_json::Error),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        DeserializeError(serde_json::Error, bytes::Bytes),
        #[error("Failed to get access token: {0}")]
        GetTokenError(azure_core::Error),
    }
}
pub async fn resource_change_details(
    operation_config: &crate::OperationConfig,
    parameters: &ResourceChangeDetailsRequestParameters,
) -> std::result::Result<ResourceChangeData, resource_change_details::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!(
        "{}/providers/Microsoft.ResourceGraph/resourceChangeDetails",
        operation_config.base_path(),
    );
    let mut url = url::Url::parse(url_str).map_err(resource_change_details::Error::ParseUrlError)?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::POST);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .map_err(resource_change_details::Error::GetTokenError)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
    let req_body = azure_core::to_json(parameters).map_err(resource_change_details::Error::SerializeError)?;
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder
        .body(req_body)
        .map_err(resource_change_details::Error::BuildRequestError)?;
    let rsp = http_client
        .execute_request(req)
        .await
        .map_err(resource_change_details::Error::ExecuteRequestError)?;
    match rsp.status() {
        http::StatusCode::OK => {
            let rsp_body = rsp.body();
            let rsp_value: ResourceChangeData = serde_json::from_slice(rsp_body)
                .map_err(|source| resource_change_details::Error::DeserializeError(source, rsp_body.clone()))?;
            Ok(rsp_value)
        }
        status_code => {
            let rsp_body = rsp.body();
            let rsp_value: ErrorResponse = serde_json::from_slice(rsp_body)
                .map_err(|source| resource_change_details::Error::DeserializeError(source, rsp_body.clone()))?;
            Err(resource_change_details::Error::DefaultResponse {
                status_code,
                value: rsp_value,
            })
        }
    }
}
pub mod resource_change_details {
    use crate::{models, models::*};
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL: {0}")]
        ParseUrlError(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequestError(http::Error),
        #[error("Failed to execute request: {0}")]
        ExecuteRequestError(azure_core::HttpError),
        #[error("Failed to serialize request body: {0}")]
        SerializeError(serde_json::Error),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        DeserializeError(serde_json::Error, bytes::Bytes),
        #[error("Failed to get access token: {0}")]
        GetTokenError(azure_core::Error),
    }
}
pub async fn resources_history(
    operation_config: &crate::OperationConfig,
    request: &ResourcesHistoryRequest,
) -> std::result::Result<serde_json::Value, resources_history::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!(
        "{}/providers/Microsoft.ResourceGraph/resourcesHistory",
        operation_config.base_path(),
    );
    let mut url = url::Url::parse(url_str).map_err(resources_history::Error::ParseUrlError)?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::POST);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .map_err(resources_history::Error::GetTokenError)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
    let req_body = azure_core::to_json(request).map_err(resources_history::Error::SerializeError)?;
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder.body(req_body).map_err(resources_history::Error::BuildRequestError)?;
    let rsp = http_client
        .execute_request(req)
        .await
        .map_err(resources_history::Error::ExecuteRequestError)?;
    match rsp.status() {
        http::StatusCode::OK => {
            let rsp_body = rsp.body();
            let rsp_value: serde_json::Value =
                serde_json::from_slice(rsp_body).map_err(|source| resources_history::Error::DeserializeError(source, rsp_body.clone()))?;
            Ok(rsp_value)
        }
        status_code => {
            let rsp_body = rsp.body();
            let rsp_value: ErrorResponse =
                serde_json::from_slice(rsp_body).map_err(|source| resources_history::Error::DeserializeError(source, rsp_body.clone()))?;
            Err(resources_history::Error::DefaultResponse {
                status_code,
                value: rsp_value,
            })
        }
    }
}
pub mod resources_history {
    use crate::{models, models::*};
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL: {0}")]
        ParseUrlError(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequestError(http::Error),
        #[error("Failed to execute request: {0}")]
        ExecuteRequestError(azure_core::HttpError),
        #[error("Failed to serialize request body: {0}")]
        SerializeError(serde_json::Error),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        DeserializeError(serde_json::Error, bytes::Bytes),
        #[error("Failed to get access token: {0}")]
        GetTokenError(azure_core::Error),
    }
}
pub mod operations {
    use crate::models::*;
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<OperationListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/providers/Microsoft.ResourceGraph/operations", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).map_err(list::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: OperationListResult =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
}
