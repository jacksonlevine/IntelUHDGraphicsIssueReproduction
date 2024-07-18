
mod shader;
mod camera;

use std::sync::Mutex;

use camera::Camera;
use glfw::{ffi::glfwGetTime, Action, Context, Key};
use gl::{types::*, ShaderSource};
use shader::Shader;

use glam::*;

struct Vars {
    first_mouse: bool,
    mouse_focused: bool,
    sensitivity: f32
}
impl Vars {
    pub fn new() -> Self {
        Self {
            first_mouse: true,
            mouse_focused: false,
            sensitivity: 5.0
        }
    }
}

struct TestThing {
    cloudshader: Shader,
    camera: Mutex<Camera>,
    vars: Vars
}

impl TestThing {
    pub fn new() -> Self {
        Self {
            cloudshader: Shader::new("cloudsvert.glsl", "cloudsfrag.glsl"),
            camera: Mutex::new(Camera::new()),
            vars: Vars::new()
        }
    }

    pub fn update() {



    }

    pub fn draw_clouds(&self) {
        static mut HASUPLOADED: bool = false;
        static mut VBO: GLuint = 0;
    
        let vdata: [f32; 30] = [
            -100.0, 100.5, -100.0,    0.0, 1.0, 
            -100.0, 100.5, 100.0,     0.0, 0.0, 
            100.0, 100.5, 100.0,      1.0, 0.0, 
    
            100.0, 100.5, 100.0,      1.0, 0.0, 
            100.0, 100.5, -100.0,     1.0, 1.0, 
            -100.0, 100.5, -100.0,    0.0, 1.0
        ];
    
        unsafe {
            gl::BindVertexArray(self.cloudshader.vao);
            gl::UseProgram(self.cloudshader.shader_id);
    
            if !HASUPLOADED {
                gl::CreateBuffers(1, &mut VBO);
                gl::NamedBufferData(
                    VBO,
                    (vdata.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                    vdata.as_ptr() as *const GLvoid,
                    gl::STATIC_DRAW,
                );
    
                // Bind vertex buffer to the vertex array object
                gl::VertexArrayVertexBuffer(self.cloudshader.vao, 0, VBO, 0, (5 * std::mem::size_of::<f32>()) as GLsizei);
    
                // Position attribute
                let pos_attrib = gl::GetAttribLocation(self.cloudshader.shader_id, b"aPos\0".as_ptr() as *const i8);
                gl::EnableVertexArrayAttrib(self.cloudshader.vao, pos_attrib as GLuint);
                gl::VertexArrayAttribFormat(
                    self.cloudshader.vao,
                    pos_attrib as GLuint,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    0,
                );
                gl::VertexArrayAttribBinding(self.cloudshader.vao, pos_attrib as GLuint, 0);
    
                // UV attribute
                let uv_attrib = gl::GetAttribLocation(self.cloudshader.shader_id, b"uv\0".as_ptr() as *const i8);
                gl::EnableVertexArrayAttrib(self.cloudshader.vao, uv_attrib as GLuint);
                gl::VertexArrayAttribFormat(
                    self.cloudshader.vao,
                    uv_attrib as GLuint,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (3 * std::mem::size_of::<f32>()) as GLuint,
                );
                gl::VertexArrayAttribBinding(self.cloudshader.vao, uv_attrib as GLuint, 0);
    
                HASUPLOADED = true;
            }
    
            // Set uniforms
            let cam_lock = self.camera.lock().unwrap();
            
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.cloudshader.shader_id, b"mvp\0".as_ptr() as *const i8),
                1, gl::FALSE, cam_lock.mvp.to_cols_array().as_ptr()
            );
    
            gl::Uniform1f(
                gl::GetUniformLocation(self.cloudshader.shader_id, b"opacity\0".as_ptr() as *const i8),
                1.0
            );

            gl::Uniform1f(
                gl::GetUniformLocation(self.cloudshader.shader_id, b"time\0".as_ptr() as *const i8),
                glfwGetTime() as f32
            );
    
    
            gl::Uniform1f(
                gl::GetUniformLocation(self.cloudshader.shader_id, b"scale\0".as_ptr() as *const i8),
                1.0
            );
    
            gl::Uniform1f(
                gl::GetUniformLocation(self.cloudshader.shader_id, b"ambientBrightMult\0".as_ptr() as *const i8),
                1.0
            );
    
            gl::Uniform3f(
                gl::GetUniformLocation(self.cloudshader.shader_id, b"camDir\0".as_ptr() as *const i8),
                cam_lock.direction.x, cam_lock.direction.y, cam_lock.direction.z
            );

            gl::Uniform3f(
                gl::GetUniformLocation(self.cloudshader.shader_id, b"camPos\0".as_ptr() as *const i8),
                cam_lock.position.x, cam_lock.position.y, cam_lock.position.z
            );
    
            gl::Uniform1f(
                gl::GetUniformLocation(self.cloudshader.shader_id, b"viewDistance\0".as_ptr() as *const i8),
                8.0
            );
    
            let fogcol = (0.0,0.0,0.0,1.0);
            gl::Uniform4f(
                gl::GetUniformLocation(self.cloudshader.shader_id, b"fogCol\0".as_ptr() as *const i8),
                fogcol.0, fogcol.1, fogcol.2, fogcol.3
            );
    
            gl::Uniform1f(
                gl::GetUniformLocation(self.cloudshader.shader_id, b"sunset\0".as_ptr() as *const i8),
                0.0
            );
    
            gl::Uniform1f(
                gl::GetUniformLocation(self.cloudshader.shader_id, b"sunrise\0".as_ptr() as *const i8),
                0.0
            );
    
            // Draw the clouds
            gl::Disable(gl::CULL_FACE);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::Enable(gl::CULL_FACE);
        }
    }



    pub fn cursor_pos(&mut self, xpos: f64, ypos: f64) {
        if self.vars.mouse_focused {
            static mut LASTX: f64 = 0.0;
            static mut LASTY: f64 = 0.0;

            if self.vars.first_mouse {
                unsafe {
                    LASTX = xpos;
                    LASTY = ypos;
                }
                self.vars.first_mouse = false;
            }

            unsafe {
                let x_offset = (xpos - LASTX) * self.vars.sensitivity as f64;
                let y_offset = (LASTY - ypos) * self.vars.sensitivity as f64;

                LASTY = ypos;
                LASTX = xpos;

                let mut camlock = self.camera.lock().unwrap();

                camlock.yaw += x_offset as f32;
                camlock.pitch += y_offset as f32;

                camlock.pitch = camlock.pitch.clamp(-89.0, 89.0);

                camlock.direction.x =
                    camlock.yaw.to_radians().cos() as f32 * camlock.pitch.to_radians().cos() as f32;
                camlock.direction.y = camlock.pitch.to_radians().sin();
                camlock.direction.z =
                    camlock.yaw.to_radians().sin() * camlock.pitch.to_radians().cos();
                camlock.direction = camlock.direction.normalize();

                camlock.right = Vec3::new(0.0, 1.0, 0.0)
                    .cross(camlock.direction)
                    .normalize();
                camlock.up = camlock.direction.cross(camlock.right).normalize();

                camlock.recalculate();
                #[cfg(feature = "show_cam_pos")]
                println!(
                    "Cam dir: {}, {}, {}",
                    camlock.direction.x, camlock.direction.y, camlock.direction.z
                );
            }
        }

        
    }

    pub fn set_mouse_focused(&mut self, tf: bool) {
        if tf {
            self.vars.mouse_focused = true;
        } else {
            self.vars.mouse_focused = false;
            self.vars.first_mouse = true;
        }
    }
}














fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw.create_window(600, 800, "Proof that intel is broken", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    window.set_key_polling(true);
    window.make_current();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
        gl::FrontFace(gl::CW);
    }


    let mut testthing = TestThing::new();

    while !window.should_close() {

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }
        
        testthing.draw_clouds();



        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::MouseButton(mousebutton, action, _) => {


                        if mousebutton == glfw::MouseButtonLeft {
                                        if action == Action::Press {
                                            window.set_cursor_mode(glfw::CursorMode::Disabled);
                                            testthing.set_mouse_focused(true);
                                        }
                                   
                   
                            
                        }
                    
                        
                }
                _ => {}

                glfw::WindowEvent::CursorPos(xpos, ypos) => {

                    testthing.cursor_pos(xpos, ypos);

                    
                }
            }
        }
    }
}
