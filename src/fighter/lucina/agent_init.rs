use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    custom_cancel::*,
    wubor_utils::{vars::*, table_const::*},
    super::{helper::*, cancel}
};

unsafe extern "C" fn yu_specialns_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, yu::instance::flag::DISABLE_SPECIAL_N_S) {
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn yu_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    spent_meter(fighter.battle_object, true).into()
}

unsafe extern "C" fn yu_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut ret = false;
    VarModule::off_flag(fighter.battle_object, yu::instance::flag::COMMAND);
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    if !ret
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND != 0
    && spent_meter_super(fighter.battle_object)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2) {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4.into(), true.into());
        ret = true;
    }
    if !ret
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND) {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
        ret = true;
    }
    if !ret
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_N_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(), true.into());
        ret = true;
    }
    if !ret
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool() {
        ControlModule::reset_flick_sub_x(fighter.module_accessor);
        ControlModule::reset_main_stick(fighter.module_accessor);
        ControlModule::reset_turn_lr(fighter.module_accessor);
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
        ret = true;
    }
    if ret {
        VarModule::on_flag(fighter.battle_object, yu::instance::flag::COMMAND);
    }
    if !ret
    && shadow_id(fighter.module_accessor)
    && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND)
    && !VarModule::is_flag(fighter.battle_object, yu::instance::flag::SHADOW_FRENZY)
    && VarModule::get_float(fighter.battle_object, yu::instance::float::SP_GAUGE) >= 25.0
    && VarModule::is_flag(fighter.battle_object, yu::instance::flag::ROMAN_ON_HIT) {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), true.into());
        ret = true;
    }
    ret.into()
}

#[event("lucina", start)]
unsafe fn agent_reset(fighter: &mut L2CFighterCommon) {
    cancel::install();
}

#[event("lucina", initialize)]
unsafe fn agent_init(fighter: &mut L2CFighterCommon) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    VarModule::set_float(fighter.battle_object, yu::instance::float::SP_GAUGE_MAX, 100.0);
    fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(yu_specialns_pre as *const () as _));
    fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(yu_specialns_pre as *const () as _));
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(yu_speciallw_pre as *const () as _));
    fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(yu_check_special_command as *const () as _));
}

pub fn install() {
    CustomCancelManager::initialize_agent(Hash40::new("fighter_kind_lucina"));
    agent_reset::install();
    agent_init::install();
}
