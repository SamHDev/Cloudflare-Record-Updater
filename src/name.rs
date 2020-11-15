use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Name {
    pub record: String,
    pub hostname: String
}

impl Name {
    pub fn from(s: &str) -> Result<Self, String> {
        let gex = match Regex::new(r"(?:(\w+)\.)?(\w+\.\w+?)$") {
            Ok(g) => g,
            Err(_) => return Err("Failed to create regex pattern".to_string())
        };
        let data = match gex.captures(&s) {
            Some(p) => p,
            None => return Err("Failed to parse name: Invalid format: <record>.<domain>.<tld>".to_string())
        };
        Ok(Self {
            record: data.get(1).unwrap_or(data.get(2).unwrap()).as_str().to_string(),
            hostname: data.get(2).unwrap().as_str().to_string()
        })
    }
    pub fn to(&self) -> String {
        if &self.record == &self.hostname {
            self.hostname.to_string()
        } else {
            format!("{}.{}", self.record, self.hostname)
        }
    }
    pub fn from_vec(ls: &Vec<String>) -> Result<Vec<Name>, String> {
        let mut build = Vec::new();
        for l in ls {
            build.push(Name::from(&l)?)
        }
        Ok(build)
    }
    pub fn separate(ls: &Vec<Name>) -> Vec<(String, Vec<String>)> {
        let mut build = HashMap::new();
        for l in ls {
            if !build.contains_key(&l.hostname) {
                build.insert(l.hostname.clone(), vec![l.record.clone()]);
            } else {
                build.get_mut(&l.hostname).unwrap().push(l.record.clone());
            }
        }
        build.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}

