use crate::imports::status_imports::*;
use super::super::param;
use std::arch::asm;

#[skyline::hook(replace = L2CFighterCommon_status_Jump_sub)]
unsafe fn status_jump_sub(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, fighter::instance::flag::SUPER_JUMP) {
        let base_speed_x;
        let mut speed_x;
        let mini_jump = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        if mini_jump {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_hyperhop"), true, false, false, false, enSEType(0));

            let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
            speed_x = air_speed_x_stable;
            speed_x *= param::jump::hyper_hop_air_speed_x_stable_mul;
            speed_x = speed_x.clamp(1.22, 1.7);
            base_speed_x = speed_x;
            let stick_x = fighter.global_table[STICK_X].get_f32();
            speed_x *= stick_x;
            // println!("Hyper Hop Speed: {}", speed_x);
        }
        else {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_superjump"), true, false, false, false, enSEType(0));

            speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            speed_x *= param::jump::super_jump_speed_x_mul;
            base_speed_x = speed_x;
        }
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x
        );
        sv_kinetic_energy!(
            mul_x_accel_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::jump::special_jump_control_mul
        );
        sv_kinetic_energy!(
            mul_x_accel_add,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::jump::special_jump_control_mul
        );
        if mini_jump {
            let stable_speed = speed_x.abs().clamp(base_speed_x * 0.5, f32::MAX);
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                stable_speed.abs()
            );
        }
        // let jump_speed_x_max = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_speed_x_max"), 0);
        // if speed_x.abs() > jump_speed_x_max {
        //     sv_kinetic_energy!(
        //         set_limit_speed,
        //         fighter,
        //         FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        //         speed_x.abs()
        //     );
        // }
    }
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&0xFE.into());
    ControlModule::reset_trigger(fighter.module_accessor);
    fighter.sub_air_check_fall_common_pre();
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_STOP_CEIL);
    let mot;
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_POWBLOCK_QUAKE_JUMP) {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let lr = PostureModule::lr(fighter.module_accessor);
        let jump_neutral_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_x"));
        let mini_jump = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        let stick_x = stick_x * lr * -1.0;
        if stick_x <= jump_neutral_x {
            if !mini_jump {
                mot = hash40("jump_f");
            }
            else {
                mot = hash40("jump_f_mini");
            }
        }
        else {
            if !mini_jump {
                mot = hash40("jump_b");
            }
            else {
                mot = hash40("jump_b_mini");
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
            SoundModule::play_se(
                fighter.module_accessor,
                Hash40::new("se_item_usagihat_jump_01"),
                true,
                false,
                false,
                false,
                enSEType(0)
            );
        }
    }
    else {
        mot = hash40("jump_f_mini");
    }
    let ret = if param_1.get_u64() != hash40("invalid") {
        param_1.get_u64()
    }
    else {
        mot
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(ret),
        0.0,
        1.0,
        false,
        param_2.get_f32(),
        false,
        false
    );
    if fighter.global_table[FALL_BRAKE_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[FALL_BRAKE_UNIQ].get_ptr());
        callable(fighter);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
    ret.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_Jump_Main)]
unsafe fn status_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_stop_ceil().get_bool() {
        return 1.into();
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        fighter.sub_air_check_superleaf_fall_slowly();
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_Jump)]
unsafe fn bind_address_call_status_end_jump(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_Jump()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Jump)]
unsafe fn status_end_jump(_fighter: &mut L2CFighterCommon) -> L2CValue {
    // VarModule::off_flag(fighter.battle_object, fighter::instance::flag::SUPER_JUMP);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_jump_sub,
            status_jump_main,
            bind_address_call_status_end_jump,
            status_end_jump
        );
    }
}

#[skyline::hook(offset = 0x6d2158, inline)]
unsafe fn jump_momentum_initial_jump_check(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[19].x.as_ref() as *mut BattleObjectModuleAccessor;
    let object_id = (*module_accessor).battle_object_id;
    let category = sv_battle_object::category(object_id);
    let speed_up = if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        let kind = sv_battle_object::kind(object_id);
        let status = StatusModule::status_kind(module_accessor);
        let sonic = kind == *FIGHTER_KIND_SONIC && status == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP;
        let rockman = kind == *FIGHTER_KIND_ROCKMAN && status == *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP;
        sonic || rockman
    }
    else {
        false
    };
    // println!("w21: {:#x}", *ctx.registers[21].w.as_mut());
    if speed_up {
        asm!("cmp w21, #0x6")
    }
    else {
        asm!("cmp w21, #0xFFF")
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);

    // Removes Accelerated Full Hops except for specific statuses.
    skyline::patching::Patch::in_text(0x6d2158).nop();
    skyline::install_hooks!(
        jump_momentum_initial_jump_check
    );
}