use std::path::PathBuf;

pub struct Config {
    static_mount: Option<PathBuf>,
}

impl Config {
    pub fn new(static_root : &'static str) -> Config {
        return Config {
            static_mount: Some(PathBuf::from(static_root))
        }
    }
    pub fn mount(&mut self, static_root: &'static str) -> (){
        self.static_mount =  Some(PathBuf::from(static_root)) ;
    }
}