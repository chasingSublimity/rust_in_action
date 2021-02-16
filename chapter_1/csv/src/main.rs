fn main() {
    let penguin_data ="\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // Vec is shorthand for Vector, which is a dynamic array
        // The underscore asks the compiler to infer the type of the vector element
        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();

        // the bang indicates a macro invocation.
        // macros are like functions, but return code rather than values.
        // they simplify common patterns
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields)
        }

        let name = fields[0];

        // the `parse` call tries to take the data from the right hand side
        // and transform it according to the type annotation on the left hand side
        // the underscore is asking the compiler to infer the error type
        let maybe_length: Result<f32, _> = fields[1].parse();

        if maybe_length.is_err() {
            continue;
        }

        let length = maybe_length.unwrap();

        println!("{}, {}cm", name, length);
    }
}