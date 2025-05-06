use std::collections::HashMap;

#[derive(Debug)]
pub struct CodeTable {
    key_to_value: HashMap<Vec<u8>, u16>,
    value_to_key: HashMap<u16, Vec<u8>>,
    next_value: u16,
}

impl CodeTable {
    pub fn create() -> CodeTable {
        let mut key_to_value = HashMap::new();
        let mut value_to_key = HashMap::new();

        CodeTable::gen_base_dic(&mut key_to_value, &mut value_to_key);

        CodeTable {
            key_to_value,
            value_to_key,
            next_value: 256,
        }
    }

    pub fn get_value(&self, key: &Vec<u8>) -> Option<u16> {
        // println!("Lookup: {:?} -> {:?}", key, self.key_to_value.get(key).copied());
        self.key_to_value.get(key).copied()
    }

    pub fn put_value(&mut self, key: &Vec<u8>) -> u16 {
        if self.next_value >= (2_u32.pow(16) - 1) as u16  {
            self.reset();
        }
        // println!("New dictionary element: {:?} -> {:?}", key, self.next_value);
        let value = self.next_value;
        self.key_to_value.insert(key.clone(), value);
        self.value_to_key.insert(value, key.clone());
        self.next_value += 1;
        value
    }

    pub fn get_key(&self, value: u16) -> Option<Vec<u8>> {
        // println!("Lookup: {:?} -> {:?}", value ,self.value_to_key.get(&value).cloned());
        self.value_to_key.get(&value).cloned()
    }

    fn gen_base_dic(key_to_value: &mut HashMap<Vec<u8>, u16>, value_to_key: &mut HashMap<u16, Vec<u8>>) {
        for i in 0..=255 {
            let char_vec = vec![i as u8];
            key_to_value.insert(char_vec.clone(), i as u16);
            value_to_key.insert(i as u16, char_vec);
        }
    }

    fn reset(&mut self) {
        self.key_to_value.clear();
        self.value_to_key.clear();

        CodeTable::gen_base_dic(&mut self.key_to_value, &mut self.value_to_key);

        self.next_value = 256;
    }
}