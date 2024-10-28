pub fn weighted_median<'a, T: PartialOrd>(values: &'a [T], weights: &[f64]) -> Option<&'a T> {
    let mut ordered: Vec<(&T, &f64)> = values.iter().zip(weights.iter()).collect();
    ordered.sort_unstable_by(|(av, _aw), (bv, _bw)| av.partial_cmp(bv).unwrap());
    let halfweights_sum: f64 = weights.iter().sum();
    let halfweights_sum = halfweights_sum / 2.0;
    let mut sum = 0.0;
    for (v, w) in ordered {
        sum += w;
        if sum >= halfweights_sum {
            return Some(v);
        }
    }
    None
}
