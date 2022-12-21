use actix_web_static_files::ResourceFiles;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub fn www() -> ResourceFiles {
    ResourceFiles::new("/", generate())
}
