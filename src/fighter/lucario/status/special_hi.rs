use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*},
    // crate::function_hooks::get_touch_normal_consider_gravity_2,
    super::super::helper::*
};

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn lucario_special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let enhanced_by = VarModule::get_int(fighter.battle_object, lucario::status::int::AURA_ENHANCED_BY);
    let cancel = if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT {
        VarModule::is_flag(fighter.battle_object, lucario::status::flag::SPECIAL_N_ENABLE_SUPERDASH)
    }
    else { false };
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    VarModule::set_flag(fighter.battle_object, lucario::status::flag::SPECIAL_HI_CANCELLED_INTO, cancel);
    if cancel {
        VarModule::set_int(fighter.battle_object, lucario::status::int::AURA_ENHANCED_BY, enhanced_by);
    }
    0.into()
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn lucario_special_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_x_mul"));
    let speed_x = speed_x / start_x_mul;
    lua_bind::KineticEnergy::reset_energy(
        stop_energy as *mut smash::app::KineticEnergy,
        *ENERGY_STOP_RESET_TYPE_AIR,
        &Vector2f{x: speed_x, y: 0.0},
        &Vector3f{x: 0.0, y: 0.0, z: 0.0},
        fighter.module_accessor
    );
    lua_bind::KineticEnergy::enable(stop_energy as *mut smash::app::KineticEnergy);
    let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    lua_bind::FighterKineticEnergyGravity::set_speed(
        gravity as *mut smash::app::FighterKineticEnergyGravity,
        0.0
    );
    lua_bind::KineticEnergy::enable(gravity as *mut smash::app::KineticEnergy);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    WorkModule::set_float(fighter.module_accessor, lr, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLOAT_START_CHARA_LR);
    FighterSpecializer_Lucario::set_mach_validity(fighter.module_accessor, false);
    if !VarModule::is_flag(fighter.battle_object, lucario::status::flag::SPECIAL_HI_CANCELLED_INTO) {
        lucario_drain_aura(fighter, false);
    }
    0.into()
}

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucario_special_hi_rush_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if situation != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
        GroundModule::set_cliff_check(fighter.module_accessor, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let end_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("end_brake_x"));
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            end_brake_x,
            0.0
        );
        let end_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("end_accel_y"));
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -end_accel_y
        );
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y.clamp(-5.0, 2.0)
        );
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_hi_end"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_hi_end"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    }
    WorkModule::set_int(fighter.module_accessor, situation, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_RUSH_END_SITUATION);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_hi_rush_end_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_hi_rush_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_RUSH_END_SITUATION);
    if sit != *SITUATION_KIND_GROUND
    && fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if sit != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            let status = if !VarModule::is_flag(fighter.battle_object, lucario::status::flag::SPECIAL_HI_SUPER_DASH_CANCEL) {
                FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
            }
            else {
                FIGHTER_STATUS_KIND_LANDING_LIGHT
            };
            fighter.change_status(status.into(), false.into());
            return 0.into();
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_AIR_END_CONTROL_X) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_AIR_END_CONTROL_X);
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
    }
    // let cont = if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
    //     lucario_special_hi_attach_wall(fighter, (1.0_f32).into()).get_bool()
    // }
    // else if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
    //     lucario_special_hi_attach_wall(fighter, (-1.0_f32).into()).get_bool()
    // }
    // else { false };
    // if cont {
    //     println!("attaching to wall!");
    //     StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACH_WALL, false);
    // }
    0.into()
}

// unsafe extern "C" fn lucario_special_hi_attach_wall(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
//     let no_attach = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_ATTACH_WALL_FRAME);
//     let wall_jump_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("wall_jump_stick_x"));
//     let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
//     if no_attach <= 0 {
//         if wall_jump_stick_x <= stick_x.abs() {
//             let lr = param_1.get_f32();
//             if lr * stick_x < 0.0 {
//                 let vector;
//                 let some = if 0.0 > lr {
//                     vector = get_touch_normal_consider_gravity_2(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32);
//                     1.0
//                 }
//                 else {
//                     vector = get_touch_normal_consider_gravity_2(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32);
//                     -1.0
//                 };
//                 println!("vector x and y: {}, {}", vector.x, vector.y);
//                 let atan = vector.x.atan2(vector.y) * some;
//                 println!("atan * some: {}", atan);
//                 let deg = atan.to_degrees() + 90.0;
//                 println!("degrees: {}", deg);
//                 let attach_wall_slant_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attach_wall_slant_max"));
//                 let attach_wall_slant_min = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attach_wall_slant_min"));
//                 println!("checking if it's within {} and {} degrees!", attach_wall_slant_min, attach_wall_slant_max);
//                 if attach_wall_slant_max > deg
//                 && deg > attach_wall_slant_min {
//                     if !GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) {
//                         let attach_on = if 0.0 <= lr {
//                             *GROUND_TOUCH_FLAG_LEFT
//                         }
//                         else { *GROUND_TOUCH_FLAG_RIGHT };
//                         if GroundModule::is_attachable(fighter.module_accessor, GroundTouchFlag(attach_on)) {
//                             return true.into();
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     false.into()
// }

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn lucario_special_hi_rush_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, lucario::status::flag::SPECIAL_HI_SUPER_DASH_CANCEL) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            VarModule::on_flag(fighter.battle_object, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL);
        }
        lucario_special_hi_end(fighter, fighter.global_table[STATUS_KIND].clone());
    }
    0.into()
}

unsafe extern "C" fn lucario_special_hi_end(fighter: &mut L2CFighterCommon, status: L2CValue) {
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
        if status.get_i32() != *FIGHTER_STATUS_KIND_ATTACH_WALL {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
        }
    }
}

pub fn install() {
    install_status_scripts!(
        lucario_special_hi_pre, lucario_special_hi_init,
        lucario_special_hi_rush_end_main, lucario_special_hi_rush_end_end
    );
}