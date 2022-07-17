pub fn f64_zequals(left: f64, right: f64, epsilon: Option<f64>) -> bool {
    if epsilon.is_some() {
        return (left - right).abs() < epsilon.unwrap();
    } else {
        return (left - right).abs() < 0.00001;
    }
}
