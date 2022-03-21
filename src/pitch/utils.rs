use itertools::Itertools;
use num_traits::signum;

fn zero_crossing_rate<I>(signal: I) -> usize
where
    I: IntoIterator<Item = f64>,
    <I as IntoIterator>::Item: std::borrow::Borrow<f64>,
{
    signal
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| (signum(a) - signum(b)).abs() as usize)
        .sum::<usize>()
        / 2
}
