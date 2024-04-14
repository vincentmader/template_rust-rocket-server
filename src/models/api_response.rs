use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{self, Responder, Response},
    serde::json::Value,
};

#[derive(Debug)]
pub struct ApiResponse {
    pub json: Value,
    pub status: Status,
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, req: &'r Request) -> response::Result<'static> {
        Response::build_from(self.json.respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
