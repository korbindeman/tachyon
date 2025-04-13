use nanoid::nanoid;

pub const MEGABYTE: usize = 1024 * 1024;
pub const GIGABYTE: usize = 1024 * MEGABYTE;

const ID_LENGTH: usize = 5;

pub fn create_id() -> String {
    nanoid!(ID_LENGTH)
}
