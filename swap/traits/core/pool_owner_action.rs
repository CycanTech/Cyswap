use primitives::Address;


#[brush::wrapper]
pub type PoolOwnerActionsRef = dyn PoolOwnerActions;

// @title Permissioned pool actions
// @notice Contains pool methods that may only be called by the factory owner
#[brush::trait_definition]
pub trait PoolOwnerActions {
    /// @notice Set the denominator of the protocol's % share of the fees
    /// @param feeProtocol0 new protocol fee for token0 of the pool
    /// @param feeProtocol1 new protocol fee for token1 of the pool
    #[ink(message)]
    fn setFeeProtocol(&mut self,feeProtocol0:u8, feeProtocol1:u8);

    /// @notice Collect the protocol fee accrued to the pool
    /// @param recipient The address to which collected protocol fees should be sent
    /// @param amount0Requested The maximum amount of token0 to send, can be 0 to collect fees in only token1
    /// @param amount1Requested The maximum amount of token1 to send, can be 0 to collect fees in only token0
    /// @return amount0 The protocol fee collected in token0
    /// @return amount1 The protocol fee collected in token1
    #[ink(message)]
    fn collectProtocol(
        &mut self,
        recipient:Address,
        amount0Requested:u128,
        amount1Requested:u128
    )->(u128 , u128);
}