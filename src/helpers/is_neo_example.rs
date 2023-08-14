use minijinja::Error;

// This is used to see if a set of attributes contains
// an example and then returns true if so and
// false if not. It's for the neo docs to make it
// easier to show the examples. Not really for
// use anywhere else.

pub fn is_neo_example(_html: String) -> Result<bool, Error> {
    Ok(true)
}
