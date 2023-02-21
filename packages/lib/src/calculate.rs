/// 计算泛型数组的平均数
pub fn get_arithmetic_mean(vec: &Vec<f64>) -> f64 {
    let length = vec.len();
    let mut sum = vec[0];
    for i in 1..length {
        sum = sum + vec[i];
    }

    return sum / length as f64;
}

/// 计算泛型数组的标准差
pub fn get_standard_deviation(vec: &Vec<f64>) -> f64 {
    let arithmetic_mean = get_arithmetic_mean(vec);
    let length = vec.len();
    let mut sum = (vec[0] - arithmetic_mean).powf(2.0);

    for i in 1..length {
        sum = sum + (vec[i] - arithmetic_mean).powf(2.0);
    }

    return (sum / length as f64).powf(0.5);
}
