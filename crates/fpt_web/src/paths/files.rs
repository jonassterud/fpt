use actix_files::Files;

pub fn www() -> Files {
    const DIR: &str = "./crates/fpt_web/www/";
    const INDEX: &str = "index.html";

    Files::new("/", DIR).index_file(INDEX)
}
