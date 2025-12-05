use axum::{
    Json, body::Bytes, extract::Multipart, http::StatusCode, response::{IntoResponse, Response}
};

use crate::editor::quest::quest_file::QuestFile;

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB

pub async fn read_quest(mut multipart: Multipart) -> Response {
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            let data: Bytes = field.bytes().await.unwrap();

            if data.is_empty() {
                return (
                    StatusCode::BAD_REQUEST,
                    "The file is empty.",
                ).into_response();
            }

            if data.len() as u64 > MAX_FILE_SIZE {
                return (
                    StatusCode::BAD_REQUEST,
                    format!("Uploaded file too large. Max is {} bytes.", MAX_FILE_SIZE),
                ).into_response();
            }

            let mut buffer = better_cursor::from_buffer(data.to_vec());

            match QuestFile::from_reader(&mut buffer) {
                Ok(quest) => return Json(quest).into_response(),
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
    }

    StatusCode::BAD_REQUEST.into_response()
}

