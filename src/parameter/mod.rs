use Schema;
#[derive(Clone, Debug)]
pub enum ParameterIn {
    QUERY,
}
#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: Option<String>,
    pub schema: Option<Schema>,
    pub in_: Option<ParameterIn>,
    pub description: Option<String>,
    pub required: bool,
    pub deprecated: bool,
    pub allow_empty_value: bool
}
impl Parameter {
    pub fn new() -> Parameter {
        Parameter {
            name: None,
            schema: None,
            in_: None,
            description: None,
            required: false,
            deprecated: false,
            allow_empty_value: false
        }
    }
    pub fn name<S: Into<String>>(&mut self, s: S) -> &mut Self {
        self.name = Some(s.into());
        self
    } 
    pub fn schema(&mut self, s: Schema) -> &mut Self {
        self.schema = Some(s);
        self
    } 
    pub fn in_(&mut self, p: ParameterIn) -> &mut Self {
        self.in_ = Some(p);
        self
    } 
    pub fn description<S: Into<String>>(&mut self, s: S) -> &mut Self {
        self.description = Some(s.into());
        self
    } 
    pub fn required(&mut self, b: bool) -> &mut Self {
        self.required = b;
        self
    }
    pub fn deprecated(&mut self, b: bool) -> &mut Self {
        self.deprecated = b;
        self
    }  
    pub fn allow_empty_value(&mut self, b: bool) -> &mut Self {
        self.allow_empty_value = b;
        self
    }
}
