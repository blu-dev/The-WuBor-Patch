use crate::imports::acmd_imports::*;

#[acmd("robot", [ "game_appealhil", "game_appealhir" ])]
unsafe fn robot_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 38.0);
    let hold_button = VarModule::get_int(agent.battle_object, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, hold_button) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.battle_object,
                true,
                hash40("appeal_hi_loop"),
                63
            );
        }
    }
}

pub fn install() {
    robot_appealhi::install();
}