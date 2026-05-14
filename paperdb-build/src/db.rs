use paperdb_core::{FieldValue, Model, Record};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::error::{InsertError, ModelError, SearchError};
use crate::model::build_record;

#[derive(Debug, Clone, PartialEq)]
pub enum FilterOperator {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct FieldFilter {
    pub field: String,
    pub value: FieldValue,
}

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub collection: String,
    pub filters: Vec<FieldFilter>,
    pub operator: FilterOperator,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

#[derive(Serialize, Deserialize)]
struct Collection {
    model: Model,
    records: Vec<Record>,
    next_id: u64,
}

#[derive(Serialize, Deserialize)]
struct PaperDBData {
    collections: HashMap<String, Collection>,
}

pub struct PaperDB {
    data: PaperDBData,
    path: PathBuf,
}

impl PaperDB {
    pub fn new(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let data = if path.exists() {
            let bytes = std::fs::read(&path)?;
            bincode::deserialize(&bytes).unwrap_or(PaperDBData { collections: HashMap::new() })
        } else {
            PaperDBData { collections: HashMap::new() }
        };
        Ok(Self { data, path })
    }

    pub fn save(&self) -> std::io::Result<()> {
        let bytes = bincode::serialize(&self.data).expect("serialization failed");
        std::fs::write(&self.path, bytes)
    }

    pub fn create_collection(&mut self, model: Model) -> Result<(), ModelError> {
        if self.data.collections.contains_key(&model.name) {
            return Err(ModelError::CollectionAlreadyExists(model.name.clone()));
        }
        self.data.collections.insert(model.name.clone(), Collection { model, records: Vec::new(), next_id: 1 });
        Ok(())
    }

    pub fn insert(&mut self, collection: &str, fields: HashMap<String, FieldValue>) -> Result<(), InsertError> {
        let col = self.data.collections.get_mut(collection)
            .ok_or_else(|| InsertError::CollectionNotFound(collection.to_string()))?;

        let id = col.next_id;
        let record = build_record(id, fields, &col.model)?;
        col.records.push(record);
        col.next_id += 1;
        Ok(())
    }

    pub fn search(&self, query: SearchQuery) -> Result<Vec<&Record>, SearchError> {
        let col = self.data.collections.get(&query.collection)
            .ok_or_else(|| SearchError::CollectionNotFound(query.collection.clone()))?;

        let model_field_names: std::collections::HashSet<&str> =
            col.model.fields.iter().map(|f| f.name.as_str()).collect();
        for filter in &query.filters {
            if !model_field_names.contains(filter.field.as_str()) {
                return Err(SearchError::UnknownField(filter.field.clone()));
            }
        }

        let mut results: Vec<&Record> = col.records.iter().filter(|record| {
            if query.filters.is_empty() {
                return true;
            }
            let matches: Vec<bool> = query.filters.iter().map(|f| {
                record.fields.get(&f.field).map_or(false, |v| field_matches(v, &f.value))
            }).collect();

            match query.operator {
                FilterOperator::And => matches.iter().all(|&m| m),
                FilterOperator::Or => matches.iter().any(|&m| m),
            }
        }).collect();

        results.sort_by_key(|r| r.id);

        if let (Some(page), Some(page_size)) = (query.page, query.page_size) {
            let start = page.saturating_sub(1) * page_size;
            let end = (start + page_size).min(results.len());
            results = results.get(start..end).unwrap_or(&[]).to_vec();
        }

        Ok(results)
    }
}

fn field_matches(value: &FieldValue, filter: &FieldValue) -> bool {
    match (value, filter) {
        (FieldValue::StringArray(arr), FieldValue::String(s)) => arr.contains(s),
        _ => value == filter,
    }
}
