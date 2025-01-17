use {
    smash::{
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    wubor_utils::controls::*
};

// #[repr(C)]
// struct BufferState {
//     pub on_last_frame: u32,
//     pub should_hold: [bool; 32],
//     pub hold_frame: [i32; 32],
//     pub hold_frame_max: [i32; 32],
// }

// impl BufferState {
//     pub fn new() -> Self {
//         Self {
//             on_last_frame: 0,
//             should_hold: [false; 32],
//             hold_frame: [0; 32],
//             hold_frame_max: [-1; 32],
//         }
//     }

//     pub fn clear(&mut self) {
//         self.on_last_frame = 0;
//         self.should_hold = [false; 32];
//         self.hold_frame = [0; 32];
//         self.hold_frame_max = [-1; 32];
//     }

//     pub fn update(
//         &mut self,
//         game_held: &mut [u8],
//         max_hold_frame: i32,
//         press_frame: i32,
//         should_hold: bool,
//     ) {
//         self.on_last_frame = 0;
//         for (idx, x) in game_held.iter_mut().enumerate() {
//             if *x != 0
//                 && (self.hold_frame[idx] < press_frame
//                     || self.should_hold[idx]
//                     || should_hold
//                     || *x != 1)
//             {
//                 self.hold_frame[idx] += 1;
//                 if self.hold_frame[idx] < press_frame {
//                     continue;
//                 }
//                 if *x == 1 {
//                     if self.should_hold[idx] {
//                         if self.hold_frame_max[idx] != -1
//                             && self.hold_frame_max[idx] < self.hold_frame[idx]
//                         {
//                             *x = 0;
//                             self.hold_frame[idx] = 0;
//                             continue;
//                         }
//                     }
//                 } else if should_hold {
//                     if max_hold_frame != -1 && max_hold_frame < self.hold_frame[idx] {
//                         *x = 0;
//                         self.hold_frame[idx] = 0;
//                         continue;
//                     }
//                 }
//                 self.on_last_frame |= 1 << idx;
//             } else {
//                 self.hold_frame[idx] = 0;
//                 *x = 0;
//             }
//         }
//     }
// }

// #[repr(C)]
// struct CommandFlagCat {
//     flags: u32,
//     _x4: u32,
//     count: usize,
//     lifetimes: *mut u8,
//     lifetimes2: *mut u8,
//     lifetimes3: *mut u64,
// }

// impl CommandFlagCat {
//     fn lifetimes(&self) -> &[u8] {
//         unsafe { std::slice::from_raw_parts(self.lifetimes, self.count) }
//     }

//     fn lifetimes_mut(&self) -> &mut [u8] {
//         unsafe { std::slice::from_raw_parts_mut(self.lifetimes, self.count) }
//     }

//     fn lifetimes2(&self) -> &[u8] {
//         unsafe { std::slice::from_raw_parts(self.lifetimes2, self.count) }
//     }
// }

// #[skyline::hook(offset = offsets::get_command_flag_cat())]
// fn get_command_flag_cat_replace(control_module: u64, cat: i32) -> u32 {
//     let boma = unsafe { *(control_module as *mut *mut BattleObjectModuleAccessor).add(1) };
//     let battle_object = unsafe { get_battle_object_from_id((*boma).battle_object_id) };

//     if cat == 4 {
//         if !has_input_module!(battle_object) {
//             return 0;
//         }

//         let im = require_input_module!(battle_object);

//         let mut output = 0;
//         // this iterates across all 32 bits of the output bitmask, where valid_frames represents how many frames
//         // left any given custom input may have left in its internal buffer state.
//         for x in 0..32 {
//             if im.hdr_cat.valid_frames[x] != 0 {
//                 output |= 1 << x;
//             }
//         }

//         return output;
//     }

//     let cats =
//         unsafe { std::slice::from_raw_parts((control_module + 0x568) as *mut CommandFlagCat, 4) };

//     let mut output = 0;
//     let lifetimes = cats[cat as usize].lifetimes();
//     let lifetimes2 = cats[cat as usize].lifetimes2();
//     for x in 0..cats[cat as usize].count {
//         if lifetimes[x] > 0 && lifetimes2[x] <= 1 {
//             output |= 1 << x;
//         }
//     }
//     output
// }

fn exec_internal(module_accessor: *mut BattleObjectModuleAccessor) {
    unsafe {
        // Prevent game from thinking you are inputting a flick on the frame the cstick stops overriding left stick
        if Buttons::from_bits_unchecked(ControlModule::get_release(module_accessor)).intersects(Buttons::CStickOverride) {
            ControlModule::reset_flick_x(module_accessor);
            ControlModule::reset_flick_y(module_accessor);
        }
    }
}

#[skyline::hook(offset = 0x6babf0)]
fn exec_command_hook(control_module: u64, flag: bool) {
    let boma = unsafe { *(control_module as *mut *mut BattleObjectModuleAccessor).add(1) };

    exec_internal(boma);
    call_original!(control_module, flag)
}

// These 2 hooks prevent buffered nair after inputting C-stick on first few frames of jumpsquat
// Both found in ControlModule::exec_command
#[skyline::hook(offset = 0x6be610)]
unsafe fn set_attack_air_stick_hook(control_module: u64, arg: u32) {
    // This check passes on the frame FighterControlModuleImpl::reserve_on_attack_button is called
    // Only happens during jumpsquat currently
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    if *((control_module + 0x645) as *const bool)
    && StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        return;
    }
    call_original!(control_module, arg);
}

#[skyline::hook(offset = 0x6bd6a4, inline)]
unsafe fn exec_command_reset_attack_air_kind_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[21].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    // For some reason, the game resets your attack_air_kind value every frame
    // even though it resets as soon as you perform an aerial attack
    // We don't want this to reset while in jumpsquat
    // to allow the game to use your initial C-stick input during jumpsquat for your attack_air_kind
    if StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        ControlModule::reset_attack_air_kind(boma);
    }
}

pub fn install() {
    // Prevents buffered C-stick aerials from triggering nair
    skyline::patching::Patch::in_text(0x6be644).data(0x52800040);

    // Prevents Aerial Kind resetting every frame
    skyline::patching::Patch::in_text(0x6bd6a4).nop();

    // Removes 10f C-stick lockout for tilt stick and special stick
    skyline::patching::Patch::in_text(0x17527dc).data(0x2A1F03FA);
    skyline::patching::Patch::in_text(0x17527e0).nop();
    skyline::patching::Patch::in_text(0x17527e4).nop();
    skyline::patching::Patch::in_text(0x17527e8).nop();

    skyline::install_hooks!(
        // get_command_flag_cat_replace,
        exec_command_hook,
        set_attack_air_stick_hook,
        exec_command_reset_attack_air_kind_hook
    );
}
