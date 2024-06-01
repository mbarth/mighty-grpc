use mighty_inference_server::mighty_inference_client::MightyInferenceClient;
use mighty_inference_server::{Empty};
use crate::mighty_inference_server::TextRequest;

pub mod mighty_inference_server {
    tonic::include_proto!("mighty_inference_server");
}

#[tokio::test]
async fn test_embeddings() {
    let mut client = MightyInferenceClient::connect("http://127.0.0.1:50051")
        .await
        .expect("Failed to connect to gRPC server");

    let request = tonic::Request::new(TextRequest {
        text: "test text".into(),
    });

    match client.embeddings(request).await {
        Ok(response) => {
            let response_inner = response.into_inner();
            assert_eq!(response_inner.text, "test text");
        },
        Err(e) => {
            println!("Error in test_get_embeddings: {:?}", e);
            panic!("Failed to get embeddings");
        }
    }
}

#[tokio::test]
async fn test_healthcheck() {
    let mut client = MightyInferenceClient::connect("http://127.0.0.1:50051")
        .await
        .expect("Failed to connect to gRPC server");

    let request = tonic::Request::new(Empty{});

    match client.health_check(request).await {
        Ok(response) => {
            let response_inner = response.into_inner();
            assert!(response_inner.success);
        },
        Err(e) => {
            println!("Error in test_healthcheck: {:?}", e);
            panic!("Failed to perform healthcheck");
        }
    }
}
