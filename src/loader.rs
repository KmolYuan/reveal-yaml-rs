use yaml_rust::{Yaml, YamlLoader};

macro_rules! unpack {
    ($v:expr, $method:ident, $msg:literal, $pos:expr) => {
        match $v.$method() {
            Some(v) => v,
            None => return Err(format!("{}: {}", $msg, $pos).into()),
        }
    };
    ($hash:ident, $key:literal, $method:ident, $msg:literal, $pos:expr) => {
        unpack!(
            $hash
                .get(&Yaml::String($key.into()))
                .unwrap_or(&Yaml::String("".into())),
            $method,
            $msg,
            $pos
        )
    };
}

fn inner_loader(yaml_str: &String) -> Result<String, String> {
    let doc = match YamlLoader::load_from_str(yaml_str.as_str()) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    if doc.len() < 2 {
        return Err(format!("Missing metadata").into());
    }
    for (i, s) in unpack!(doc[1], as_vec, "slides must be array", 0)
        .iter()
        .enumerate()
    {
        let slide = unpack!(s, as_hash, "unpack slide failed", i);
        let title = unpack!(slide, "title", as_str, "", i);
        todo!()
    }
    Ok(yaml_str.clone())
}

pub(crate) fn loader(yaml_str: String) -> String {
    match inner_loader(&yaml_str) {
        Ok(v) => v,
        Err(e) => e,
    }
}
