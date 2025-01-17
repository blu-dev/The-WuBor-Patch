use crate::imports::acmd_imports::*;

#[acmd("marth", "game_speciallwspecials")]
unsafe fn marth_speciallwspecials(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.battle_object, marth::instance::flag::PARRY_XLU) {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
            VarModule::off_flag(agent.battle_object, marth::instance::flag::PARRY_XLU);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, marth::status::flag::SPECIAL_S_ENABLE_SPECIALS);
    }
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, marth::status::flag::SPECIAL_S_DASH);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 27.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        VarModule::on_flag(agent.battle_object, marth::status::flag::SPECIAL_S_END);
    }
    macros::FT_MOTION_RATE(agent, 0.5);
}

#[acmd("marth", "effect_speciallwspecials")]
unsafe fn marth_speciallwspecials_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, 0, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.6, 1);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, 0, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.6, 1);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, 0, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.6, 1);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, 0, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.6, 1);
    }
    frame(agent.lua_state_agent, 30.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 3.0);
    }
}

#[acmd("marth", "sound_speciallwspecials")]
unsafe fn marth_speciallwspecials_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_marth_rnd_attack"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_escape"));
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_dash_stop"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_marth_dash_stop"), 20);
    }
}

#[acmd("marth", "expression_speciallwspecials")]
unsafe fn marth_speciallwspecials_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_dash"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        )
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_dash"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        )
    }
}

#[acmd("marth", "game_speciallwspecialairs")]
unsafe fn marth_speciallwspecialairs(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.battle_object, marth::instance::flag::PARRY_XLU) {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
            VarModule::off_flag(agent.battle_object, marth::instance::flag::PARRY_XLU);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, marth::status::flag::SPECIAL_S_ENABLE_SPECIALS);
    }
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, marth::status::flag::SPECIAL_S_DASH);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 27.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        VarModule::on_flag(agent.battle_object, marth::status::flag::SPECIAL_S_END);
    }
    macros::FT_MOTION_RATE(agent, 0.5);
}

#[acmd("marth", "effect_speciallwspecialairs")]
unsafe fn marth_speciallwspecialairs_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        let angle = VarModule::get_float(agent.battle_object, marth::status::float::SPECIAL_S_ANGLE);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, angle, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.6, 1);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        let angle = VarModule::get_float(agent.battle_object, marth::status::float::SPECIAL_S_ANGLE);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, angle, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.6, 1);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        let angle = VarModule::get_float(agent.battle_object, marth::status::float::SPECIAL_S_ANGLE);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, angle, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.6, 1);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        let angle = VarModule::get_float(agent.battle_object, marth::status::float::SPECIAL_S_ANGLE);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, angle, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.6, 1);
    }
}

#[acmd("marth", "sound_speciallwspecialairs")]
unsafe fn marth_speciallwspecialairs_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_marth_rnd_attack"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_escape"));
    }
}

#[acmd("marth", "expression_speciallwspecialairs")]
unsafe fn marth_speciallwspecialairs_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_dash"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        )
    }
}

#[acmd("marth", "game_speciallwspecials2start")]
unsafe fn marth_speciallwspecials2start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.8);
}

#[acmd("marth", "effect_speciallwspecials2start")]
unsafe fn marth_speciallwspecials2start_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_marth_stance_sword1"), Hash40::new("tex_marth_sword2"), 10, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd("marth", "sound_speciallwspecials2start")]
unsafe fn marth_speciallwspecials2start_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_swing_l"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_special_n03"));
    }
}

