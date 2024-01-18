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
    pub name: Option<String>,
    pub short: Option<char>,
    pub description: Option<String>,
    pub has_arg: bool,
    pub required: bool,
}

impl Default for ParOptionBuilder {
    fn default() -> Self {
        Self {
            name: None,
            short: None,
            description: None,
            has_arg: false,
            required: false,
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

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn has_arg(mut self, has_arg: bool) -> Self {
        self.has_arg = has_arg;
        self
    }

    pub fn build(self) -> ParOption {
        ParOption {
            name: self.name.unwrap_or_default(),
            description: self.description,
            short: self.short.unwrap_or_default(),
            has_arg: self.has_arg,
            required: self.required,
        }
    }
}

pub struct ParOption {
    pub name: String,
    pub short: char,
    pub description: Option<String>,
    pub has_arg: bool,
    pub required: bool,
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
            required,
        }
    }
}

impl PartialEq for ParOption {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name 
            || self.short == other.short
    }
}
