use crate::*;
use std::{
    fs::read_to_string,
    fs::File,
    io::{Result, Write},
    path::Path,
};
use yaml_rust::{Yaml, YamlLoader};

trait Dumper {
    fn dump(&self, level: usize, is_map: bool) -> String;
}

impl Dumper for Yaml {
    fn dump(&self, level: usize, is_map: bool) -> String {
        match self {
            Yaml::Real(s) => s.clone(),
            Yaml::Integer(i) => i.to_string(),
            Yaml::String(s) => {
                if s.contains("\n") {
                    let s = s
                        .trim()
                        .replace("\n", &(String::from("\n") + &"  ".repeat(level)));
                    String::from("|\n") + &"  ".repeat(level) + &s
                } else {
                    s.clone()
                }
            }
            Yaml::Boolean(b) => b.to_string(),
            Yaml::Array(a) => {
                let mut doc = String::from(if level == 0 { "" } else { "\n" });
                for (i, y) in a.iter().enumerate() {
                    let mut s = format!("- {}\n", y.dump(level + 1, false));
                    if i != 0 || level != 0 {
                        s = "  ".repeat(level) + &s;
                    }
                    doc.push_str(&s);
                }
                doc.pop();
                doc
            }
            Yaml::Hash(h) => {
                let mut doc = String::from(if is_map { "\n" } else { "" });
                for (i, (k, v)) in h.iter().enumerate() {
                    let mut s = format!(
                        "{}: {}\n",
                        k.dump(level + 1, false),
                        v.dump(level + 1, true)
                    );
                    if i != 0 || is_map {
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

pub fn fmt<P: AsRef<Path>>(path: P, dry: bool) -> Result<()> {
    let path = path.as_ref().join(ROOT);
    if !path.is_file() {
        return err!("can not found project file");
    }
    let yaml = match YamlLoader::load_from_str(&read_to_string(&path)?) {
        Ok(v) => v,
        Err(e) => return err!(e.to_string()),
    };
    let yaml = yaml.iter().enumerate().map(|(i, node)| {
        String::from(if i == 0 { "" } else { "---\n" })
            + &node
                .dump(0, false)
                .split('\n')
                .map(|s| s.trim_end())
                .collect::<Vec<_>>()
                .join("\n")
    });
    if dry {
        for s in yaml {
            println!("{}", s);
        }
    } else {
        let mut f = File::create(&path)?;
        for s in yaml {
            f.write((s + "\n").as_bytes())?;
        }
    }
    Ok(())
}
