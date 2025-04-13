use nanoid::nanoid;

pub const KILOBYTE: usize = 1024;
pub const MEGABYTE: usize = usize::pow(KILOBYTE, 2);
pub const GIGABYTE: usize = usize::pow(MEGABYTE, 2);

const ID_LENGTH: usize = 5;

pub fn create_id() -> String {
    nanoid!(ID_LENGTH)
}
