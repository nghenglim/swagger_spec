use parameter::Parameter;
#[derive(Clone, Debug)]
pub struct PathItem {
    pub parameters: Vec<Parameter>
}
impl PathItem {
    pub fn new() -> PathItem {
        PathItem {
            parameters: Vec::new(),
        }
    }
    pub fn add_parameter(&mut self, p: Parameter) -> &mut Self {
        self.parameters.push(p);
        self
    }
}