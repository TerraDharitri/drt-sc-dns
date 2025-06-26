use numbat_wasm::{api::ManagedTypeApi, types::ManagedAddress};

numbat_wasm::derive_imports!();

/// Copied from numbat-wasm serialization tests.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Debug)]
pub enum ValueState<M: ManagedTypeApi> {
    None,
    Pending(ManagedAddress<M>),
    Committed(ManagedAddress<M>),
}

impl<M: ManagedTypeApi> ValueState<M> {
    pub fn is_available(&self) -> bool {
        matches!(self, ValueState::None)
    }
}
