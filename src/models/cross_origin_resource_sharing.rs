use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, Cors, CorsOptions};
use std::str::FromStr;

pub fn cors() -> Cors {
    let allowed_origins = AllowedOrigins::all();
    let allowed_headers = AllowedHeaders::all();
    let allowed_methods: AllowedMethods = ["Get", "Post"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();

    CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Configuration of Cross-Origin-Resource-Sharing (CORS) failed.")
}
