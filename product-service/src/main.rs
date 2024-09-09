use warp::Filter;

#[tokio::main]
async fn main() {
    // Create a CORS filter to allow access from any origin
    let cors = warp::cors().allow_any_origin().allow_methods(vec!["GET"]);

    // Updated: Return an array of product objects (not arrays)
    let products = warp::path("products")
        .map(|| {
            warp::reply::json(&vec![
                // Return products as objects
                serde_json::json!({ "id": 1, "name": "Dog Food", "price": 19.99 }),
                serde_json::json!({ "id": 2, "name": "Cat Food", "price": 34.99 }),
                serde_json::json!({ "id": 3, "name": "Bird Seeds", "price": 10.99 }),
            ])
        })
        .with(cors);

    warp::serve(products).run(([0, 0, 0, 0], 3030)).await;
}
