use crate::imports::acmd_imports::*;

#[acmd("eflame", [ "game_specials", "game_specialairs", "game_specialsflick", "game_specialairsflick" ])]
unsafe fn eflame_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    if macros::is_excute(agent) {
        WorkModule::set_int64(agent.module_accessor, hash40("special_s") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        // No hitbox
    }
    // frame 16 clears the old hitbox
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

#[acmd("eflame_esword", [ "game_flyl", "game_flyr" ])]
unsafe fn eflame_esword_fly(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 0.75);
        VarModule::on_flag(agent.battle_object, eflame_esword::status::flag::ENABLE_EARLY_SPIN);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        // Exists so it can be reflected
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -1.5, -3.0, Some(0.0), Some(-1.5), Some(2.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.battle_object, eflame_esword::status::flag::ENABLE_EARLY_SPIN);
    }
}

#[acmd("eflame_esword", [ "game_flyflickl", "game_flyflickr" ])]
unsafe fn eflame_esword_flyflick(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 0.75);
        VarModule::on_flag(agent.battle_object, eflame_esword::status::flag::ENABLE_EARLY_SPIN);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        // Exists so it can be reflected
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -1.5, -3.0, Some(0.0), Some(-1.5), Some(2.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.battle_object, eflame_esword::status::flag::ENABLE_EARLY_SPIN);
    }
}

#[acmd("eflame_esword", "game_rotate")]
unsafe fn eflame_esword_rotate(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::disable_tip(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 0.8);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            7,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 0.8, 0, 60, 20, 10, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 0.8, 90, 70, 20, 10, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 0.8, 190, 60, 20, 5, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("sword1"), 0.8, 120, 70, 20, 5, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("sword1"), 0.8, 190, 60, 20, 10, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 6, 0, Hash40::new("sword1"), 0.8, 120, 70, 30, 10, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 0, 1, Hash40::new("top"), 1.5, 90, 100, 10, 20, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        for x in 0..7 {
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, x, 25.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, x as u64, 5.0);
        }
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 367, 100, 100, 50, 2.5, 0.0, 2.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -3, -1.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            7,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            7,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            7,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            7,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
        MotionModule::set_rate(agent.module_accessor, 0.5);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 0.8);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 10.0, 65, 50, 0, 50, 3.5, 5.0, 0.0, 0.0, None, None, None, 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 10.0, 65, 50, 0, 50, 3.5, 9.0, 0.0, 0.0, None, None, None, 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 10.0, 65, 50, 0, 50, 3.5, 11.0, 0.0, 0.0, None, None, None, 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
        WorkModule::on_flag(agent.module_accessor, *WEAPON_EFLAME_ESWORD_STATUS_SPECIAL_S_FLAG_FINISH);
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 0.5);
    }
}

#[acmd("eflame_esword", "effect_rotate")]
unsafe fn eflame_esword_rotate_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_blazeend_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_blazeend_trail"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_blazeend_trail"), false, true);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_blazeend_trail_last_back"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_blazeend_trail_last"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_blazeend_trail_last_spin"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_blazeend_trail_last_back"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_blazeend_trail_last_spin"), false, true);
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_blazeend_sword"), false, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("eflame_blazeend_trail_last"), -1);
    }
}

#[acmd("eflame_esword", [ "game_reflectedl", "game_reflectedr" ])]
unsafe fn eflame_esword_reflected(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.5);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        // Exists so it can be reflected
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -1.5, -3.0, Some(0.0), Some(-1.5), Some(2.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, -1.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
    }
}

