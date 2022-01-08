use {
    smash::{
        lua2cpp::{L2CFighterCommon, L2CWeaponCommon, *},
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    crate::{
        vars::*,
        gameplay::*,
        table_const::*
    },
    super::vl::*
};

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn samus_attacks3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS3Common();
    if !StopModule::is_stop(fighter.module_accessor) {
        samus_attacks3_substatus2(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(samus_attacks3_substatus2 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackS3_Main as *const () as _))
}

unsafe extern "C" fn samus_attacks3_substatus2(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.global_table[IN_HITLAG].get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_BEAM_RAPID) {
            cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3, false);
        }
    }
    0.into()
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn samus_attacks3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK_S3 {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_BEAM_RAPID);
    // }
    0.into()
}

#[status_script(agent = "samus_cshot", status = WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn samus_cshot_shoot_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_cshot"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED) {
        if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            effect!(
                weapon,
                MA_MSC_EFFECT_REQUEST_FOLLOW,
                Hash40::new("samus_cshot_bullet"),
                Hash40::new("top"),
                7.98004,
                -0.50584,
                -0.25092,
                -91.2728,
                -1.7974,
                176.373,
                1.0,
                false,
                0,
                0,
                0
            );
            weapon.clear_lua_stack();
            lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
            sv_module_access::effect(weapon.lua_state_agent);
            let handle = weapon.pop_lua_stack(1).get_i32();
            WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
        }
    }
    let lr = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_SHOOT_LR);
    let mut angle : f32 = 0.0;
    let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    if utility::get_kind(&mut *oboma)  == *FIGHTER_KIND_SAMUS {
        let mot = MotionModule::motion_kind(oboma);
        if mot == hash40("attack_s3_hi") {
            angle = cshot_angle;
        }
        else if mot == hash40("attack_s3_lw") {
            angle = -cshot_angle;
        }
    }
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("min_speed"));
    let speed_x = angle.to_radians().cos() * speed * lr;
    let speed_y = angle.to_radians().sin() * speed;
    weapon.clear_lua_stack();
    lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);
    weapon.clear_lua_stack();
    lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -1.0, -1.0);
    sv_kinetic_energy::set_stable_speed(weapon.lua_state_agent);
    weapon.clear_lua_stack();
    lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy::set_accel(weapon.lua_state_agent);
    if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        let scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("min_scale"));
        effect!(
            weapon,
            MA_MSC_EFFECT_REQUEST_FOLLOW,
            Hash40::new("samus_cshot_bullet_sub_b"),
            Hash40::new("top"),
            7.98004,
            -0.50584,
            -0.25092,
            -91.2728,
            -1.7974,
            176.373,
            scale,
            false,
            0,
            0,
            0
        );
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW);
        effect!(
            weapon,
            MA_MSC_EFFECT_REQUEST_FOLLOW,
            Hash40::new("samus_cshot_bullet_sub"),
            Hash40::new("top"),
            7.98004,
            -0.50584,
            -0.25092,
            -91.2728,
            -1.7974,
            176.373,
            scale,
            false,
            0,
            0,
            0
        );
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW_SUB);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        samus_attacks3_main,
        samus_attacks3_end,
        samus_cshot_shoot_init
    );
}