use paperdb_build::db::{FilterOperator, PaperDB, SearchQuery};

fn main() {
    let db = PaperDB::new("data/items.paperdb").unwrap();

    let query = SearchQuery {
        collection: "items".to_string(),
        filters: vec![],
        operator: FilterOperator::And,
        page: Some(1),
        page_size: Some(10),
    };

    let results = db.search(query).unwrap();
    for record in results {
        println!("id: {} | name: {:?}", record.id, record.fields.get("name"));
    }
}