#[acmd("marth", "expression_speciallwspecials2start")]
unsafe fn marth_speciallwspecials2start_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd("marth", "game_speciallwspecials2loop")]
unsafe fn marth_speciallwspecials2loop(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 0, 12, 60, 20, 2.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(15.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 0, 12, 100, 60, 2.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 30, 12, 100, 60, 2.0, 0.0, 4.0, 6.0, Some(0.0), Some(4.0), Some(20.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.0, 350, 12, 100, 60, 2.0, 0.0, 13.0, 6.0, Some(0.0), Some(13.0), Some(20.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 3.6);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 0, 12, 60, 20, 2.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(15.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 0, 12, 100, 60, 2.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(6.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 30, 12, 100, 60, 2.0, 0.0, 4.0, 6.0, Some(0.0), Some(4.0), Some(20.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.0, 350, 12, 100, 60, 2.0, 0.0, 13.0, 6.0, Some(0.0), Some(13.0), Some(20.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 3.6);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 0, 12, 60, 20, 2.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(15.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 0, 12, 100, 60, 2.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(6.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 30, 12, 100, 60, 2.0, 0.0, 4.0, 6.0, Some(0.0), Some(4.0), Some(20.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.0, 350, 12, 100, 60, 2.0, 0.0, 13.0, 6.0, Some(0.0), Some(13.0), Some(20.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        let loop_count = VarModule::get_int(agent.battle_object, marth::status::int::SPECIAL_S2_LOOP_COUNT) - 1;
        let setoff_mul = 10.0 - (loop_count as f32 * 2.5);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, setoff_mul);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, setoff_mul);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, setoff_mul);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 3, setoff_mul);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd("marth", "effect_speciallwspecials2loop")]
unsafe fn marth_speciallwspecials2loop_eff(agent: &mut L2CAgentBase) {
    if VarModule::get_int(agent.battle_object, marth::status::int::SPECIAL_S2_LOOP_COUNT) == 1 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 8.0, 10, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 3.0, 5, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 14.0, 7, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

#[acmd("marth", "sound_speciallwspecials2loop")]
unsafe fn marth_speciallwspecials2loop_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_swing_s"));
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_marth_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_marth_swing_s"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_swing_s"));
    }
}

#[acmd("marth", "expression_speciallwspecials2loop")]
unsafe fn marth_speciallwspecials2loop_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd("marth", "game_speciallwspecials2end2")]
unsafe fn marth_speciallwspecials2end2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 50, 0, 90, 3.0, 0.0, 7.5, 20.0, Some(0.0), Some(7.5), Some(6.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 15, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_FINAL, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd("marth", "effect_speciallwspecials2end2")]
unsafe fn marth_speciallwspecials2end2_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -0.0, 9.0, -10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_marth_stance_sword1"), Hash40::new("tex_marth_sword2"), 10, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 8.0, 10, 0, 0, 0, 1.0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd("marth", "sound_speciallwspecials2end2")]
unsafe fn marth_speciallwspecials2end2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_marth_rnd_special_l"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_swing_l"));
    }
}

#[acmd("marth", "expression_speciallwspecials2end2")]
unsafe fn marth_speciallwspecials2end2_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slash_critical"), 0);
    }
}

#[acmd("marth", "game_speciallwspecialairs2start")]
unsafe fn marth_speciallwspecialairs2start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.8);
}

#[acmd("marth", "effect_speciallwspecialairs2start")]
unsafe fn marth_speciallwspecialairs2start_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_marth_stance_sword1"), Hash40::new("tex_marth_sword2"), 10, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd("marth", "sound_speciallwspecialairs2start")]
unsafe fn marth_speciallwspecialairs2start_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_swing_l"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_special_n03"));
    }
}

#[acmd("marth", "expression_speciallwspecialairs2start")]
unsafe fn marth_speciallwspecialairs2start_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd("marth", "game_speciallwspecialairs2loop")]
unsafe fn marth_speciallwspecialairs2loop(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 30, 20, 60, 4.5, 0.0, 0.0, 12.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 305, 30, 60, 60, 4.5, 0.0, 0.0, 12.0, Some(0.0), Some(8.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 3.6);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 30, 20, 60, 4.5, 0.0, 0.0, 12.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 305, 30, 60, 60, 4.5, 0.0, 0.0, 12.0, Some(0.0), Some(8.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 3.6);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 30, 20, 60, 4.5, 0.0, 0.0, 12.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 305, 30, 60, 60, 4.5, 0.0, 0.0, 12.0, Some(0.0), Some(8.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3.6);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 3.6);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd("marth", "effect_speciallwspecialairs2loop")]
unsafe fn marth_speciallwspecialairs2loop_eff(agent: &mut L2CAgentBase) {
    if VarModule::get_int(agent.battle_object, marth::status::int::SPECIAL_S2_LOOP_COUNT) == 1 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 8.0, 10, 50, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 3.0, 5, 50, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 14.0, 7, 50, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

#[acmd("marth", "sound_speciallwspecialairs2loop")]
unsafe fn marth_speciallwspecialairs2loop_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_swing_s"));
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_marth_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_marth_swing_s"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_swing_s"));
    }
}

