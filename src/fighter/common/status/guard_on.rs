use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    custom_var::*,
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOn_initStatus_common)]
unsafe fn sub_ftstatusuniqprocessguardon_initstatus_common(fighter: &mut L2CFighterCommon) {
    // Original
    ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
    // Additions
    if FighterUtil::is_valid_just_shield(fighter.module_accessor) {
        let shield_just_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_just_frame")) as f32;
        let just_shield_check_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("just_shield_check_frame"), 0);
        let just_frame = (shield_just_frame * just_shield_check_frame + 0.5) as i32;
        WorkModule::set_int(fighter.module_accessor, just_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
        ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(*SHIELD_TYPE_JUST_SHIELD), *FIGHTER_SHIELD_KIND_GUARD, 0);
        if FighterUtil::is_valid_just_shield_reflector(fighter.module_accessor) {
            ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        }
        fighter.FighterStatusGuard__set_just_shield_scale();
    }
    // Also Original, but moved down
    let hit_stop_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64u64);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
    let guard_off_disable_shield_recovery = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_disable_shield_recovery"));
    WorkModule::set_int(fighter.module_accessor, guard_off_disable_shield_recovery, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_SHIELD_RECOVERY_FRAME);
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_on_common)]
unsafe fn sub_status_guard_on_common(fighter: &mut L2CFighterCommon) {
    let shield_min_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_min_frame"));
    WorkModule::set_int(fighter.module_accessor, shield_min_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("guard_on"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
        MotionModule::add_motion_2nd(
            fighter.module_accessor,
            Hash40::new("guard"),
            0.0,
            1.0,
            false,
            1.0
        );
        MotionModule::set_rate_2nd(fighter.module_accessor, 0.0);
        fighter.sub_ftStatusUniqProcessGuardFunc_updateShield(true.into());
    }
    fighter.sub_guard_cont_pre();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_guard_on_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_guard_on_uniq as *const () as _));
}

#[skyline::hook(replace = L2CFighterCommon_sub_guard_on_uniq)]
unsafe fn sub_guard_on_uniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        fighter.FighterStatusGuard__landing_effect_control();
    }
    else {
        if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME) {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
            let just_guard_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
            if just_guard_frame == 0 {
                ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
                let guard_type = if FighterUtil::get_shield_type_of_guard(fighter.global_table[KIND].get_i32()) {
                    *SHIELD_TYPE_GUARD
                }
                else {
                    *SHIELD_TYPE_UNDEFINED
                };
                ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(guard_type), *FIGHTER_SHIELD_KIND_GUARD, 0);
                if FighterUtil::is_valid_just_shield_reflector(fighter.module_accessor) {
                    ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
                }
            }
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_LOCK) {
            let shield_dec1 = if !VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
                WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_dec1"))
            }
            else {
                0.0
            };
            let shield_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_frame"), 0);
            let decrease = shield_dec1 / shield_frame;
            WorkModule::sub_float(fighter.module_accessor, decrease, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        }
        let shield_health = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        let shield_health_min = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MIN);
        if shield_health < shield_health_min {
            WorkModule::set_float(fighter.module_accessor, shield_health_min, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        }
        let shield_min_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        if 0 < shield_min_frame {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        }
        let catch_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
        if catch_frame < 0 {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
            let catch_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
            if catch_frame < 0 {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
            }
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_end_guard_on_common)]
unsafe fn sub_status_end_guard_on_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_STATUS_KIND_GUARD
    && (status != *FIGHTER_STATUS_KIND_GUARD_DAMAGE
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD)) {
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0xafae75f05), true, true);
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0x10da0b43c8), true, true);
    }
    else if !param_1.get_bool() {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_ftstatusuniqprocessguardon_initstatus_common,
            sub_guard_on_uniq,
            sub_status_end_guard_on_common,
            sub_status_guard_on_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}