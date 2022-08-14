pub fn if_short_circuit_and(a: i32, b: i32, c: i32, d: i32) -> bool {
    if a < b {
        c < d
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // llvm-cov 100% if all are enabled.

    // llvm-cov 85.71% if only this is enabled.
    #[test]
    pub fn test_short_circuit_and_first_false() {
        assert_eq!(if_short_circuit_and(20, 10, 30, 40), false);
    }

    // llvm-cov 85.71% if only this is enabled
    #[test]
    pub fn test_short_circuit_and_both_true() {
        assert_eq!(if_short_circuit_and(10, 20, 30, 40), true);
    }
}
