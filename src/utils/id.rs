use nanoid::nanoid;

use crate::utils::env::get_id_length;

pub fn create_id() -> String {
    let length = get_id_length();
    nanoid!(length)
}
