use actix_files as fs;
use actix_web::{get, Error, HttpRequest};
use std::path::PathBuf;

pub(super) fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(view_public_file);
}

#[get("/assets/{filename:.*}")]
async fn view_public_file(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let mut path: PathBuf = PathBuf::from("public/");
    let req_path: PathBuf = req.match_info().query("filename").parse().unwrap();
    path.push(req_path.file_name().unwrap());

    let file = fs::NamedFile::open(path)?;

    // cors
    Ok(file.use_last_modified(true))
}