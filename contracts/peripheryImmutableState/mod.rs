
use ink_env::AccountId;
// SPDX-License-Identifier: GPL-2.0-or-later
use ink_lang as ink;
/// @title Immutable state
/// @notice Functions that return immutable state of the router
#[ink::trait_definition]
pub trait PeripheryImmutableState {
    /// @return Returns the address of the Uniswap V3 factory
    #[ink(message)]
    fn factory(&self)->AccountId;

    /// @return Returns the address of WETH9
    #[ink(message)]
    fn WETH9(&self)->AccountId;
}




// // SPDX-License-Identifier: GPL-2.0-or-later
// pragma solidity =0.7.6;

// import '../interfaces/IPeripheryImmutableState.sol';

// /// @title Immutable state
// /// @notice Immutable state used by periphery contracts
// abstract contract PeripheryImmutableState is IPeripheryImmutableState {
//     /// @inheritdoc IPeripheryImmutableState
//     address public immutable override factory;
//     /// @inheritdoc IPeripheryImmutableState
//     address public immutable override WETH9;

//     constructor(address _factory, address _WETH9) {
//         factory = _factory;
//         WETH9 = _WETH9;
//     }
// }

// // SPDX-License-Identifier: GPL-2.0-or-later
// pragma solidity >=0.5.0;

// /// @title Immutable state
// /// @notice Functions that return immutable state of the router
// interface IPeripheryImmutableState {
//     /// @return Returns the address of the Uniswap V3 factory
//     function factory() external view returns (address);

//     /// @return Returns the address of WETH9
//     function WETH9() external view returns (address);
// }