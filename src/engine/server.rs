use glm::*;
use crate::*;
pub struct ServerEngine{
    pub objects : Vec<Model>,
    pub lights : Vec<Light>
}


impl ServerEngine{
    pub fn update_scene(&mut self, step : f32){
        for i in 0..self.objects.len(){

            self.objects[i].view_obj.update_object(step)
        }
    }
}