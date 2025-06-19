use serde_json::Value;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub struct Frontmatter {
    pub fields: HashMap<String, Value>,
}

#[allow(dead_code)]
impl Frontmatter {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn get<T>(&self, key: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.fields
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.fields
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.fields.get(key).and_then(|v| v.as_bool())
    }

    pub fn get_number(&self, key: &str) -> Option<f64> {
        self.fields.get(key).and_then(|v| v.as_f64())
    }

    pub fn get_array(&self, key: &str) -> Option<Vec<Value>> {
        self.fields.get(key).and_then(|v| v.as_array()).cloned()
    }

    pub fn has_field(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    pub fn insert(&mut self, key: String, value: Value) {
        self.fields.insert(key, value);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatterParser {
    frontmatter: Frontmatter,
    content: String,
}

#[allow(dead_code)]
impl MatterParser {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(file_path)?;
        Self::from_str(&file_content)
    }

    pub fn from_str(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut lines = input.lines().peekable();

        if let Some(first_line) = lines.peek() {
            if first_line.trim() == "---" {
                lines.next();

                let mut frontmatter_lines = Vec::new();
                let mut content_lines = Vec::new();
                let mut in_frontmatter = true;

                while let Some(line) = lines.next() {
                    if line.trim() == "---" && in_frontmatter {
                        in_frontmatter = false;
                    } else if in_frontmatter {
                        frontmatter_lines.push(line);
                    } else {
                        content_lines.push(line);
                    }
                }

                while let Some(line) = lines.next() {
                    content_lines.push(line);
                }

                let frontmatter_yaml = frontmatter_lines.join("\n");
                let frontmatter = Self::parse_yaml_frontmatter(&frontmatter_yaml)?;
                let content = content_lines.join("\n");

                Ok(Self {
                    frontmatter,
                    content,
                })
            } else {
                Ok(Self {
                    frontmatter: Frontmatter::new(),
                    content: input.to_string(),
                })
            }
        } else {
            Ok(Self {
                frontmatter: Frontmatter::new(),
                content: String::new(),
            })
        }
    }

    fn parse_yaml_frontmatter(yaml_str: &str) -> Result<Frontmatter, Box<dyn std::error::Error>> {
        if yaml_str.trim().is_empty() {
            return Ok(Frontmatter::new());
        }

        let value: Value = serde_yaml::from_str(yaml_str)?;

        match value {
            Value::Object(map) => {
                let mut fields = HashMap::new();
                for (key, value) in map {
                    fields.insert(key, value);
                }
                Ok(Frontmatter { fields })
            }
            _ => Err("Frontmatter must be a YAML object".into()),
        }
    }

    pub fn frontmatter(&self) -> &Frontmatter {
        &self.frontmatter
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn into_parts(self) -> (Frontmatter, String) {
        (self.frontmatter, self.content)
    }
}
