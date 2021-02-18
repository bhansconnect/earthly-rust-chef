use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub phones: Vec<String>,
}

pub fn serialize_person(p: &Person) -> Result<String> {
    serde_json::to_string(p)
}

pub fn deserialize_person(s: &str) -> Result<Person> {
    serde_json::from_str(s)
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_person_serde() {
        let p = Person {
            name: "John Smith".to_string(),
            age: 42,
            phones: vec!["123".to_string(), "456".to_string(), "7890".to_string()],
        };
        let s = serialize_person(&p).unwrap();
        let d = deserialize_person(&s).unwrap();
        assert_eq!(d.name, p.name);
        assert_eq!(d.age, p.age);
        assert_eq!(d.phones, p.phones);
    }
}
