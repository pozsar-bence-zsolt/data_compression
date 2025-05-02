use std::collections::HashMap;

#[derive(Debug)]
pub struct CodeTable {
    key_to_value: HashMap<Vec<u8>, u32>,
    value_to_key: HashMap<u32, Vec<u8>>,
    next_value: u32,
}

pub trait LZWBase {
    fn create() -> CodeTable;
    fn get_value(&self, key: &Vec<u8>) -> Option<u32>;
    fn put_value(&mut self, key: &Vec<u8>) -> u32;
    fn get_key(&self, value: u32) -> Option<Vec<u8>>;
}

impl LZWBase for CodeTable {
    fn create() -> CodeTable {
        let mut key_to_value = HashMap::new();
        let mut value_to_key = HashMap::new();
        
        for i in 0..=255 {
            let char_vec = vec![i as u8];
            key_to_value.insert(char_vec.clone(), i as u32);
            value_to_key.insert(i as u32, char_vec);
        }

        CodeTable {
            key_to_value,
            value_to_key,
            next_value: 256,
        }
    }

    fn get_value(&self, key: &Vec<u8>) -> Option<u32> {
        self.key_to_value.get(key).copied()
    }

    fn put_value(&mut self, key: &Vec<u8>) -> u32 {
        let value = self.next_value;
        self.key_to_value.insert(key.clone(), value);
        self.value_to_key.insert(value, key.clone());
        self.next_value += 1;
        value
    }

    fn get_key(&self, value: u32) -> Option<Vec<u8>> {
        self.value_to_key.get(&value).cloned()
    }
}