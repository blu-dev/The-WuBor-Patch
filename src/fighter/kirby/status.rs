mod kirby;
mod ganon;
mod lucario;
mod ike;

pub fn install() {
    kirby::install();
    ganon::install();
    lucario::install();
    ike::install();
}