use paperdb_core::{FieldValue, Model, Record};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[pyclass]
struct PaperDB {
    data: PaperDBData,
}

#[pymethods]
impl PaperDB {
    #[staticmethod]
    fn open(path: &str) -> PyResult<Self> {
        let bytes = std::fs::read(path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        let data: PaperDBData = bincode::deserialize(&bytes)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(Self { data })
    }

    fn search(&self, py: Python, collection: &str, page: usize, page_size: usize) -> PyResult<PyObject> {
        let col = self.data.collections.get(collection)
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err(format!("collection '{collection}' not found")))?;

        let start = (page.saturating_sub(1)) * page_size;
        let end = (start + page_size).min(col.records.len());
        let slice = &col.records[start..end];

        let list = PyList::empty(py);
        for record in slice {
            let dict = PyDict::new(py);
            dict.set_item("id", record.id)?;
            for (k, v) in &record.fields {
                match v {
                    FieldValue::String(s)      => dict.set_item(k, s)?,
                    FieldValue::Int(i)         => dict.set_item(k, i)?,
                    FieldValue::Float(f)       => dict.set_item(k, f)?,
                    FieldValue::Bool(b)        => dict.set_item(k, b)?,
                    FieldValue::StringArray(a) => dict.set_item(k, a.clone())?,
                    FieldValue::FloatArray(a)  => dict.set_item(k, a.clone())?,
                }
            }
            list.append(dict)?;
        }
        Ok(list.into())
    }
}

#[pymodule]
fn paperdb_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PaperDB>()?;
    Ok(())
}
