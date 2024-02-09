use std::f32::consts::PI;
use std::{fmt, vec};

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(DebugPrintable);

struct Printable(i32);

impl fmt::Display for Printable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name is {}, and age is {}", self.name, self.age)
    }
}

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{0}: {1}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn main() {
    println!("Hello, {world:>10}!", world = "world");

    let x = "x";
    println!("Hello {x} is {pi:.2}", pi = PI);

    let debug_printable = DebugPrintable(123);
    println!("{:?}", debug_printable);

    let deep_printable = Deep(debug_printable);
    println!("{:?}", deep_printable);
    println!("{:#?}", deep_printable);

    let display_printable = Printable(64);
    println!("{}", display_printable);

    let pretty_person = Person {
        name: "Zach",
        age: 39,
    };
    println!("{:?}", pretty_person);
    println!("{:#?}", pretty_person);
    println!("{}", pretty_person);

    let list = List(vec![1, 2, 3]);
    println!("{}", list);
}
