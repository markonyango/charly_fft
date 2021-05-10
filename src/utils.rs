pub mod utils {
  pub fn power_of_two(n: i64) -> bool {
    (n != 0) && ((n & (n - 1)) == 0)
  }
}

#[cfg(test)]
mod tests {
  use super::utils::*;

  #[test]
  fn is_power_of_two() {
    assert_eq!(power_of_two(16), true);
    assert_eq!(power_of_two(4), true);
    assert_eq!(power_of_two(3), false);
  }
}
