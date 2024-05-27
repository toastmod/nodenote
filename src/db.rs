use std::{collections::{HashMap, HashSet}, fs::File, io::Read, rc::Rc };
use serde_json::{self, Result};
use serde::{self, Serialize};

pub struct DB {
    map: HashMap<Rc<String>, HashSet<Rc<String>>>,
    context: Option<Rc<String>>,
    prev: Vec<Rc<String>>
}
 
impl DB {

    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            context: None,
            prev: vec![]
        }
    }

    pub fn get_context(&self) -> Option<&Rc<String>> {
        self.context.as_ref()
    }

    pub fn add_term(&mut self, term: &str) -> Rc<String> {
        let r = Rc::new(term.to_string());
        if !self.map.contains_key(&r) {
            self.map.insert(Rc::clone(&r), HashSet::new());
            r
        }else{
            Rc::clone(self.map.entry(r).key()) 
        }
    }

    pub fn append(&mut self, term: &str) -> Option<Rc<String>> {

        let ctx_ref = if let Some(ctx_term) = self.context.as_ref() {
            Rc::clone(ctx_term)
        }else{
            return None;
        };

        let appending_existing_key = self.add_term(term);

        self.map.get_mut(&ctx_ref).unwrap().insert(Rc::clone(&appending_existing_key));

        Some(appending_existing_key)

    }

    pub fn subsearch(&mut self, term: &str) {

        if let Some(ctx) = self.context.as_ref() {
            let term = term.to_string();
            let ctx_links = self.map.get_mut(ctx).unwrap();

            if ctx_links.contains(&term) {
                let r = Rc::clone(self.map.entry(Rc::new(term)).key());
                self.set_context(r);
                println!("Found!");
            }

        }
    }

    pub fn print_links(&self) {
        if let Some(ctx) = self.context.as_ref() {
            let ctx_links = self.map.get(ctx).unwrap();
            for link in ctx_links {
                println!("\t{}", link);
            }
        }else{
            for link in self.map.keys() {
                println!("\t{}", link);
            }
        }
    }

    pub fn search(&mut self, term: String) {
        if self.map.contains_key(&term) {
            let r = Rc::clone(self.map.entry(Rc::new(term)).key());
            self.set_context(r);
        }else{
            let mut res: Vec<&Rc<String>> = self.map.iter().filter_map(|x|{

                if !x.0.len() < term.len() {
                    if x.0.contains(&term) {
                        Some(x.0)
                    }else{
                        None
                    }
                }else{
                    None
                }

            }).collect();
            res.sort();

            println!("Select result (or leave blank to cancel):");
            for (i, result) in res.iter().enumerate() {
                println!("\t[{}]\t{}", i, result);
            }

            let mut choice_buf = String::new();
            std::io::stdin().read_line(&mut choice_buf).expect("Error occured at readline.");
            if let Ok(choice) = choice_buf.parse::<usize>() {
                if let Some(term_ref) = res.get(choice) {
                    self.set_context(Rc::clone(term_ref));
                }
            }
        }
    }

    pub fn set_context(&mut self, term_ref: Rc<String>) {
        if let Some(ctx) = self.context.take() {
            self.prev.push(ctx);
        }
        self.context = Some(term_ref);
    }

    pub fn back(&mut self) {
        self.context = self.prev.pop(); 
    }

    pub fn back_to_root(&mut self) {
        if let Some(ctx) = self.context.take() {
            self.prev.push(ctx);
        }
    }


    pub fn save(&self, path: &str) -> std::result::Result<(), std::io::Error> {
        let f = File::create(path)?;

        serde_json::to_writer(f, &self)?;

        Ok(())
    }

    fn load_data(&mut self, json: &str) -> Result<()> {
        *self = DB::new();
        let mut v: serde_json::Value = serde_json::from_str(json)?;
        let root = v.as_object_mut().expect("Could not load root JSON object.");

        for (node, subnodes) in root {
            let mut set = HashSet::new();
            for subnode_v in subnodes.as_array().expect("Invalid database format, expected subnode array.") {
                let subnode = subnode_v.as_str().expect("Invalid database format, expected string in array.");
                set.insert(Rc::new(String::from(subnode)));
            }
            self.map.insert(Rc::new(node.clone()), set);
        }

        Ok(())

    }

    pub fn load(&mut self, path: &str) -> std::result::Result<(), std::io::Error> {

        // TODO: "do you want to save?" prompt 

        let mut f = File::open(path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;

        self.load_data(buf.as_str())?;

        Ok(())
    } 

}

impl Serialize for DB {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let iter: Vec<(&str, Vec<&str>)> = self.map.iter().map(|x|{
            (x.0.as_str(), x.1.iter().map(|y| y.as_str()).collect())
        }).collect();
        serializer.collect_map(iter)
    }
} 