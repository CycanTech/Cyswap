use ink_prelude::string::String;
use primitives::{Uint160, Int24};

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PoolError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
}
