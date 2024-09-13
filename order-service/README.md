# Order Service

The Order Service is a backend service that receives orders from the store-front and sends these orders to a RabbitMQ message queue. It enables decoupling of the order processing logic from the product service, allowing for more scalable and maintainable architecture.

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

   The service depends on several npm packages (such as express for the web server and amqplib for RabbitMQ communication). Install these dependencies using npm:

   ```bash
   npm install
6. Run the service:
   
   Once all dependencies are installed, start the service:

   ```bash
   node index.js 
The service will be running on http://localhost:3000.

## Testing
1. Before the service can send orders to RabbitMQ, make sure RabbitMQ is installed and running locally. If not, follow the setup instructions in the RabbitMQ documentation provided earlier.
2. Install the REST Client extension in VS Code to use .http files.
3. Use the provided test-order-service.http file to test the service.

