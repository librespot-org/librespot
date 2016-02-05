use std::collections::HashMap;
use std::ffi::{CString, CStr};

pub struct CStringCache {
    cache: HashMap<String, CString>
}

impl CStringCache {
    pub fn new() -> CStringCache {
        CStringCache {
            cache: HashMap::new()
        }
    }

    pub fn intern(&mut self, string: &str) -> &CStr {
        self.cache.entry(string.to_owned()).or_insert_with(|| {
            CString::new(string).unwrap()
        })
    }
}

