mod math {
  pub fn gcd<T: PartialOrd + Default + std::ops::Rem<Output = T> + Copy>(mut a: T, mut b: T) -> T {
      let zero = T::default();
      while a > zero {
          (a, b) = (b % a, a);
      }
      b
  }
}