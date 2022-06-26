pub use super::data::*;
pub use crate::traits::periphery::psp34_base::*;

use ink_prelude::string::String;

impl<T:PSP34BaseStorage<Data = PSP34BaseData>> PSP34Base for T{
    default fn name(&self)->String{
        let psp34_base_data =PSP34BaseStorage::get(self);
        psp34_base_data.name.clone()
    }

    default fn symbol(&self)->String{
        let psp34_base_data = PSP34BaseStorage::get(self);
        psp34_base_data.symbol.clone()
    }
}