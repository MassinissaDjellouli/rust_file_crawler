use std::collections::{HashMap, VecDeque};
use std::path::Path;
use std::sync::{Arc, mpsc, Mutex, MutexGuard};
use std::sync::mpsc::{Receiver, Sender};

use threadpool::ThreadPool;

const READABLE_FILE_FORMAT: [&'static str; 18] = [
    "txt", "css", "js", "html", "c", "cpp", "h", "hpp", "rs", "java", "toml", "json", "csv", "sh",
    "yaml", "md", "xml", "xslt",
];
pub struct FileCrawler {
    file_cache:  HashMap<&'static str, Arc<Mutex<Vec<&'static str>>>>,
    folders_cache: HashMap<&'static str, Arc<Mutex<Vec<&'static str>>>>,
    searchable_files_cache: HashMap<&'static str, Arc<Mutex<Vec<&'static str>>>>,
    sep: &'static str,
    roots: Vec<&'static str>
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
        let mut to_explore_queue:VecDeque<&str> = self.roots.clone().iter().map(|s|*s).collect::<VecDeque<&str>>();
        println!("{:?}",self.roots);
        println!("{:?}",to_explore_queue);
        let threadpool:ThreadPool = ThreadPool::new(10);
        let (tx, rx): (Sender<Vec<&str>>, Receiver<Vec<&str>>) = mpsc::channel();
        loop {
            for to_explore in to_explore_queue.clone(){
                let thr_sender = tx.clone();
                threadpool.execute(move || {
                    let res = Self::explore_dir(to_explore);
                    thr_sender.send(res).expect("Should work");
                })
            }
            threadpool.join();
            for paths in rx.recv(){
                to_explore_queue.clear();
                for path in paths{
                    to_explore_queue.push_back(path);
                }
            }
            if to_explore_queue.len() == 0{
                break
            }
        }

    }
    fn explore_dir(dir_to_explore:&str) -> Vec<&str>{
        let path = Path::new(dir_to_explore);
        if !path.exists() {
            println!("path {} does not exist",dir_to_explore);
            return vec![];
        };
        if !path.is_dir(){
            println!("path {} is not a directory",dir_to_explore);
            return vec![];
        }

        vec![]
    }
    fn get_roots() -> Vec<&'static str> {
        vec![r#"C:\"#]
    }
    fn edit_cache(
        cache: &mut HashMap<&'static str, Arc<Mutex<Vec<&'static str>>>>,
        file_name: &'static str,
        path: &'static str,
    ) {
        let paths: &mut Arc<Mutex<Vec<&'static str>>> =
            cache.entry(file_name).or_insert(Arc::new(Mutex::new(vec![])));
        paths.lock().unwrap().push(path);
    }
    pub fn find<'a>(&self, file_name:&str,root_folders:Option<Vec<&'a str>>,full_search:bool) -> Vec<&str>{
        let root_folders = match root_folders {
            None => self.roots.clone(),
            Some(r) => r
        };
        let filter = self.file_cache.iter().filter(|entry| entry.0.contains(file_name));
        let vecs = filter.map(|entry| entry.1.lock().unwrap()).collect::<Vec<MutexGuard<Vec<&str>>>>();
        let mut elements:Vec<&str> = vec![];
        vecs.iter().for_each(|vec|{
            vec.iter().for_each(|el|{
                for root in root_folders.clone(){
                    if el.starts_with(root){
                        continue;
                    }
                    elements.push(el);
                    break;
                }
            })
        });
        elements
    }
    pub fn add_to_file_cache(&mut self, path: &'static str) {
        Self::add_to_cache(&mut self.file_cache, path,self.sep);
        match path.split('.').last(){
            Some(ext) => {
                if READABLE_FILE_FORMAT.contains(&ext){
                    Self::add_to_cache(&mut self.searchable_files_cache, path,self.sep);
                }
            }
            None => return
        }
    }

    pub fn add_to_folders_cache(&mut self, path: &'static str) {
        Self::add_to_cache(&mut self.folders_cache, path,self.sep);
    }
    fn add_to_cache(cache: &mut HashMap<&'static str, Arc<Mutex<Vec<&'static str>>>>,
        path: &'static str,
        sep:&'static str
    ) {
        match path.split(sep).last() {
            Some(last) => Self::edit_cache(cache, last, path),
            None => {
                println!("Exit");
                return;
            }
        }
    }
}
fn get_sep() -> &'static str {
    "\\"
//    return if cfg!(windows) { "\\" } else { "/" };
}
