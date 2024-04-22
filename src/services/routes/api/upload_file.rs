use crate::models::api_response::ApiResponse;
use rocket::http::ContentType;
use rocket::{http::Status, post, Data};
use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use rs_web_api_models::api_message::{ApiMessage, ApiOk};
use std::{fs, path::PathBuf};

#[post("/upload_file", data = "<data>")]
pub async fn upload_file(content_type: &ContentType, data: Data<'_>) -> ApiResponse {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("photo")
            .size_limit(10 * 1024 * 1024) // 10 MiB
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
    ]);

    let multipart_form_data = MultipartFormData::parse(content_type, data, options)
        .await
        .unwrap();
    let photo = multipart_form_data.files.get("photo");

    if let Some(file_fields) = photo {
        let file_field = &file_fields[0];

        let content_type = &file_field.content_type;
        let file_name = &file_field.file_name;
        let path = &file_field.path;

        println!("Content Type: {:?}", content_type);
        println!("File Name:    {:?}", file_name);
        println!("File Path:    {:?}", path);

        // Assuming 'save_path' is the path where you want to save the file
        let save_path = "./tmp"; // Modify this according to your directory structure
        let file_name = file_field.file_name.as_ref().unwrap(); // Assuming file_name is always present

        // Get the path where the file is stored temporarily
        let temp_file_path = &file_field.path;

        // Create a new path where the file will be saved permanently
        let mut new_file_path = PathBuf::from(save_path);
        new_file_path.push(file_name);

        // Move the temporary file to the permanent location
        fs::copy(temp_file_path, &new_file_path).expect("Failed to save file");
    }

    let msg = ApiOk::FileUploadSucceeded;
    let msg = ApiMessage::Ok(msg);
    ApiResponse {
        json: serde_json::to_value(msg).unwrap(),
        status: Status::InternalServerError,
    }
}
