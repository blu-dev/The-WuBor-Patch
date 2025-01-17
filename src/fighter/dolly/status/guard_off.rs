use crate::imports::status_imports::*;

#[status("dolly", FIGHTER_STATUS_KIND_GUARD_OFF)]
unsafe fn dolly_guardoff_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOff()
}

pub fn install() {
    dolly_guardoff_main::install();
}