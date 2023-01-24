use std::path::Path;

use url::Url;

#[derive(Debug)]
pub struct Node {
    pub base: String,
    pub content: String,
}

pub fn convert_name(tree: &str, base: &str) -> String {
    if url::Url::parse(&tree).is_ok() && !url::Url::parse(&base).is_ok() {
        return Url::parse(&tree).unwrap().join(base).unwrap().to_string();
    } else if url::Url::parse(&base).is_ok() {
        return base.to_string();
    } else  {
        Path::new(&tree).parent().unwrap().join(base).to_str().unwrap().to_owned()
    }
}

pub fn read(base: &str)  -> String {
    if url::Url::parse(&base).is_ok() {
        println!("is ok");
        match reqwest::blocking::get(base){
          Ok(v) => match v.text() {
            Ok(v) => v,
            Err(_) => {
              unimplemented!()
            }
          },
          Err(_) => {
            unimplemented!()
          }
        }
      } else {
        match std::fs::read_to_string(base) {
          Ok(v) => v,
          Err(_) => {
            println!("{}", base);
            unimplemented!()
          }
        }
      }
}
impl Node {
    pub fn new(tree: &str, base: &str) -> Self {
        let name = convert_name(tree, base);
        Node {
            content: read(&name),
            base: name,
        }
    }

    pub fn add_node(&mut self, url: &str) -> Node {
        Node::new(&self.base, url)
    }

    pub fn get(&mut self) -> &str {
        &self.content.as_str()
    }
}

pub fn base(url: &str) -> Node {
    Node {
        content: read(url),
        base: url.to_string(),
    }
}