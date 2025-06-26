#![no_main]
use libfuzzer_sys::fuzz_target;

use numbat_wasm::numbat_codec::*;
use numbat_wasm::numbat_codec::test_util::*;
use dns::value_state::*;


fuzz_target!(|data: &[u8]| {
    if let Ok(decoded) = ValueState::top_decode(data) {
        let encoded_clean = check_top_encode(&decoded);
        let decoded_again = check_top_decode::<ValueState>(&encoded_clean[..]);
        let encoded_again = check_top_encode(&decoded_again);
        assert_eq!(encoded_clean, encoded_again);
    }
});
