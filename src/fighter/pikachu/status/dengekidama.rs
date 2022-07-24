use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    super::super::vl
};

#[status_script(agent = "pikachu_dengekidama", status = WEAPON_PIKACHU_DENGEKIDAMA_STATUS_KIND_REGULAR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn pikachu_dengekidama_regular_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("regular"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let lr = PostureModule::lr(weapon.module_accessor);
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        vl::dengekidama::ACCEL * lr,
        0.0
    );
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        vl::dengekidama::SPEED_MAX,
        0.0
    );
    weapon.fastshift(L2CValue::Ptr(pikachu_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn pikachu_special_lw_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    if !StatusModule::is_changing(weapon.module_accessor)
    && GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        // notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        pikachu_dengekidama_regular_main
    );
}