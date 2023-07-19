// use actix_web::middleware::Logger;
// use actix_web::{dev, Error, HttpRequest, HttpResponse, Result};

// // Function to check user token, you should implement this based on your authentication mechanism
// fn check_user_token(req: &HttpRequest) -> bool {
//     if let Some(token) = req.headers().get("Authorization") {
//         // Perform token validation here (e.g., check if the token is valid and belongs to a user).
//         // Return true if the token is valid, false otherwise.
//         // For simplicity, we assume all tokens are valid here.
//         return true;
//     }
//     false
// }

// pub async fn middleware_handler(req: HttpRequest, srv: &actix_web::App) -> Result<_, Error> {
//     // Check if the user token is valid
//     if !check_user_token(&req) {
//         // If the token is not valid, return an unauthorized response.
//         return Ok(HttpResponse::Unauthorized().body("Invalid or missing token"));
//     }

//     // If the token is valid, pass the request down the chain to the next middleware or the route handler.

//     Ok(())
// }
