use dharitri_sc::types::ManagedBuffer;
use dharitri_sc_scenario::DebugApi;

fn validate_name(name_str: &str) -> Result<(), &'static str> {
    let mb = ManagedBuffer::<DebugApi>::from(name_str.as_bytes());
    dns::name_validation::validate_name(&mb)
}

fn check(name: &str, expected: &Result<(), &'static str>) {
    assert_eq!(validate_name(name), *expected);
}

#[test]
fn test_validate_name() {
    let _ = DebugApi::dummy();

    let ok = &Ok(());
    let wrong_suffix = &Err("wrong suffix");
    let name_is_too_short = &Err("name is too short");
    let character_not_allowed = &Err("character not allowed");
    let name_too_long = &Err("name too long");

    // .x
    check("aaa.x", ok);
    check("aaaaaaaaaa.x", ok);
    check("zzzzzzzzzz.x", ok);
    check("0000000000.x", ok);
    check("9999999999.x", ok);
    check("coolname0001.x", ok);

    // .numbat
    check("aaa.numbat", wrong_suffix);
    check("aaaaaaaaaa.numbat", wrong_suffix);
    check("zzzzzzzzzz.numbat", wrong_suffix);
    check("0000000000.numbat", wrong_suffix);
    check("9999999999.numbat", wrong_suffix);
    check("coolname0001.numbat", wrong_suffix);

    // too short
    check(".x", name_is_too_short);
    check("aa.x", name_is_too_short);
    check(".numbat", wrong_suffix);
    check("aa.numbat", wrong_suffix);

    // lowercase only
    check("Aaaaaaaaaa.x", character_not_allowed);
    check("Aaaaaaaaaa.numbat", wrong_suffix);

    // no other chars
    check("aaaaa.aaaa.x", character_not_allowed);
    check("aaaaa@aaaa.x", character_not_allowed);
    check("aaaaa+aaaa.x", character_not_allowed);
    check("aaaaa-aaaa.x", character_not_allowed);
    check("aaaaa_aaaa.x", character_not_allowed);
    check("aaaaa.aaaa.numbat", wrong_suffix);
    check("aaaaa@aaaa.numbat", wrong_suffix);
    check("aaaaa+aaaa.numbat", wrong_suffix);
    check("aaaaa-aaaa.numbat", wrong_suffix);
    check("aaaaa_aaaa.numbat", wrong_suffix);

    // without suffix
    check("aaaaaaaaaa", wrong_suffix);
    check("zzzzzzzzzz", wrong_suffix);
    check("0000000000", wrong_suffix);
    check("9999999999", wrong_suffix);
    check("coolname0001", wrong_suffix);

    // name too long
    check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.x", ok); // 32 bytes
    check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.x", name_too_long); // 33 bytes
    check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.x", name_too_long); // 37 bytes
    check("aaaaaaaaaaaaaaaaaaaaaaaaa.numbat", wrong_suffix); // 32 bytes
    check("aaaaaaaaaaaaaaaaaaaaaaaaaa.numbat", name_too_long); // 33 bytes
    check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.numbat", name_too_long); // 37 bytes
}
