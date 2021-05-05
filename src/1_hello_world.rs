// Main function
fn main() {
    /**
     * Prints Hello, world!
     */
    println!("Hello, world!");

    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes i32. You can change what type 21 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    // As can named arugments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't.",
        1, 2
    );

    // You can right-align text witha specified width. The will output
    // "      1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);

    // You can pad numbers with extra zeros. This will output "000001";
    // println!("{number:>06}", number = 1) gives same result as below ;
    println!("{number:>0width$}", number = 1, width = 6);

    // Rust even checks to make sure the correct number of arguments are
    // used
    println!("My name is {0}, {1}, {0}.", "Bond", "James");

    let pi = 3.141592;
    println!("{pi:.precision$}", pi=pi, precision = 2);

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will now work.
    // println!("This struct `{}` won't print....", Structure(3));
}
