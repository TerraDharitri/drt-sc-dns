use numbat_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("dns");

    blockchain.register_contract_builder(
        "file:output/numbat-wasm-sc-dns.wasm",
        numbat_wasm_sc_dns::ContractBuilder,
    );
    blockchain
}

#[test]
fn test_denali_main_rs() {
    numbat_wasm_debug::denali_rs("denali/main.scen.json", world());
}
