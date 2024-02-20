use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use crate::bin::objects::ObjectType::{Directory, File, Link};


pub struct ExploredObject{
    name:String,
    path:String,
    obj_type:ObjectType
}

impl Debug for ExploredObject{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExploredObject")
                    .field("name", &self.name)
                    .field("path", &self.path)
                    .field("obj_type", &ObjectType::as_str(&self.obj_type))
                    .finish()
    }
}
impl ExploredObject{
    pub fn new(name:String,path:String, obj_type:ObjectType) -> ExploredObject{
        ExploredObject {
            name,
            path,
            obj_type
        }

    }
    pub fn get_name(&self) -> String{
        (&self).name.clone()
    }

    pub fn get_path(&self) -> String{
        (&self).path.clone()
    }

    pub fn get_type(&self) -> &ObjectType{
        &self.obj_type
    }
}
pub enum ObjectType{
    Directory,
    File,
    Link
}

impl ObjectType {
    pub fn from(path:&PathBuf) -> ObjectType{
        if path.is_dir() {
            return Directory
        }
        if path.is_symlink() {
            return Link
        }
        File
    }

    pub fn as_str(origin:&ObjectType) -> &'static str{
        match origin {
            Directory => "Directory",
            File => "File",
            Link => "Link"
        }
    }
}