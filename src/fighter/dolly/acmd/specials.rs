use crate::imports::acmd_imports::*;
use super::super::{helper::*, vl, vtable_hook::*};

#[acmd("dolly", "game_specialn")]
unsafe fn dolly_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        MotionModule::set_rate(agent.module_accessor, 6.0 / 7.0);
    }
    else {
        MotionModule::set_rate(agent.module_accessor, 3.0 / 2.0);
    }
    frame(agent.lua_state_agent, 12.0);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        VarModule::on_flag(agent.battle_object, dolly::status::flag::SPECIAL_N_FEINT);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, dolly::status::flag::SPECIAL_N_FEINT) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        }
        else {
            MotionModule::set_rate(agent.module_accessor, 1.5);
        }
    }
}

#[acmd("dolly", "game_specialairn")]
unsafe fn dolly_specialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 12.0);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        VarModule::on_flag(agent.battle_object, dolly::status::flag::SPECIAL_N_FEINT);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        if !VarModule::is_flag(agent.battle_object, dolly::status::flag::SPECIAL_N_FEINT) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        }
        else {
            KineticModule::add_speed(agent.module_accessor, &Vector3f {x: 0.0, y: vl::param_special_n::feint_speed_y_add, z: 0.0});
            MotionModule::set_rate(agent.module_accessor, 1.5);
        }
    }
}

#[acmd("dolly", "game_specialsfattack")]
unsafe fn dolly_specialsfattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_S_WORK_FLAG_AIR_ATTACK) {
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                MotionModule::set_rate(agent.module_accessor, 1.2);
            }
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.5);
        }
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) != *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 12.0, 65);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 40, 74, 0, output.bkb, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 10.0, 67);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 40, 74, 0, output.bkb, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
            macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        }
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) != *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 15.0, 65);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 40, 74, 0, output.bkb, 4.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 13.0, 67);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 40, 74, 0, output.bkb, 4.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            let output = dolly_calc_special_cancel(agent, 9.0, 50);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 40, 78, 0, output.bkb, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            let output = dolly_calc_special_cancel(agent, 12.0, 52);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 40, 78, 0, output.bkb, 4.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
}

#[acmd("dolly", "game_specialsbattackw")]
unsafe fn dolly_specialsbattackw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        let output1 = dolly_calc_special_cancel(agent, 8.0, 95);
        let output2 = dolly_calc_special_cancel(agent, 8.0, 75);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output1.dmg, 270, 10, 0, 95, 5.0, 0.0, 14.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), output2.dmg, 68, 10, 0, output2.bkb, 5.0, 0.0, 14.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output1.dmg, 270, 10, 0, 95, 5.0, 0.0, 5.0, 1.0, Some(0.0), Some(8.0), Some(1.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), output2.dmg, 68, 10, 0, output2.bkb, 5.0, 0.0, 5.0, 1.0, Some(0.0), Some(8.0), Some(1.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        let output1 = dolly_calc_special_cancel(agent, 8.0, 95);
        let output2 = dolly_calc_special_cancel(agent, 8.0, 75);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output1.dmg, 270, 10, 0, 95, 5.0, 0.0, 9.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), output2.dmg, 68, 10, 0, output2.bkb, 5.0, 0.0, 9.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        let output1 = dolly_calc_special_cancel(agent, 8.0, 95);
        let output2 = dolly_calc_special_cancel(agent, 8.0, 75);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output1.dmg, 270, 10, 0, 95, 5.0, 0.0, 6.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), output2.dmg, 68, 10, 0, output2.bkb, 5.0, 0.0, 6.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd("dolly", "game_specialsbattack")]
