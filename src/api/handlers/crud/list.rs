use crate::api::adapters::api_adapter::{ApiRequest, ApiResponse, ApiResponseBody, EndpointHandler};
use crate::api::handlers::common::utils::{default_headers, handle_datasource_error};
use crate::data::datasource::base::DataSource;
use crate::error::Result;
use crate::api::common::api_entity::ApiEntity;
use std::collections::HashMap;
use std::sync::Arc;

/// Registers a list endpoint for an entity
pub fn register_list_endpoint<T>(
    datasource: Box<dyn DataSource<T>>,
    base_path: &str,
    endpoints: &mut HashMap<String, EndpointHandler<T>>,
)
where
    T: ApiEntity,
{
    if base_path.is_empty() || base_path.contains(' ') {
        eprintln!("Invalid base_path: {}", base_path);
        return;
    }

    // Define paths for different scenarios (with and without API prefix)
    // Full path
    let endpoint_key = format!("GET:{}", base_path);
    let entity_name = base_path.to_string();
    // Handler for the list endpoint
    let handler = Arc::new(move |_request: ApiRequest| -> Result<ApiResponse<T>> {
        match datasource.get_all( Some(&entity_name)) {
            Ok(items) => {
                let headers = default_headers();
                Ok(ApiResponse {
                    status: 200,
                    headers,
                    body: Some(ApiResponseBody::List(items)),
                })
            }
            Err(err) => Err(handle_datasource_error(err)),
        }
    });

    // Handler and endpoint key registration for the base path
    if endpoints.insert(endpoint_key.clone(), handler.clone()).is_some() {
        eprintln!("Warning: Overwriting existing handler for endpoint key: {}", endpoint_key);
    }
    
    // Also register with a full API path to handle both cases
    let api_endpoint_key = format!("GET:api/{}", base_path);
    if endpoints.insert(api_endpoint_key.clone(), handler.clone()).is_some() {
        eprintln!("Warning: Overwriting existing handler for endpoint key: {}", api_endpoint_key);
    }
}