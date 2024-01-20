pub struct Options {
    pub options: Vec<ParOption>
}

impl Default for Options {
    fn default() -> Self { 
        Self {
            options: Vec::new()
        }
    }
}

impl Options {
    pub fn add_option(&mut self, option: ParOption) -> bool {
        if self.exists(&option) {
            false
        } else {
            self.options.push(option);
            true
        }
    }

    /// Checks if `option` already exists in the
    /// `options` container.
    pub fn exists(&self, option: &ParOption) -> bool {
        self.options.iter()
            .any(|inside_option| {
                inside_option == option
            })
    }

    /// Checks if there is an option with the provided name.
    pub fn has_option_with_name(&self, option_name: &str) -> bool {
        self.options.iter()
            .any(|option| option.name == option_name)
    }
}

pub struct ParOptionBuilder {
    name: Option<String>,
    short: Option<char>,
    description: Option<String>,
    has_arg: bool,
    required: bool,
    default_value: Option<String>,
}

impl Default for ParOptionBuilder {
    fn default() -> Self {
        Self {
            name: None,
            short: None,
            description: None,
            has_arg: false,
            required: false,
            default_value: None
        }
    }
}

impl ParOptionBuilder {
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Marks the option as required (*meaning that if the option
    /// doesn't appear in the arguments when parsing, this will fail*)
    /// NOTE: *If `required` is set to `true`, and the default value has 
    /// been set to something, this will automatically be set to `None`*.
    pub fn required(mut self, required: bool) -> Self {
        if required && self.default_value.is_some() {
            self.default_value = None;
        }

        self.required = required;
        self
    }

    pub fn has_arg(mut self, has_arg: bool) -> Self {
        self.has_arg = has_arg;
        self
    }

    /// Defines the default value that the option will get in case
    /// that it doesn't get specified in the arguments.
    /// NOTE: *If `required` is set to `true`, this attribute will 
    /// automatically be turned into `false`*
    pub fn default_value(mut self, default_value: String) -> Self {
        if self.required {
            self.required = false
        }
        self.default_value = Some(default_value);
        self
    }

    pub fn build(self) -> ParOption {
        ParOption {
            name: self.name.unwrap_or_default(),
            description: self.description,
            short: self.short.unwrap_or_default(),
            has_arg: self.has_arg,
            required: self.required,
            default_value: self.default_value
        }
    }
}

pub struct ParOption {
    pub name: String,
    pub short: char,
    pub description: Option<String>,
    pub has_arg: bool,
    pub required: bool,
    pub default_value: Option<String>
}

impl ParOption {
    pub fn new(name: String, has_arg: bool, description: Option<&str>, required: bool) -> Self {
        // TODO:
        // - Handle situations where the name is empty.
        // - Make sure that a name is valid
        Self {
            name: name.to_string(),
            has_arg,
            description: description.map_or_else(|| None, |desc| Some(desc.to_string())),
            short: name.chars().next().unwrap(),
            default_value: None,
            required,
        }
    }

    /// Returns a default instance of `ParOptionBuilder`
    pub fn builder() -> ParOptionBuilder {
        ParOptionBuilder::default()
    }
}

impl PartialEq for ParOption {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name 
            || self.short == other.short
    }
}
