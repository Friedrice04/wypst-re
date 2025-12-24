pub enum OpMap {
    Cmd(&'static str),
    Char(char),
}

pub fn lookup(name: &str) -> Option<OpMap> {
    match name {
        "inter" => Some(OpMap::Char('∩')),
        "union" => Some(OpMap::Cmd("\\cup")),
        "times" => Some(OpMap::Cmd("\\times")),
        "cdot" => Some(OpMap::Cmd("\\cdot")),
        // add more mappings here...
        _ => None,
    }
}