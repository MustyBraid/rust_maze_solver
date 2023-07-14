/* Psuedocode:

Obtain JSON and open it
look around exterior of maze to identify entrance and exit coords
maybe receive entrance and exit coords from JSON? Could add flags and things in this way, too
Implement an established maze-slver algorithm
>Figure out how to implement that algorithm in a multithreaded way

Extension ideas :
GUI? Updates? Percentage bars? Method for selecting algorithm? Racing algorithms? Multi-file maze sets?

*/
// This is the most common "deseerializer" in rust. Our JSONS are very simple so this may be overkill, but I want to practice working with at least one rust package
use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");
}
