use crate::traits::periphery::weth9::*;
use brush::contracts::psp22::extensions::metadata::*;
use brush::contracts::psp22::PSP22Storage;
use brush::traits::Balance;
use ink_env::DefaultEnvironment;
use primitives::{Address, U256};
use ink_prelude::vec;
use ink_env::CallFlags;
use ink_prelude::vec::Vec;

use crate::impls::pool_initialize::PoolInitializeStorage;
pub use crate::traits::periphery::PeripheryPayments::*;

impl<T: PoolInitializeStorage> PeripheryPaymentsTrait for T {
    /// @inheritdoc IPeripheryPayments
    default fn refundETH(&mut self) {
        // if (address(this).balance > 0) TransferHelper.safeTransferETH(msg.sender, address(this).balance);
        if ink_env::balance::<DefaultEnvironment>() > 0 {
            ink_env::transfer::<DefaultEnvironment>(
                ink_env::caller::<DefaultEnvironment>(),
                ink_env::balance::<DefaultEnvironment>(),
            ).unwrap();
        }
    }

    /// @param token The token to pay
    /// @param payer The entity that must pay
    /// @param recipient The entity that will receive payment
    /// @param value The amount to pay
    default fn pay(&mut self, mut token: Address, payer: Address, recipient: Address, value: U256) {
        ink_env::debug_message("&&&&&&&&&&8");
        let mut WETH9 = <Self as PoolInitializeStorage>::get(self).WETH9;
        let balance_of_contract: Balance = ink_env::balance::<DefaultEnvironment>();
        let address_of_this: Address = ink_env::account_id::<DefaultEnvironment>();
        if token == WETH9 && balance_of_contract >= value.as_u128() {
            ink_env::debug_message("&&&&&&&&&&9");
            // pay with WETH9
            // IWETH9(WETH9).deposit{value: value}(); // wrap only what is needed to pay
            // <&mut Erc20Minable>::call_mut(&mut *self.erc20_minable)
            //     .mine(who, value)
            //     .transferred_value(value) // 加上了调用 payable 的方法的时候，提供transfer
            //     .fire()
            //     .expect("something wrong");
            ink_env::transfer::<DefaultEnvironment>(WETH9, value.as_u128()).unwrap();
            ink_env::debug_message("&&&&&&&&&&10");
            // <&mut Weth9Ref>::call_mut(&mut *WETH9).deposit().transferred_value(value).fire().expect("weth9 deposit error!");
            // TODO add deposit to transfer.
            Weth9Ref::deposit(&WETH9).unwrap();
            ink_env::debug_message("&&&&&&&&&&11");
            // IWETH9(WETH9).transfer(recipient, value);
            PSP22Ref::transfer(&mut WETH9, recipient, value.as_u128(), vec![0u8]).unwrap();
            ink_env::debug_message("&&&&&&&&&&12");
        } else if payer == address_of_this {
            ink_env::debug_message("&&&&&&&&&&13");
            // pay with tokens already in the contract (for the exact input multihop case)
            // TransferHelper.safeTransfer(token, recipient, value);
            PSP22Ref::transfer(&mut token, recipient, value.as_u128(), vec![0u8]).unwrap();
            ink_env::debug_message("&&&&&&&&&&14");
        } else {
            // pull payment
            // TransferHelper.safeTransferFrom(token, payer, recipient, value);
            ink_env::debug_message("&&&&&&&&&&15");
            let result:Result<(),PSP22Error> = PSP22Ref::transfer_from_builder(&mut token, payer, recipient, value.as_u128(), Vec::<u8>::new())
            .call_flags(CallFlags::default().set_allow_reentry(true)).fire().unwrap();
            match result{
                Ok(s)=>ink_env::debug_message("success!!!!!!!!!!"),
                Err(e)=>ink_env::debug_message("fail!!!!!!!!!!"),
            }
            ink_env::debug_message("&&&&&&&&&&16");
        }
    }
}
