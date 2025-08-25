use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    auth::SignatureProvider,
};

/**
 * Generates timestamp and signature for authenticated API requests.
 *
 * # Arguments
 * - `params`: Serializable parameters for the request.
 * - `signer`: Signature provider for generating Ed25519 signatures.
 * - `recv_window`: Request timing window in milliseconds.
 * - `include_api_key`: Whether to include the API key in the signature.
 *
 * # Returns
 * - `(u64, String)`: Tuple of (signature, signature_payload)
 */
pub async fn generate_signature<T: Serialize>(
    params: &T,
    signer: &dyn SignatureProvider,
    recv_window: u64,
    include_api_key: bool,
) -> Result<(String, String)> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
        
    let params_query = serde_urlencoded::to_string(params)
        .context("Failed to serialize parameters")?;
        
    let mut all_params = std::collections::BTreeMap::new();
    
    if !params_query.is_empty() {
        for pair in params_query.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                all_params.insert(key.to_string(), value.to_string());
            }
        }
    }
    
    if include_api_key {
        all_params.insert("apiKey".to_string(), signer.get_api_key().to_string());
    }
    
    all_params.insert("timestamp".to_string(), timestamp.to_string());
    all_params.insert("recvWindow".to_string(), recv_window.to_string());
    
    let signature_payload = all_params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");
        
    let signature = signer.sign(&signature_payload).await?;
    Ok((signature, signature_payload))
}