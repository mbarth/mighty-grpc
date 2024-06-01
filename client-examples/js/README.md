# Javascript gRPC Client for Mighty Inference gRPC Services

This is a simple JavaScript client for interacting with the Mighty Inference gRPC service.

## Prerequisites

- Node.js (version 14 or higher recommended)
- npm (Node Package Manager)

## Setup

1. Install the required npm packages:

    ```bash
    npm install
    ```

2. Generate the necessary gRPC files from the protobuf definitions.

    ```bash
    npm run generate
    ```

## Running the Client

1. Ensure the Mighty Inference server is [running](https://max.io/documentation.html#Installation%20and%20Quick%20Start).
2. Run the gRPC server in another terminal using `cargo run --bin grpc`.
3. After installing the dependencies and generating the gRPC files, you can run the client.

    ```bash
    npm start
    ```

This will connect to the Mighty Inference gRPC server running on `localhost:50051` and call the `HealthCheck` and `Metadata` methods.
