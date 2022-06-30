use {
    smash::{
        hash40,
        app::{BattleObjectModuleAccessor, Fighter, Article, lua_bind::*},
        lib::lua_const::*
    },
    smash_rs::{
        phx::Hash40,
        app::LinkEventCapture
    },
    crate::function_hooks::get_battle_object_from_id,
    custom_var::*,
    wubor_utils::vars::*
};

pub struct GenericModule {
    _vtable: *const u64,
    owner: *mut BattleObjectModuleAccessor,
    // ...
}

#[skyline::hook(offset = 0xc5bff0)]
pub unsafe extern "C" fn lucario_check_aura(module_accessor: *mut BattleObjectModuleAccessor) -> f32 {
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let object = get_battle_object_from_id((*module_accessor).battle_object_id);
    let aura = VarModule::get_int(object, lucario::instance::int::AURA_LEVEL) as f32;
    1.0 + (0.3 * aura.clamp(0.0, 2.0) / 2.0)
}

#[skyline::hook(offset = 0xc5be20)]
pub unsafe extern "C" fn lucario_check_aura2(module: &mut GenericModule) -> f32 {
    let module_accessor = module.owner;
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let object = get_battle_object_from_id((*module_accessor).battle_object_id);
    let aura = VarModule::get_int(object, lucario::instance::int::AURA_LEVEL) as f32;
    1.0 + (0.3 * aura.clamp(0.0, 2.0) / 2.0)
}

#[skyline::hook(offset = 0xc5e530)]
pub unsafe extern "C" fn lucario_handle_aura(_vtable: u64, fighter: &mut Fighter) {
    let object = &mut fighter.battle_object;
    let module_accessor = object.module_accessor;
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let aura = VarModule::get_int(object, lucario::instance::int::AURA_LEVEL) as f32;
    let aura = 1.0 + (0.3 * aura.clamp(0.0, 2.0) / 2.0);
    WorkModule::set_float(module_accessor, aura, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
}

#[skyline::hook(offset = 0xc5e6d0)]
pub unsafe extern "C" fn lucario_handle_aura2(_vtable: u64, fighter: &mut Fighter) {
    let object = &mut fighter.battle_object;
    let module_accessor = object.module_accessor;
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let aura = VarModule::get_int(object, lucario::instance::int::AURA_LEVEL) as f32;
    let aura = 1.0 + (0.3 * aura.clamp(0.0, 2.0) / 2.0);
    WorkModule::set_float(module_accessor, aura, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
    let prev_charge = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_PREV_AURABALL_CHARGE_FRAME);
    let mut charge_frame;
    if !ArticleModule::is_exist(module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL) {
        charge_frame = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
        if !WorkModule::is_flag(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURABALL_HAD) {
            charge_frame = i32::MAX;
        }
    }
    else {
        let article = ArticleModule::get_article(module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL);
        if article != 0 {
            let auraball = (*(article as *mut Article)).battle_object.module_accessor;
            charge_frame = WorkModule::get_int(auraball, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
        }
        else {
            charge_frame = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
            if !WorkModule::is_flag(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURABALL_HAD) {
                charge_frame = i32::MAX;
            }
        }
    }
    if prev_charge != charge_frame {
        let max_charge_frame = WorkModule::get_param_float(module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
        if charge_frame < max_charge_frame as i32 {
            let left_eff = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_L) as u32;
            if left_eff != 0 {
                EffectModule::remove(module_accessor, left_eff, 0);
                WorkModule::set_int(module_accessor, 0, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_L);
            }
            let right_eff = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_R) as u32;
            if right_eff != 0 {
                EffectModule::remove(module_accessor, right_eff, 0);
                WorkModule::set_int(module_accessor, 0, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_R);
            }
        }
        else {
            let right_eff = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_R) as u32;
            if right_eff == 0 {
                let bone = if object.kind == 6 {
                    hash40("haver")
                }
                else {
                    hash40("handr")
                };
                let eff = EffectModule::req_follow(
                    module_accessor,
                    smash::phx::Hash40::new_raw(0x16b1f651c2),
                    smash::phx::Hash40::new_raw(bone),
                    &ZERO_VECTOR,
                    &ZERO_VECTOR,
                    1.0,
                    false,
                    0,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false
                ) as u32;
                WorkModule::set_int(module_accessor, eff as i32, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_R);
            }
            let left_eff = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_L) as u32;
            if left_eff == 0 {
                let bone = if object.kind == 6 {
                    hash40("havel")
                }
                else {
                    hash40("handl")
                };
                let eff = EffectModule::req_follow(
                    module_accessor,
                    smash::phx::Hash40::new_raw(0x164bf96ca1),
                    smash::phx::Hash40::new_raw(bone),
                    &ZERO_VECTOR,
                    &ZERO_VECTOR,
                    1.0,
                    false,
                    0,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false
                ) as u32;
                WorkModule::set_int(module_accessor, eff as i32, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_L);
            }
        }
    }
    WorkModule::set_int(module_accessor, charge_frame, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_PREV_AURABALL_CHARGE_FRAME);
    WorkModule::set_int(module_accessor, charge_frame, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
}

#[skyline::hook(offset = 0xc5d580)]
pub unsafe extern "C" fn lucario_on_grab(_vtable: u64, fighter: &mut Fighter, capture_event: &mut LinkEventCapture) -> u64 {
    // param_3 + 0x10
    if capture_event.link_event_kind.as_u64() == hash40("capture") {
        let module_accessor = fighter.battle_object.module_accessor;
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S {
            // param_3 + 0x30
            capture_event.node = Hash40::new("throw");
            // param_3[0x28]
            capture_event.result = true;
            let offset = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
            // param_3 + 0x44
            capture_event.motion_offset = offset;
            let offset_lw = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
            // param_3 + 0x48
            capture_event.motion_offset_lw = offset_lw;
            StatusModule::change_status_request(module_accessor, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW, false);
            return 0;
        }
    }
    1
}

pub fn install() {
    skyline::install_hooks!(
        lucario_check_aura,
        lucario_check_aura2,
        lucario_handle_aura,
        lucario_on_grab
    );
}