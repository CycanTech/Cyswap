use primitives::Address;


#[brush::wrapper]
pub type PeripheryImmutableStateRef = dyn PeripheryImmutableState;

#[brush::trait_definition]
pub trait PeripheryImmutableState{
    /// @return Returns the address of the Uniswap V3 factory
    #[ink(message)]
    fn factory(&self) -> Address;

    /// @return Returns the address of WETH9
    #[ink(message)]
    fn WETH9(&self) -> Address;
}