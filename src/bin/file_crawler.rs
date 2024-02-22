use std::collections::{HashMap, VecDeque};
use std::fmt::format;
use std::path::Path;
use std::process::exit;
use std::slice::Windows;
use std::sync::{Arc, mpsc, Mutex, MutexGuard};
use std::sync::mpsc::{Receiver, Sender};

use threadpool::ThreadPool;
use crate::bin::objects::{ExploredObject, ObjectType};

const READABLE_FILE_FORMAT: [&'static str; 18] = [
    "txt", "css", "js", "html", "c", "cpp", "h", "hpp", "rs", "java", "toml", "json", "csv", "sh",
    "yaml", "md", "xml", "xslt",
];
pub struct FileCrawler {
    file_cache:  HashMap<String, Arc<Mutex<Vec<String>>>>,
    folders_cache: HashMap<String, Arc<Mutex<Vec<String>>>>,
    searchable_files_cache: HashMap<String, Arc<Mutex<Vec<String>>>>,
    sep: &'static str,
    roots: Vec<String>
}

impl FileCrawler {
    pub fn new() -> FileCrawler {
        FileCrawler {
            file_cache: HashMap::new(),
            folders_cache: HashMap::new(),
            searchable_files_cache: HashMap::new(),
            sep: get_sep(),
            roots: Self::get_roots()
        }
    }
    pub fn start_indexing(&mut self){
        println!("Starting indexing files");
        let mut layers_explored = 0;
        let mut to_explore_queue:VecDeque<String> = self.roots.clone().iter().map(|s|s.clone()).collect::<VecDeque<String>>();
        let threadpool:ThreadPool = ThreadPool::new(10);
        let (tx, rx): (Sender<Vec<ExploredObject>>, Receiver<Vec<ExploredObject>>) = mpsc::channel();
        loop {
            println!("Starting indexing layer {}",layers_explored + 1);
            let expected_msg_length = to_explore_queue.len();
            for to_explore in to_explore_queue.clone(){
                let thr_sender = tx.clone();
                threadpool.execute(move || {
                    let res = Self::explore_dir(to_explore);
                    thr_sender.send(res).expect("Should work");
                })
            }
            to_explore_queue.clear();
            let mut msg_rcvd = 0;
            loop{
                let rcv = match rx.recv() {
                    Ok(rcv) => {
                        msg_rcvd += 1;
                        rcv
                    },
                    Err(_) => {
                        println!("ERRRRRRR");
                        break
                    }
                };
                for explored_obj in rcv{
                    match explored_obj.get_type() {
                        ObjectType::Directory => {
                            to_explore_queue.push_back(explored_obj.get_path());
                            self.add_to_folders_cache(explored_obj.get_path());
                        },
                        ObjectType::File => {
                            self.add_to_file_cache(explored_obj.get_path())
                        },
                        _ => {}
                    }
                }
                if msg_rcvd == expected_msg_length {
                    break
                }
            }
            if to_explore_queue.len() == 0{
                break
            }
            layers_explored += 1;
            println!("Finished indexing layer {}",layers_explored);
        }
        println!("Finished indexing.");
    }
    fn explore_dir(dir_to_explore:String) -> Vec<ExploredObject>{
        let path = Path::new(&dir_to_explore);
        if !path.exists() {
            println!("path {} does not exist",dir_to_explore);
            return vec![];
        };
        if !path.is_dir(){
            println!("path {} is not a directory",dir_to_explore);
            return vec![];
        }
        let mut explored:Vec<ExploredObject> = vec![];
        let entries = match path.read_dir() {
            Ok(entries) => entries,
            Err(_) => {
                println!("Could not access dir {}",dir_to_explore);
                return vec![];
            }
        };
        for entry in entries{
            if let Ok(entry) = entry{
                let path = entry.path();
                let explored_obj = ExploredObject::new(String::from(entry.file_name().to_str().unwrap()),
                                                       String::from(path.to_str().unwrap()),
                                                       ObjectType::from(&path));
                explored.push(explored_obj)
            }
        }
        explored
    }

