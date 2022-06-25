use openbrush::contracts::psp22::{PSP22Storage, PSP22Internal, PSP22Error};
// use openbrush::traits::Flush;
use ink_env::{DefaultEnvironment};
pub use crate::traits::periphery::weth9::*;
use ink_prelude::string::String;

impl<T: PSP22Storage + PSP22Internal> Weth9 for T {

    /// @notice Deposit ether to get wrapped ether
    default fn deposit(&mut self)->Result<(),PSP22Error>{
        let transfer_value = ink_env::transferred_value::<DefaultEnvironment>();
        let caller = ink_env::caller::<DefaultEnvironment>();
        let result = PSP22Internal::_mint(self, caller, transfer_value);
        result
    }

    /// @notice Withdraw wrapped ether to get ether
    default fn withdraw(&mut self,amount:u128)->Result<(),PSP22Error>{
        let caller = ink_env::caller::<DefaultEnvironment>();
        let balance = PSP22Internal::_balance_of(self, &caller);
        assert!(balance >= amount,"balance not enough!");
        PSP22Internal::_burn_from(self, caller, amount)?;
        let result = ink_env::transfer::<DefaultEnvironment>(caller, amount);
        result.map_err(|_|PSP22Error::Custom(String::from("transfer error!")))
    }

}
