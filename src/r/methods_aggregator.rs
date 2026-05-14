use super::data_classes::Aggregator;

pub fn aggregate<T, Init, Agg>(
    values: impl IntoIterator<Item = (String, T)>,
    agg: &mut Aggregator<T, Init, Agg>,
) where
    T: Clone,
    Init: Fn(&str, &T) -> T,
    Agg: Fn(&str, &T, &T) -> T,
{
    for (name, value) in values {
        agg.aggregate(&name, value);
    }
}