    #[cfg(target_os= "windows")]
    fn get_windows_roots() -> Vec<String>{
        const WINDOWS_DRIVE_LETTERS:[&'static str;26] = ["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];
        let mut roots:Vec<String> = vec![];
        unsafe {
            let drives = windows::Win32::Storage::FileSystem::GetLogicalDrives();
            let bits = unicode_segmentation::UnicodeSegmentation::graphemes(format!("{drives:b}").as_str(),true).rev().collect::<String>();
            let mut idx = 0;
            let mut chars = bits.chars();
            while idx != bits.len() {
                let bit = chars.next().unwrap();
                if bit == '1' {
                    roots.push(format!("{}:\\",WINDOWS_DRIVE_LETTERS[idx]).clone())
                }
                idx += 1;
            }
        }
        roots
    }
    fn get_roots() -> Vec<String> {
        #[cfg(target_os= "windows")]{
               return Self::get_windows_roots()
        }
        return vec![String::from("/")]
    }
    fn edit_cache(
        cache: &mut HashMap<String, Arc<Mutex<Vec<String>>>>,
        file_name: String,
        path: String,
    ) {
        let paths: &mut Arc<Mutex<Vec<String>>> =
            cache.entry(file_name).or_insert(Arc::new(Mutex::new(vec![])));
        paths.lock().unwrap().push(path);
    }
    pub fn find(&self, file_name:String,root_folders:Option<Vec<String>>) -> Vec<String>{
        Self::find_in_cache(&self.file_cache,self.get_real_roots(root_folders),file_name)
    }

    pub fn find_content(&self, content:String, root_folders:Option<Vec<String>>) -> Vec<String>{
        vec![]
    }
    pub fn find_dir(&self, folder_name:String, root_folders:Option<Vec<String>>) -> Vec<String>{
        Self::find_in_cache(&self.folders_cache, self.get_real_roots(root_folders), folder_name)
    }

    fn get_real_roots(&self,opt:Option<Vec<String>>) -> Vec<String>{
        match opt {
            None => self.roots.clone().iter().map(|s| s.clone()).collect::<Vec<String>>(),
            Some(r) => r
        }
    }
    fn find_in_cache(cache:&HashMap<String, Arc<Mutex<Vec<String>>>>,roots:Vec<String>,to_find:String) -> Vec<String>{
        let filter = cache.iter().filter(|entry| entry.0.contains(to_find.as_str()));
        let vecs = filter.map(|entry| entry.1.lock().unwrap()).collect::<Vec<MutexGuard<Vec<String>>>>();
        let mut elements:Vec<String> = vec![];
        vecs.iter().for_each(|vec|{
            vec.iter().for_each(|el|{
                for root in roots.clone(){
                    if !el.starts_with(&root){
                        continue;
                    }
                    elements.push(el.clone());
                    break;
                }
            })
        });
        elements
    }
    pub fn add_to_file_cache(&mut self, path: String) {
        Self::add_to_cache(&mut self.file_cache, path.clone(),self.sep);
        match path.split('.').last(){
            Some(ext) => {
                if READABLE_FILE_FORMAT.contains(&ext){
                    Self::add_to_cache(&mut self.searchable_files_cache, path.clone(),self.sep);
                }
            }
            None => return
        }
    }

    pub fn add_to_folders_cache(&mut self, path:String) {
        Self::add_to_cache(&mut self.folders_cache, path,self.sep);
    }
    fn add_to_cache(cache: &mut HashMap<String, Arc<Mutex<Vec<String>>>>,
        path: String,
        sep:&str
    ) {
        match path.split(sep).last() {
            Some(last) => Self::edit_cache(cache, last.to_string().clone(), path),
            None => {
                println!("Exit");
                return;
            }
        }
    }
}
fn get_sep() -> &'static str {
    return if cfg!(windows) { "\\" } else { "/" };
}
