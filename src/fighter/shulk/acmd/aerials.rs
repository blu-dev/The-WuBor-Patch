use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "shulk", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn shulk_attackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("swordr"), 12.5, 361, 100, 0, 35, 5.5, 10.0, 0.0, 1.0, Some(2.0), Some(0.0), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("swordr"), 8.5, 361, 100, 0, 35, 4.0, 24.0, 0.0, 1.0, Some(2.0), Some(0.0), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "shulk", script = "effect_attackairb", category = ACMD_EFFECT, low_priority )]
unsafe fn shulk_attackairb_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("shulk_monad_sword2"), Hash40::new("haver"), 0, 2.5, 0, 0, 0, 0, 0.95, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("shulk_monad_circle"), Hash40::new("swordr"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("shulk_monad_sword2"), false, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("shulk_monad_sword3"), Hash40::new("haver"), 0, 1, 0, 0, 0, 0, 0.85, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("shulk_monad_sword2"), false, false);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("shulk_monad_sword3_end"), Hash40::new("haver"), 0, 2.5, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("shulk_monad_sword3"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("shulk_monad_circle"), false, false);
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("shulk_monad_sword3_end"), false, false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        shulk_attackairb, shulk_attackairb_eff
    );
}
