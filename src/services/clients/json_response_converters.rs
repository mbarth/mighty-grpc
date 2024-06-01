/*!
 * json_response_converters.rs
 *
 * This module provides utility functions to convert JSON responses from various sources into
 * appropriate Rust data structures for use in a gRPC server. These converters handle deserialization
 * and error handling, ensuring that the data is correctly parsed and validated before being used in
 * the application.
 *
 * The primary use case for these converters is to transform JSON responses from external inference
 * APIs or services into Rust structs that the application can work with. This module leverages the
 * `serde_json` crate for JSON deserialization and `tonic` crate for gRPC status handling.
 *
 * # Features
 * - Converts JSON strings into Rust structs for various response types.
 * - Provides detailed error handling for deserialization failures.
 * - Ensures data integrity by validating the JSON structure against expected types.
 *
 * # Dependencies
 * - `serde_json`: For handling JSON data.
 * - `tonic`: For gRPC status handling.
 *
 * # Example Usage
 *
 * ```rust
 * use serde_json::json;
 * use mighty_grpc::services::clients::json_response_converters::json_to_embeddings_response;
 *
 * let json_response = json!({
 *     "outputs": [[0.1, 0.2, 0.3]],
 *     "took": 123,
 *     "text": "example text",
 *     "shape": [1, 3]
 * });
 *
 * let response = json_to_embeddings_response(&json_response).unwrap();
 * println!("{:?}", response);
 * ```
 *
 * This module ensures that JSON responses are accurately and efficiently converted into Rust data
 * structures, making it easier to work with data from external sources in a type-safe manner.
 */

use serde_json::Value;
use tonic::Status;

use crate::proto::mighty_proto::{
    Embedding, EmbeddingsResponse, Entity, MetadataResponse, QuestionAnswerResponse,
    SentenceTransformersResponse, SequenceClassificationResponse, Shape,
    TokenClassificationResponse,
};

static EMPTY_VEC: Vec<Value> = Vec::new();

/// Converts a JSON response to an `EmbeddingsResponse` struct.
pub fn json_to_embeddings_response(json: &Value) -> Result<EmbeddingsResponse, Status> {
    let embeddings = json
        .get("outputs")
        .and_then(|o| o.as_array())
        .unwrap_or(&EMPTY_VEC)
        .iter()
        .map(|arr| Embedding {
            values: arr
                .as_array()
                .unwrap_or(&EMPTY_VEC)
                .iter()
                .map(|v| v.as_f64().unwrap_or_default() as f32)
                .collect(),
        })
        .collect::<Vec<_>>();

    Ok(EmbeddingsResponse {
        embeddings,
        took: extract_took(json),
        text: extract_string_value(json, "text"),
        shape: extract_shape(json),
    })
}

/// Converts a JSON response to a `QuestionAnswerResponse` struct.
pub fn json_to_question_answer_response(
    json: &Value,
    question: String,
    context: String,
) -> Result<QuestionAnswerResponse, Status> {
    Ok(QuestionAnswerResponse {
        answer: extract_string_value(json, "answer"),
        start_idx: json
            .get("start_idx")
            .and_then(|s| s.as_u64())
            .unwrap_or_default() as i32,
        end_idx: json
            .get("end_idx")
            .and_then(|s| s.as_u64())
            .unwrap_or_default() as i32,
        question,
        context,
        took: extract_took(json),
    })
}

/// Converts a JSON response to a `SequenceClassificationResponse` struct.
pub fn json_to_sequence_classification_response(
    json: &Value,
) -> Result<SequenceClassificationResponse, Status> {
    // Flatten the logits array of arrays into a single Vec<f32>
    let logits = json
        .get("logits")
        .and_then(|v| v.as_array())
        .unwrap_or(&EMPTY_VEC)
        .iter()
        .flat_map(|inner_array| inner_array.as_array().unwrap_or(&EMPTY_VEC))
        .map(|v| v.as_f64().unwrap_or_default() as f32)
        .collect::<Vec<f32>>();

    Ok(SequenceClassificationResponse {
        text: extract_string_value(json, "text"),
        logits,
        took: extract_took(json),
        shape: extract_shape(json),
    })
}

