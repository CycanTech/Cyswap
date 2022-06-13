use brush::contracts::psp22::PSP22Error;
/// base info for PSP34
#[brush::wrapper]
pub type Weth9Ref = dyn Weth9;

#[brush::trait_definition]
pub trait Weth9 {
    /// @notice Deposit ether to get wrapped ether
    #[ink(message,payable)]
    fn deposit(&mut self)->Result<(),PSP22Error>;

    /// @notice Withdraw wrapped ether to get ether
    #[ink(message)]
    fn withdraw(&mut self,amount:u128)->Result<(),PSP22Error>;
}
