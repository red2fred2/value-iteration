#[cfg(test)]
mod tests {
    #[test]
    fn add_2_2() {
        let result = value_iteration::add(2, 2);
        assert_eq!(result, 4);
    }
}
