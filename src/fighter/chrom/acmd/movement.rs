use crate::imports::acmd_imports::*;

#[acmd_script( agent = "chrom", script = "game_dash", category = ACMD_GAME, low_priority )]
unsafe fn chrom_dash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

#[acmd_script( agent = "chrom", script = "game_turndash", category = ACMD_GAME, low_priority )]
unsafe fn chrom_turndash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

pub fn install() {
    install_acmd_scripts!(
        chrom_dash,
        chrom_turndash
    );
}