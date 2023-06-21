use crate::imports::status_imports::*;

#[status_script(agent = "diddy", status = FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn diddy_special_hi_upper_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into()); // Was ALWAYS_BOTH_SIDES
    ret
}

pub fn install() {
    install_status_scripts!(
        diddy_special_hi_upper_main
    );
}