mod db;

use db::InMemoryDb;

fn main() {
    let mut db = InMemoryDb::new();
    db.set("hello".to_string(), "world".to_string());
    println!("SET hello -> world");

    match db.get("hello") {
        Some(val) => println!("GET hello: {}", val),
        None => println!("GET hello: <not found>"),
    }

    let deleted = db.delete("hello");
    println!("DELETE hello: {}", deleted);
    match db.get("hello") {
        Some(val) => println!("GET hello: {}", val),
        None => println!("GET hello: <not found>"),
    }
}
