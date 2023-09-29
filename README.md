
Input a line and convert it to any type which implement trait `FromBuf`

Support Type:

(unsigned) `integer` (*binary, octal, decimal, hex*)

`float`, `char`, `String` and their `Vec<T>`

# Usage
```rust
use consio::input;
fn main() {
    let str = input!(String).unwrap(); // Console input "Hello World!"
    assert_eq!(str.as_str(), "Hello World!");

    let n = input!(i32).unwrap();  // Console input "11"
    assert_eq!(n, 11);

    let hex = input!(i32).unwrap(); // Console input "11h" or "0x11"
    assert_eq!(hex, 17);

    // print something before input
    let value = input!(print "Input a string: ").unwrap();

    // input with a default value
    let value = input!(default); // Console input any invalid number
    assert_eq!(value, Default::default());
}
```