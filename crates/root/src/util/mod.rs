use regex::Regex;

pub mod download;
pub mod dir;



pub fn create_node_version_vaildate_reg(version: &str) -> Regex {
    if version.is_empty() {
        return Regex::new(r"^(v)?\d+(\.\d+)*$").unwrap();
    }
    Regex::new(&format!("^(v)?{}", version)).unwrap()
}
