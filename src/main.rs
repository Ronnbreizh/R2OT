mod interface;
mod mail;
pub mod r2ot;
pub mod task;
pub mod event;

use r2ot::R2ot;

fn main() {
    let app = R2ot::new();
    app.run().unwrap();
}
