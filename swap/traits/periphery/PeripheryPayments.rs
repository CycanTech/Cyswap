use primitives::{Address, U256};

#[brush::wrapper]
pub type PeripheryPaymentsTraitRef = dyn PeripheryPaymentsTrait;

#[brush::trait_definition]
pub trait PeripheryPaymentsTrait {
    /// @notice Refunds any ETH balance held by this contract to the `msg.sender`
    /// @dev Useful for bundling with mint or increase liquidity that uses ether, or exact output swaps
    /// that use ether for the input amount
    #[ink(message, payable)]
    fn refundETH(&mut self);

    #[ink(message)]
    fn pay(&mut self, token: Address, payer: Address, recipient: Address, value: U256);
}