use actix_files as fs;

pub fn files() -> actix_files::Files {
    fs::Files::new(".", "../www/").show_files_listing()
}
