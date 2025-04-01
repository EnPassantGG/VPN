fn main() {
    // Saves penguin name and length. Looks like an object
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    // Put all the 'objects' into an 'array'
    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 { // skips the header row
            continue;
        }


        let fields: Vec<_> = record // start with line of text
            .split(',')
            .map(|field| field.trim()) // trim whitespace
            .collect();
        if cfg!(debug_assertions) { // checks at compile time (similar to C macros)
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() { // tries to parse field as floating point
            println!("{}, {}cm", name, length);
        }
    }
}
