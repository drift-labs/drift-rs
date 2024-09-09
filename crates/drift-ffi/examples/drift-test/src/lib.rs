mod drift_idl;

use drift_idl::types::SpotPosition;

extern "C" {
    pub fn spot_position_is_available(position: &SpotPosition) -> bool;
}

#[test]
fn test_ffi_call() {
    unsafe {
        let x = spot_position_is_available(
            &SpotPosition::default()
        );
        dbg!(x.to_string());
    }
}
