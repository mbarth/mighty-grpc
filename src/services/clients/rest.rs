use std::error::Error;

use async_trait::async_trait;
use log::{debug, error, trace};
use reqwest::Client;
use serde_json::Value;
use tonic::{Request, Response, Status};

use crate::proto::mighty_proto::{
    EmbeddingsResponse, Empty, HealthcheckResponse, MetadataResponse, QuestionAnswerRequest,
    QuestionAnswerResponse, SentenceTransformersResponse, SequenceClassificationResponse,
    TextRequest, TokenClassificationResponse,
};
use crate::services::clients::json_response_converters::{
    json_to_embeddings_response, json_to_metadata_response, json_to_question_answer_response,
    json_to_sentence_transformers_response, json_to_sequence_classification_response,
    json_to_token_classification_response,
};

use super::MightyClient;

/// The `MightyServerRestClient` struct implements the `MightyClient` trait and provides a client that
/// makes HTTP requests to the Mighty Inference Server REST API endpoints.
///
/// It leverages the `reqwest` library to perform asynchronous HTTP requests and handles
/// responses, converting them into appropriate gRPC responses.
#[derive(Debug, Default)]
pub struct MightyServerRestClient {
    client: Client,
    base_url: String,
}

impl MightyServerRestClient {
    pub fn new(base_url: String) -> Self {
        MightyServerRestClient {
            base_url,
            client: Client::new(),
        }
    }

    async fn fetch_json(&self, url: &str) -> Result<Value, Box<dyn Error>> {
        let res = self.client.get(url).send().await?.text().await?;
        let json: Value = serde_json::from_str(&res)?;
        Ok(json)
    }
}

#[async_trait]
impl MightyClient for MightyServerRestClient {
    async fn health_check(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<HealthcheckResponse>, Status> {
        debug!("Received health check request: {:?}", _request);
        let url = format!("{}/healthcheck", self.base_url);

        let res = self.client.get(&url).send().await.map_err(|e| {
            error!("HTTP request error: {}", e);
            Status::internal(format!("HTTP request error: {}", e))
        })?;

        if res.status() == reqwest::StatusCode::OK {
            debug!("Healthcheck response status is OK");
            let response = HealthcheckResponse { success: true };
            return Ok(Response::new(response));
        } else {
            error!("Healthcheck response status is not OK");
            return Err(Status::internal("Healthcheck failed"));
        }
    }

    async fn embeddings(
        &self,
        request: Request<TextRequest>,
    ) -> Result<Response<EmbeddingsResponse>, Status> {
        debug!("Received embeddings request: {:?}", request);
        let text = request.into_inner().text;
        let url = format!("{}/embeddings?text={}", self.base_url, text);
        let json = self
            .fetch_json(&url)
            .await
            .map_err(|e| Status::internal(format!("Error fetching embeddings: {}", e)))?;

        trace!("Parsed JSON: {:?}", json);

        json_to_embeddings_response(&json)
            .map(Response::new)
            .map_err(|e| Status::internal(format!("Error creating response: {}", e)))
    }

    async fn question_answering(
        &self,
        request: Request<QuestionAnswerRequest>,
    ) -> Result<Response<QuestionAnswerResponse>, Status> {
        debug!("Received question answering request: {:?}", request);
        let req = request.into_inner();
        let url = format!(
            "{}/question-answering?question={}&context={}",
            self.base_url, req.question, req.context
        );

        let json: Value = self
            .fetch_json(&url)
            .await
            .map_err(|e| Status::internal(format!("Failed to parse JSON: {}", e)))?;

        trace!("Parsed JSON: {:?}", json);

        json_to_question_answer_response(&json, req.question, req.context)
            .map(Response::new)
            .map_err(|e| Status::internal(format!("Error creating response: {}", e)))
    }

    async fn sentence_transformers(
        &self,
        request: Request<TextRequest>,
    ) -> Result<Response<SentenceTransformersResponse>, Status> {
        debug!("Received sentence_transformers request: {:?}", request);
        let text = request.into_inner().text;
        let url = format!("{}/sentence-transformers?text={}", self.base_url, text);

        let json: Value = self
            .fetch_json(&url)
            .await
            .map_err(|e| Status::internal(format!("Failed to parse JSON: {}", e)))?;

        trace!("Parsed JSON: {:?}", json);

        json_to_sentence_transformers_response(&json)
            .map(Response::new)
            .map_err(|e| Status::internal(format!("Error creating response: {}", e)))
    }

    async fn sequence_classification(
        &self,
        request: Request<TextRequest>,
    ) -> Result<Response<SequenceClassificationResponse>, Status> {
        debug!("Received sequence_classification request: {:?}", request);
        let text = request.into_inner().text;
        let url = format!("{}/sequence-classification?text={}", self.base_url, text);

        let json: Value = self
            .fetch_json(&url)
            .await
            .map_err(|e| Status::internal(format!("Failed to parse JSON: {}", e)))?;

        trace!("Parsed JSON: {:?}", json);

        json_to_sequence_classification_response(&json)
            .map(Response::new)
            .map_err(|e| Status::internal(format!("Error creating response: {}", e)))
    }

    async fn token_classification(
        &self,
        request: Request<TextRequest>,
    ) -> Result<Response<TokenClassificationResponse>, Status> {
        debug!("Received token_classification request: {:?}", request);
        let text = request.into_inner().text;
        let url = format!("{}/token-classification?text={}", self.base_url, text);

        let json: Value = self
            .fetch_json(&url)
            .await
            .map_err(|e| Status::internal(format!("Failed to parse JSON: {}", e)))?;

        trace!("Parsed JSON: {:?}", json);

        json_to_token_classification_response(&json)
            .map(Response::new)
            .map_err(|e| Status::internal(format!("Error creating response: {}", e)))
    }
    async fn metadata(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<MetadataResponse>, Status> {
        debug!("Received metadata request");
        let url = format!("{}/metadata", self.base_url);
        let json: Value = self
            .fetch_json(&url)
            .await
            .map_err(|e| Status::internal(format!("Failed to fetch metadata JSON: {}", e)))?;

        trace!("Parsed JSON: {:?}", json);

        let metadata_response = json_to_metadata_response(&json)?;
        Ok(Response::new(metadata_response))
    }
}
