mod submodule_a;
mod submodule_b;

pub const COMMON_THING: &str = "hehehe";

pub fn common_fn() -> String {
    "jajaja".to_owned()
}

pub use submodule_a::thing_a;
pub use submodule_b::thing_b;
