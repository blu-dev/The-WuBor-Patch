use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::WorkModule,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::{
        fighter::{
            mario::{
                agent_init::mario_speciallw_pre,
                fgc::mario_fgc
            },
            bayonetta::{
                agent_init::bayonetta_specials_pre,
                fgc::bayonetta_fgc
            },
            chrom::fgc::chrom_fgc,
            daisy::{
                agent_init::{
                    daisy_speciallw_pre,
                    daisy_itemtoss_pre
                },
                fgc::daisy_fgc
            },
            dolly::agent_init::*,
            element::fgc::element_fgc,
            ganon::fgc::ganon_fgc,
            kirby::agent_init::kirby_specialn_pre,
            kamui::agent_init::kamui_escapeair_pre,
            ken::agent_init::ken_speciallw_pre,
            lucario::{
                agent_init::lucario_specialhi_pre,
                fgc::lucario_fgc
            },
            lucina::agent_init::{
                yu_specialns_pre,
                yu_speciallw_pre,
                yu_check_special_command
            },
            miifighter::fgc::miifighter_fgc,
            richter::fgc::richter_fgc,
            samusd::fgc::samusd_fgc,
            shizue::agent_init::shizue_special_lw_pre,
            toonlink::fgc::toonlink_fgc,
            trail::agent_init::trail_guard_cont_pre
        }
    },
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

unsafe extern "C" fn specialn_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N) {
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn specials_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S) {
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn specialhi_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI) {
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn speciallw_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_LW) {
        return 0.into();
    }
    1.into()
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind == *FIGHTER_KIND_MARIO {
            fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(mario_speciallw_pre as *const () as _));
            fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(mario_fgc as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_DONKEY {
            DK_COUNT += 1;
        }
        else if fighter_kind == *FIGHTER_KIND_SAMUSD {
            WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, FIGHTER_SAMUSD_INSTANCE_WORK_ID_INT_CSHOT_ID);
            fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(samusd_fgc as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_KIRBY {
            fighter.global_table[SPECIAL_N_PRE].assign(&L2CValue::Ptr(kirby_specialn_pre as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_DAISY {
            fighter.global_table[CHECK_AIR_SPECIAL_PRE].assign(&false.into());
            fighter.global_table[CHECK_GROUND_ATTACK_PRE].assign(&L2CValue::Ptr(daisy_itemtoss_pre as *const () as _));
            fighter.global_table[CHECK_AIR_ITEM_THROW_PRE].assign(&false.into());
            fighter.global_table[CHECK_AIR_JUMP_PRE].assign(&false.into());
            fighter.global_table[CHECK_AIR_JUMP_AERIAL_POST].assign(&false.into());
            fighter.global_table[SPECIAL_S_PRE].assign(&L2CValue::Ptr(specials_pre_generic as *const () as _));
            fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(daisy_speciallw_pre as *const () as _));
            fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(daisy_fgc as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_PEACH {
            fighter.global_table[CHECK_GROUND_ATTACK_PRE].assign(&L2CValue::Ptr(daisy_itemtoss_pre as *const () as _));
            fighter.global_table[CHECK_AIR_ITEM_THROW_PRE].assign(&L2CValue::Ptr(daisy_itemtoss_pre as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_GANON {
            fighter.global_table[SPECIAL_N_PRE].assign(&L2CValue::Ptr(specialn_pre_generic as *const () as _));
            fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(ganon_fgc as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_LUCINA {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
            WorkModule::set_float(fighter.module_accessor, 100.0, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE_MAX);
            fighter.global_table[SPECIAL_N_PRE].assign(&L2CValue::Ptr(yu_specialns_pre as *const () as _));
            fighter.global_table[SPECIAL_S_PRE].assign(&L2CValue::Ptr(yu_specialns_pre as *const () as _));
            fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(yu_speciallw_pre as *const () as _));
            fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(yu_check_special_command as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_CHROM {
            fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(chrom_fgc as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_LUCARIO {
            fighter.global_table[SPECIAL_HI_PRE].assign(&L2CValue::Ptr(lucario_specialhi_pre as *const () as _));
            fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(lucario_fgc as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_TOONLINK {
            fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(toonlink_fgc as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_SHULK {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_SHULK_MONAD_TYPE_NONE, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE);
            fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(speciallw_pre_generic as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_MIIFIGHTER {
            fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(miifighter_fgc as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_KEN {
            fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(ken_speciallw_pre as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_KAMUI {
            fighter.global_table[CHECK_AIR_ESCAPE_PRE].assign(&L2CValue::Ptr(kamui_escapeair_pre as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_BAYONETTA {
            fighter.global_table[SPECIAL_S_PRE].assign(&L2CValue::Ptr(bayonetta_specials_pre as *const () as _));
            fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(bayonetta_fgc as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_RICHTER {
            fighter.global_table[SPECIAL_HI_PRE].assign(&L2CValue::Ptr(specialhi_pre_generic as *const () as _));
            fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(richter_fgc as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_RYU {
            WorkModule::set_float(fighter.module_accessor, -0.6, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_SEC_SEN_TIMER);
            // fighter.global_table[STATUS_END_CONTROL].assign(&false.into());
        }
        else if fighter_kind == *FIGHTER_KIND_SHIZUE {
            fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(shizue_special_lw_pre as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_DOLLY {
            fighter.global_table[GUARD_CONT_PRE].assign(&L2CValue::Ptr(dolly_guard_cont_pre as *const () as _));
            fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(dolly_check_special_command as *const () as _));
            fighter.global_table[CHECK_GROUND_CATCH_PRE].assign(&L2CValue::Ptr(dolly_check_ground_catch_pre as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_EFLAME
        || fighter_kind == *FIGHTER_KIND_ELIGHT {
            fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(element_fgc as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_TRAIL {
            fighter.global_table[GUARD_CONT_PRE].assign(&L2CValue::Ptr(trail_guard_cont_pre as *const () as _));
        }
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}