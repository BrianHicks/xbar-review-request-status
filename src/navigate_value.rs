use anyhow::anyhow;
use anyhow::Result;
use serde_json::Value;

pub trait NavigateValue {
    fn get_str(&self, pointer: &str) -> Result<&str>;

    fn get_array(&self, pointer: &str) -> Result<&Vec<Self>>
    where
        Self: Sized;
}

impl NavigateValue for Value {
    fn get_str(&self, pointer: &str) -> Result<&str> {
        self.pointer(pointer)
            .ok_or_else(|| anyhow!("could not get {}", pointer))?
            .as_str()
            .ok_or_else(|| anyhow!("{} was not a string", pointer))
    }

    fn get_array(&self, pointer: &str) -> Result<&Vec<Self>> {
        self.pointer(pointer)
            .ok_or_else(|| anyhow!("could not get {}", pointer))?
            .as_array()
            .ok_or_else(|| anyhow!("{} was not an array", pointer))
    }
}
