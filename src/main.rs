mod core;

// TODO: decide
// write a parser/tokenizer for sql or have something like mongo?
// maybe write an easy to use and simple database
// selling point -> is EZ and performant (won't be lol), kind of like go
//
// pseudo create table
// db.create("table_name", {
//      id: { p_key: true, type: int, auto_inc: true },
//      first_name: { type: char, max_length: 16, default: "Morty" },
//      email: { type: char, unique: true },
//      pets: { type: Pet, f_key: true }
//
//     constraints: {
//         email: { unique },
//         id: { p_key }
//     }
// })

// TODO:
// create a board dude
// core entities
// core file structure -> bTree and how to utilize it (indexing as well)
// figure out read/write -> do I rly need mutex?
// parser
// tokenizer
// client

fn main() {
    println!("Hello, world!");
}