unsafe fn dolly_specialsbattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 9.0);
    let angle = if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_S_WORK_FLAG_AIR_ATTACK) {
        68
    }
    else {
        0
    };
    if macros::is_excute(agent) {
        let output = dolly_calc_special_cancel(agent, 8.0, 70);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, angle, 30, 0, output.bkb, 5.0, 0.0, 16.5, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), output.dmg, angle, 30, 0, output.bkb, 5.0, 0.0, 8.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output.dmg, angle, 30, 0, output.bkb, 5.0, 0.0, 16.5, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), output.dmg, angle, 30, 0, output.bkb, 5.0, 0.0, 8.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        let output = dolly_calc_special_cancel(agent, 8.0, 70);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, angle, 30, 0, output.bkb, 5.0, 0.0, 16.0, 3.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output.dmg, angle, 30, 0, output.bkb, 5.0, 0.0, 16.0, 3.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        let output = dolly_calc_special_cancel(agent, 8.0, 70);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, angle, 30, 0, output.bkb, 5.0, 0.0, 15.0, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output.dmg, angle, 30, 0, output.bkb, 5.0, 0.0, 15.0, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        let output1 = dolly_calc_special_cancel(agent, 10.0, 95);
        let output2 = dolly_calc_special_cancel(agent, 10.0, 75);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output1.dmg, 270, 15, 0, 95, 5.0, 0.0, 14.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), output2.dmg, 68, 15, 0, output2.bkb, 5.0, 0.0, 14.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output1.dmg, 270, 15, 0, 95, 5.0, 0.0, 5.0, 1.0, Some(0.0), Some(8.0), Some(1.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), output2.dmg, 68, 15, 0, output2.bkb, 5.0, 0.0, 5.0, 1.0, Some(0.0), Some(8.0), Some(1.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        let output1 = dolly_calc_special_cancel(agent, 10.0, 95);
        let output2 = dolly_calc_special_cancel(agent, 10.0, 75);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output1.dmg, 270, 15, 0, 95, 5.0, 0.0, 11.0, 8.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), output2.dmg, 68, 15, 0, output2.bkb, 5.0, 0.0, 11.0, 8.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        let output1 = dolly_calc_special_cancel(agent, 10.0, 95);
        let output2 = dolly_calc_special_cancel(agent, 10.0, 75);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output1.dmg, 270, 15, 0, 95, 5.0, 0.0, 9.0, 8.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), output2.dmg, 68, 15, 0, output2.bkb, 5.0, 0.0, 9.0, 8.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd("dolly", "effect_specialsbattack")]
unsafe fn dolly_specialsbattack_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    let alpha: f32 = if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("dolly_kick_arc_s_command"), Hash40::new("dolly_kick_arc_s_command"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ);
            EffectModule::set_disable_render_offset_last(agent.module_accessor);
        }
        1.3
    }
    else {
        0.9
    };
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let eff = match color {
        0 => hash40("dolly_kick_arc_s01"),
        1 => hash40("dolly_kick_arc_s02"),
        2 => hash40("dolly_kick_arc_s03"),
        3 => hash40("dolly_kick_arc_s04"),
        4 => hash40("dolly_kick_arc_s05"),
        5 => hash40("dolly_kick_arc_s06"),
        6 => hash40("dolly_kick_arc_s07"),
        _ => hash40("dolly_kick_arc_s08")
    };
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new_raw(eff), Hash40::new_raw(eff), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, alpha)
    }
}

#[acmd("dolly", "sound_specialsbattack")]
unsafe fn dolly_specialsbattack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::PLAY_STATUS(agent, Hash40::new("se_dolly_special_sb03_command"));
            macros::PLAY_SEQUENCE(agent, Hash40::new("seq_dolly_rnd_special_b02"));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::PLAY_STATUS(agent, Hash40::new("se_dolly_special_sb03"));
            macros::PLAY_SEQUENCE(agent, Hash40::new("seq_dolly_rnd_special_b02"));
        }
    }
}

