use algorithms::algorithms::weighted_median::weighted_median;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn _weighted_median_book_test() {
    let values = [3, 8, 2, 5, 4, 6, 1];
    let weights = [0.12, 0.35, 0.025, 0.08, 0.15, 0.2, 0.075];
    let result = weighted_median(&values, &weights);
    println!("{:?}", result);

    for (curr_v, curr_w) in values.iter().zip(weights.iter()) {
        let mut cost = 0.0;
        for (v, w) in values.iter().zip(weights.iter()) {
            let diff: i32 = curr_v - v;
            cost += w * diff.abs() as f64;
        }
        println!("{} {}: cost = {}", curr_v, curr_w, cost);
    }
}

fn two_weighted_median(values: &[Point], weights: &[f64]) -> Option<Point> {
    let v_x = values.iter().map(|p| p.x).collect::<Vec<_>>();
    let v_y = values.iter().map(|p| p.y).collect::<Vec<_>>();

    let result_x = weighted_median(&v_x, weights);
    let result_y = weighted_median(&v_y, weights);

    Some(Point {
        x: *(result_x?),
        y: *(result_y?),
    })
}

fn min_max_interval_testing(values: &[Point], weights: &[f64]) -> Option<Point> {
    let mut min_cost = f64::MAX;
    let mut min_point = None;
    let (x_min, y_min) = values.iter().fold((i32::MAX, i32::MAX), |acc, p| {
        (acc.0.min(p.x), acc.1.min(p.y))
    });
    let (x_max, y_max) = values.iter().fold((i32::MIN, i32::MIN), |acc, p| {
        (acc.0.max(p.x), acc.1.max(p.y))
    });

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let curr_v = Point { x, y };
            let mut cost = 0.0;
            for (v, w) in values.iter().zip(weights.iter()) {
                let dx = curr_v.x - v.x;
                let dy = curr_v.y - v.y;
                cost += w * (dx.abs() + dy.abs()) as f64;
            }
            if cost < min_cost {
                min_cost = cost;
                min_point = Some(curr_v);
            }
        }
    }
    min_point
}

fn main() {
    let values = [
        // Point { x: 0, y: 0 }, // should be the result
        Point { x: 1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: -1, y: 0 },
        Point { x: 0, y: -1 },
        Point { x: 1, y: 1 },
        Point { x: -1, y: -1 },
    ];
    let weights = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    // let weights = [20.0, 1.0, 1.0, 1.0, 50.0, 1.0];
    let sum_exp: f64 = weights.iter().map(|i: &f64| i.exp()).sum();
    let weights: Vec<f64> = weights.iter().map(|i| i.exp() / sum_exp).collect();

    let now = std::time::Instant::now();
    let r = two_weighted_median(&values, &weights);
    println!("{:?}", now.elapsed());
    println!("{:?}", r);

    let now = std::time::Instant::now();
    let r = min_max_interval_testing(&values, &weights);
    println!("{:?}", now.elapsed());

    println!("{:?}", r);
}
