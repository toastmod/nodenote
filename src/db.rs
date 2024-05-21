use std::{collections::{HashMap, HashSet}, fs::File, rc::Rc };
use serde_json;
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


    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let f = File::create(path)?;

        serde_json::to_writer(f, &self)?;

        Ok(())
    }

    // pub fn load(mut self, path: &str) -> Result<(), std::io::Error> {
    //     self = Self::new();
    //     let mut f = File::create(path)?;

    //     let mut buf = [0u8];
    //     let mut read_buffer = String::new();

    //     loop {

    //         while buf[0] as char != '\n' {
    //             f.read(&mut buf);
    //             read_buffer.push(buf[0] as char);
    //         }

    //         // TODO!

    //         read_buffer.clear();
    //     }


    //     Ok(())
    // } 

}

impl Serialize for DB {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let iter: Vec<(&str, Vec<&str>)> = self.map.iter().map(|x|{
            (x.0.as_str(), x.1.iter().map(|y| y.as_str()).collect())
        }).collect();
        serializer.collect_map(iter)
    }
} 