#[acmd("marth", "expression_speciallwspecialairs2loop")]
unsafe fn marth_speciallwspecialairs2loop_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd("marth", "game_speciallwspecialairs2end")]
unsafe fn marth_speciallwspecialairs2end(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 30, 12, 0, 65, 4.0, 0.0, 0.0, 12.0, Some(0.0), Some(8.5), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -4, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd("marth", "effect_speciallwspecialairs2end")]
unsafe fn marth_speciallwspecialairs2end_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 8.0, 10, 50, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

#[acmd("marth", "sound_speciallwspecialairs2end")]
unsafe fn marth_speciallwspecialairs2end_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_swing_m"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_marth_rnd_attack"));
    }
}

#[acmd("marth", "expression_speciallwspecialairs2end")]
unsafe fn marth_speciallwspecialairs2end_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd("marth", "game_speciallwspecialhi")]
unsafe fn marth_speciallwspecialhi(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.battle_object, marth::instance::flag::PARRY_XLU) {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
            VarModule::off_flag(agent.battle_object, marth::instance::flag::PARRY_XLU);
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    macros::FT_MOTION_RATE(agent, 8.0 / 13.0);
    frame(agent.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 80, 30, 0, 120, 4.0, 0.0, 20.0, 12.0, Some(0.0), Some(7.5), Some(8.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 80, 30, 0, 120, 4.0, 0.0, 20.0, 12.0, Some(0.0), Some(7.5), Some(12.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 80, 30, 0, 110, 3.5, 0.0, 20.0, 12.0, Some(0.0), Some(7.5), Some(8.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 28.0);
    macros::FT_MOTION_RATE(agent, 1.5);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 37.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE);
    }
}

#[acmd("marth", "effect_speciallwspecialhi")]
unsafe fn marth_speciallwspecialhi_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_dolphin_jump"), Hash40::new("top"), -0.0, 0, -5, 0, 0, 0, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_marth_stance_sword1"), Hash40::new("tex_marth_sword2"), 12, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("marth_dolphin_jump"), -1);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("marth_dolphin_jump"), -1);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

#[acmd("marth", "sound_speciallwspecialhi")]
unsafe fn marth_speciallwspecialhi_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_special_h01"));
    }
}

#[acmd("marth", "expression_speciallwspecialhi")]
unsafe fn marth_speciallwspecialhi_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

pub fn install() {
    marth_speciallwspecials::install();
    marth_speciallwspecials_eff::install();
    marth_speciallwspecials_snd::install();
    marth_speciallwspecials_exp::install();
    marth_speciallwspecialairs::install();
    marth_speciallwspecialairs_eff::install();
    marth_speciallwspecialairs_snd::install();
    marth_speciallwspecialairs_exp::install();
    marth_speciallwspecials2start::install();
    marth_speciallwspecials2start_eff::install();
    marth_speciallwspecials2start_snd::install();
    marth_speciallwspecials2start_exp::install();
    marth_speciallwspecials2loop::install();
    marth_speciallwspecials2loop_eff::install();
    marth_speciallwspecials2loop_snd::install();
    marth_speciallwspecials2loop_exp::install();
    marth_speciallwspecials2end2::install();
    marth_speciallwspecials2end2_eff::install();
    marth_speciallwspecials2end2_snd::install();
    marth_speciallwspecials2end2_exp::install();
    marth_speciallwspecialairs2start::install();
    marth_speciallwspecialairs2start_eff::install();
    marth_speciallwspecialairs2start_snd::install();
    marth_speciallwspecialairs2start_exp::install();
    marth_speciallwspecialairs2loop::install();
    marth_speciallwspecialairs2loop_eff::install();
    marth_speciallwspecialairs2loop_snd::install();
    marth_speciallwspecialairs2loop_exp::install();
    marth_speciallwspecialairs2end::install();
    marth_speciallwspecialairs2end_eff::install();
    marth_speciallwspecialairs2end_snd::install();
    marth_speciallwspecialairs2end_exp::install();
    marth_speciallwspecialhi::install();
    marth_speciallwspecialhi_eff::install();
    marth_speciallwspecialhi_snd::install();
    marth_speciallwspecialhi_exp::install();
}