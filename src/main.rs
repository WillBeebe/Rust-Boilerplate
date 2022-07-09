#[macro_use] mod rbp;
use rbp::*;

fn main() {
    vt_open();
    let mut capitals = vec_of_strings!["CITY", "London", "Paris", "Berlin", "Rome"];
    let ans = vt_menu(&mut capitals);
    vt_cls();
    let s0 = format!("You selected option {}.\n\n", ans);
    vt_put_slice(&s0);
    vt_close("Press any key to close ");
}