use actix_files::NamedFile;
use actix_web::Result;

pub async fn webapp() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/tarun.html")?)
}

pub fn configure_static_files(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.route("/", actix_web::web::get().to(webapp));
}
