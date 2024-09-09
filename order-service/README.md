# Order Service

The order-service handles incoming orders from the store-front and sends them to RabbitMQ.

## Requirements

- Node.js
- npm
- RabbitMQ

## Setup Instructions
1. Install Node.js and npm
   ```bash
   sudo apt install -y nodejs npm
2. Navigate to the `order-service` directory:
   ```bash
   cd order-service
3. Install dependencies:
   ```bash
   npm install
4. Run the service:
   ```bash
   node index.js 
The service will be running on http://localhost:3000.

## Testing
1. Install the REST Client extension in VS Code to use .http files.
2. Use the provided test-order-service.http file to test the service.

