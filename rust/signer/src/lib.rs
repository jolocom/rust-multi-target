pub mod derivation;
pub mod error;
pub mod event;
pub mod event_message;
pub mod prefix;
pub mod state;
pub mod util;

mod export;

use prefix::Prefix;

// export! {
//     @Java_io_parity_substrateSign_SubstrateSignModule_desPrefix
//     fn deserialize_prefix(seed: &str) -> String {
//         let test: bool = match seed.parse::<Prefix>() {
//             Ok(res) => true,
//             Err(e) => false,
//         };
// 
//         String::from("hello")
//     }
// }
