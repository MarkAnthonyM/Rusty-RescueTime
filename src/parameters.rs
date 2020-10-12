#[derive(IntoStaticStr)]
pub enum PerspectiveOptions {
    Rank,
    Interval,
}

impl PerspectiveOptions {
    pub fn new(option: PerspectiveOptions) -> QueryParameter<PerspectiveOptions> {
        QueryParameter {
            parameter_name: "perspective",
            parameter_option: option,
        }
    }
}

#[derive(IntoStaticStr)]
pub enum ResolutionOptions {
    Month,
    Week,
    Day,
    Hour,
    Minute,
}

impl ResolutionOptions {
    pub fn new(option: ResolutionOptions) -> QueryParameter<ResolutionOptions> {
        QueryParameter {
            parameter_name: "resolution",
            parameter_option: option,
        }
    }
}

pub struct QueryParameter<T> {
    parameter_name: &'static str,
    parameter_option: T,
}

pub struct Parameters {
    pub perspective: Option<QueryParameter<PerspectiveOptions>>,
    pub resolution: Option<QueryParameter<ResolutionOptions>>,
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
        let mut url = format!("https://www.rescuetime.com/anapi/data?key={}", key);
        let query_parameters = self.process_fields();

        for parameter in query_parameters {
            url.push_str(&parameter);
        }

        let format_type = format!("&format={}", format);
        url.push_str(&format_type);

        url
    }
}