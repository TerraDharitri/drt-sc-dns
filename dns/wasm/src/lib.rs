////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////
#![no_std]
#![feature(alloc_error_handler)]
use core::alloc::Layout;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    loop {}
}

numbat_wasm_node::wasm_endpoints! {
    numbat_wasm_sc_dns
    (
        callBack
        canRegister
        checkPending
        claim
        getContractOwner
        getOwnShardId
        getRegistrationCost
        nameHash
        nameShard
        register
        resetPending
        resolve
        resolveFromHash
        setFeatureFlag
        validateName
        version
    )
}
