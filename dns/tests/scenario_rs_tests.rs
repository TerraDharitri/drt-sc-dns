use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("dns");

    blockchain.register_contract("file:output/dns.wasm", dns::ContractBuilder);
    blockchain
}

#[test]
fn test_denali_main_rs() {
    dharitri_sc_scenario::run_rs("scenarios/main.scen.json", world());
}

#[test]
fn test_denali_resolve_numbat_rs() {
    dharitri_sc_scenario::run_rs("scenarios/resolve-numbat.scen.json", world());
}

#[test]
#[ignore = "migrateUserName builtin function not implemented yet"]
fn test_denali_migrate_rs() {
    dharitri_sc_scenario::run_rs("scenarios/migrate.scen.json", world());
}