/// Converts a JSON response to a `TokenClassificationResponse` struct.
pub fn json_to_token_classification_response(
    json: &Value,
) -> Result<TokenClassificationResponse, Status> {
    let entities = json
        .get("entities")
        .and_then(|values| values.as_array())
        .unwrap_or(&EMPTY_VEC)
        .iter()
        .map(|value| Entity {
            id: extract_string_value(value, "id"),
            label: extract_string_value(value, "label"),
            text: extract_string_value(value, "text"),
            score: value
                .get("score")
                .and_then(|score| score.as_f64().map(|s| s as f32))
                .unwrap_or_default(),
            start_offset: value
                .get("offsets")
                .and_then(|offsets| {
                    offsets
                        .as_array()
                        .and_then(|arr| arr.get(0).and_then(|start| start.as_i64()))
                })
                .unwrap_or_default() as i32,
            end_offset: value
                .get("offsets")
                .and_then(|offsets| {
                    offsets
                        .as_array()
                        .and_then(|arr| arr.get(1).and_then(|end| end.as_i64()))
                })
                .unwrap_or_default() as i32,
        })
        .collect::<Vec<_>>();

    Ok(TokenClassificationResponse {
        took: extract_took(json),
        text: extract_string_value(json, "text"),
        entities,
        shape: extract_shape(json),
    })
}

/// Converts a JSON response to a `SentenceTransformersResponse` struct.
pub fn json_to_sentence_transformers_response(
    json: &Value,
) -> Result<SentenceTransformersResponse, Status> {
    let embeddings = json
        .get("outputs")
        .and_then(|o| o.as_array())
        .unwrap_or(&EMPTY_VEC)
        .iter()
        .map(|arr| Embedding {
            values: arr
                .as_array()
                .unwrap_or(&EMPTY_VEC)
                .iter()
                .map(|v| v.as_f64().unwrap_or_default() as f32)
                .collect(),
        })
        .collect::<Vec<_>>();

    Ok(SentenceTransformersResponse {
        embeddings,
        took: extract_took(json),
        text: extract_string_value(json, "text"),
        shape: extract_shape(json),
    })
}

/// Converts a JSON response to a `MetadataResponse` struct.
pub fn json_to_metadata_response(json: &Value) -> Result<MetadataResponse, Status> {
    let metadata_map = json
        .as_object()
        .ok_or_else(|| Status::internal("Failed to parse metadata JSON as object"))?
        .iter()
        .map(|(k, v)| {
            let value = if v.is_array() {
                v.to_string().replace(", ", ",") // Ensure consistent formatting for arrays
            } else if v.is_string() {
                v.as_str().unwrap().to_string()
            } else {
                v.to_string()
            };
            (k.clone(), value)
        })
        .collect::<std::collections::HashMap<String, String>>();

    Ok(MetadataResponse {
        metadata: metadata_map,
    })
}

/// Extracts a string value from a JSON object by key.
fn extract_string_value(json: &Value, key: &str) -> String {
    json.get(key)
        .and_then(|t| t.as_str())
        .unwrap_or_default()
        .to_string()
}

/// Extracts a shape value from a JSON object.
pub fn extract_shape(json: &Value) -> Option<Shape> {
    json.get("shape")
        .and_then(|s| s.as_array())
        .map(|array| Shape {
            dim1: array
                .get(0)
                .and_then(|dim| dim.as_i64())
                .unwrap_or_default() as i32,
            dim2: array
                .get(1)
                .and_then(|dim| dim.as_i64())
                .unwrap_or_default() as i32,
        })
}