#[acmd("dolly", "expression_specialsbattack")]
unsafe fn dolly_specialsbattack_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd("dolly", [ "game_specialhi1", "game_specialairhi1" ])]
unsafe fn dolly_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_XLU);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
        let output = dolly_calc_special_cancel(agent, 4.0, 30);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 98, 110, 110, 30, 4.0, 0.0, 10.0, 6.5, Some(0.0), Some(10.0), Some(2.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), output.dmg, 75, 110, 110, 30, 4.0, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-3.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 4.5, 0.0, Some(0.0), Some(4.5), Some(6.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 17.0, 1.0, Some(0.0), Some(19.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 10, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 4.5, -4.0, Some(0.0), Some(4.5), Some(-6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 3.5, 6.0, Some(0.0), Some(3.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 3.5, -4.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 3.5, 0.0, Some(0.0), Some(3.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 3.5, -4.0, Some(0.0), Some(3.5), Some(-6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            let output = dolly_calc_special_cancel(agent, 7.0, 90);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 83, 65, 0, output.bkb, 7.0, 0.0, 7.0, 2.0, Some(0.0), Some(14.0), Some(2.0), 2.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            let output = dolly_calc_special_cancel(agent, 9.0, 90);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 83, 55, 0, output.bkb, 7.0, 0.0, 7.0, 2.0, Some(0.0), Some(14.0), Some(2.0), 2.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        MotionModule::set_rate(agent.module_accessor, 1.5);
    }
}

#[acmd("dolly", [ "game_specialhicommand", "game_specialairhicommand" ])]
unsafe fn dolly_specialhicommand(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    if VarModule::is_flag(agent.battle_object, dolly::instance::flag::RISING_FORCE) {
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.battle_object, dolly::status::flag::DISABLE_METER_GAIN);
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_XLU);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
        let output = dolly_calc_special_cancel(agent, 4.5, 30);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 98, 110, 110, 30, 4.5, 0.0, 10.0, 6.5, Some(0.0), Some(10.0), Some(2.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), output.dmg, 75, 110, 110, 30, 4.5, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-3.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 7.5, 0.0, Some(0.0), Some(7.5), Some(6.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 20.0, 1.0, Some(0.0), Some(22.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 20, 4.5, 0.0, 13.0, 3.0, Some(0.0), Some(13.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 13.0, 3.0, Some(0.0), Some(13.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 7.5, -4.0, Some(0.0), Some(7.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 6.5, 7.5, Some(0.0), Some(6.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 19.0, 1.0, Some(0.0), Some(21.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 12.0, 3.0, Some(0.0), Some(12.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 12.0, 3.0, Some(0.0), Some(12.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 6.5, -4.0, Some(0.0), Some(6.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 0.0, Some(0.0), Some(5.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 7.5, Some(0.0), Some(5.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 0.0, Some(0.0), Some(5.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 4.5, 7.5, Some(0.0), Some(4.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 17.0, 1.0, Some(0.0), Some(19.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 4.5, -4.0, Some(0.0), Some(4.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 90, 30, 50, 30, 3.5, 0.0, 3.5, 5.0, Some(0.0), Some(3.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 45, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 50, 45, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            let output = dolly_calc_special_cancel(agent, 8.0, 90);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 83, 73, 0, output.bkb, 7.0, 0.0, 8.0, 2.0, Some(0.0), Some(15.0), Some(2.0), 2.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            let output = dolly_calc_special_cancel(agent, 10.0, 90);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 83, 62, 0, output.bkb, 7.0, 0.0, 8.0, 2.0, Some(0.0), Some(15.0), Some(2.0), 2.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        MotionModule::set_rate(agent.module_accessor, 1.5);
    }
}

#[acmd("dolly", "game_speciallwstart")]
unsafe fn dolly_speciallwstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        let output = dolly_calc_special_cancel(agent, 4.5, 30);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 65, 30, 30, 73, 5.0, 0.0, 10.0, 7.5, Some(0.0), Some(6.0), Some(7.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
        VarModule::on_flag(agent.battle_object, dolly::status::flag::SPECIAL_LW_CHECK_BREAK);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd("dolly", "effect_speciallwstart")]
unsafe fn dolly_speciallwstart_eff(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 8, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("dolly_down_start"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if VarModule::is_flag(agent.battle_object, dolly::status::flag::SPECIAL_LW_BREAK) {
        if macros::is_excute(agent) {
            for _ in 0..5 {
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flyroll_smoke"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1.0, true);
                macros::LAST_EFFECT_SET_RATE(agent, 0.1);
                macros::LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 1.0);
                macros::LAST_EFFECT_SET_ALPHA(agent, 0.4);
            }
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_flyroll_smoke"), false, false);
    }
}

#[acmd("dolly", "sound_speciallwstart")]
unsafe fn dolly_speciallwstart_snd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_dolly_command_success"));
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if VarModule::is_flag(agent.battle_object, dolly::status::flag::SPECIAL_LW_BREAK) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_dolly_ottotto"));
        }
    }
}

#[acmd("dolly", "expression_speciallwstart")]
unsafe fn dolly_speciallwstart_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_rush"),
            8,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd("dolly", "game_specialairlwrisew")]
unsafe fn dolly_specialairlwrisew(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::clear_speed_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 63, 100, 10, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 63, 80, 10, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 5.0, y: -1.5, z: 0.0});
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
}

#[acmd("dolly", "game_specialairlwrise")]
unsafe fn dolly_specialairlwrise(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::clear_speed_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 63, 100, 30, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 63, 80, 30, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.3, y: -1.5, z: 0.0});
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
}

#[acmd("dolly", "game_specialairlw")]
unsafe fn dolly_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.3, y: -1.0, z: 0.0 });
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        }
    }
    else {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 1.3, y: -1.5, z: 0.0 });
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        }
    }
    frame(agent.lua_state_agent, 1.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: -0.3, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: -1.0, z: 0.0 });
        }
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 11.0, 80);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 70, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 12.0, 80);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 65, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    else {
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 11.0, 80);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 75, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 12.0, 80);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 70, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &ZERO_VECTOR);
        }
    }
    else {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: -0.5, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 14.0, 60);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 80, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 14.0, 60);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 80, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    else {
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) != *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 14.0, 30);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 310, 95, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 14.0, 60);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 80, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.2);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 12.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &ZERO_VECTOR);
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &ZERO_VECTOR);
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) != *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 14.0, 60);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 80, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 10.0);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_DOLLY_SPECIAL_LW_FALL);
        MotionModule::set_rate(agent.module_accessor, 1.0);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_LANDING_HEAVY);
    }
}

