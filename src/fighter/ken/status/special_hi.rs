use crate::imports::status_imports::*;

#[status("ken", FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND)]
unsafe fn ken_specialhi_command_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ken_specialhi_main(fighter);
    0.into()
}

unsafe extern "C" fn ken_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let command = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);
    let mot = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if command {
            Hash40::new("special_hi_command")
        }
        else {
            Hash40::new("special_hi")
        }
    }
    else {
        if command {
            Hash40::new("special_air_hi_command")
        }
        else {
            Hash40::new("special_air_hi")
        }
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        mot,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    ItemModule::set_change_status_event(fighter.module_accessor, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        ken_specialhi_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ken_specialhi_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_specialhi_main_loop as *const () as _))
}

unsafe extern "C" fn ken_specialhi_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("lr_stick_x"));
            if lr_stick_x < stick_x.abs() {
                PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            }
        }
    }
    else {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    0.into()
}

unsafe extern "C" fn ken_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, ken::status::flag::SPECIAL_HI_CHANGE_REPPA)
    && VarModule::is_flag(fighter.battle_object, ken::instance::flag::V_TRIGGER)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new("special_hi_reppa"),
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        VarModule::off_flag(fighter.battle_object, ken::status::flag::SPECIAL_HI_CHANGE_REPPA);
    }
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            // if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi_reppa") {
            //     MotionModule::change_motion_inherit_frame(
            //         fighter.module_accessor,
            //         Hash40::new("special_hi_command"),
            //         -1.0,
            //         1.0,
            //         0.0,
            //         false,
            //         false
            //     );
            // }
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if !StatusModule::is_changing(fighter.module_accessor) {
                if VarModule::is_flag(fighter.battle_object, ken::status::flag::SPECIAL_HI_CHANGE_REPPA) {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                }
                else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                    sv_kinetic_energy!(
                        reset_energy,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_MOTION,
                        ENERGY_MOTION_RESET_TYPE_AIR_TRANS,
                        0.0,
                        0.0,
                        0.0,
                        0.0,
                        0.0
                    );
                    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                    let start_accel_y = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("start_accel_y"));
                    sv_kinetic_energy!(
                        set_accel,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        -start_accel_y
                    );
                }
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            if !StatusModule::is_changing(fighter.module_accessor) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                sv_kinetic_energy!(
                    reset_energy,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_MOTION,
                    ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                );
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);              
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP.into(), false.into());
    }
    0.into()
}

pub fn install() {
    ken_specialhi_command_main::install();
}