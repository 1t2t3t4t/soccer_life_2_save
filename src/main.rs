use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct NameChange(String, String);

impl From<[&str; 2]> for NameChange {
    fn from(value: [&str; 2]) -> Self {
        Self(value[0].to_string(), value[1].to_string())
    }
}

fn find_idx_str(bytes: &[u8], target: &str) -> Option<usize> {
    let target_bytes = target.as_bytes();
    let mut idx = 0;

    for i in 0..bytes.len() {
        if bytes[i] == target_bytes[idx] {
            idx += 1;
        } else {
            idx = 0;
        }

        if idx == target_bytes.len() {
            return Some(i - target_bytes.len() + 1);
        }
    }
    None
}

fn replace_inplace(bytes: &mut [u8], at: usize, size: usize, target: &str) {
    let t_bytes = target.as_bytes();
    assert!(
        t_bytes.len() <= 11,
        "{target} ({}) has more than 11 chars",
        t_bytes.len()
    );
    for i in at..(at + size) {
        bytes[i] = 0;
    }
    for i in 0..t_bytes.len() {
        let idx = at + i;
        bytes[idx] = t_bytes[i];
    }
}

fn main() {
    let bytes = include_bytes!("savefile");
    let mut bytes = bytes.clone();

    let name_map = include_bytes!("name_map.json");
    let names: Vec<NameChange> = serde_json::from_slice(name_map).unwrap();

    for name in names {
        if let Some(idx) = find_idx_str(&bytes, &name.0) {
            replace_inplace(&mut bytes, idx, name.0.len(), &name.1);
        } else {
            println!("Cannot find {}", name.0);
        }
    }

    std::fs::write("BESLES-53846N0", &bytes).unwrap();
    println!("Done");
}
