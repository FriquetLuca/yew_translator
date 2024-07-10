mod errors;
mod generate;
mod options;
mod parse_to_hashmap;

pub use errors::StringTemplaterError;
pub use generate::generate;
pub use options::StringTemplaterOptions;
pub use parse_to_hashmap::{encode_json_to_hashmap, parse_to_hashmap};
