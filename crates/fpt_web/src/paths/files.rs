pub fn www() -> actix_files::Files {
    const DIR: &str = "./crates/fpt_web/www/";
    const INDEX: &str = "index.html";

    actix_files::Files::new("/", DIR).index_file(INDEX)
}