#[acmd("eflame", "game_specialairhijump")]
unsafe fn eflame_specialairhijump(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 85, 100, 64, 16, 2.5, 0.0, 18.0, 5.0, Some(0.0), Some(2.5), Some(5.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 95, 100, 64, 16, 4.0, 0.0, 15.0, 11.0, Some(0.0), Some(4.0), Some(11.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 100, 100, 64, 16, 4.0, 0.0, 14.0, 15.0, Some(0.0), Some(4.0), Some(15.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 10.0, false);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 95, 100, 53, 20, 2.5, 0.0, 21.5, 5.0, Some(0.0), Some(15.0), Some(5.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 100, 100, 53, 20, 4.0, 0.0, 20.0, 11.0, Some(0.0), Some(14.0), Some(11.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 100, 100, 53, 20, 4.0, 0.0, 20.0, 15.0, Some(0.0), Some(14.0), Some(15.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 10.0, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_START_CONTROL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 95, 100, 50, 20, 2.5, 0.0, 23.0, 5.0, Some(0.0), Some(19.0), Some(5.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 100, 100, 50, 20, 4.0, 0.0, 24.0, 11.0, Some(0.0), Some(20.0), Some(11.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 100, 100, 50, 20, 4.0, 0.0, 24.0, 15.0, Some(0.0), Some(20.0), Some(15.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 10.0, false);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP); // Was ALWAYS
    }
    frame(agent.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 27.0);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    if macros::is_excute(agent) {
        WorkModule::set_int64(agent.module_accessor, hash40("special_air_hi_jump") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
    }
    // frame(agent.lua_state_agent, 31.0);
    // if macros::is_excute(agent) {
    //     notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    // }
    frame(agent.lua_state_agent, 32.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.0, y: -7.0, z: 0.0});
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_END_CONTROL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 280, 130, 39, 0, 6.0, 0.0, 3.0, 9.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_ENABLE_GROUND);
    }
}

#[acmd("eflame", [ "game_speciallwattack", "game_specialairlwattack" ])]
unsafe fn eflame_speciallwattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.5, 60, 78, 10, 40, 3.5, 0.0, 4.0, 17.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.5, 50, 78, 10, 40, 4.5, 0.0, 8.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.5, 40, 78, 10, 40, 5.5, 0.0, 12.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.5);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.5);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 1.5);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 40, 78, 0, 60, 3.5, 0.0, 4.0, 20.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 40, 78, 0, 60, 4.5, 0.0, 7.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 40, 78, 0, 60, 5.5, 0.0, 10.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 60.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, element::status::flag::SPECIAL_LW_OUT_ATTACK_FALL);
    }
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 1.0, 1.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
}

#[acmd("eflame", [ "effect_speciallwattack", "effect_specialairlwattack" ])]
unsafe fn eflame_speciallwattack_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_change_end"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.3, true);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_core_start"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), true, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -3);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword1"), Hash40::new("tex_eflame_sword2"), 5, Hash40::new("sword1"), 0.3, 0.0, 0.0, Hash40::new("sword1"), 18.5, 0.0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), -0.5, 0, 0, 0, 0, 0, 0.9, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 1.5, 0, 0, -5, 0, 0, 0.5, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        macros::AFTER_IMAGE_OFF(agent, 5);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire2"), false, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 0.4, true);
        macros::LAST_EFFECT_SET_RATE(agent, 4);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), true, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -3);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword1"), Hash40::new("tex_eflame_sword2"), 5, Hash40::new("sword1"), 0.3, 0.0, 0.0, Hash40::new("sword1"), 18.5, 0.0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), -0.5, 0, 0, 0, 0, 0, 0.9, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 1.5, 0, 0, -5, 0, 0, 0.5, true);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        macros::AFTER_IMAGE_OFF(agent, 7);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire2"), false, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 0.4, true);
        macros::LAST_EFFECT_SET_RATE(agent, 4);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_m"), true, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_core_start"), false, true);
    }
}

#[acmd("eflame", "sound_speciallwattack")]
unsafe fn eflame_speciallwattack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_eflame_attack04"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_eflame_swing_m01"));
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_eflame_rnd_special_l02"));
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_eflame_swing_l01"));
    }
    frame(agent.lua_state_agent, 107.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_eflame_step_right_m"));
    }
}

#[acmd("eflame", "sound_specialairlwattack")]
unsafe fn eflame_specialairlwattack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_eflame_attack04"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_eflame_swing_m01"));
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_eflame_rnd_special_l02"));
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_eflame_swing_l01"));
    }
}

#[acmd("eflame", [ "expression_speciallwattack", "expression_specialairlwattack" ])]
unsafe fn eflame_speciallwattack_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_79_changef"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_79_flameslashl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_79_flamesmash"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
    }
}

pub fn install() {
    eflame_specials::install();
    eflame_esword_fly::install();
    eflame_esword_flyflick::install();
    eflame_esword_rotate::install();
    eflame_esword_rotate_eff::install();
    eflame_esword_reflected::install();
    eflame_specialairhijump::install();
    eflame_speciallwattack::install();
    eflame_speciallwattack_eff::install();
    eflame_speciallwattack_snd::install();
    eflame_specialairlwattack_snd::install();
    eflame_speciallwattack_exp::install();
}
