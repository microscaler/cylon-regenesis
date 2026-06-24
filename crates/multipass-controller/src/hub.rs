use anyhow::{anyhow, Result};
use tracing::{info, warn};

/// Send drain request to the Cylon Resurrection Hub.
/// If the node is not found (404), it treats it as success since there's nothing to drain.
pub async fn drain_node(hub_api: &str, node_id: &str) -> Result<()> {
    let url = format!("{}/v2/nodes/{}/drain", hub_api.trim_end_matches('/'), node_id);
    info!("Sending drain request to hub: {}", url);

    let client = reqwest::Client::new();
    let res = client.post(&url).send().await?;

    let status = res.status();
    if status.is_success() {
        let body = res.text().await.unwrap_or_default();
        info!("Successfully initiated drain for node {}: {}", node_id, body);
        return Ok(());
    }

    if status == reqwest::StatusCode::NOT_FOUND {
        warn!("Hub returned 404 for node {} (might not be registered yet), continuing drain.", node_id);
        return Ok(());
    }

    let err_text = res.text().await.unwrap_or_default();
    Err(anyhow!(
        "Failed to drain node {} on hub: status={}, error={}",
        node_id,
        status,
        err_text
    ))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    use axum::{routing::post, Router};

    #[tokio::test]
    async fn test_drain_node_success() {
        let app = Router::new().route(
            "/v2/nodes/node-1/drain",
            post(|| async { "drained" }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let hub_api = format!("http://{}", addr);
        assert!(drain_node(&hub_api, "node-1").await.is_ok());
    }

    #[tokio::test]
    async fn test_drain_node_404() {
        let app = Router::new().route(
            "/v2/nodes/node-1/drain",
            post(|| async { (axum::http::StatusCode::NOT_FOUND, "not found") }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let hub_api = format!("http://{}", addr);
        assert!(drain_node(&hub_api, "node-1").await.is_ok());
    }

    #[tokio::test]
    async fn test_drain_node_500() {
        let app = Router::new().route(
            "/v2/nodes/node-1/drain",
            post(|| async { (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "error") }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let hub_api = format!("http://{}", addr);
        assert!(drain_node(&hub_api, "node-1").await.is_err());
    }
}
