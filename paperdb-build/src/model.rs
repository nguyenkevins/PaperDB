use paperdb_core::{FieldType, FieldValue, Model, Record};
use std::collections::HashMap;
use crate::error::InsertError;

pub fn validate(record: &HashMap<String, FieldValue>, model: &Model) -> Result<(), InsertError> {
    for field in &model.fields {
        match record.get(&field.name) {
            None if field.required => return Err(InsertError::MissingRequiredField(field.name.clone())),
            None => continue,
            Some(value) => {
                let matches = matches!(
                    (&field.field_type, value),
                    (FieldType::String, FieldValue::String(_))
                    | (FieldType::Int, FieldValue::Int(_))
                    | (FieldType::Float, FieldValue::Float(_))
                    | (FieldType::Bool, FieldValue::Bool(_))
                    | (FieldType::StringArray, FieldValue::StringArray(_))
                    | (FieldType::FloatArray, FieldValue::FloatArray(_))
                );
                if !matches {
                    return Err(InsertError::TypeMismatch(field.name.clone()));
                }
            }
        }
    }

    let model_fields: std::collections::HashSet<&str> =
        model.fields.iter().map(|f| f.name.as_str()).collect();
    for key in record.keys() {
        if !model_fields.contains(key.as_str()) {
            return Err(InsertError::UnknownField(key.clone()));
        }
    }

    Ok(())
}

pub fn build_record(id: u64, fields: HashMap<String, FieldValue>, model: &Model) -> Result<Record, InsertError> {
    validate(&fields, model)?;
    Ok(Record { id, fields })
}
