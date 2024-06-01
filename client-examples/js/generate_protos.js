// const path = require('path');
// const { execSync } = require('child_process');
//
// const PROTO_DIR = path.resolve(__dirname, '../../src/proto');
// const OUT_DIR = __dirname;
//
// execSync(`npx grpc_tools_node_protoc --js_out=import_style=commonjs,binary:${OUT_DIR} --grpc_out=grpc_js:${OUT_DIR} --proto_path=${PROTO_DIR} ${PROTO_DIR}/mighty_inference.proto`, { stdio: 'inherit' });
// execSync(`npx grpc_tools_node_protoc --ts_out=${OUT_DIR} --proto_path=${PROTO_DIR} ${PROTO_DIR}/mighty_inference.proto`, { stdio: 'inherit' });
//
// // generate_protos.js

const { execSync } = require('child_process');
const path = require('path');

// Path to the .proto file
const PROTO_PATH = path.resolve(__dirname, '../../src/proto/mighty_inference.proto');

// Command to generate JavaScript and gRPC files from the .proto file
const command = `npx grpc_tools_node_protoc --js_out=import_style=commonjs,binary:. --grpc_out=grpc_js:. --proto_path=${path.dirname(PROTO_PATH)} ${PROTO_PATH}`;

try {
    execSync(command, { stdio: 'inherit' });
    console.log('Proto files generated successfully.');
} catch (error) {
    console.error('Error generating proto files:', error);
}
