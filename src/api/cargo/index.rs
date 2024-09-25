use actix_files::Files;

pub fn index() -> Files {
    Files::new("/index", "assets")
}