#[acmd("dolly", "game_superspecial")]
unsafe fn dolly_superspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, dolly::status::flag::DISABLE_METER_GAIN);
        FGCModule::update_meter(agent.battle_object, -100.0, 200.0, dolly::instance::float::GO_METER);
        dolly_check_super_special_pre(agent.module_accessor, 0);
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 4.0, 4.0);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 7);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_DOLLY_GENERATE_ARTICLE_BURST, false, -1);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.5);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 0.0, 8.0);
    }
}

#[acmd("dolly", "effect_superspecial")]
unsafe fn dolly_superspecial_eff(agent: &mut L2CAgentBase) {
    FGCModule::ex_flash(agent);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_aura"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 1, true);
        // macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 13, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        // macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("dolly_wave_arc"), Hash40::new("dolly_wave_arc"), Hash40::new("top"), 0, 10, 4, 69, -46, -45, 1.2, true, *EF_FLIP_YZ);
    }
}

#[acmd("dolly", "sound_superspecial")]
unsafe fn dolly_superspecial_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_waza_super_kof"));
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_superspecial_success"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_dolly_rnd_special_ss01"));
        macros::PLAY_SE(agent, Hash40::new("se_dolly_superspecial01_01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_superspecial01_02"));
    }
}

#[acmd("dolly", "game_superspecial2start")]
unsafe fn dolly_superspecial2start(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, dolly::status::flag::DISABLE_METER_GAIN);
        FGCModule::update_meter(agent.battle_object, -100.0, 200.0, dolly::instance::float::GO_METER);
        dolly_check_super_special_pre(agent.module_accessor, 0);
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8);
        MotionModule::set_rate(agent.module_accessor, 2.0);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 0.5);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 2.0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 45, 100, 100, 30, 6.0, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 45, 100, 100, 30, 6.0, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_DOLLY_GENERATE_ARTICLE_FIRE, false, -1);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
    }
}

#[acmd("dolly", "effect_superspecial2start")]
unsafe fn dolly_superspecial2start_eff(agent: &mut L2CAgentBase) {
    FGCModule::ex_flash(agent);
    if macros::is_excute(agent) {
        // macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -15, 16, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        // macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_hold"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("dolly_buster_punch"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, false);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("dolly_buster_dash"), Hash40::new("top"), 0, 0, -8, 0, 0, 0, 1, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("dolly_buster_punch"), 6);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("dolly_buster_dash"), false, true);
    }
}

#[acmd("dolly", "sound_superspecial2start")]
unsafe fn dolly_superspecial2start_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_waza_super_kof"));
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_superspecial_success"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_dolly_superspecial02_01"));
        macros::PLAY_SE(agent, Hash40::new("se_dolly_superspecial02_01"));
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_superspecial02_02"));
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_superspecial02_03"));
    }
}

