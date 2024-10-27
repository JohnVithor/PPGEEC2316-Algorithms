pub fn weighted_median<'a, T: PartialOrd>(values: &'a [T], weights: &[f64]) -> Option<&'a T> {
    let mut ordered: Vec<(&T, &f64)> = values.iter().zip(weights.iter()).collect();
    ordered.sort_unstable_by(|(av, _aw), (bv, _bw)| av.partial_cmp(bv).unwrap());
    let mut sum = 0.0;
    for (v, w) in ordered {
        sum += w;
        if sum >= 0.5 {
            return Some(v);
            // return Some(values.iter().position(|&x| x == *v).unwrap());
        }
    }
    None
}
