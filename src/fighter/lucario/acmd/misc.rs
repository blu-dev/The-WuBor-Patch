use crate::imports::acmd_imports::*;

// Temporary, for debug purposes
#[acmd("lucario", [ "game_appealhil", "game_appealhir" ])]
unsafe fn lucario_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
        if macros::is_excute(agent) {
            VarModule::inc_int(agent.battle_object, lucario::instance::int::AURA_LEVEL);
        }
    }
    else {
        if macros::is_excute(agent) {
            VarModule::set_int(agent.battle_object, lucario::instance::int::AURA_LEVEL, 0);
        }
    }
}

pub fn install() {
    lucario_appealhi::install();
}