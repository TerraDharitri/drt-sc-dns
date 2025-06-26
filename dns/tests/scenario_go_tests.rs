#[test]
#[ignore = "there are differences in the emitted logs (transferValueOnly)"]
fn test_denali_main_go() {
    dharitri_sc_scenario::run_go("scenarios/main.scen.json");
}

#[test]
fn test_denali_resolve_numbat_go() {
    dharitri_sc_scenario::run_go("scenarios/resolve-numbat.scen.json");
}

#[test]
#[ignore = "migrateUserName builtin function not implemented yet"]
fn test_denali_migrate_go() {
    dharitri_sc_scenario::run_go("scenarios/migrate.scen.json");
}
