use crate::imports::acmd_imports::*;

#[acmd("rockman", [ "game_busterchargeshot", "game_busterairchargeshot" ])]
unsafe fn rockman_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(agent, 1.0 / 7.0);
    frame(agent.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CHARGESHOT, false, -1);
        VarModule::off_flag(agent.battle_object, rockman::status::flag::CHARGE_SHOT_KEEP_CHARGE);
    }
}

#[acmd("rockman", [ "effect_busterchargeshot", "effect_busterairchargeshot" ])]
unsafe fn rockman_specialn_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("rockman_chargeshot_elec"), Hash40::new("havel"), 0, 0, -1.5, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("rockman_chargeshot_shot"), Hash40::new("rockman_chargeshot_shot"), Hash40::new("top"), 0, 7.2, 9, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd("rockman", [ "sound_busterchargeshot", "sound_busterairchargeshot" ])]
unsafe fn rockman_specialn_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_rockman_smash_s02"));
    }
}

#[acmd("rockman", [ "expression_busterchargeshot", "expression_busterairchargeshot" ])]
unsafe fn rockman_specialn_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_ROCKBUSTER, 5);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 18.0);
    if WorkModule::get_float(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_HOLD_RATE) < 1.0 {
        if macros::is_excute(agent) {
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beaml"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

#[acmd("rockman_chargeshot", "game_regular")]
unsafe fn rockman_chargeshot_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let is_charge_max = 1.0 <= WorkModule::get_float(agent.module_accessor, *WEAPON_ROCKMAN_CHARGESHOT_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
        let damage;
        let bkb;
        let kbg;
        if is_charge_max {
            damage = 15.0;
            bkb = 40;
            kbg = 90;
        }
        else {
            damage = 9.0;
            bkb = 50;
            kbg = 85;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 361, kbg, 0, bkb, 2.6, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.32);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

#[acmd("rockman", [ "game_specialhi", "game_specialairhi" ])]
unsafe fn rockman_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES); // Was ALWAYS_BOTH_SIDES
    }
}

#[acmd("rockman", "game_speciallw")]
unsafe fn rockman_speciallw(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 5.0);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, false, -1);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
}

#[acmd("rockman", "game_specialairlw")]
unsafe fn rockman_specialairlw(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 5.0);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, false, -1);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
}

#[acmd("rockman_leafshield", [ "game_start", "game_startreverse" ])]
unsafe fn rockman_leafshield_start(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 4.0 / 3.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, 0x20000006) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_0
            macros::ATTACK(agent, 0, 0, Hash40::new("leafshield1"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000007) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_1
            macros::ATTACK(agent, 1, 0, Hash40::new("leafshield2"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000008) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_2
            macros::ATTACK(agent, 2, 0, Hash40::new("leafshield3"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000009) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_3
            macros::ATTACK(agent, 3, 0, Hash40::new("leafshield4"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

#[acmd("rockman_leafshield", [ "game_shield", "game_shieldreverse" ])]
unsafe fn rockman_leafshield_shield(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 4.0 / 3.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, 0x20000006) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_0
            macros::ATTACK(agent, 0, 0, Hash40::new("leafshield1"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000007) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_1
            macros::ATTACK(agent, 1, 0, Hash40::new("leafshield2"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000008) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_2
            macros::ATTACK(agent, 2, 0, Hash40::new("leafshield3"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000009) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_3
            macros::ATTACK(agent, 3, 0, Hash40::new("leafshield4"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

#[acmd("rockman_leafshield", [ "game_fly", "game_flyreverse" ])]
unsafe fn rockman_leafshield_fly(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 4.0 / 3.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, 0x20000006) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_0
            macros::ATTACK(agent, 0, 0, Hash40::new("leafshield1"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000007) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_1
            macros::ATTACK(agent, 1, 0, Hash40::new("leafshield2"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000008) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_2
            macros::ATTACK(agent, 2, 0, Hash40::new("leafshield3"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000009) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_3
            macros::ATTACK(agent, 3, 0, Hash40::new("leafshield4"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

pub fn install() {
    rockman_specialn::install();
    rockman_specialn_eff::install();
    rockman_specialn_snd::install();
    rockman_specialn_exp::install();
    rockman_chargeshot_regular::install();
    rockman_specialhi::install();
    rockman_speciallw::install();
    rockman_specialairlw::install();
    rockman_leafshield_start::install();
    rockman_leafshield_shield::install();
    rockman_leafshield_fly::install();
}