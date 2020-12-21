use std::env;
use std::string::String;
mod helpers;
use helpers::*;
extern crate peg;

static mut UID: i32 = 0;

peg::parser!( grammar resolver() for str {
    rule eq() -> bool
        = n:$("uid=="['0'..='9']+) {
            let v = uid_from_str(n,5);
            unsafe{
                UID == v
            }
    }

    rule gt() -> bool
        = n:$("uid>"['0'..='9']+) {
            let v = uid_from_str(n,4);
            unsafe{
                UID > v
            }
        }

    rule lt() -> bool
        = n:$("uid<"['0'..='9']+) {
            let v = uid_from_str(n,4);
            unsafe{
                UID < v
            }
        }

    pub(crate) rule calculate() -> bool = precedence!{
        x:(@) "|" y:@ { x || y }
        x:(@) "&" y:@ { x && y }
        --
        "(" v:calculate() ")" { v }
        n:eq() {n}
        n:gt() {n}
        n:lt() {n}
    }

});

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = helpers::get_rules(&args);
    unsafe {
        UID = get_uid(&args);
    }

    for (n, s) in input.iter().enumerate() {
        match resolver::calculate(&s) {
            Ok(res) => unsafe {
                println!("{}. {} --> {:?} with UID={}", n, s, res, UID);
            },
            Err(e) => {
                println!("{}. {} --> syntax error. {}", n, s, e);
            }
        }
    }
}
