use brush::traits::AccountId;
use primitives::Int256;

pub const NUMERATOR_MOST: Int256 = 300;
pub const NUMERATOR_MORE: Int256 = 200;
pub const NUMERATOR: Int256 = 100;

pub const DENOMINATOR_MOST: Int256 = -300;
pub const DENOMINATOR_MORE: Int256 = -200;
pub const DENOMINATOR: Int256 = -100;

pub fn trans_hex_2_account(token:&str)->AccountId{
    ink_env::debug_print!("token is:{:?}",token);
    let token_vec = hex::decode(token).expect("Decoding failed");
    // let token_vec = hex::decode("e4678b676433e1f6f3f6a77730f4ede1737ad4b828b229def437e3e2ab46eb01").expect("Decoding failed");
    let token_array:&[u8] = &token_vec;
    let account:[u8;32] = token_array.try_into().expect("");
    account.into()
}