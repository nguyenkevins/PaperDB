use paperdb_build::db::PaperDB;
use paperdb_core::{Field, FieldType, FieldValue, Model};
use rand::Rng;
use std::collections::HashMap;

const RECORD_COUNT: usize = 100;
const EMBEDDING_SIZE: usize = 384;

fn random_string(rng: &mut impl Rng, len: usize) -> String {
    (0..len).map(|_| rng.gen_range(b'a'..=b'z') as char).collect()
}

fn random_string_array(rng: &mut impl Rng) -> FieldValue {
    let count = rng.gen_range(2..=6);
    FieldValue::StringArray((0..count).map(|_| random_string(rng, 6)).collect())
}

fn random_embedding(rng: &mut impl Rng) -> FieldValue {
    FieldValue::FloatArray((0..EMBEDDING_SIZE).map(|_| rng.gen_range(-1.0f32..=1.0f32)).collect())
}

fn build_model() -> Model {
    Model {
        name: "items".to_string(),
        fields: vec![
            Field { name: "embedding".to_string(),   field_type: FieldType::FloatArray,  required: true },
            Field { name: "name".to_string(),         field_type: FieldType::String,      required: true },
            Field { name: "description".to_string(),  field_type: FieldType::String,      required: true },
            Field { name: "category".to_string(),     field_type: FieldType::String,      required: true },
            Field { name: "status".to_string(),       field_type: FieldType::String,      required: true },
            Field { name: "country".to_string(),      field_type: FieldType::String,      required: true },
            Field { name: "score".to_string(),        field_type: FieldType::Float,       required: true },
            Field { name: "rating".to_string(),       field_type: FieldType::Float,       required: true },
            Field { name: "price".to_string(),        field_type: FieldType::Float,       required: true },
            Field { name: "stock".to_string(),        field_type: FieldType::Int,         required: true },
            Field { name: "views".to_string(),        field_type: FieldType::Int,         required: true },
            Field { name: "age".to_string(),          field_type: FieldType::Int,         required: true },
            Field { name: "year".to_string(),         field_type: FieldType::Int,         required: true },
            Field { name: "active".to_string(),       field_type: FieldType::Bool,        required: true },
            Field { name: "verified".to_string(),     field_type: FieldType::Bool,        required: true },
            Field { name: "featured".to_string(),     field_type: FieldType::Bool,        required: true },
            Field { name: "published".to_string(),    field_type: FieldType::Bool,        required: true },
            Field { name: "deleted".to_string(),      field_type: FieldType::Bool,        required: true },
            Field { name: "tags".to_string(),         field_type: FieldType::StringArray, required: true },
            Field { name: "labels".to_string(),       field_type: FieldType::StringArray, required: true },
            Field { name: "keywords".to_string(),     field_type: FieldType::StringArray, required: true },
            Field { name: "regions".to_string(),      field_type: FieldType::StringArray, required: true },
            Field { name: "aliases".to_string(),      field_type: FieldType::StringArray, required: true },
            Field { name: "rank".to_string(),         field_type: FieldType::Int,         required: true },
            Field { name: "weight".to_string(),       field_type: FieldType::Float,       required: true },
        ],
    }
}

fn random_record(rng: &mut impl Rng) -> HashMap<String, FieldValue> {
    let mut r = HashMap::new();
    r.insert("embedding".to_string(),   random_embedding(rng));
    r.insert("name".to_string(),        FieldValue::String(random_string(rng, 8)));
    r.insert("description".to_string(), FieldValue::String(random_string(rng, 20)));
    r.insert("category".to_string(),    FieldValue::String(random_string(rng, 6)));
    r.insert("status".to_string(),      FieldValue::String(random_string(rng, 5)));
    r.insert("country".to_string(),     FieldValue::String(random_string(rng, 4)));
    r.insert("score".to_string(),       FieldValue::Float(rng.gen_range(0.0..100.0)));
    r.insert("rating".to_string(),      FieldValue::Float(rng.gen_range(0.0..5.0)));
    r.insert("price".to_string(),       FieldValue::Float(rng.gen_range(0.0..1000.0)));
    r.insert("stock".to_string(),       FieldValue::Int(rng.gen_range(0..10000)));
    r.insert("views".to_string(),       FieldValue::Int(rng.gen_range(0..1_000_000)));
    r.insert("age".to_string(),         FieldValue::Int(rng.gen_range(0..100)));
    r.insert("year".to_string(),        FieldValue::Int(rng.gen_range(1900..2024)));
    r.insert("active".to_string(),      FieldValue::Bool(rng.gen_bool(0.5)));
    r.insert("verified".to_string(),    FieldValue::Bool(rng.gen_bool(0.5)));
    r.insert("featured".to_string(),    FieldValue::Bool(rng.gen_bool(0.5)));
    r.insert("published".to_string(),   FieldValue::Bool(rng.gen_bool(0.5)));
    r.insert("deleted".to_string(),     FieldValue::Bool(rng.gen_bool(0.1)));
    r.insert("tags".to_string(),        random_string_array(rng));
    r.insert("labels".to_string(),      random_string_array(rng));
    r.insert("keywords".to_string(),    random_string_array(rng));
    r.insert("regions".to_string(),     random_string_array(rng));
    r.insert("aliases".to_string(),     random_string_array(rng));
    r.insert("rank".to_string(),        FieldValue::Int(rng.gen_range(1..10000)));
    r.insert("weight".to_string(),      FieldValue::Float(rng.gen_range(0.0..500.0)));
    r
}

fn main() {
    std::fs::create_dir_all("data").unwrap();
    let _ = std::fs::remove_file("data/items.paperdb");
    let mut db = PaperDB::new("data/items.paperdb").unwrap();
    let mut rng = rand::thread_rng();

    db.create_collection(build_model()).ok();

    println!("inserting {} records...", RECORD_COUNT);
    for _ in 0..RECORD_COUNT {
        db.insert("items", random_record(&mut rng)).unwrap();
    }

    db.save().unwrap();
    println!("done — saved to data/items.paperdb");
}
