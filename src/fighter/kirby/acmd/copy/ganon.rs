use crate::imports::acmd_imports::*;

#[acmd("kirby", ["effect_ganonspecialn", "effect_ganonspecialairn"])]
unsafe fn kirby_ganonspecialn_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        WorkModule::set_flag(agent.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ganon_entry"), Hash40::new("top"), 0, 6, -2, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.7);
        macros::FLASH(agent, 1, 0, 1, 1.0);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ganon_entry"), Hash40::new("top"), 0, 6.0, -2.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.7);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        WorkModule::set_flag(agent.module_accessor, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        VisibilityModule::set_whole(agent.module_accessor, true);
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(agent.module_accessor, true, 0);
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
}

#[acmd("kirby", ["sound_ganonspecialn", "sound_ganonspecialairn"])]
unsafe fn kirby_ganonspecialn_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_model_visible(agent.module_accessor, true);
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(agent.module_accessor, true, 0);
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
}

pub fn install() {
    kirby_ganonspecialn_eff::install();
    kirby_ganonspecialn_snd::install();
}