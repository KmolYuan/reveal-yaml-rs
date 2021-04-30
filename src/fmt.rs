use crate::*;
use std::{fs::read_to_string, io::Result, path::Path};
use yaml_rust::{Yaml, YamlLoader};

trait Dumper {
    fn dump(&self, level: usize) -> String;
}

impl Dumper for Yaml {
    fn dump(&self, level: usize) -> String {
        match self {
            Yaml::Real(s) => s.clone(),
            Yaml::Integer(i) => i.to_string(),
            Yaml::String(s) => {
                if s.contains("\n") {
                    let s = s
                        .strip_suffix("\n")
                        .unwrap_or(s)
                        .replace("\n", &(String::from("\n") + &"  ".repeat(level)));
                    String::from("|\n") + &"  ".repeat(level) + &s
                } else {
                    s.clone()
                }
            }
            Yaml::Boolean(b) => b.to_string(),
            Yaml::Array(a) => {
                let mut doc = String::new();
                for (i, y) in a.iter().enumerate() {
                    let mut s = format!("- {}\n", y.dump(level + 1));
                    if i != 0 {
                        s = "  ".repeat(level) + &s;
                    }
                    doc.push_str(&s);
                }
                doc.pop();
                doc
            }
            Yaml::Hash(h) => {
                let mut doc = String::new();
                for (i, (k, v)) in h.iter().enumerate() {
                    let mut s = format!("{}: {}\n", k.dump(level + 1), v.dump(level + 1));
                    if i != 0 {
                        s = "  ".repeat(level) + &s;
                    }
                    doc.push_str(&s);
                }
                doc.pop();
                doc
            }
            Yaml::Null => String::from("null"),
            _ => panic!("Invalid format"),
        }
    }
}

pub fn fmt<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref().join(ROOT);
    if !path.is_file() {
        return err!("can not found project file");
    }
    let yaml = match YamlLoader::load_from_str(&read_to_string(path)?) {
        Ok(v) => v,
        Err(e) => return err!(e.to_string()),
    };
    for node in yaml {
        println!("---");
        println!("{}", node.dump(0));
    }
    Ok(())
}
