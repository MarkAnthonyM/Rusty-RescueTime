enum QueryParameters{
    Perspective(PerspectiveOptions),
    Resolution(ResolutionOptions),
}

enum PerspectiveOptions {
    Rank,
    Interval,
}

enum ResolutionOptions {
    Month,
    Week,
    Day,
    Hour,
    Minute,
}