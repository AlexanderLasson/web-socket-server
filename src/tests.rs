#[cfg(test)]
mod tests {
    // We import necessary dependencies
    use super::*;
    use warp::test::request;

    // A unit test for WebSocket connection
    #[tokio::test]
    async fn test_websocket_connection() {
        // Create a database connection pool for testing
        let pool = create_pool("postgres://user:password@localhost/test_database").await;

        // Define WebSocket route for testing
        let ws_route = warp::path("ws")
            .and(warp::ws())
            .and(with_db(pool.clone()))
            .map(|ws: warp::ws::Ws, pool: sqlx::Pool<sqlx::Postgres>| {
                // Handle WebSocket upgrade and pass the connection pool
                ws.on_upgrade(move |socket| handle_connection(socket, pool))
            });

        // Send a test request to the WebSocket route
        let resp = request().method("GET").path("/ws").reply(&ws_route).await;

        // Assert that the WebSocket handshake was successful
        assert_eq!(resp.status(), 101, "WebSocket handshake failed");
    }
}
