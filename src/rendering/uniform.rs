use gl33::*;

pub unsafe fn set_uniform(gl : &GlFns, shader_program : u32, name : &str, value : *const f32) {
    gl.UniformMatrix4fv(
        gl.GetUniformLocation(shader_program, format!("{}\0", name.to_string()).as_ptr()), 
        1, 
        GL_FALSE.0.try_into().unwrap(), 
        value
    );
}

pub unsafe fn set_uniform_block(gl : &GlFns, shader_program : u32, name : &str, value : *const f32){
    gl.UniformBlockBinding(
        shader_program, 
        gl.GetUniformBlockIndex(shader_program, format!("{}\0", name.to_string()).as_ptr()), 
        uniformBlockBinding
    );
}

