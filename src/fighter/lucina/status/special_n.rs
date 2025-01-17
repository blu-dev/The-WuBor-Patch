use crate::imports::status_imports::*;
use super::super::helper::*;

#[status("lucina", FIGHTER_STATUS_KIND_SPECIAL_N)]
unsafe fn lucina_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CHARGE_MAX);
    VarModule::on_flag(fighter.battle_object, yu::instance::flag::DISABLE_SPECIAL_N_S);
    lucina_specialn_mot_helper(fighter);
    if VarModule::is_flag(fighter.battle_object, yu::instance::flag::COMMAND)
    && spent_meter(fighter.battle_object, false) {
        let spent = VarModule::get_float(fighter.battle_object, yu::instance::float::SPENT_SP);
        let meter_max = VarModule::get_float(fighter.battle_object, yu::instance::float::SP_GAUGE_MAX);
        FGCModule::update_meter(fighter.battle_object, -spent, meter_max, yu::instance::float::SP_GAUGE);
        VarModule::set_int(fighter.battle_object, yu::instance::int::SP_FLASH_TIMER, 40);
        VarModule::on_flag(fighter.battle_object, yu::status::flag::IS_EX);
        sp_diff_checker(fighter.module_accessor);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specialn_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_specialn_mot_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_air_n_start"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_air_n_start"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_n_start"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_n_start"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
}

unsafe extern "C" fn lucina_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        lucina_specialn_mot_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP.into(), false.into());
    }
    0.into()
}

pub fn install() {
    lucina_specialn_main::install();
}