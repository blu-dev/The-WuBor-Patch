use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::{
        wua_bind::*,
        vars::*
    }
};

pub unsafe extern "C" fn toonlink_fgc(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
        let status = StatusModule::status_kind(fighter.module_accessor);
        let mut special_cancels : Vec<i32> = [].to_vec();
        let mut normal_cancels : Vec<i32> = [].to_vec();
        let mut aerial_cancel = false;
        let mut jump_cancel = 0;
        MiscModule::set_hp(fighter, 118.0);
        if [
            *FIGHTER_STATUS_KIND_ATTACK
        ].contains(&status) {
            special_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
            ].to_vec();
            normal_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3
            ].to_vec();
        }
        else if [
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_DASH,
            *FIGHTER_STATUS_KIND_ATTACK_AIR
        ].contains(&status) {
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") {
                jump_cancel = 1;
            }
            special_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
            ].to_vec();
            normal_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
            ].to_vec();
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
                VarModule::set_int(fighter.battle_object, commons::status::int::ENABLED_AERIALS, 0b01111);
                aerial_cancel = true;
                VarModule::on_flag(fighter.battle_object, commons::status::flag::NORMAL_CANCEL);
            }
        }
        else if [
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4
        ].contains(&status) {
            if status == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                jump_cancel = 1;
            }
            special_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
            ].to_vec();
        }
        if FGCModule::cancel_system(fighter, normal_cancels, special_cancels, aerial_cancel, jump_cancel).get_bool() {
            return;
        }
        if status == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            FGCModule::cancel_exceptions(
                fighter,
                *FIGHTER_STATUS_KIND_ATTACK_DASH,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3,
                true
            );
        }
    }
}
