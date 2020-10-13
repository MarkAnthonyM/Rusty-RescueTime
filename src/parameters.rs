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
            parameter_name: "resolution_time",
            parameter_option: option,
        }
    }
}

struct RestrictDates {
    begin_date: &'static str,
    end_date: &'static str,
}

#[derive(IntoStaticStr)]
pub enum RestrictOptions {
    Category,
    Activity,
    Productivity,
    Document,
}

impl RestrictOptions {
    pub fn new(option: RestrictOptions) -> QueryParameter<RestrictOptions> {
        QueryParameter {
            parameter_name: "restrict_kind",
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

    fn process_fields(self) -> Vec<String> {
        let mut query_parameters = Vec::new();

        if self.perspective.is_some() {
            let parameter_struct = self.perspective.unwrap();
            let parameter_name = parameter_struct.parameter_name;
            let parameter_option: &'static str = parameter_struct.parameter_option.into();
            let parameter_string = format!("&{}={}", parameter_name, parameter_option.to_lowercase());
            query_parameters.push(parameter_string);
        }

        if self.resolution.is_some() {
            let parameter_struct = self.resolution.unwrap();
            let parameter_name = parameter_struct.parameter_name;
            let parameter_option: &'static str = parameter_struct.parameter_option.into();
            let parameter_string = format!("&{}={}", parameter_name, parameter_option.to_lowercase());
            query_parameters.push(parameter_string);
        }

        query_parameters
    }
}