mod pathing;
mod config;
#[cfg(test)]
mod test;
mod binding;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}