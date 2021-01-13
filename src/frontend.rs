use actix_web::{Error, HttpRequest, HttpResponse};
use std::io::Cursor;
use std::io::Read;
use zip::ZipArchive;

pub async fn get(req: HttpRequest) -> Result<HttpResponse, Error> {
    let path = req.uri().path();
    let bytes = include_bytes!("../frontend/build/build.zip");
    let cursor = Cursor::new(bytes);
    let mut path = if path.ends_with("/") {
        format!("{}index.html", path)
    } else {
        path.to_string()
    };
    path.remove(0);
    let zip = ZipArchive::new(cursor);
    match zip.unwrap().by_name(path.as_str()) {
        Ok(mut zipfile) => {
            let mut buffer: Vec<u8> = vec![];
            zipfile.read_to_end(&mut buffer).unwrap();
            Ok(HttpResponse::Ok()
                .content_type(
                    mime_guess::from_path(path)
                        .first_or_octet_stream()
                        .to_string(),
                )
                .body(buffer))
        }
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}
