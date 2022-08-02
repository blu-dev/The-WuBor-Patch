mod dash;
mod attack;
mod attacklw3;

pub fn install() {
    dash::install();
    attack::install();
    attacklw3::install();
}