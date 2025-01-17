use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*},
    super::helper::*
};

#[status("ganon", FIGHTER_STATUS_KIND_SPECIAL_N)]
unsafe fn ganon_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(
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
        MotionModule::change_motion(
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
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_specialn_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let step = VarModule::get_int(fighter.battle_object, ganon::status::int::TELEPORT_STEP);
    if step < ganon::TELEPORT_STEP_CHECK_FEINT
    && ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        VarModule::on_flag(fighter.battle_object, ganon::status::flag::TELEPORT_FEINT);
    }
    if step == ganon::TELEPORT_STEP_INIT {
        deception_init(fighter);
    }
    if step == ganon::TELEPORT_STEP_MOVE {
        deception_movement(fighter);
    }
    if step == ganon::TELEPORT_STEP_CHECK_FEINT {
        deception_feint_handler(fighter);
    }
    if VarModule::is_flag(fighter.battle_object, ganon::status::flag::TELEPORT_STOP) {
        KineticModule::unable_energy_all(fighter.module_accessor);
    }
    let curr_sit = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_sit = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if curr_sit != prev_sit {
        if curr_sit == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(
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
            MotionModule::change_motion_inherit_frame(
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
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    0.into()
}

#[status("ganon", FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH)]
unsafe fn ganon_specials_air_catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    MotionModule::set_rate(fighter.module_accessor, 0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_special_s_air_catch_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_special_s_air_catch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut ret = 0;
    if fighter.global_table[STATUS_FRAME].get_f32() == 1.0 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_catch"), 1.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    if fighter.global_table[STATUS_FRAME].get_f32() >= 1.0 {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END.into(), false.into());
        }
        ret = 1;
    }
    ret.into()
}

#[status("ganon", FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END)]
unsafe fn ganon_sspecial_air_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        1
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        true,
        false,
        *WEAPON_MARIO_PUMP_WATER_STATUS_KIND_REGULAR as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

#[status("ganon", FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END)]
unsafe fn ganon_sspecial_air_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.4
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_specials_air_end_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_specials_air_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_air_check_fall_common().get_bool() {
        return 1.into()
    }
    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    1.into()
}

pub fn install() {
    ganon_specialn_main::install();
    ganon_specials_air_catch_main::install();
    ganon_sspecial_air_end_pre::install();
    ganon_sspecial_air_end_main::install();
}