# Order Service

The order-service handles incoming orders from the store-front and sends them to RabbitMQ.

## Requirements

- Node.js
- npm
- RabbitMQ

## Setup Instructions
1. Update the package list:
   ```bash
   sudo apt update
2. Install the NodeSource repository for Node.js 20: First, download and add the NodeSource repository for Node.js 20.x.
   ```bash
   curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
3. Install Node.js and npm
   ```bash
   sudo apt update
   sudo apt install -y nodejs
4. Navigate to the `order-service` directory:
   ```bash
   cd order-service
5. Install dependencies:
   ```bash
   npm install
6. Run the service:
   ```bash
   node index.js 
The service will be running on http://localhost:3000.

## Testing
1. Install the REST Client extension in VS Code to use .http files.
2. Use the provided test-order-service.http file to test the service.

