use crate::imports::status_imports::*;
use crate::fighter::common::status::escape::escape_air_slide::*;

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn lucario_escape_air_slide_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    escape_air_slide_end_inner(fighter);
    if VarModule::is_flag(fighter.battle_object, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL) {
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        lucario_escape_air_slide_end
    );
}