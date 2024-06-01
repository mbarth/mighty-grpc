const grpc = require('@grpc/grpc-js');
const protoLoader = require('@grpc/proto-loader');
const path = require('path');

const PROTO_PATH = path.resolve(__dirname, '../../src/proto/mighty_inference.proto');

// Load the proto file
const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
    keepCase: true,
    longs: String,
    enums: String,
    defaults: true,
    oneofs: true
});
const mightyProto = grpc.loadPackageDefinition(packageDefinition).mighty_inference_server;

function main() {
    const client = new mightyProto.MightyInference('localhost:50051', grpc.credentials.createInsecure());

    client.HealthCheck({}, (error, response) => {
        if (error) {
            console.error(`Error calling HealthCheck: ${error.message}`);
            console.error(`Status code: ${error.code}`);
            return;
        }
        console.log(`HealthCheck Response: ${response.success}`);
    });

    client.Metadata({}, (error, response) => {
        if (error) {
            console.error(`Error calling Metadata: ${error.message}`);
            console.error(`Status code: ${error.code}`);
            return;
        }
        console.log(`Metadata Response: ${JSON.stringify(response)}`);
    });
}

main();
