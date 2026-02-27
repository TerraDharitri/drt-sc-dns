use numbat_wasm::types::ManagedBuffer;
use numbat_wasm_debug::DebugApi;

fn validate_name(name_str: &str) -> Result<(), &'static str> {
    let mb = ManagedBuffer::<DebugApi>::from(name_str.as_bytes());
    numbat_wasm_sc_dns::name_validation::validate_name(&mb)
}

#[test]
fn test_validate_name() {
    let _ = DebugApi::dummy();

    // ok
    assert!(validate_name("aaaaaaaaaa.numbat").is_ok());
    assert!(validate_name("zzzzzzzzzz.numbat").is_ok());
    assert!(validate_name("0000000000.numbat").is_ok());
    assert!(validate_name("9999999999.numbat").is_ok());
    assert!(validate_name("coolname0001.numbat").is_ok());

    // too short
    assert!(validate_name(".numbat").is_err());
    assert!(validate_name("aa.numbat").is_err());

    // lowercase only
    assert!(!validate_name("Aaaaaaaaaa.numbat").is_ok());

    // no other chars
    assert!(validate_name("Aaaaa.aaaa.numbat").is_err());
    assert!(validate_name("Aaaaa@aaaa.numbat").is_err());
    assert!(validate_name("Aaaaa+aaaa.numbat").is_err());
    assert!(validate_name("Aaaaa-aaaa.numbat").is_err());
    assert!(validate_name("Aaaaa_aaaa.numbat").is_err());

    // without suffix
    assert!(validate_name("aaaaaaaaaa").is_err());
    assert!(validate_name("zzzzzzzzzz").is_err());
    assert!(validate_name("0000000000").is_err());
    assert!(validate_name("9999999999").is_err());
    assert!(validate_name("coolname0001").is_err());

    // name too long
    assert!(validate_name("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.numbat").is_err());
}
