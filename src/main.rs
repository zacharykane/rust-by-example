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
struct Person {
    name: String,
    age: u8,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name is {}, and age is {}", self.name, self.age)
    }
}

// Define a structure named `List` containing a `Vec` of 32 bit integers.
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

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: u32 =
            u32::from(self.red) * 65536 + u32::from(self.green) * 256 + u32::from(self.blue);

        let red = self.red;
        let green = self.green;
        let blue = self.blue;
        let hex = format!("{:0>6X}", value);

        write!(f, "RGB ({red}, {green}, {blue}) 0x{hex}")
    }
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Matrix(a, b, c, d) = self;

        write!(f, "( {a} {b} )\n( {c} {d} )")
    }
}

fn transpose(pair: Matrix) -> Matrix {
    let Matrix(a, b, c, d) = pair;
    Matrix(a, c, b, d)
}

fn main() {
    println!("Hello, {world:>10}!", world = "world, padded with spaces");

    let x = "x";
    println!(
        "Hello {x} is {pi:.2}, set to 2 significant figures",
        pi = PI
    );

    let debug_printable = DebugPrintable(123);
    println!("Debug printable: {:?}", debug_printable);

    let deep_printable = Deep(debug_printable);
    println!("Nested Debug printable: {:?}", deep_printable);
    println!("Nested Debug printable (multiline): {:#?}", deep_printable);

    let display_printable = Printable(64);
    println!(
        "Display printable (see Struct Impl above) {}",
        display_printable
    );

    let pretty_person = Person {
        name: String::from("Zach"),
        age: 39,
    };
    println!("Debug printable: {:?}", pretty_person);
    println!("Debug printable (multiline): {:#?}", pretty_person);
    println!("Display printable (custom Impl): {}", pretty_person);

    let list = List(vec![1, 2, 3]);
    println!("Display printable Struct w/ Vector: {}", list);

    println!("Display printable in for loop:");
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        println!("{}", color);
    }

    let long_tup = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!(
        "Display printable primitive from Tuple: {}, {}, {}",
        long_tup.0, long_tup.5, long_tup.10
    );

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("Debug, tuple of tuples: {:?}", tuple_of_tuples);

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("Debug deconstructed: {:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Debug printable {:?}", matrix);
    println!("Display Matrix:\n{}", matrix);
    println!("Display Transposed Matrix:\n{}", transpose(matrix))
}
