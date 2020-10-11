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
    parameter_name: &'static str,
    parameter_option: T,
}

impl<T> QueryParameter<T> {
    pub fn new(name: &'static str, option: T) -> Self {
        QueryParameter {
            parameter_name: name,
            parameter_option: option,
        }
    }
}

pub struct Parameters {
    pub perspective: Option<QueryParameter<PerspectiveOptions>>,
    pub resolution_time: Option<QueryParameter<ResolutionOptions>>,
}

impl Parameters {
    pub fn new(perspective: Option<QueryParameter<PerspectiveOptions>>, resolution: Option<QueryParameter<ResolutionOptions>>) -> Self {
        Parameters {
            perspective,
            resolution,
        }
    }
}

impl Parameters {
    pub fn construct_query(self, key: &String, format: String) -> String {
        let perspective_parameter: &'static str  = self.perspective.unwrap().parameter_option.into();
        let url = format!("https://www.rescuetime.com/anapi/data?key={}&perspective={}&format={}", key, perspective_parameter, format);

        url
    }
}