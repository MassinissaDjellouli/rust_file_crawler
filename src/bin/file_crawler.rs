use std::collections::HashMap;
use std::sync::{Arc, Mutex};
const READABLE_FILE_FORMAT: [&'static str; 18] = [
    "txt", "css", "js", "html", "c", "cpp", "h", "hpp", "rs", "java", "toml", "json", "csv", "sh",
    "yaml", "md", "xml", "xslt",
];
pub struct FileCrawler {
    file_cache: HashMap<char, HashMap<&'static str, Arc<Mutex<Vec<&'static str>>>>>,
    folders_cache: HashMap<char, HashMap<&'static str, Arc<Mutex<Vec<&'static str>>>>>,
    searchable_files_cache: HashMap<char, HashMap<&'static str, Arc<Mutex<Vec<&'static str>>>>>,
    sep: &'static str,
    roots: Vec<&'static str>
}

impl FileCrawler {
    pub fn new() -> FileCrawler {
        FileCrawler {
            file_cache: init_cache(),
            folders_cache: init_cache(),
            searchable_files_cache: init_cache(),
            sep: get_sep(),
            roots: Self::get_roots()
        }
    }
    pub fn start_indexing(&mut self){

    }

    fn get_roots() -> Vec<&'static str> {
        vec!["/"]
    }
    fn edit_cache(
        cache: &mut HashMap<char, HashMap<&'static str, Arc<Mutex<Vec<&'static str>>>>>,
        key: char,
        file_name: &'static str,
        path: &'static str,
    ) {
        cache.entry(key).and_modify(|map| {
            let paths: &mut Arc<Mutex<Vec<&'static str>>> =
                map.entry(file_name).or_insert(Arc::new(Mutex::new(vec![])));
            paths.lock().unwrap().push(path);
        });
    }
    pub fn find<'a>(&self, file_name:&str,root_folders:Option<Vec<&'a str>>,full_search:bool){

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
    fn add_to_cache(cache: &mut HashMap<char, HashMap<&'static str, Arc<Mutex<Vec<&'static str>>>>>,
        path: &'static str,
        sep:&'static str
    ) {
        match path.split(sep).last() {
            Some(last) => {
                match last.chars().next() {
                    Some(first) => {
                        Self::edit_cache(cache, first, last, path);
                    }
                    None => {
                        println!("Exit");
                        return;
                    }
                }
            }
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

//Vomit inducing shit
fn init_cache() -> HashMap<char, HashMap<&'static str, Arc<Mutex<Vec<&'static str>>>>> {
    let mut map: HashMap<char, HashMap<&'static str, Arc<Mutex<Vec<&'static str>>>>> =
        HashMap::new();
    map.insert('A', HashMap::new());
    map.insert('B', HashMap::new());
    map.insert('C', HashMap::new());
    map.insert('D', HashMap::new());
    map.insert('E', HashMap::new());
    map.insert('F', HashMap::new());
    map.insert('G', HashMap::new());
    map.insert('H', HashMap::new());
    map.insert('I', HashMap::new());
    map.insert('J', HashMap::new());
    map.insert('K', HashMap::new());
    map.insert('L', HashMap::new());
    map.insert('M', HashMap::new());
    map.insert('N', HashMap::new());
    map.insert('O', HashMap::new());
    map.insert('P', HashMap::new());
    map.insert('Q', HashMap::new());
    map.insert('R', HashMap::new());
    map.insert('S', HashMap::new());
    map.insert('T', HashMap::new());
    map.insert('U', HashMap::new());
    map.insert('V', HashMap::new());
    map.insert('W', HashMap::new());
    map.insert('X', HashMap::new());
    map.insert('Y', HashMap::new());
    map.insert('Z', HashMap::new());
    map.insert('0', HashMap::new());
    map.insert('1', HashMap::new());
    map.insert('2', HashMap::new());
    map.insert('3', HashMap::new());
    map.insert('4', HashMap::new());
    map.insert('5', HashMap::new());
    map.insert('6', HashMap::new());
    map.insert('7', HashMap::new());
    map.insert('8', HashMap::new());
    map.insert('9', HashMap::new());
    map.insert('.', HashMap::new());
    map.insert('_', HashMap::new());
    map.insert('-', HashMap::new());
    map
}
