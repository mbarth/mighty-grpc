use async_trait::async_trait;
use tonic::{Request, Response, Status};

use crate::proto::mighty_proto::{
    EmbeddingsResponse, Empty, HealthcheckResponse, MetadataResponse, QuestionAnswerRequest,
    QuestionAnswerResponse, SentenceTransformersResponse, SequenceClassificationResponse,
    TextRequest, TokenClassificationResponse,
};

use super::MightyClient;

/// The `BinaryClient` struct is a placeholder implementation of the `MightyClient` trait.
/// This client is intended to interface with a binary executable for making inference requests.
///
/// Currently, it provides dummy implementations for all required methods in the `MightyClient`
/// trait. These methods should be updated to invoke the actual binary and handle responses
/// accordingly.
pub struct BinaryClient;

impl BinaryClient {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BinaryClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MightyClient for BinaryClient {
    async fn health_check(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<HealthcheckResponse>, Status> {
        unimplemented!()
    }
    async fn embeddings(
        &self,
        _request: Request<TextRequest>,
    ) -> Result<Response<EmbeddingsResponse>, Status> {
        unimplemented!()
    }

    async fn question_answering(
        &self,
        _request: Request<QuestionAnswerRequest>,
    ) -> Result<Response<QuestionAnswerResponse>, Status> {
        unimplemented!()
    }

    async fn sentence_transformers(
        &self,
        _request: Request<TextRequest>,
    ) -> Result<Response<SentenceTransformersResponse>, Status> {
        unimplemented!()
    }

    async fn sequence_classification(
        &self,
        _request: Request<TextRequest>,
    ) -> Result<Response<SequenceClassificationResponse>, Status> {
        unimplemented!()
    }

    async fn token_classification(
        &self,
        _request: Request<TextRequest>,
    ) -> Result<Response<TokenClassificationResponse>, Status> {
        unimplemented!()
    }
    async fn metadata(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<MetadataResponse>, Status> {
        unimplemented!()
    }
}
