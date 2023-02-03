use std::fs::{File};
use std::io::{BufReader, BufRead};


pub fn load_obj_file(file_path: String, tex: String, col: vec4, rfl: f32, trs: f32) -> Self {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut ts: Vec<Vertex> = Vec::new();
    let mut t_n: Vec<Vertex> = Vec::new();
    let mut points: Vec<Vertex> = Vec::new();
    let mut t_c: Vec<Vertex> = Vec::new();
    let obj_key: [&str; 4] = ["v", "f", "vt", "vn"];

    for line in reader.lines() {
        let ln = Box::leak(line.unwrap().into_boxed_str());
        let vals: Vec<&str> = ln.split_whitespace().collect();
        if !vals.is_empty() {
            if *vals[0] == *obj_key[0] {
                points.push([
                    vals[1].parse::<f32>().unwrap(),
                    vals[2].parse::<f32>().unwrap(),
                    vals[3].parse::<f32>().unwrap(),
                ]);
            } else if *vals[0] == *obj_key[1] {
                let p1: Vec<&str> = vals[1].split('/').collect();
                let p2: Vec<&str> = vals[2].split('/').collect();
                let p3: Vec<&str> = vals[3].split('/').collect();
                if p1.len() == 2 {
                    ts.push(Tri3d::new(
                        [
                            points[p1[0].parse::<usize>().unwrap() - 1],
                            points[p2[0].parse::<usize>().unwrap() - 1],
                            points[p3[0].parse::<usize>().unwrap() - 1],
                        ],
                        [
                            t_c[p1[1].parse::<usize>().unwrap() - 1],
                            t_c[p2[1].parse::<usize>().unwrap() - 1],
                            t_c[p3[1].parse::<usize>().unwrap() - 1],
                        ],
                        [
                            [0.0, 0.0, 0.0, 1.0],
                            [0.0, 0.0, 0.0, 1.0],
                            [0.0, 0.0, 0.0, 1.0],
                        ],
                        col,
                        rfl,
                        trs,
                    ));
                } else if p1.len() == 1 {
                    ts.push(Tri3d::new(
                        [
                            points[vals[1].parse::<usize>().unwrap() - 1],
                            points[vals[2].parse::<usize>().unwrap() - 1],
                            points[vals[3].parse::<usize>().unwrap() - 1],
                        ],
                        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
                        [
                            [0.0, 0.0, 0.0, 1.0],
                            [0.0, 0.0, 0.0, 1.0],
                            [0.0, 0.0, 0.0, 1.0],
                        ],
                        col,
                        rfl,
                        trs,
                    ));
                } else if p1.len() == 3 {
                    ts.push(Tri3d::new(
                        [
                            points[p1[0].parse::<usize>().unwrap() - 1],
                            points[p2[0].parse::<usize>().unwrap() - 1],
                            points[p3[0].parse::<usize>().unwrap() - 1],
                        ],
                        [
                            t_c[p1[1].parse::<usize>().unwrap() - 1],
                            t_c[p2[1].parse::<usize>().unwrap() - 1],
                            t_c[p3[1].parse::<usize>().unwrap() - 1],
                        ],
                        [
                            t_n[p1[2].parse::<usize>().unwrap() - 1],
                            t_n[p2[2].parse::<usize>().unwrap() - 1],
                            t_n[p3[2].parse::<usize>().unwrap() - 1],
                        ],
                        col,
                        rfl,
                        trs,
                    ));
                }
            } else if *vals[0] == *obj_key[2] {
                t_c.push([
                    1.0 - vals[1].parse::<f32>().unwrap(),
                    1.0 - vals[2].parse::<f32>().unwrap(),
                    1.0,
                ]);
            } else if *vals[0] == *obj_key[3] {
                t_n.push(
                    [
                        vals[1].parse::<f32>().unwrap(),
                        vals[2].parse::<f32>().unwrap(),
                        vals[3].parse::<f32>().unwrap(),
                        1.0,
                    ]
                    .normalize(),
                )
            }
        }
    }
    Mesh {
        tris: ts,
        vel: [0.0, 0.0, 0.0, 0.0],
        rot_vel: [0.0, 0.0, 0.0, 0.0],
        tex,
    }
}
