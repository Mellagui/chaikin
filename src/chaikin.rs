pub fn chaikin(points: &Vec<(f32, f32)>, iterations: usize) -> Vec<(f32, f32)> {
    let mut result = points.clone();

    for _ in 0..iterations {
        let mut new_points = Vec::new();
        new_points.push(points[0]);

        for i in 0..result.len() - 1 {
            let p0 = result[i];
            let p1 = result[i + 1];

            let q = (0.75 * p0.0 + 0.25 * p1.0, 0.75 * p0.1 + 0.25 * p1.1);
            let r = (0.25 * p0.0 + 0.75 * p1.0, 0.25 * p0.1 + 0.75 * p1.1);

            new_points.push(q);
            new_points.push(r);
        }

        new_points.push(points[points.len() - 1]);
        result = new_points;
    }

    result
}