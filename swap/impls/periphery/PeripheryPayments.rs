use crate::impls::periphery_immutable_state::ImmutableStateStorage;
use crate::traits::periphery::weth9::*;
use brush::contracts::psp22::extensions::metadata::*;
use brush::traits::Balance;
use ink_env::CallFlags;
use ink_env::DefaultEnvironment;
use ink_prelude::vec;
use ink_prelude::vec::Vec;
use primitives::{Address, U256};

pub use crate::traits::periphery::PeripheryPayments::*;

impl<T: ImmutableStateStorage> PeripheryPaymentsTrait for T {
    /// @inheritdoc IPeripheryPayments
    default fn refundETH(&mut self) {
        // if (address(this).balance > 0) TransferHelper.safeTransferETH(msg.sender, address(this).balance);
        if ink_env::balance::<DefaultEnvironment>() > 0 {
            ink_env::transfer::<DefaultEnvironment>(
                ink_env::caller::<DefaultEnvironment>(),
                ink_env::balance::<DefaultEnvironment>(),
            )
            .unwrap();
        }
    }

    /// @param token The token to pay
    /// @param payer The entity that must pay, is the user (alice)
    /// @param recipient The entity that will receive payment
    /// @param value The amount to pay
    default fn pay(&mut self, mut token: Address, payer: Address, recipient: Address, value: U256) {
        let mut WETH9 = self.get().WETH9;
        let balance_of_contract: Balance = ink_env::balance::<DefaultEnvironment>();
        let address_of_this: Address = ink_env::account_id::<DefaultEnvironment>();
        if token == WETH9 && balance_of_contract >= value.as_u128() {
            // pay with WETH9
            // IWETH9(WETH9).deposit{value: value}(); // wrap only what is needed to pay
            Weth9Ref::deposit_builder(&mut WETH9)
                .transferred_value(value.as_u128())
                .fire()
                .unwrap()
                .unwrap();
            // <&mut Weth9Ref>::call_mut(&mut WETH9).deposit().transferred_value(value).fire().expect("weth9 deposit error!");
            // // TODO add deposit to transfer.
            // Weth9Ref::deposit(&WETH9).unwrap();
            // IWETH9(WETH9).transfer(recipient, value);
            PSP22Ref::transfer(&mut WETH9, recipient, value.as_u128(), vec![0u8]).unwrap();
        } else if payer == address_of_this {
            // pay with tokens already in the contract (for the exact input multihop case)
            // TransferHelper.safeTransfer(token, recipient, value);
            ink_env::debug_println!("-------------+6");
            PSP22Ref::transfer(&mut token, recipient, value.as_u128(), vec![0u8]).unwrap();
        } else {
            // pull payment
            // TransferHelper.safeTransferFrom(token, payer, recipient, value);
            //Here will make a error.panicked at 'dispatching ink! message failed: could not read input',But the transfer is success.
            let result: Result<(), PSP22Error> = PSP22Ref::transfer_from_builder(
                &mut token,
                payer,
                recipient,
                value.as_u128(),
                Vec::<u8>::new(),
            )
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire()
            .unwrap();
            match result {
                Ok(_s) => ink_env::debug_message("success!!!!!!!!!!"),
                Err(err) => {
                    ink_env::debug_println!("fail!!!!!!!!!! reason is:{:?}", err);
                    panic!("transfer error!");
                }
            }
        }
    }
}