#[acmd("dolly", "game_superspecial2")]
unsafe fn dolly_superspecial2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, dolly::status::flag::DISABLE_METER_GAIN);
        VarModule::off_flag(agent.battle_object, dolly::instance::flag::RISING_FORCE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        let output = dolly_calc_special_cancel(agent, 20.0, 73);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, output.dmg, 45, 63, 0, output.bkb, 0.8, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        let output1 = dolly_calc_special_cancel(agent, 15.0, 90);
        let output2 = dolly_calc_special_cancel(agent, 12.0, 90);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output1.dmg, 45, 63, 0, output1.bkb, 8.0, 0.0, 8.0, 5.0, Some(0.0), Some(8.0), Some(15.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), output2.dmg, 45, 48, 0, output2.bkb, 3.0, 0.0, 2.0, -5.0, Some(0.0), Some(25.0), Some(17.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output2.dmg, 45, 48, 0, output2.bkb, 3.0, 0.0, 2.0, -5.0, Some(0.0), Some(20.0), Some(25.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), output2.dmg, 45, 48, 0, output2.bkb, 3.0, 0.0, 2.0, -5.0, Some(0.0), Some(12.0), Some(30.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), output2.dmg, 45, 48, 0, output2.bkb, 3.0, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(33.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_INT_TARGET_OBJECT_ID);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

#[acmd("dolly_wave", "game_normalw")]
unsafe fn dolly_wave_normalw(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let object = MiscModule::get_battle_object_from_id(otarget_id);
    if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && utility::get_kind(&mut *oboma) == *FIGHTER_KIND_DOLLY
    && VarModule::is_flag(object, dolly::status::flag::IS_SPECIAL_CANCEL) {
        VarModule::on_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL);
    }
    if macros::is_excute(agent) {
        let mut damage = 6.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL)  {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 2.4, 0.0, 12.0, -5.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 1.8, 0.0, 2.0, -10.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let mut damage = 6.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 2.4, 0.0, 9.0, -4.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 1.8, 0.0, 2.0, -8.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        let mut damage = 6.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 2.4, 0.0, 6.5, -2.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 1.8, 0.0, 2.0, -6.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

#[acmd("dolly_wave", "game_normal")]
unsafe fn dolly_wave_normal(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let object = MiscModule::get_battle_object_from_id(otarget_id);
    if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && utility::get_kind(&mut *oboma) == *FIGHTER_KIND_DOLLY
    && VarModule::is_flag(object, dolly::status::flag::IS_SPECIAL_CANCEL) {
        VarModule::on_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL);
    }
    if macros::is_excute(agent) {
        let mut damage = 8.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 2.4, 0.0, 12.0, -5.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 1.8, 0.0, 2.0, -10.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let mut damage = 8.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 2.4, 0.0, 9.0, -4.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 1.8, 0.0, 2.0, -8.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        let mut damage = 8.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 2.4, 0.0, 6.5, -2.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 55, 20, 0, 50, 1.8, 0.0, 2.0, -6.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

#[acmd("dolly_wave", "game_normalairw")]
unsafe fn dolly_wave_normalairw(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let object = MiscModule::get_battle_object_from_id(otarget_id);
    if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && utility::get_kind(&mut *oboma) == *FIGHTER_KIND_DOLLY
    && VarModule::is_flag(object, dolly::status::flag::IS_SPECIAL_CANCEL) {
        VarModule::on_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL);
    }
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 10.0, -2.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -3.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 9.4, -0.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -4.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 8.8, 0.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -6.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 8.2, 1.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -7.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 7.6, 2.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -9.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 7.0, 4.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -11.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 5.8, 6.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -14.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 5.2, 7.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -15.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 10.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 4.6, 8.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -17.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

#[acmd("dolly_wave", "game_normalair")]
unsafe fn dolly_wave_normalair(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let object = MiscModule::get_battle_object_from_id(otarget_id);
    if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && utility::get_kind(&mut *oboma) == *FIGHTER_KIND_DOLLY
    && VarModule::is_flag(object, dolly::status::flag::IS_SPECIAL_CANCEL) {
        VarModule::on_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL);
    }
    if macros::is_excute(agent) {
        let mut damage = 11.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 10.0, -2.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -3.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 11.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 9.2, -0.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -5.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 11.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 8.4, 1.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -7.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 11.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 7.6, 3.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -10.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 11.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 6.8, 5.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -12.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 11.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 6.0, 7.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -15.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage = 11.0;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 5.2, 8.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 45, 30, 0, 85, 3.0, 0.0, 0.0, -17.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

#[acmd("dolly_burst", "game_superspecial")]
unsafe fn dolly_burst_superspecial(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let object = MiscModule::get_battle_object_from_id(otarget_id);
    if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && utility::get_kind(&mut *oboma) == *FIGHTER_KIND_DOLLY
    && VarModule::is_flag(object, dolly::status::flag::IS_SPECIAL_CANCEL) {
        VarModule::on_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL);
    }
    if macros::is_excute(agent) {
        let mut damage = 26.0;
        let mut bkb = 100;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
            bkb = (bkb as f32 * vl::param_private::special_cancel_bkb_mul) as i32;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 70, 41, 0, bkb, 1.0, 0.0, 2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 70, 41, 0, bkb, 3.0, 0.0, 3.0, -8.0, Some(0.0), Some(3.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage1 = 26.0;
        let mut damage2 = 23.0;
        let mut damage3 = 20.0;
        let mut bkb = 100;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage1 *= vl::param_private::special_cancel_damage_mul;
            damage2 *= vl::param_private::special_cancel_damage_mul;
            damage3 *= vl::param_private::special_cancel_damage_mul;
            bkb = (bkb as f32 * vl::param_private::special_cancel_bkb_mul) as i32;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage1, 70, 41, 0, bkb, 11.0, 0.0, 10.0, -1.0, Some(0.0), Some(10.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage2, 70, 42, 0, bkb, 11.0, 0.0, 25.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), damage3, 70, 44, 0, bkb, 8.0, 0.0, 40.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let mut damage1 = 19.0;
        let mut damage2 = 17.0;
        let mut damage3 = 15.0;
        let mut bkb1 = 120;
        let mut bkb2 = 110;
        let mut bkb3 = 100;
        if VarModule::is_flag(agent.battle_object, dolly_wave::instance::flag::FROM_CANCEL) {
            damage1 *= vl::param_private::special_cancel_damage_mul;
            damage2 *= vl::param_private::special_cancel_damage_mul;
            damage3 *= vl::param_private::special_cancel_damage_mul;
            bkb1 = (bkb1 as f32 * vl::param_private::special_cancel_bkb_mul) as i32;
            bkb2 = (bkb2 as f32 * vl::param_private::special_cancel_bkb_mul) as i32;
            bkb3 = (bkb3 as f32 * vl::param_private::special_cancel_bkb_mul) as i32;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage1, 70, 41, 0, bkb1, 11.0, 0.0, 10.0, -1.0, Some(0.0), Some(10.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage2, 70, 42, 0, bkb2, 11.0, 0.0, 25.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), damage3, 70, 44, 0, bkb3, 8.0, 0.0, 40.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
    }
}

pub fn install() {
    dolly_specialn::install();
    dolly_specialairn::install();
    dolly_specialsfattack::install();
    dolly_specialsbattackw::install();
    dolly_specialsbattack::install();
    dolly_specialsbattack_eff::install();
    dolly_specialsbattack_snd::install();
    dolly_specialsbattack_exp::install();
    dolly_specialhi::install();
    dolly_specialhicommand::install();
    dolly_speciallwstart::install();
    dolly_speciallwstart_eff::install();
    dolly_speciallwstart_snd::install();
    dolly_speciallwstart_exp::install();
    dolly_specialairlwrisew::install();
    dolly_specialairlwrise::install();
    dolly_specialairlw::install();
    dolly_superspecial::install();
    dolly_superspecial_eff::install();
    dolly_superspecial_snd::install();
    dolly_superspecial2start::install();
    dolly_superspecial2start_eff::install();
    dolly_superspecial2start_snd::install();
    dolly_superspecial2::install();
    dolly_wave_normalw::install();
    dolly_wave_normal::install();
    dolly_wave_normalairw::install();
    dolly_wave_normalair::install();
    dolly_burst_superspecial::install();
}