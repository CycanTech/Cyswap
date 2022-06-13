/// base info for PSP34
use ink_prelude::string::String;

#[brush::wrapper]
pub type PSP34BaseRef = dyn PSP34Base;

#[brush::trait_definition]
pub trait PSP34Base {
    /**
     * @dev See {IERC721Metadata-name}.
     */
    #[ink(message)]
    fn name(&self)->String;

    /**
     * @dev See {IERC721Metadata-symbol}.
     */
    #[ink(message)]
    fn symbol(&self) -> String;
}
