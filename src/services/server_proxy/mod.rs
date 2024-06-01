use tonic::{Request, Response, Status};

use crate::proto::mighty_proto::{
    EmbeddingsResponse, Empty, HealthcheckResponse, MetadataResponse, QuestionAnswerRequest,
    QuestionAnswerResponse, SentenceTransformersResponse, SequenceClassificationResponse,
    TextRequest, TokenClassificationResponse,
};
use crate::proto::mighty_proto::mighty_inference_server::{MightyInference, MightyInferenceServer};
use crate::services::clients::MightyClient;

/// The `MightyInferenceServerProxy` struct acts as a proxy to interact with the Mighty Inference
/// Services.
///
/// # Description
/// This struct allows for the integration of different clients (e.g., REST or Binary)
/// to communicate with the inference services. By using a proxy pattern, it enables
/// the switching of underlying service implementations without changing the external interface.
///
/// # Features
/// - Supports multiple client implementations.
/// - Provides a unified interface to interact with various inference services.
/// - Easily extendable to add new service methods or client types.
///
/// # Example
/// ```rust
/// use mighty_grpc::services::clients::MightyClient;
/// use mighty_grpc::services::clients::rest::MightyServerRestClient;
/// use mighty_grpc::services::server_proxy::MightyInferenceServerProxy;
///
/// let client: Box<dyn MightyClient> = Box::new(MightyServerRestClient::new("http://localhost:5050".to_string()));
/// let proxy = MightyInferenceServerProxy::new(client);
/// // Use the proxy to interact with inference services
/// ```
pub struct MightyInferenceServerProxy {
    client: Box<dyn MightyClient>,
}

impl MightyInferenceServerProxy {
    pub fn new(client: Box<dyn MightyClient>) -> Self {
        Self { client }
    }
}

#[tonic::async_trait]
impl MightyInference for MightyInferenceServerProxy {
    async fn embeddings(
        &self,
        request: Request<TextRequest>,
    ) -> Result<Response<EmbeddingsResponse>, Status> {
        let embeddings = self
            .client
            .embeddings(request)
            .await
            .map_err(|e| Status::internal(format!("Error fetching embeddings: {}", e)))?;
        Ok(embeddings)
    }

    async fn question_answering(
        &self,
        request: Request<QuestionAnswerRequest>,
    ) -> Result<Response<QuestionAnswerResponse>, Status> {
        let answer = self
            .client
            .question_answering(request)
            .await
            .map_err(|e| Status::internal(format!("Error fetching question answering: {}", e)))?;
        Ok(answer)
    }

    async fn sentence_transformers(
        &self,
        request: Request<TextRequest>,
    ) -> Result<Response<SentenceTransformersResponse>, Status> {
        let response = self
            .client
            .sentence_transformers(request)
            .await
            .map_err(|e| {
                Status::internal(format!("Error fetching sentence transformers: {}", e))
            })?;
        Ok(response)
    }

    async fn sequence_classification(
        &self,
        request: Request<TextRequest>,
    ) -> Result<Response<SequenceClassificationResponse>, Status> {
        let response = self
            .client
            .sequence_classification(request)
            .await
            .map_err(|e| {
                Status::internal(format!("Error fetching sequence classification: {}", e))
            })?;
        Ok(response)
    }

    async fn token_classification(
        &self,
        request: Request<TextRequest>,
    ) -> Result<Response<TokenClassificationResponse>, Status> {
        let response = self
            .client
            .token_classification(request)
            .await
            .map_err(|e| Status::internal(format!("Error fetching token classification: {}", e)))?;
        Ok(response)
    }

    async fn metadata(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<MetadataResponse>, Status> {
        let response = self
            .client
            .metadata(request)
            .await
            .map_err(|e| Status::internal(format!("Error fetching metadata: {}", e)))?;
        Ok(response)
    }

    async fn health_check(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<HealthcheckResponse>, Status> {
        let response = self
            .client
            .health_check(request)
            .await
            .map_err(|e| Status::internal(format!("Error getting healthcheck: {}", e)))?;
        Ok(response)
    }
}

pub fn create_mighty_inference_server(
    client: Box<dyn MightyClient>,
) -> MightyInferenceServer<MightyInferenceServerProxy> {
    MightyInferenceServer::new(MightyInferenceServerProxy::new(client))
}
