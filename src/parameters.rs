enum QueryParameters{
    Perspective(PerspectiveOptions),
    Resolution(ResolutionOptions),
}

enum PerspectiveOptions {
    Rank,
    Interval,
}

#[derive(IntoStaticStr)]
enum ResolutionOptions {
    Month,
    Week,
    Day,
    Hour,
    Minute,
}

pub struct Parameters {
    perspective: Option<QueryParameters>,
    resolution_time: Option<QueryParameters>,
}