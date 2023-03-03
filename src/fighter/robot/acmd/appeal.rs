use crate::imports::acmd_imports::*;

#[acmd_script( agent = "robot", scripts = [ "game_appealhil", "game_appealhir" ], category = ACMD_GAME, low_priority )]
unsafe fn robot_appealhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 38.0);
    let hold_button = VarModule::get_int(fighter.battle_object, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(fighter.module_accessor, hold_button) {
        if macros::is_excute(fighter) {
            MiscModule::set_appeal_loop(
                fighter.battle_object,
                true,
                hash40("appeal_hi_loop"),
                63
            );
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        robot_appealhi
    );
}