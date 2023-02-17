use glm::*;
use crate::*;
pub struct ServerEngine{
    pub objects : Vec<Model<'static>>,
    pub lights : Vec<Light>
}