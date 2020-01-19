use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ConfigFile {
    cache: HashMap<String, String>
}

impl ConfigFile {
    pub fn load<P: AsRef<Path> + Debug>(path: P) -> ConfigFile {
        let mut cache = HashMap::new();
        let file = File::open(&path).expect(&format!("Arquivo {:?} não pode ser aberto", path));
        let buf = BufReader::new(file);

        for line in buf.lines() {
            let line = line.unwrap();
            
            let truncate_idx = 
                match (line.find(";"), line.find("#")) {
                    (Some(idxsemi), Some(idxhash)) => std::cmp::min(idxsemi, idxhash),
                    (Some(idxsemi), None) => idxsemi,
                    (None, Some(idxhash)) => idxhash,
                    (None, None) => line.len()
                };

            let (line, _) = line.split_at(truncate_idx);

            if line.trim_end() == "" {
                continue;
            }

            let splitted: Vec<_> = line.split("=").collect();
            if splitted.len() < 2 {
                panic!("Line unexpected: '{}'", line);
            }

            if splitted[1] != "NULL" {
                cache.insert(
                    splitted[0].to_string(),
                    splitted[1].to_string());
            }
        }

        ConfigFile { cache: cache }
    }

    pub fn get_string(&self, key: &str) -> Option<&String> {
        self.cache.get(key)
    }

    pub fn get<V: FromStr>(&self, key: &str) -> Option<V> 
        where V::Err: std::fmt::Debug {
        self.cache
            .get(key)
            .map(|v| V::from_str(v).expect("Não é um inteiro válido."))
    }

    /*
    pub fn get_int(&self, key: &str) -> Option<i32> {
        self.cache
            .get(key)
            .map(|v| i32::from_str(v).expect("Não é um inteiro válido."))
    }

    pub fn get_float(&self, key: &str) -> Option<f32> {
        self.cache
            .get(key)
            .map(|v| f32::from_str(v).expect("Não é um float válido."))
    }
    */
}
