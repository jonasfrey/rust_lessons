
struct O_config {
    
}
fn main() {
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
        city: "Somewhere".to_string(),
    };

    // Serialize it to a JSON string.
    let serialized = serde_json::to_string(&person)?;

    // Write to a file
    let mut file = File::create("person.json").expect("Failed to create file");
    file.write_all(serialized.as_bytes()).expect("Failed to write to file");

    Ok(())
}