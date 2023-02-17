extern crate gl33;
use glm::{*};
use sdl2::surface::Surface;
use std::fs::{File};
use std::io::{BufReader, BufRead};

pub struct Material{
    pub ambient : Vector4<f32>,
    pub diffuse : Vector4<f32>,
    pub specular : Vector4<f32>,
    pub shininess : f32,
    
}
pub struct Mesh{
    pub vertex_buffer: Vec<f32>,
    pub index_buffer : Vec<u32>,
    
}
pub struct Model<'a>{
    pub mesh : Mesh,
    pub material : Material,
    pub texture : &'a Surface<'a>,
    pub velocity : Vector3<f32>,
    pub rotational_velocity : Vector3<f32>
}


impl Mesh{


    pub fn load_obj_file(file_path: String) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let mut points: Vec<f32> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut normals: Vec<f32> = Vec::new();
        let mut vertices: Vec<f32> = Vec::new();
        let mut tex_coords: Vec<f32> = Vec::new();
        let mut index_uses: Vec<(usize,usize,usize)> = Vec::new();
        let obj_key: [&str; 4] = ["v", "f", "vt", "vn"];

        for line in reader.lines() {
            let ln = Box::leak(line.unwrap().into_boxed_str());
            let vals: Vec<&str> = ln.split_whitespace().collect();
            if !vals.is_empty() {
                if *vals[0] == *obj_key[0] {
                    points.extend([
                        vals[1].parse::<f32>().unwrap(),
                        vals[2].parse::<f32>().unwrap(),
                        vals[3].parse::<f32>().unwrap(),
                    ].iter());
                } else if *vals[0] == *obj_key[1] {
                    let p1: Vec<&str> = vals[1].split('/').collect();
                    let p2: Vec<&str> = vals[2].split('/').collect();
                    let p3: Vec<&str> = vals[3].split('/').collect();
                    for i in [p1,p2,p3].iter(){
                        let (point_i, tex_i, norm_i) = (
                            i[0].parse::<usize>().unwrap()-1,
                            i[1].parse::<usize>().unwrap()-1,
                            i[2].parse::<usize>().unwrap()-1
                        );
                        
                        if index_uses.contains(&(point_i,tex_i,norm_i)) {

                            indices.push(index_uses.iter().position(|&x|x==(point_i,tex_i,norm_i)).unwrap() as u32);
                        } else {
                            indices.push((vertices.len() as u32)/8);
                            vertices.extend(
                                [
                                    points[point_i*3],
                                    points[point_i*3+1],
                                    points[point_i*3+2],
                                    normals[norm_i*3],
                                    normals[norm_i*3+1],
                                    normals[norm_i*3+2],
                                    tex_coords[tex_i*2],
                                    tex_coords[tex_i*2+1],

                                ]
                            );
                            index_uses.push((point_i, tex_i, norm_i));
                        }
                    }
                } else if *vals[0] == *obj_key[2] {
                    tex_coords.extend([
                        1.0 - vals[1].parse::<f32>().unwrap(),
                        1.0 - vals[2].parse::<f32>().unwrap(),
                    ].iter());
                } else if *vals[0] == *obj_key[3] {
                    normals.extend(
                        [
                            vals[1].parse::<f32>().unwrap(),
                            vals[2].parse::<f32>().unwrap(),
                            vals[3].parse::<f32>().unwrap(),
                            
                        ].iter()
                    );
                    
                }
            }
        }
        Mesh { 
            vertex_buffer: vertices, 
            //tex_buffer: tex_coords, 
            //normal_buffer: normals, 
            index_buffer: indices
        }
    }

}


impl Model<'static>{

}