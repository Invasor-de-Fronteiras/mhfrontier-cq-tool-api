use axum::{
    body::Bytes, extract::Multipart, http::StatusCode, response::{IntoResponse, Response}
};

use crate::editor::quest::quest_file::QuestFile;

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB

pub async fn edit_quest(mut multipart: Multipart) -> Response {
    let mut file_bytes: Option<Bytes> = None;
    let mut quest_json: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name() {
            Some("file") => {
                let data = field.bytes().await.unwrap();
                if data.len() as u64 > MAX_FILE_SIZE {
                    return (
                        StatusCode::BAD_REQUEST,
                        format!("Uploaded file too large. Max is {} bytes.", MAX_FILE_SIZE),
                    ).into_response();
                }
                file_bytes = Some(data);
            }
            Some("quest") => {
                quest_json = Some(field.text().await.unwrap());
            }
            _ => {}
        };
    }

    let file_bytes = match file_bytes {
        Some(f) if !f.is_empty() => f,
        _ => return StatusCode::BAD_REQUEST.into_response(),
    };

    let quest_json = match quest_json {
        Some(q) => q,
        None => return StatusCode::BAD_REQUEST.into_response(),
    };

    let mut quest: QuestFile = match serde_json::from_str(&quest_json) {
        Ok(q) => q,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    let mut writer = better_cursor::from_buffer(file_bytes.to_vec());

    if let Err(_err) = QuestFile::save_buffer(&mut writer, &mut quest) {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let buffer = writer.into_inner();

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/octet-stream")
        .body(buffer.into())
        .unwrap()
}