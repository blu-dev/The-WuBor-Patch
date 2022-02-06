pub mod dash;
mod run;
mod jump_squat;
mod tread_jump;
mod shield;
mod escape;
pub mod attack;
mod attack_3;
// mod attack_3_common;
mod attack_4;
mod attack_air;
mod catch;
mod damage;
mod damage_air;
mod damage_fall;
mod landing;
mod appeal;
mod sub_transitions;

pub fn install() {
    dash::install();
    run::install();
    jump_squat::install();
    tread_jump::install();
    shield::install();
    escape::install();
    attack::install();
    attack_3::install();
    // attack_3_common::install();
    attack_4::install();
    attack_air::install();
    catch::install();
    damage::install();
    damage_air::install();
    damage_fall::install();
    landing::install();
    appeal::install();
    sub_transitions::install();
}