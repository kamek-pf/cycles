use super::{common_fn, COMMON_THING};

pub fn thing_b() -> String {
    common_fn() + COMMON_THING + "thing_b"
}
