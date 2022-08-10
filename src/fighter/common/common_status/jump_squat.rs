use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::Hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    custom_var::*,
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

#[skyline::hook(replace = L2CFighterCommon_sub_jump_squat_uniq_process_init_param)]
unsafe fn sub_jump_squat_uniq_process_init_param(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if !super_jump_restrict(fighter)
    && (VarModule::get_float(fighter.battle_object, commons::instance::float::FLICK_DOWN) > 0.0
    || (fighter.global_table[STICK_Y].get_f32() < -0.5
    && fighter.global_table[FLICK_Y].get_i32() < 7
    && fighter.global_table[FLICK_Y_DIR].get_f32() < 0.0)) {
        VarModule::on_flag(fighter.battle_object, commons::instance::flag::SUPER_JUMP);
    }
    else {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::SUPER_JUMP);
    }
    let mut jump_squat_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_squat_frame"), 0) as f32;
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::SUPER_JUMP) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        jump_squat_frame += 1.0;
    }
    let mot = param_1.get_u64();
    let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new_raw(mot));
    let mut rate = end_frame / jump_squat_frame;
    if rate < 1.0 {
        rate += 0.001;
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(mot),
        0.0,
        rate,
        false,
        0.0,
        false,
        false
    );
}

unsafe fn super_jump_restrict(fighter: &mut L2CFighterCommon) -> bool {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let status = fighter.global_table[STATUS_KIND].get_i32();
    let steve = {
        fighter_kind == *FIGHTER_KIND_PICKEL &&
        [
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP_SQUAT,
            *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT,
            *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_SQUAT
        ].contains(&status)
    };
    let minmin = {
        fighter_kind == *FIGHTER_KIND_TANTAN &&
        [*FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_SQUAT].contains(&status)
    };
    let kirby = {
        fighter_kind == *FIGHTER_KIND_KIRBY &&
        [
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP_SQUAT,
            *FIGHTER_KIRBY_STATUS_KIND_PICKEL_SPECIAL_N1_JUMP_SQUAT,
            *FIGHTER_KIRBY_STATUS_KIND_PICKEL_SPECIAL_N3_JUMP_SQUAT,
            *FIGHTER_KIRBY_STATUS_KIND_SNAKE_SPECIAL_N_HOLD_JUMP_SQUAT,
            *FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_JUMP_SQUAT
        ].contains(&status)
    };
    let dedede = {
        fighter_kind == *FIGHTER_KIND_DEDEDE &&
        [*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP_SQUAT].contains(&status)
    };
    let donkey = {
        fighter_kind == *FIGHTER_KIND_DONKEY &&
        [*FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT].contains(&status)
    };
    let snake = {
        fighter_kind == *FIGHTER_KIND_SNAKE &&
        [*FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_SQUAT].contains(&status)
    };
    let megaman = {
        fighter_kind == *FIGHTER_KIND_ROCKMAN &&
        [*FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP_SQUAT].contains(&status)
    };
    let villager = {
        fighter_kind == *FIGHTER_KIND_MURABITO &&
        [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP_SQUAT].contains(&status)
    };
    let normal = [
        *FIGHTER_STATUS_KIND_HAMMER_JUMP_SQUAT,
        *FIGHTER_STATUS_KIND_ITEM_SHOOT_JUMP_SQUAT
    ].contains(&status);
    steve || minmin || kirby || dedede || donkey || snake || megaman || villager || normal
}

#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_common)]
unsafe fn status_jumpsquat_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let stick_jump_command_life = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
    if stick_jump_command_life == 0
    || fighter.global_table[FLICK_Y_DIR].get_i32() <= 0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_BUTTON);
        if ControlModule::is_jump_mini_button(fighter.module_accessor) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
    if param_1.get_bool() {
        PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    // ControlModule::reset_flick_y(fighter.module_accessor);
    // ControlModule::reset_flick_sub_y(fighter.module_accessor);
    // fighter.global_table[FLICK_Y].assign(&0xFE.into());
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ABNORMAL_MINIJUMP_SLOWWALK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON);
        // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK)
    || [
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_GUARD
    ].contains(&fighter.global_table[PREV_STATUS_KIND].get_i32())
    || (fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_GUARD_OFF
    && !VarModule::is_flag(fighter.battle_object, commons::instance::flag::GUARD_OFF_ATTACK_CANCEL)) {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::GUARD_OFF_ATTACK_CANCEL);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_Main)]
unsafe fn status_jumpsquat_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[JUMP_SQUAT_MAIN_UNIQ].get_bool() && {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[JUMP_SQUAT_MAIN_UNIQ].get_ptr());
        callable(fighter).get_bool()
    } {
        return 1.into();
    }
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::SUPER_JUMP) {
        let jump_squat_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_squat_frame"), 0) as f32;
        if fighter.global_table[MOTION_FRAME].get_f32() == jump_squat_frame {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START) {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if !fighter.sub_transition_group_check_ground_item().get_bool() {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI)
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
            return 0.into();
        }
        if !fighter.sub_transition_specialflag_hoist().get_bool() {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START)
            && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                if fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_bool() && {
                    let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_ptr());
                    callable(fighter).get_bool()
                } {
                    return 0.into();
                }
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0
                && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), true.into());
                    return 0.into();
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U)
            && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1daca540be));
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
                    return 0.into();
                }
            }
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_jump_squat_uniq_check_sub)]
unsafe fn sub_jump_squat_uniq_check_sub(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP) {
        return;
    }
    let jump_squat_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_squat_frame"), 0) as f32;
    if fighter.global_table[MOTION_FRAME].get_f32() <= jump_squat_frame {
        if !WorkModule::is_flag(fighter.module_accessor, param_1.get_i32()) {
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            let jump_neutral_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_y"));
            if stick_y < jump_neutral_y {
                if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)
                && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
                    return;
                }
            }
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)
            && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            }
        }
        else {
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
            || ControlModule::is_jump_mini_button(fighter.module_accessor) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            }
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_jump_squat_uniq_check_sub_mini_attack)]
unsafe fn sub_jump_squat_uniq_check_sub_mini_attack(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP) {
        return;
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        let mut cont = ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK);
        if !cont {
            cont = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK);
        }
        else {
            cont = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP);
            if !cont {
                cont = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK);
            }
        }
        if !cont {
            return;
        }
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
        // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON)
        && 1.0 <= fighter.global_table[MOTION_FRAME].get_f32() {
            FighterControlModuleImpl::reserve_on_attack_button(fighter.module_accessor);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON);
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_end_JumpSquat)]
unsafe fn status_end_jumpsquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_JUMP {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::SUPER_JUMP);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_jump_squat_uniq_process_init_param,
            status_jumpsquat_common,
            status_jumpsquat_main,
            sub_jump_squat_uniq_check_sub,
            sub_jump_squat_uniq_check_sub_mini_attack
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}