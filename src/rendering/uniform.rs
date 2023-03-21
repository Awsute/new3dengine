use gl33::*;
use crate::types::*;
pub unsafe fn uniform_matrix4(gl : &GlFns, shader_program : u32, name : &str, value : *const f32) {
    gl.UniformMatrix4fv(
        gl.GetUniformLocation(shader_program, format!("{}\0", name.to_string()).as_ptr()), 
        1, 
        GL_FALSE.0.try_into().unwrap(), 
        value
    );
}
pub unsafe fn uniform_matrix3(gl : &GlFns, shader_program : u32, name : &str, value : *const f32) {
    gl.UniformMatrix3fv(
        gl.GetUniformLocation(shader_program, format!("{}\0", name.to_string()).as_ptr()), 
        1, 
        GL_FALSE.0.try_into().unwrap(), 
        value
    );
}

pub unsafe fn uniform_f32(gl : &GlFns, shader_program : u32, name : &str, value : f32) {
    gl.Uniform1f(
        gl.GetUniformLocation(shader_program, format!("{}\0", name.to_string()).as_ptr()),
        value

    );
}

pub unsafe fn uniform_vec4(gl : &GlFns, shader_program : u32, name : &str, vector : *const f32) {
    gl.Uniform4fv(
        gl.GetUniformLocation(shader_program, format!("{}\0", name.to_string()).as_ptr()),
        1,
        vector
    );
}

pub unsafe fn uniform_vec3(gl : &GlFns, shader_program : u32, name : &str, vector : *const f32) {
    gl.Uniform3fv(
        gl.GetUniformLocation(shader_program, format!("{}\0", name.to_string()).as_ptr()),
        1,
        vector
    );
}


