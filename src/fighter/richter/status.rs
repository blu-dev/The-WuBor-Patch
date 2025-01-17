mod attack;
mod attack_lw3;
mod attack_air;
mod special_n;
mod special_hi;
mod special_lw;
mod holywater;

pub fn install() {
    attack::install();
    attack_lw3::install();
    attack_air::install();
    special_n::install();
    special_hi::install();
    special_lw::install();
    holywater::install();
}