use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::{
        vars::*,
        table_const::*,
        fighter::ganon::helper::*
    }
};

#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_GANON_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn kirby_ganon_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        FighterMotionModuleImpl::change_motion_kirby_copy(
            fighter.module_accessor,
            Hash40::new("special_n"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        FighterMotionModuleImpl::change_motion_kirby_copy(
            fighter.module_accessor,
            Hash40::new("special_air_n"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_ganon_specialn_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_ganon_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_GANON_STATUS_WORK_ID_INT_TELEPORT_STEP) == FIGHTER_GANON_TELEPORT_STEP_INIT {
        deception_init(fighter);
    }
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_GANON_STATUS_WORK_ID_INT_TELEPORT_STEP) == FIGHTER_GANON_TELEPORT_STEP_CHECK_FEINT {
        deception_feint_handler(fighter);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GANON_STATUS_WORK_ID_FLAG_TELEPORT_STOP) {
        KineticModule::unable_energy_all(fighter.module_accessor);
    }
    let curr_sit = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_sit = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if curr_sit != prev_sit {
        if curr_sit == *SITUATION_KIND_GROUND {
            FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(
                fighter.module_accessor,
                Hash40::new("special_n"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        else {
            FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(
                fighter.module_accessor,
                Hash40::new("special_air_n"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
    if MotionModule::frame(fighter.module_accessor) >= 65.0 {
        if curr_sit == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        kirby_ganon_specialn_main
    );
}