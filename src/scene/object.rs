extern crate gl33;
use glm::{*};
use crate::types::*;
use sdl2::surface::Surface;
use std::fs::{File};
use std::io::{BufReader, BufRead};

use super::ViewObject;

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
pub struct Model{
    pub mesh : Mesh,
    pub material : Material,
    pub texture : &'static str,
    pub view_obj : ViewObject
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

                            indices.push(index_uses.iter().position(|&x|x==(point_i, tex_i, norm_i)).unwrap() as u32);
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

                                    tex_coords[tex_i*2+1],
                                    tex_coords[tex_i*2],

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
            index_buffer: indices
        }
    }

    pub fn triangle_buffer(&self) -> Vec<[[f32;8];3]> {
        let mut triangle_buffer : Vec<[[f32;8];3]> = Vec::new();
        let x = | index : usize | -> [f32;8] {
            let i = index*8; 
            return [
                self.vertex_buffer[i],
                self.vertex_buffer[i+1],
                self.vertex_buffer[i+2],

                self.vertex_buffer[i+3],
                self.vertex_buffer[i+4],
                self.vertex_buffer[i+5],

                self.vertex_buffer[i+6],
                self.vertex_buffer[i+7],

                ]
        };
        for i in 0..self.index_buffer.len()/3{
            triangle_buffer.push([x(i), x(i+1), x(i+2)]);
        }
        return triangle_buffer;
    }

    pub fn approximate_normals(&mut self) {
        for i in 0..self.index_buffer.len()/3 {
            let tri1 = [self.index_buffer[i*3] as usize, self.index_buffer[i*3+1] as usize, self.index_buffer[i*3+2] as usize];
            
            for p1 in tri1{
                let pt = [self.vertex_buffer[p1], self.vertex_buffer[p1+1], self.vertex_buffer[p1+2]];
                let mut normal = Vec3::new(0.0,0.0,0.0);
                for o in 0..self.index_buffer.len()/3 {
                    let tri2 = [self.index_buffer[o*3] as usize, self.index_buffer[o*3+1] as usize, self.index_buffer[o*3+2] as usize];
                    if 
                        (pt[0] == self.vertex_buffer[tri2[0]] && pt[1] == self.vertex_buffer[tri2[0]+1] && pt[1] == self.vertex_buffer[tri2[0]+2]) ^ 
                        (pt[0] == self.vertex_buffer[tri2[1]] && pt[1] == self.vertex_buffer[tri2[1]+1] && pt[1] == self.vertex_buffer[tri2[1]+2]) ^
                        (pt[0] == self.vertex_buffer[tri2[2]] && pt[1] == self.vertex_buffer[tri2[2]+1] && pt[1] == self.vertex_buffer[tri2[2]+2])
                    {
                        
                        normal += Vec3::new(
                            self.vertex_buffer[tri2[0]+3],
                            self.vertex_buffer[tri2[0]+4], 
                            self.vertex_buffer[tri2[0]+5]
                        ).cross(
                            &Vec3::new(
                                self.vertex_buffer[tri2[1]+3],
                                self.vertex_buffer[tri2[1]+4],
                                self.vertex_buffer[tri2[1]+5]

                            )
                        );
                    }
                }
                
                normal = normal.normalize();
                self.vertex_buffer[p1+3] = normal.x;
                self.vertex_buffer[p1+4] = normal.y;
                self.vertex_buffer[p1+5] = normal.z;
            }
        }
    }
}


impl Model{

}