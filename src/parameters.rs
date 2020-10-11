// enum QueryParameters{
//     Perspective(PerspectiveOptions),
//     Resolution(ResolutionOptions),
// }

#[derive(IntoStaticStr)]
pub enum PerspectiveOptions {
    Rank,
    Interval,
}

#[derive(IntoStaticStr)]
pub enum ResolutionOptions {
    Month,
    Week,
    Day,
    Hour,
    Minute,
}

pub struct QueryParameter<T> {
    parameter_name: String,
    parameter_option: T,
}

pub struct Parameters {
    pub perspective: Option<QueryParameter<PerspectiveOptions>>,
    pub resolution_time: Option<QueryParameter<ResolutionOptions>>,
}

impl Parameters {
    pub fn construct_query(self, key: &String, format: String) -> String {
        let perspective_parameter: &'static str  = self.perspective.unwrap().into();
        let url = format!("https://www.rescuetime.com/anapi/data?key={}&perspective={}&format={}", key, perspective_parameter, format);

        url
    }
}