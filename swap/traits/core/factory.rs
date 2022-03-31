use brush::{
    traits::{
        AccountId,
    },
};
use primitives::Address;


#[brush::wrapper]
pub type FactoryRef = dyn Factory;



#[brush::trait_definition]
pub trait Factory{
    #[ink(message)]
    fn get_pool(&self,fee:u32,token0:AccountId, token1:AccountId)->AccountId;

    //此处原有modifier,限制不可以使用delegateCall的方式调用该方法,因为ink!中没有delegate call 调用,所以按时不使用NoDelegateCall
    #[ink(message)]
    fn create_pool(&mut self,fee:u32,token_a:Address,token_b:Address)->AccountId;
}