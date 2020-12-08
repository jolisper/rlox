pub type Value = f64;

#[derive(Debug)]
pub struct ValueArray {
    values: Option<Vec<Value>>,
}

impl ValueArray {
    pub fn count(&self) -> usize {
        match &self.values {
            Some(c) => c.len(),
            None => 0,
        }
    }

    pub fn get(&self, offset: usize) -> Value {
        match &self.values {
            Some(c) => c[offset],
            None => panic!("No code on value_array!"),
        }
    }
}

pub fn init_value_array() -> ValueArray {
    ValueArray {
        values: Some(vec![]),
    }
}

pub fn write_value_array(value_array: &mut ValueArray, value: Value) {
    let values = value_array.values.as_mut();
    values.expect("Uninitialized value array!").push(value);
}
