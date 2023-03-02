pub enum FieldType {
    Int64,
    Float64,
    String,
}

pub struct Field {
    pub length: u8,
    pub field_type: FieldType,
    pub original_value: f64,
}

impl Field {
    pub fn new(length: u8, field_type: FieldType, original_value: f64) -> Self {
        Self {
            length,
            field_type,
            original_value,
        }
    }
}