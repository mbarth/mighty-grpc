import grpc
import mighty_inference_pb2
import mighty_inference_pb2_grpc

def handle_grpc_error(e):
    print(f"Error calling gRPC service: {e.details()}")
    status_code = e.code()
    print(f"Status code: {status_code.name}")

def run():
    # Set up a connection to the gRPC server
    channel = grpc.insecure_channel('localhost:50051')

    # Create a stub (client)
    stub = mighty_inference_pb2_grpc.MightyInferenceStub(channel)

    # Example for HealthCheck service
    empty_request = mighty_inference_pb2.Empty()
    try:
        health_response = stub.HealthCheck(empty_request)
        print("HealthCheck Response:", health_response)
    except grpc.RpcError as e:
        handle_grpc_error(e)

    # Example for Metadata service
    try:
        metadata_response = stub.Metadata(empty_request)
        print("Metadata Response:", metadata_response)
    except grpc.RpcError as e:
        handle_grpc_error(e)

    # Example for Embeddings service
    text_request = mighty_inference_pb2.TextRequest(text="Hello, gRPC!")
    try:
        response = stub.Embeddings(text_request)
        print("Embeddings Response:", response)
    except grpc.RpcError as e:
        handle_grpc_error(e)


if __name__ == '__main__':
    run()
