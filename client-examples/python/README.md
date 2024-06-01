# Python Client for Mighty Inference gRPC Services

This directory contains a Python client example to access the Mighty Inference gRPC services.

## Requirements

- Python 3.7+
- `grpcio` and `grpcio-tools` packages

## Setup

1. Install the required Python packages:

    ```bash
    pip install grpcio grpcio-tools
    ```

2. Ensure you have the `mighty_inference.proto` file located in the `src/proto` directory.

## Generating Python Code from Protobuf

Navigate to the root directory of the project and run the following command to generate Python code from the Protobuf definitions:

    ```bash
    python -m grpc_tools.protoc -I./src/proto --python_out=./client-examples/python --grpc_python_out=./client-examples/python src/proto/mighty_inference.proto
    ```

This will generate `mighty_inference_pb2.py` and `mighty_inference_pb2_grpc.py` files in the `client-examples/python` directory.

## Python Client Example

Here is an example Python client to access the Mighty Inference gRPC services.

### `mighty_client.py`

```python
import grpc
from mighty_inference_pb2 import TextRequest
from mighty_inference_pb2_grpc import MightyInferenceStub

def run():
    # Connect to the gRPC server
    channel = grpc.insecure_channel('localhost:50051')
    stub = MightyInferenceStub(channel)
    
    # Example: Fetch embeddings
    text_request = TextRequest(text="example text")
    response = stub.Embeddings(text_request)
    print("Embeddings:", response.embeddings)

if __name__ == '__main__':
    run()
```

## Running the Example

1. Ensure the Mighty Inference server is [running](https://max.io/documentation.html#Installation%20and%20Quick%20Start).
2. Run the gRPC server in another terminal using `cargo run --bin grpc`.
3. Execute the Python client:

    ```sh
    python client-examples/python/mighty_client.py
    ```
