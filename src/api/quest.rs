use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder, Scope};

use crate::editor::quest::quest_file::QuestFile;

const MAX_FILE_SIZE: u64 = 1024 * 1024 * 10; // 10 MB

#[derive(MultipartForm)]
pub struct FormRead {
    file: TempFile
}

#[post("/read")]
async fn read_quest(form: MultipartForm<FormRead>) -> impl Responder {
    match form.file.size {
        0 => return HttpResponse::BadRequest().finish(),
        length if length as u64 > MAX_FILE_SIZE => {
            return HttpResponse::BadRequest()
                .body(format!("The uploaded file is too large. Maximum size is {} bytes.", MAX_FILE_SIZE));
        },
        _ => {}
    };
    
    let file_path = form.file.file.path().to_str().unwrap();

    match QuestFile::from_path(file_path) {
        Ok(quest) => HttpResponse::Ok().json(quest),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[derive(MultipartForm)]
pub struct FormEdit {
    file: TempFile,
    quest: Text<String>
}

#[post("/edit")]
async fn edit_quest(req: HttpRequest, form: MultipartForm<FormEdit>) -> impl Responder {
    match form.file.size {
        0 => return HttpResponse::BadRequest().finish(),
        length if length as u64 > MAX_FILE_SIZE => {
            return HttpResponse::BadRequest()
                .body(format!("The uploaded file is too large. Maximum size is {} bytes.", MAX_FILE_SIZE));
        },
        _ => {}
    };

    let quest = serde_json::from_str::<QuestFile>(&form.quest);

    if let Err(err) = quest {
        return HttpResponse::BadRequest().body(err.to_string())
    }

    let mut quest = quest.unwrap();
    let file_path = form.file.file.path().to_str().unwrap();

    match  QuestFile::save_to(file_path, &mut quest) {
        Ok(_) => {
            let file = actix_files::NamedFile::open_async(file_path).await.unwrap();

            file.into_response(&req)
           
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


pub fn get_routers() -> Scope {
    web::scope("/quest")
        .service(read_quest)
        .service(edit_quest)
}