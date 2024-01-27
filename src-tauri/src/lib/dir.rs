use std::path;

use directories::*;

pub struct Dir {
    project_dirs: ProjectDirs,
    userdirs: UserDirs,
}

impl Dir {
    pub fn new() -> Dir {
        Dir {
            project_dirs: ProjectDirs::from("com", "tentaShiratori", "mindmap").unwrap(),
            userdirs: UserDirs::new().unwrap(),
        }
    }
    pub fn home(&self) -> &path::Path {
        self.userdirs.home_dir()
    }
    pub fn data(&self) -> &path::Path {
        self.project_dirs.data_local_dir()
    }
}
