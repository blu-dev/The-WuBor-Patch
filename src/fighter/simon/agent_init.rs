use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::fighter::belmont::agent_init::*,
    wubor_utils::table_const::*
};

#[event(start)]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_SIMON {
            return;
        }
        fighter.global_table[CHECK_GROUND_SPECIAL_UNIQ].assign(&L2CValue::Bool(false));
        fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&L2CValue::Bool(false));
        fighter.global_table[CHECK_GROUND_ATTACK_UNIQ].assign(&L2CValue::Bool(false));
        fighter.global_table[CHECK_AIR_ITEM_THROW_UNIQ].assign(&L2CValue::Bool(false));
        fighter.global_table[GUARD_CONT_UNIQ].assign(&L2CValue::Ptr(belmont_guard_cont_pre as *const () as _));
    }
}

pub fn install() {
    agent_reset::install();
}
