import paperdb_py

db = paperdb_py.PaperDB.open("data/items.paperdb")
results = db.search("items", page=1, page_size=25)

for record in results:
    print(f"id: {record['id']} | name: {record['name']}")
