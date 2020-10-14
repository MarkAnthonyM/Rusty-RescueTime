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

pub enum RestrictData {
    Date(&'static str, &'static str),
    Thing(&'static str),
    Thingy(&'static str),
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
            },
            RestrictData::Thing(_) => {
                let parameter_struct = self.process_thing().unwrap();
                let query_string = format!("&{}={}", parameter_struct.parameter_name, parameter_struct.parameter_option);

                query_string
            },
            RestrictData::Thingy(_) => {
                let parameter_struct = self.process_thingy().unwrap();
                let query_string = format!("&{}={}", parameter_struct.parameter_name, parameter_struct.parameter_option);

                query_string
            }
        }
    }

    fn process_date(self) -> Result<(QueryParameter<&'static str>, QueryParameter<&'static str>), String> {
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

    fn process_thing(self) -> Result<QueryParameter<&'static str>, String> {
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

    fn process_thingy(self) -> Result<QueryParameter<&'static str>, String> {
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

// pub struct RestrictData {
//     begin_date: &'static str,
//     end_date: &'static str,
// }

// impl RestrictData {
//     pub fn new(begin_date: &'static str, end_date: &'static str) -> (QueryParameter<&'static str>, QueryParameter<&'static str>) {
//         let begin = QueryParameter {
//             parameter_name: "restrict_begin",
//             parameter_option: begin_date,
//         };

//         let end = QueryParameter {
//             parameter_name: "restrict_end",
//             parameter_option: end_date,
//         };

//         (begin, end)
//     }
// }

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
    pub restrict_date: Option<RestrictData>,
    pub restrict_kind: Option<QueryParameter<RestrictOptions>>,
    pub restrict_thing: Option<RestrictData>,
    pub restrict_thingy: Option<RestrictData>,
}

impl Parameters {
    pub fn new(
        perspective: Option<QueryParameter<PerspectiveOptions>>,
        resolution: Option<QueryParameter<ResolutionOptions>>,
        restrict_date: Option<RestrictData>,
        restrict_kind: Option<QueryParameter<RestrictOptions>>,
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

        if self.restrict_date.is_some() {
            let parameter_string = self.restrict_date.unwrap().build_query_string();

            query_parameters.push(parameter_string);
        }

        if self.restrict_thing.is_some() {
            let parameter_string = self.restrict_thing.unwrap().build_query_string();

            query_parameters.push(parameter_string);
        }

        query_parameters
    }
}