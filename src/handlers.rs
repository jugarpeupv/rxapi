use crate::model::CreateDirQuery;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web, Error, HttpResponse, Responder};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}


#[get("/upload")]
async fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[post("/upload")]
async fn save_files(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    println!("we are here");
    //std::fs::create_dir_all("./hi")?;
    //std::fs::create_dir_all("./tmp")?;
    for f in form.files {
        println!("pasando por aquí");
        let path = format!("./tmp/{}", f.file_name.unwrap());
        log::info!("saving to {path}");
        f.file.persist(path).unwrap();
    }
    Ok(HttpResponse::Ok())
}


#[get("/create_dir")]
async fn create_dir(create_dir_query: web::Query<CreateDirQuery>) -> Result<impl Responder, Error> {
    let parsed_path = format!("./tmp/{}", create_dir_query.path);
    println!("parsed_path: {}", parsed_path);
    let path_exists = std::path::Path::new(&parsed_path).try_exists()?;
    println!("path_exists: {}", path_exists);

    match path_exists {
        true => {
            println!("path exists");
        }
        false => {
            println!("path does not exist");
            std::fs::create_dir_all(parsed_path)?;
        }
    }
    Ok(HttpResponse::Ok())
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(index)
        .service(save_files)
        .service(create_dir);

    conf.service(scope);
}
