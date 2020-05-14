use super::{common_fn, COMMON_THING};

pub fn thing_a() -> String {
    common_fn() + COMMON_THING + "thing_a"
}
