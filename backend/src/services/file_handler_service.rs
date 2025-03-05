use crate::error::app_error::AppError;
use crate::models::property_model::CreatePropertyModel;
use crate::utils::constants::MAX_FILE_SIZE;
use actix_multipart::form::MultipartForm;
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use uuid::Uuid;

pub fn process_image(
    create_property_model: MultipartForm<CreatePropertyModel>,
) -> Result<String, AppError> {
    let check_name = create_property_model
        .image
        .file_name
        .clone()
        .unwrap_or("null".to_string());
    let max_file_size = (MAX_FILE_SIZE).clone();

    match &check_name[check_name.len() - 4..] {
        ".png" | ".jpg" => {}
        _ => return Err(AppError::NotFoundError(" Invalid File type".to_string())),
    };

    match create_property_model.image.size {
        0 => return Err(AppError::ValidationError("File not found".to_string())),
        length if length > max_file_size as usize => {
            return Err(AppError::ValidationError(
                "File size not alloud more than 10M".to_string(),
            ))
        }
        _ => {}
    };
    let upload_dir = "./upload/images";
    let mut file_path = PathBuf::from(upload_dir.clone());
    fs::create_dir_all(upload_dir).map_err(|err| AppError::ValidationError(err.to_string()))?;
    let temp_file_path = create_property_model.image.file.path();

    let filename = format!("{}.jpg", Uuid::new_v4());
    file_path.push(&filename);

    std::fs::copy(&temp_file_path, &file_path)
        .map_err(|error| AppError::IoError(error.to_string()))?;
    Ok(filename)
}

// pub async fn upload_image(mut payload: Multipart) -> Result<String, String> {
//     let upload_dir = "./upload/images";
//     fs::create_dir_all(upload_dir).map_err(|err| format!("Failed to create directory: {}", err))?;
//     let mut file_url = None;
//     while let Some(item) = payload.next().await {
//         let mut field = item.map_err(|err| format!("Failed to read field: {}", err))?;
//         let content_type = field
//             .content_disposition()
//             .unwrap()
//             .get_filename_ext()
//             .unwrap()
//             .to_string();
//         let allowed_file_extensions: Vec<String> =
//             vec!["jpg".to_string(), "jpeg".to_string(), "png".to_string()];

//         if allowed_file_extensions.contains(&content_type) {
//             return Err("Only image files are allowed".to_string());
//         }

//         let filename = format!("{}.jpg", Uuid::new_v4());
//         let filepath = format!("{}/{}", upload_dir, filename);
//         let mut file =
//             fs::File::create(&filepath).map_err(|err| format!("Failed to create file: {}", err))?;
//         while let Some(chunk) = field.next().await {
//             let data = chunk.map_err(|err| format!("Failed to read chunk: {}", err))?;
//             file.write_all(&data)
//                 .map_err(|err| format!("Failed to write to file: {}", err))?;
//         }
//         file_url = Some(format!(
//             "{}/{}",
//             constants::DOMAIN_URL.to_string(),
//             filename
//         ))
//     }
//     if let Some(url) = file_url {
//         Ok(url)
//     } else {
//         Err("No file uploaded".to_string())
//     }
// }
