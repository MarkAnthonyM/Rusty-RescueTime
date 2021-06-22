#[derive(IntoStaticStr)]
pub enum PerspectiveOptions {
    Rank,
    Interval,
}

impl PerspectiveOptions {
    fn build_query_string(self) -> String {
        self.format_query()
    }

    //TODO: This function might be redundant. Maybe be possible to move this logic to build_query_string method
    fn format_query(self) -> String {
        let query_option: &'static str = self.into();
        let query_struct = QueryParameter {
            parameter_name: "perspective",
            parameter_option: query_option.to_lowercase(),
        };
        let query_string = format!(
            "&{}={}",
            query_struct.parameter_name, query_struct.parameter_option
        );

        query_string
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
    // Build string that queries for resolution information
    fn build_query_string(self) -> String {
        self.format_query()
    }

    //TODO: Find way to implement this as a generic trait to cut down on code repetition
    fn format_query(self) -> String {
        let query_option: &'static str = self.into();
        let query_struct = QueryParameter {
            parameter_name: "resolution_time",
            parameter_option: query_option.to_lowercase(),
        };
        let query_string = format!(
            "&{}={}",
            query_struct.parameter_name, query_struct.parameter_option
        );

        query_string
    }
}

pub enum RestrictData {
    Date(String, String),
    Thing(String),
    Thingy(String),
}

impl RestrictData {
    fn build_query_string(self) -> String {
        match self {
            RestrictData::Date(_, _) => {
                let parameter_structs = self.process_date().unwrap();
                let query_string = format!(
                    "&{}={}&{}={}",
                    parameter_structs.0.parameter_name,
                    parameter_structs.0.parameter_option,
                    parameter_structs.1.parameter_name,
                    parameter_structs.1.parameter_option,
                );

                query_string
            }
            RestrictData::Thing(_) => {
                let parameter_struct = self.process_thing().unwrap();
                let query_string = format!(
                    "&{}={}",
                    parameter_struct.parameter_name, parameter_struct.parameter_option
                );

                query_string
            }
            RestrictData::Thingy(_) => {
                let parameter_struct = self.process_thingy().unwrap();
                let query_string = format!(
                    "&{}={}",
                    parameter_struct.parameter_name, parameter_struct.parameter_option
                );

                query_string
            }
        }
    }

    fn process_date(self) -> Result<(QueryParameter<String>, QueryParameter<String>), String> {
        if let RestrictData::Date(begin_date, end_date) = self {
            let restrict_begin = QueryParameter {
                parameter_name: "restrict_begin",
                parameter_option: begin_date,
            };

            let restrict_end = QueryParameter {
                parameter_name: "restrict_end",
                parameter_option: end_date,
            };

            Ok((restrict_begin, restrict_end))
        } else {
            let error = String::from("Process_date method failed");

            Err(error)
        }
    }

    fn process_thing(self) -> Result<QueryParameter<String>, String> {
        if let RestrictData::Thing(thing) = self {
            let restrict_thing = QueryParameter {
                parameter_name: "restrict_thing",
                parameter_option: thing,
            };

            let result = Ok(restrict_thing);
            result
        } else {
            let error = String::from("Process_thing method failed");
            Err(error)
        }
    }

    fn process_thingy(self) -> Result<QueryParameter<String>, String> {
        if let RestrictData::Thingy(thingy) = self {
            let restrict_thingy = QueryParameter {
                parameter_name: "restrict_thingy",
                parameter_option: thingy,
            };

            let result = Ok(restrict_thingy);
            result
        } else {
            let error = String::from("Process_thingy method failed");
            Err(error)
        }
    }
}

#[derive(IntoStaticStr)]
pub enum RestrictOptions {
    Activity,
    Category,
    Document,
    Overview,
    Productivity,
}

impl RestrictOptions {
    fn build_query_string(self) -> String {
        let query_option: &'static str = self.into();
        let query_struct = QueryParameter {
            parameter_name: "restrict_kind",
            parameter_option: query_option.to_lowercase(),
        };
        let query_string = format!(
            "&{}={}",
            query_struct.parameter_name, query_struct.parameter_option
        );

        query_string
    }
}

pub struct QueryParameter<T> {
    parameter_name: &'static str,
    parameter_option: T,
}

pub struct Parameters {
    pub perspective: Option<PerspectiveOptions>,
    pub resolution: Option<ResolutionOptions>,
    pub restrict_date: Option<RestrictData>,
    pub restrict_kind: Option<RestrictOptions>,
    pub restrict_thing: Option<RestrictData>,
    pub restrict_thingy: Option<RestrictData>,
}

impl Parameters {
    pub fn new(
        perspective: Option<PerspectiveOptions>,
        resolution: Option<ResolutionOptions>,
        restrict_date: Option<RestrictData>,
        restrict_kind: Option<RestrictOptions>,
        restrict_thing: Option<RestrictData>,
        restrict_thingy: Option<RestrictData>,
    ) -> Self {
        Parameters {
            perspective,
            resolution,
            restrict_date,
            restrict_kind,
            restrict_thing,
            restrict_thingy,
        }
    }

    // Iterate through vector of parameter strings, concatenate snippets, and return full url query string
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
            let parameter_string = self.perspective.unwrap().build_query_string();

            query_parameters.push(parameter_string);
        }

        if self.resolution.is_some() {
            let parameter_string = self.resolution.unwrap().build_query_string();

            query_parameters.push(parameter_string);
        }

        if self.restrict_date.is_some() {
            let parameter_string = self.restrict_date.unwrap().build_query_string();

            query_parameters.push(parameter_string);
        }

        if self.restrict_kind.is_some() {
            let parameter_string = self.restrict_kind.unwrap().build_query_string();

            query_parameters.push(parameter_string);
        }

        if self.restrict_thing.is_some() {
            let parameter_string = self.restrict_thing.unwrap().build_query_string();

            query_parameters.push(parameter_string);
        }

        if self.restrict_thingy.is_some() {
            let parameter_string = self.restrict_thingy.unwrap().build_query_string();

            query_parameters.push(parameter_string);
        }

        query_parameters
    }
}
