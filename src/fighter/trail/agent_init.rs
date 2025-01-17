use crate::imports::status_imports::*;

unsafe extern "C" fn trail_guard_cont_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        WorkModule::on_flag(fighter.module_accessor, 0x2100000C);
        let fighta = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
        FighterSpecializer_Trail::change_magic(fighta);
    }
    false.into()
}

#[event(initialize)]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_TRAIL {
            return;
        }
        fighter.global_table[GUARD_CONT_UNIQ].assign(&L2CValue::Ptr(trail_guard_cont_pre as *const () as _));
        VarModule::add_reset_statuses(
            fighter.battle_object_id,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            vec![
                *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N,
                *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F
            ]
        );
    }
}

pub fn install() {
    agent_init::install();
}
