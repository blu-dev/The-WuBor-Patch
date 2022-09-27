pub mod dash;
mod run;
mod run_brake;
mod jump_squat;
mod jump;
mod tread_jump;
mod shield;
mod escape;
mod escape_air;
mod passive;
pub mod attack;
mod attack_100;
mod attack_dash;
mod attack_3;
// mod attack_3_common;
mod attack_4;
mod attack_air;
mod catch;
mod capture_pulled;
mod damage;
mod damage_air;
mod damage_fall;
mod damage_fly;
mod damage_fly_roll;
mod damage_fly_reflect;
mod landing;
mod pass;
mod cliff;
mod appeal;
mod sub_transitions;

pub fn install() {
    dash::install();
    run::install();
    run_brake::install();
    jump_squat::install();
    jump::install();
    tread_jump::install();
    shield::install();
    escape::install();
    escape_air::install();
    passive::install();
    attack::install();
    attack_100::install();
    attack_dash::install();
    attack_3::install();
    // attack_3_common::install();
    attack_4::install();
    attack_air::install();
    catch::install();
    capture_pulled::install();
    damage::install();
    damage_air::install();
    damage_fall::install();
    damage_fly::install();
    damage_fly_roll::install();
    damage_fly_reflect::install();
    landing::install();
    pass::install();
    cliff::install();
    appeal::install();
    sub_transitions::install();
}