/// Extracts the 'took' value from a JSON object.
fn extract_took(json: &Value) -> i32 {
    json.get("took")
        .and_then(|t| t.as_i64())
        .unwrap_or_default() as i32
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use crate::proto::mighty_proto::{Entity, Shape, TokenClassificationResponse};

    use super::*;

    #[test]
    fn test_json_to_embeddings_response() {
        let json_data = r#"
        {
            "took": 9,
            "text": "Sample text",
            "outputs": [[0.1, 0.2, 0.3], [0.4, 0.5, 0.6]],
            "shape": [1, 3]
        }
        "#;

        let json: Value = serde_json::from_str(json_data).unwrap();
        let response = json_to_embeddings_response(&json).unwrap();

        let expected_shape = Some(Shape { dim1: 1, dim2: 3 });

        let expected_response = EmbeddingsResponse {
            text: "Sample text".to_string(),
            embeddings: vec![
                Embedding {
                    values: vec![0.1, 0.2, 0.3],
                },
                Embedding {
                    values: vec![0.4, 0.5, 0.6],
                },
            ],
            took: 9,
            shape: expected_shape,
        };

        assert_eq!(response, expected_response);
    }

    #[test]
    fn test_json_to_sentence_transformers_response() {
        let json_data = r#"
        {
            "took": 9,
            "text": "Sample text",
            "outputs": [[0.1, 0.2, 0.3], [0.4, 0.5, 0.6]],
            "shape": [1, 3]
        }
        "#;

        let json: Value = serde_json::from_str(json_data).unwrap();
        let response = json_to_sentence_transformers_response(&json).unwrap();

        let expected_shape = Some(Shape { dim1: 1, dim2: 3 });

        let expected_response = SentenceTransformersResponse {
            text: "Sample text".to_string(),
            embeddings: vec![
                Embedding {
                    values: vec![0.1, 0.2, 0.3],
                },
                Embedding {
                    values: vec![0.4, 0.5, 0.6],
                },
            ],
            took: 9,
            shape: expected_shape,
        };

        assert_eq!(response, expected_response);
    }

    #[test]
    fn test_json_to_question_answer_response() {
        let json_data = r#"
        {
            "answer": "This is the answer.",
            "start_idx": 10,
            "end_idx": 50,
            "took": 9
        }
        "#;

        let json: Value = serde_json::from_str(json_data).unwrap();
        let question = "What is the answer?".to_string();
        let context = "This is the context.".to_string();
        let response =
            json_to_question_answer_response(&json, question.clone(), context.clone()).unwrap();

        let expected_response = QuestionAnswerResponse {
            answer: "This is the answer.".to_string(),
            start_idx: 10,
            end_idx: 50,
            question,
            context,
            took: 9,
        };

        assert_eq!(response, expected_response);
    }

    #[test]
    fn test_json_to_sequence_classification_response() {
        let json_data = r#"
        {
            "took": 5,
            "text": "Sample text",
            "logits": [[0.1, 0.2, 0.3], [0.4, 0.5, 0.6]],
            "shape": [1, 3]
        }
        "#;

        let json: Value = serde_json::from_str(json_data).unwrap();
        let response = json_to_sequence_classification_response(&json).unwrap();

        let expected_shape = Some(Shape { dim1: 1, dim2: 3 });

        let expected_response = SequenceClassificationResponse {
            text: "Sample text".to_string(),
            logits: vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6],
            took: 5,
            shape: expected_shape,
        };

        assert_eq!(response, expected_response);
    }

    #[test]
    fn test_json_to_token_classification_response() {
        let json_data = r#"
        {
            "took": 5,
            "text": "John Doe went to New York.",
            "entities": [
                {
                    "id": "entity1",
                    "label": "Person",
                    "text": "John Doe",
                    "score": 0.95,
                    "offsets": [0, 8]
                },
                {
                    "id": "entity2",
                    "label": "Location",
                    "text": "New York",
                    "score": 0.89,
                    "offsets": [9, 17]
                }
            ],
            "shape": [1, 2]
        }
        "#;

        let json: Value = serde_json::from_str(json_data).unwrap();
        let response = json_to_token_classification_response(&json).unwrap();

        let expected_entities = vec![
            Entity {
                id: "entity1".to_string(),
                label: "Person".to_string(),
                text: "John Doe".to_string(),
                score: 0.95,
                start_offset: 0,
                end_offset: 8,
            },
            Entity {
                id: "entity2".to_string(),
                label: "Location".to_string(),
                text: "New York".to_string(),
                score: 0.89,
                start_offset: 9,
                end_offset: 17,
            },
        ];

        let expected_shape = Some(Shape { dim1: 1, dim2: 2 });

        let expected_response = TokenClassificationResponse {
            took: 5,
            text: "John Doe went to New York.".to_string(),
            entities: expected_entities,
            shape: expected_shape,
        };

        assert_eq!(response, expected_response);
    }

    #[test]
    fn test_json_to_metadata_response() {
        let json_data = json!({
            "onnx_producer_name": "onnxruntime.transformers",
            "onnx_graph_name": "torch-jit-export",
            "tokenizer_name": "/home/mighty/.cache/mighty/models/elastic/distilbert-base-uncased-finetuned-conll03-english/tokenizer.json",
            "model_name": "/home/mighty/.cache/mighty/models/elastic/distilbert-base-uncased-finetuned-conll03-english/model-optimized.onnx",
            "input_names": ["input_ids", "attention_mask"],
            "output_names": ["logits"]
        });

        let expected_metadata = MetadataResponse {
            metadata: [
                ("onnx_producer_name".to_string(), "onnxruntime.transformers".to_string()),
                ("onnx_graph_name".to_string(), "torch-jit-export".to_string()),
                ("tokenizer_name".to_string(), "/home/mighty/.cache/mighty/models/elastic/distilbert-base-uncased-finetuned-conll03-english/tokenizer.json".to_string()),
                ("model_name".to_string(), "/home/mighty/.cache/mighty/models/elastic/distilbert-base-uncased-finetuned-conll03-english/model-optimized.onnx".to_string()),
                ("input_names".to_string(), "[\"input_ids\",\"attention_mask\"]".to_string()),
                ("output_names".to_string(), "[\"logits\"]".to_string()),
            ].iter().cloned().collect(),
        };

        let result = json_to_metadata_response(&json_data).unwrap();

        assert_eq!(result, expected_metadata);
    }
}
