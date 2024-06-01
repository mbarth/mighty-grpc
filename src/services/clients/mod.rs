use async_trait::async_trait;
use tonic::{Request, Response, Status};

use crate::proto::mighty_proto::{
    EmbeddingsResponse, Empty, HealthcheckResponse, MetadataResponse, QuestionAnswerRequest,
    QuestionAnswerResponse, SentenceTransformersResponse, SequenceClassificationResponse,
    TextRequest, TokenClassificationResponse,
};

#[cfg(feature = "binary")]
pub mod binary;
pub mod json_response_converters;
#[cfg(feature = "rest")]
pub mod rest;

/// The `MightyClient` trait defines a set of asynchronous methods for interacting with a variety of
/// natural language processing (NLP) services. Implementations of this trait are expected to provide
/// methods for health checking, obtaining embeddings, answering questions, performing sentence
/// transformations, sequence classification, token classification, and fetching metadata.
///
/// Each method takes a specific request type and returns a corresponding response type wrapped in
/// a `Result`. The `Result` type contains either a successful `Response` or a `Status` indicating an
/// error. The trait requires implementations to be both `Send` and `Sync` to ensure thread safety in
/// asynchronous contexts.
///
/// # Methods
///
/// * `health_check`: Performs a health check on the service to ensure it's operational.
/// * `embeddings`: Retrieves embeddings for a given text input.
/// * `question_answering`: Returns the inferred answer based on a provided context.
/// * `sentence_transformers`: Transforms sentences into embeddings.
/// * `sequence_classification`: Returns output probabilities (logits - unnormalized final scores of your model).
/// * `token_classification`: Identifies and classifies tokens (e.g., named entities) within a text.
/// * `metadata`: Fetches The Mighty Inference Server configuration and model metadata.
#[async_trait]
pub trait MightyClient: Send + Sync {
    async fn health_check(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<HealthcheckResponse>, Status>;

    async fn embeddings(
        &self,
        request: Request<TextRequest>,
    ) -> Result<Response<EmbeddingsResponse>, Status>;

    async fn question_answering(
        &self,
        request: Request<QuestionAnswerRequest>,
    ) -> Result<Response<QuestionAnswerResponse>, Status>;

    async fn sentence_transformers(
        &self,
        _request: Request<TextRequest>,
    ) -> Result<Response<SentenceTransformersResponse>, Status>;

    async fn sequence_classification(
        &self,
        _request: Request<TextRequest>,
    ) -> Result<Response<SequenceClassificationResponse>, Status>;

    async fn token_classification(
        &self,
        _request: Request<TextRequest>,
    ) -> Result<Response<TokenClassificationResponse>, Status>;

    async fn metadata(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<MetadataResponse>, Status>;
}
