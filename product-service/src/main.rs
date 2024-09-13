use warp::Filter; // Import the warp web framework and the Filter trait for handling HTTP requests.

#[tokio::main] // This macro marks the main function as asynchronous and uses the Tokio runtime.
async fn main() {
    // Create a CORS (Cross-Origin Resource Sharing) filter to allow access from any origin.
    // CORS is used to handle requests coming from different domains (e.g., the frontend running on another server).
    let cors = warp::cors()
        .allow_any_origin() // Allow requests from any domain (not restricted).
        .allow_methods(vec!["GET"]); // Restrict the allowed HTTP methods (in this case, only GET requests).

    // Define a route that listens for requests to the "/products" path.
    // When a GET request is made to "/products", the server will respond with a JSON array of product objects.
    let products = warp::path("products") // Define the "/products" path.
        .map(|| {
            // Map the request to a response that returns a JSON array of product objects.
            warp::reply::json(&vec![
                // Use `serde_json::json!` macro to create JSON objects representing the products.
                serde_json::json!({ "id": 1, "name": "Dog Food", "price": 19.99 }), // Product 1: Dog Food
                serde_json::json!({ "id": 2, "name": "Cat Food", "price": 34.99 }), // Product 2: Cat Food
                serde_json::json!({ "id": 3, "name": "Bird Seeds", "price": 10.99 }), // Product 3: Bird Seeds
            ])
        })
        .with(cors); // Apply the CORS filter to this route to allow cross-origin requests.

    // Start the web server on IP address 0.0.0.0 and port 3030.
    // The server will listen for incoming requests and route them to the "/products" path.
    warp::serve(products).run(([0, 0, 0, 0], 3030)).await; // Await the server to ensure it's running asynchronously.
}
