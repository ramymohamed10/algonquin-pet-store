# Product Service

The product-service is responsible for providing the product catalog. It's built using Rust.

## Requirements

- Rust (latest stable version)
- Cargo

## Setup Instructions
1. Update your package list:
   ```bash
   sudo apt update
2. Install the build-essential package, which includes gcc and other essential build tools
   ```bash
   sudo apt install build-essential
3. Install Rust
   ```bash
   curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
4. Add the Rust toolchain directory to the PATH environment variable,
   ```bash
   source "$HOME/.cargo/env"
5. Navigate to the `product-service` directory:
   ```bash
   cd product-service
6. Install dependencies:
   ```bash
   cargo run
The service will be running on http://localhost:3030.

## Testing
1. Install the REST Client extension in VS Code to use .http files.
2. Use the provided test-product-service.http file to test the service.

