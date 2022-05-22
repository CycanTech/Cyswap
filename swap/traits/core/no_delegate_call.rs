use brush::modifier_definition;


#[modifier_definition]
pub fn noDelegateCall<T, F, R>(instance: &mut T, body: F) -> R
where
    F: FnOnce(&mut T) -> R,
    T:NoDelegateCall,
{
    instance.checkNotDelegateCall();
    body(instance)
}


#[brush::trait_definition]
pub trait NoDelegateCall{
    /// @dev Private method is used instead of inlining into modifier because modifiers are copied into each method,
    ///     and the use of immutable means the address bytes are copied in every place the modifier is used.
    fn checkNotDelegateCall(&self);
}