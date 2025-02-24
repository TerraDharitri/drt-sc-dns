use dns::name_validation::SuffixType;
use dharitri_sc::types::ManagedBuffer;
use dharitri_sc_scenario::DebugApi;

fn prepare_name_for_hash_and_classify(name_str: &str) -> (ManagedBuffer<DebugApi>, SuffixType) {
    let mb = ManagedBuffer::<DebugApi>::from(name_str.as_bytes());
    dns::name_validation::prepare_name_for_hash_and_classify(&mb)
}

fn check(name: &str, expected: &str, suffix_type: SuffixType) {
    assert_eq!(
        prepare_name_for_hash_and_classify(name),
        (ManagedBuffer::from(expected.as_bytes()), suffix_type)
    );
}

#[test]
fn prepare_name_for_hash_test() {
    let _ = DebugApi::dummy();

    // .x is replaced with .numbat
    check("aaa.x", "aaa.numbat", SuffixType::X);
    check("aaaaaaaaaa.x", "aaaaaaaaaa.numbat", SuffixType::X);
    check("zzzzzzzzzz.x", "zzzzzzzzzz.numbat", SuffixType::X);
    check("0000000000.x", "0000000000.numbat", SuffixType::X);
    check("9999999999.x", "9999999999.numbat", SuffixType::X);
    check("coolname0001.x", "coolname0001.numbat", SuffixType::X);

    // .numbat names are returned unchanged
    check("aaa.numbat", "aaa.numbat", SuffixType::Numbat);
    check("aaaaaaaaaa.numbat", "aaaaaaaaaa.numbat", SuffixType::Numbat);
    check("zzzzzzzzzz.numbat", "zzzzzzzzzz.numbat", SuffixType::Numbat);
    check("0000000000.numbat", "0000000000.numbat", SuffixType::Numbat);
    check("9999999999.numbat", "9999999999.numbat", SuffixType::Numbat);
    check(
        "coolname0001.numbat",
        "coolname0001.numbat",
        SuffixType::Numbat,
    );

    // undefined behavior for invalid names
    // tests are only for ensuring that the function doesn't panic
    check("aa.x", "aa.numbat", SuffixType::X);
    check("test.abc.x", "test.abc.numbat", SuffixType::X);
    check("test.abc.foo", "test.abc.foo", SuffixType::Numbat);
    check("test", "test", SuffixType::Numbat);
    check("test.abc", "test.abc", SuffixType::Numbat);
}
