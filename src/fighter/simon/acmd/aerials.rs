use crate::imports::acmd_imports::*;

#[acmd("simon", "game_attackairf")]
unsafe fn simon_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 2.3, 0.0, 10.5, 7.0, Some(0.0), Some(10.5), Some(43.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            FighterSpecializer_Simon::set_whip_reflect_attack_off_id(
                object as *mut Fighter,
                0,
                1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1
            );
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 361, 55, 0, 80, 2.3, 0.0, 10.5, 43.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 361, 55, 0, 80, 2.3, 0.0, 10.5, 10.0, Some(0.0), Some(10.5), Some(43.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 55, 0, 80, 5.5, 0.0, 10.5, 8.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        search!(agent, MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd("simon", "game_attackairfhi")]
unsafe fn simon_attackairfhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 2.3, 0.0, 16.0, 9.0, Some(0.0), Some(31.0), Some(39.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            FighterSpecializer_Simon::set_whip_reflect_attack_off_id(
                object as *mut Fighter,
                0,
                1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1
            );
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 361, 55, 0, 80, 2.3, 0.0, 31.0, 39.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 361, 55, 0, 80, 2.3, 0.0, 16.0, 9.0, Some(0.0), Some(31.0), Some(39.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 55, 0, 80, 5.5, 0.0, 16.0, 7.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        search!(agent, MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd("simon", "game_attackairflw")]
unsafe fn simon_attackairflw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 2.3, 0.0, 9.0, 11.0, Some(0.0), Some(-2.0), Some(41.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            FighterSpecializer_Simon::set_whip_reflect_attack_off_id(
                object as *mut Fighter,
                0,
                1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1
            );
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 361, 55, 0, 80, 2.3, 0.0, -2.0, 41.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 361, 55, 0, 80, 2.3, 0.0, 9.0, 11.0, Some(0.0), Some(-2.0), Some(41.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 55, 0, 80, 5.5, 0.0, 9.0, 9.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        search!(agent, MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd("simon", [ "sound_attackairf", "sound_attackairfhi", "sound_attackairflw" ])]
unsafe fn simon_attackairf_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_simon_whip_holding"));
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_simon_attackair_b01"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_simon_attackair_b01"));
        macros::PLAY_SE(agent, Hash40::new("se_simon_attackair_b02"));
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_simon_rnd_attack"));
    }
}

#[acmd("simon", "game_attackairhi")]
unsafe fn simon_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 2.5, 0.0, 15.0, 2.5, Some(0.0), Some(53.0), Some(2.5), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
        macros::SEARCH(agent, 1, 0, Hash40::new("top"), 1.0, 0.0, 15.0, 2.5, Some(0.0), Some(48.0), Some(2.5), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            FighterSpecializer_Simon::set_whip_reflect_attack_off_id(
                object as *mut Fighter,
                0,
                1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1
            );
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 90, 75, 0, 60, 2.5, 0.0, 53.0, 2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 90, 75, 0, 60, 2.5, 0.0, 17.0, 2.5, Some(0.0), Some(53.0), Some(2.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 90, 75, 0, 60, 5.5, 0.0, 19.0, 2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        search!(agent, MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd("simon", "game_attackairlw")]
unsafe fn simon_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 2.5, 0.0, 0.0, 4.5, Some(0.0), Some(-27.0), Some(4.5), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
        macros::SEARCH(agent, 1, 0, Hash40::new("top"), 1.0, 0.0, 0.0, 4.5, Some(0.0), Some(-24.0), Some(4.5), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            FighterSpecializer_Simon::set_whip_reflect_attack_off_id(
                object as *mut Fighter,
                0,
                1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1
            );
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 270, 98, 0, 14, 2.5, 0.0, -27.0, 4.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 270, 64, 0, 16, 2.5, 0.0, 8.0, 4.5, Some(0.0), Some(-27.0), Some(4.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.5, 270, 32, 0, 18, 5.5, 0.0, 6.0, 4.5, Some(0.0), Some(5.0), Some(4.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        search!(agent, MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd("simon", "effect_attackairlw")]
unsafe fn simon_attackairlw_eff(_agent: &mut L2CAgentBase) {
    
}

#[acmd("simon", "sound_attackairlw")]
unsafe fn simon_attackairlw_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_simon_whip_holding"));
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_simon_attackair_h01"));
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_simon_rnd_attack"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_simon_attackair_h01"));
        macros::PLAY_SE(agent, Hash40::new("se_simon_attackair_h02"));
    }
}

#[acmd("simon", "expression_attackairlw")]
unsafe fn simon_attackairlw_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd("simon_whip", "game_attackairlw")]
unsafe fn simon_whip_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_NONE);
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(
                object as *mut smash::app::Weapon
            );
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_COLLIDE);
    }
}

#[acmd("simon_whip", "effect_attackairlw")]
unsafe fn simon_whip_attackairlw_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::set_chain_2_visibility(
                object as *mut smash::app::Weapon,
                true
            );
        }
        macros::EFFECT_FOLLOW(agent, Hash40::new("simon_whip_straight"), Hash40::new("hookshot3"), -6, 0, 0, 180, 50, 90, 1.1, true);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("simon_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1, true, 0.65);
        macros::EFFECT_FOLLOW(agent, Hash40::new("simon_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("simon_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::set_chain_2_visibility(
                object as *mut smash::app::Weapon,
                false
            );
        }
    }
}

#[acmd("simon", "effect_landingairlw")]
unsafe fn simon_landingairlw_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd("simon", "sound_landingairlw")]
unsafe fn simon_landingairlw_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_simon_landing02"));
    }
}

#[acmd("simon", "expression_landingairlw")]
unsafe fn simon_landingairlw_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_lands"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd("simon_whip", "game_landingairlw")]
unsafe fn simon_whip_landingairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_MOVE);
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(
                object as *mut smash::app::Weapon
            );
        }
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_NONE);
    }
}

pub fn install() {
    simon_attackairf::install();
    simon_attackairfhi::install();
    simon_attackairflw::install();
    simon_attackairf_snd::install();
    simon_attackairhi::install();
    simon_attackairlw::install();
    simon_attackairlw_eff::install();
    simon_attackairlw_snd::install();
    simon_attackairlw_exp::install();
    simon_whip_attackairlw::install();
    simon_whip_attackairlw_eff::install();
    simon_landingairlw_eff::install();
    simon_landingairlw_snd::install();
    simon_landingairlw_exp::install();
    simon_whip_landingairlw::install();
}