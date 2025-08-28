use tauri::Manager;
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FetchRequest {
    url: String,
    method: Option<String>,
    headers: Option<std::collections::HashMap<String, String>>,
    body: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FetchResponse {
    status: u16,
    body: String,
    headers: std::collections::HashMap<String, String>,
}

#[tauri::command]
pub(crate) async fn tauri_fetch(request: FetchRequest) -> Result<FetchResponse, String> {
    let client = reqwest::Client::new();

    let mut req = client
        .request(
            request
                .method
                .as_deref()
                .unwrap_or("GET")
                .parse()
                .map_err(|e| format!("Invalid method: {}", e))?,
            &request.url,
        );

    if let Some(body) = &request.body {
        req = req.body(body.clone());
    }

    if let Some(headers) = &request.headers {
        let mut header_map = HeaderMap::new();
        for (key, value) in headers {
            let header_name = HeaderName::from_bytes(key.as_bytes())
                .map_err(|e| format!("Invalid header name: {}", e))?;
            let header_value = HeaderValue::from_str(value)
                .map_err(|e| format!("Invalid header value: {}", e))?;
            header_map.insert(header_name, header_value);
        }
        req = req.headers(header_map);
    }

    let resp = req
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = resp.status().as_u16();
    let headers = resp
        .headers()
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                v.to_str().unwrap_or("").to_string(),
            )
        })
        .collect();

    let body = resp
        .text()
        .await
        .map_err(|e| format!("Failed to read body: {}", e))?;

    Ok(FetchResponse {
        status,
        headers,
        body,
    })
}
