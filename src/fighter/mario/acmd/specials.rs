use crate::imports::acmd_imports::*;
use super::super::vl;

#[acmd("mario", [ "game_specials", "game_specialairs" ])]
unsafe fn mario_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, 9.0, *FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP)
        && StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_AIR {
            macros::SET_SPEED_EX(
                agent,
                0.0,
                vl::param_special_s::spin_hop_speed_y,
                *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN
            );
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 65, 0, 65, 4.0, 0.0, 9.0, 10.0, Some(0.0), Some(5.0), Some(10.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 361, 50, 0, 65, 3.0, 0.0, 9.0, 7.0, Some(0.0), Some(9.0), Some(-10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 65, 3.0, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(-10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd("mario", "effect_specials")]
unsafe fn mario_specials_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mario_supermant_wind_r"), Hash40::new("mario_supermant_wind_l"), Hash40::new("top"), 2.5, 5, -2.0, 22, 0, 0, 1.2, true, *EF_FLIP_NONE);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.75);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
    }
}

#[acmd("mario", "effect_specialairs")]
unsafe fn mario_specialairs_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mario_supermant_wind_r"), Hash40::new("mario_supermant_wind_l"), Hash40::new("top"), 2.5, 5, -2.0, 22, 0, 0, 1.2, true, *EF_FLIP_NONE);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.75);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
    }
}

#[acmd("mario", [ "expression_specials", "expression_specialairs" ])]
unsafe fn expression_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 1, 0, 300, 0.5, 12, 10, 30, 20, 50);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

#[acmd("mario", [ "game_specialhi", "game_specialairhi" ])]
unsafe fn mario_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        let mut effect = Hash40::new("collision_attr_coin");
        let mut sound = *COLLISION_SOUND_ATTR_COIN;
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY) {
            effect.hash = hash40("collision_attr_mario_local_coin");
            sound = *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 100, 160, 0, 2.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 86, 100, 150, 0, 4.0, 0.0, 6.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 100, 100, 150, 0, 4.0, 0.0, 6.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        let mut effect = Hash40::new("collision_attr_coin");
        let mut sound = *COLLISION_SOUND_ATTR_COIN;
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY) {
            effect.hash = hash40("collision_attr_mario_local_coin");
            sound = *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 6.5, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.6, 92, 100, 170, 0, 3.8, 0.0, 6.5, 8.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.6, 60, 100, 110, 0, 3.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.6, 92, 100, 110, 0, 3.0, 0.0, 11.5, 8.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 2, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 3, true, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        let mut effect = Hash40::new("collision_attr_coin");
        let mut sound = *COLLISION_SOUND_ATTR_MARIO_COIN_LAST;
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY) {
            effect.hash = hash40("collision_attr_mario_local_coin");
            sound = *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 9.0, 0.0, 11.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 9.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd("mario", "sound_speciallwstart")]
unsafe fn mario_longjumpstart_snd(_agent: &mut L2CAgentBase) {
}

#[acmd("mario", "game_speciallwjump")]
unsafe fn mario_longjump(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        if VarModule::get_int(agent.battle_object, mario::status::int::SPECIAL_LW_LONG_JUMP_KIND) == mario::LONG_JUMP_B {
            VarModule::on_flag(agent.battle_object, mario::status::flag::SPECIAL_LW_LANDING);
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, mario::status::flag::SPECIAL_LW_LANDING);
        if [mario::LONG_JUMP_B, mario::LONG_JUMP_M].contains(&VarModule::get_int(agent.battle_object, mario::status::int::SPECIAL_LW_LONG_JUMP_KIND)) {
            CancelModule::enable_cancel(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        if VarModule::get_int(agent.battle_object, mario::status::int::SPECIAL_LW_LONG_JUMP_KIND) == mario::LONG_JUMP_W {
            CancelModule::enable_cancel(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        if VarModule::get_int(agent.battle_object, mario::status::int::SPECIAL_LW_LONG_JUMP_KIND) == mario::LONG_JUMP_S {
            CancelModule::enable_cancel(agent.module_accessor);
        }
    }
}

#[acmd("mario", "sound_speciallwjump")]
unsafe fn mario_longjump_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_mario_009"));
    }
}

#[acmd("mario", "expression_speciallwjump")]
unsafe fn mario_longjump_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd("mario", "sound_speciallwlanding")]
unsafe fn mario_longjumpland_snd(_agent: &mut L2CAgentBase) {
}

#[acmd("mario", "effect_speciallwlanding")]
unsafe fn mario_longjumpland_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd("mario", "expression_speciallwlanding")]
unsafe fn mario_longjumpland_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd("mario", "game_specialairlwstart")]
unsafe fn mario_groundpoundstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        KineticModule::unable_energy_all(agent.module_accessor);
    }
}

#[acmd("mario", "sound_specialairlwstart")]
unsafe fn mario_groundpoundstart_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mario_special_l04"));
    }
}

#[acmd("mario", "game_specialairlwfall")]
unsafe fn mario_groundpoundfall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 30, 50, 0, 60, 5.0, 0.0, 2.8, -2.0, Some(0.0), Some(2.8), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
    }
}

#[acmd("mario", "effect_specialairlwfall")]
unsafe fn mario_groundpoundfall_eff(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 1, -90, 0, 0, 1, true);
        }
        wait(agent.lua_state_agent, 3.0);
    }
}

#[acmd("mario", "expression_specialairlwfall")]
unsafe fn mario_groundpoundfall_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd("mario", "game_specialairlwlanding")]
unsafe fn mario_groundpoundland(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 30.0 / 25.0);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 30, 50, 0, 60, 5.0, 0.0, 2.8, -10.0, Some(0.0), Some(2.8), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd("mario", "sound_specialairlwlanding")]
unsafe fn mario_groundpoundland_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mario_special_l03"));
    }
}

#[acmd("mario", "effect_specialairlwlanding")]
unsafe fn mario_groundpoundland_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.7, 0, 0, 0, 0, 0, 0, false)
    }
}

#[acmd("mario", "expression_specialairlwlanding")]
unsafe fn mario_groundpoundland_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

pub fn install() {
    mario_specials::install();
    mario_specials_eff::install();
    mario_specialairs_eff::install();
    expression_specials::install();
    mario_specialhi::install();
    mario_longjumpstart_snd::install();
    mario_longjump::install();
    mario_longjump_snd::install();
    mario_longjump_exp::install();
    mario_longjumpland_snd::install();
    mario_longjumpland_eff::install();
    mario_longjumpland_exp::install();
    mario_groundpoundstart::install();
    mario_groundpoundstart_snd::install();
    mario_groundpoundfall::install();
    mario_groundpoundfall_eff::install();
    mario_groundpoundfall_exp::install();
    mario_groundpoundland::install();
    mario_groundpoundland_snd::install();
    mario_groundpoundland_eff::install();
    mario_groundpoundland_exp::install();
}