#[path = "./data.rs"] mod data;

mod bufferManager;

use glfw::{Action, Context, Key};
use gl;

use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::OpResponse;
use deno_core::plugin_api::ZeroCopyBuf;
use futures::future::FutureExt;
use std::convert::TryInto;



static mut DATA: Option<data::RenderData> = None;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    println!("registrerer metoder");

    bufferManager::initialize(interface);
    
    interface.register_op("op_render_initialize", initialize_render);
    interface.register_op("op_render_create_window", create_new_window);
    interface.register_op("op_render_window_make_current", window_make_current);
    interface.register_op("op_render_window_poll_events",window_poll);
    interface.register_op("op_render_window_should_close", window_should_close);
    interface.register_op("op_render_swap_buffers", window_swap_buffer);
    interface.register_op("op_render_window_activate_key_poling", window_activate_key_polling);

    interface.register_op("op_set_clear_color", set_clear_color);
    interface.register_op("op_clear",clear_window);

    interface.register_op("op_create_buffer", create_buffer);
    interface.register_op("op_set_buffer_data", set_buffer_data);
}

pub fn initialize_render(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op {

    let ginstance = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize glfw");

    unsafe {
        DATA = Some(data::RenderData {
            glfw: ginstance,
            windows: std::collections::BTreeMap::new(),
            shared_buffers: std::collections::BTreeMap::new(),
        });
    }
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}

pub fn create_new_window(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op {
    unsafe {
        match DATA {
            Some(ref mut render_data) => {
                let g: &glfw::Glfw = &render_data.glfw;
                let window_tuple = g.create_window(800, 600, "test", glfw::WindowMode::Windowed);

                match window_tuple {
                    Some(window_tuple) => {
                        let index = create_identifier();

                        let w = &mut render_data.windows;
                        w.insert(index, window_tuple);
                        Op::Sync(OpResponse::Buffer(Box::new(index.to_be_bytes())))
                    }
                    _ => panic!("Failed to create window")
                }
            }
            _ => panic!("Render not initialized")
        }
    }
}

pub fn window_make_current(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op {
    match zero_copy {
        Some(b) => {
            if b.len() == 4 {
                let index = u32::from_be_bytes(
                    b[..]
                        .try_into()
                        .expect("Failed to convert slice to array"),
                );

                unsafe{
                     match DATA{
                         Some(ref mut d)=>{
                            let w = &mut d.windows;
                            let tuple = w.get_mut(&index);
                            match tuple{
                                Some((window,_) ) =>{
                                    window.set_key_polling(true);
                                    window.make_current();
                                    
                                    gl::load_with(|s| window.get_proc_address(s) as *const _);
                                },
                                None => panic!("No window with the given id {}",index)
                            }    
                         },
                         None => panic!("Render not initialized")
                     }
                }
            }
            Op::Sync(OpResponse::Buffer(Box::new([0])))
        }, 
        _ => panic!("expected int arg, recieved none")
    }
}

pub fn window_should_close(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op {
    let mut should_close: u8 = 1;
    let index = zero_copy_to_int(&zero_copy.unwrap(),0).expect("Failed to convert buffer to int");
    unsafe{
        match DATA{
            Some(ref d)=>{
                let w = &d.windows;
                let tuple = w.get(&index);
                match tuple{
                    Some((window,_) ) =>{
                        should_close = match window.should_close() {true => 1u8, _ => 0};
                    },
                    None => panic!("No window with the given id {}",index)
                }
            },
            None => panic!("Render not initialized")
        }
    }
    Op::Sync(OpResponse::Buffer(Box::new([should_close])))
}

pub fn window_activate_key_polling(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let index = zero_copy_to_int(&zero_copy.unwrap(),0).expect("Failed to convert buffer to int");
    unsafe{
        match DATA{
            Some(ref mut d)=>{
                let w = &mut d.windows;
                let tuple = w.get_mut(&index);
                match tuple{
                    Some(tuple) =>{
                        tuple.0.set_key_polling(true);
                    },
                    None => panic!("No window with the given id {}",index)
                }
            },
            None => panic!("Render not initialized")
        }
    }

    Op::Sync(OpResponse::Buffer(Box::new([0])))    
}


pub fn window_poll(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    unsafe{
        match DATA{
            Some(ref mut d) =>{

                let index = zero_copy_to_int(&zero_copy.unwrap(),0).expect("Failed to convert buffer to int");

                d.glfw.poll_events();

                let mut result: Vec<u8> = Vec::new();

                let w = d.windows.get(&index);
                match w{
                    Some(w) =>{
                        for (_,event) in glfw::flush_messages(&w.1){
                            match event{
                                glfw::WindowEvent::Key(key,scancode,action,_) => {
                                    result.push(1u8);        
                                    let bytes = scancode.to_be_bytes();
                                    result.push(bytes[0]);
                                    result.push(bytes[1]);
                                    result.push(bytes[2]);
                                    result.push(bytes[3]);
                                    
                                    match action {
                                        Action::Release => result .push(0u8),
                                        Action::Press => result.push(1u8),
                                        Action::Repeat => result.push(2u8),
                                    }
                                }
                                _ => panic!("unhandled event {:#?}",event)
                            }
                        }
                        if result.len() == 0{
                            Op::Sync(OpResponse::Buffer(Box::new([0])))
                        }
                        else{
                            Op::Sync(OpResponse::Buffer(result.into_boxed_slice()))
                        }
                    },
                    None => panic!("No window with id {}", index)
                }

            },
            _ => panic!("Render not initialized")
        }
    }
}

pub fn window_swap_buffer(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{

    let index = zero_copy_to_int(&zero_copy.unwrap(),0).expect("Failed to convert buffer to int");
    unsafe{
        match DATA{
            Some(ref mut d)=>{
                let mut w = &mut d.windows;
                let mut tuple = w.get_mut(&index);
                match tuple{
                    Some(tuple) =>{
                        tuple.0.swap_buffers();
                    },
                    None => panic!("No window with the given id {}",index)
                }
            },
            None => panic!("Render not initialized")
        }
    }

    Op::Sync(OpResponse::Buffer(Box::new([0])))    
}

pub fn set_clear_color(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let slice = &zero_copy.unwrap();
    let r = zero_copy_to_float(&slice, 0);
    let g = zero_copy_to_float(&slice, 4);
    let b = zero_copy_to_float(&slice, 8);
    let a = zero_copy_to_float(&slice, 12);
    println!("{} {} {} {}",r,g,b,a);
    unsafe{
        gl::ClearColor(r,g,b,a);
    }
    Op::Sync(OpResponse::Buffer(Box::new([0])))    
}

pub fn clear_window(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let mask = zero_copy_to_int(&zero_copy.unwrap(),0).or::<u32>(Ok(0u32)).unwrap();
    unsafe{
        gl::Clear(mask);
    }
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}


pub fn create_buffer(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let mut bufferId: u32 = 0;

    unsafe{
        
        gl::CreateBuffers(1,&mut bufferId);
    }

    Op::Sync(OpResponse::Buffer(Box::new(bufferId.to_be_bytes())))
}

pub fn set_buffer_data(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op {
    let slice = zero_copy.unwrap();
    let bufferId = zero_copy_to_int(&slice, 0).expect("Failed to get BufferId from buffer");
    let target = zero_copy_to_int(&slice, 4).expect("Failed to get target from buffer");;
    let size = zero_copy_to_int(&slice, 8).expect("Failed to get size from buffer");;
    let usage = zero_copy_to_int(&slice, 12).expect("Failed to get usage from buffer");;
    let data_slice = &slice[16..slice.len()];
    unsafe{
        gl::BindBuffer(target, bufferId);
        gl::BufferData(target, size as isize, data_slice.as_ptr() as *const std::ffi::c_void, usage);
        gl::BindBuffer(0,bufferId);
    }
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}

pub fn terminate_render(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op {
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}


pub fn setup_shared_buffer(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op {
    let mut index: u32 = 0;
    unsafe{
        match &mut DATA{
            Some(ref mut d) =>{
                match zero_copy{
                    Some(zc) =>{
                        index = create_identifier();
                        d.shared_buffers.insert(index,zc);
                    },
                    None => (),
                }
            },
            None => panic!("Render not initialized")
        }
    }
    Op::Sync(OpResponse::Buffer(Box::new(index.to_be_bytes())))
}

pub fn drop_shared_buffer(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op {
    let mut index: u32 = zero_copy_to_int(&zero_copy.unwrap(), 0).expect("Failed to get int from buffer");
    unsafe{
        match &mut DATA{
            Some(ref mut d) =>{
                d.shared_buffers.remove(&index);
            },
            None => panic!("Render not initialized")
        }
    }
    Op::Sync(OpResponse::Buffer(Box::new(index.to_be_bytes())))
}


fn zero_copy_to_float(d:&[u8],index: usize) -> f32{
    if d.len() - index >= 4{

        f32::from_be_bytes(
           d[index..index+4]
           .try_into().
           expect("Failed to convert buffer to float")
        )
   }
   else{
       0f32
   }
}

pub fn zero_copy_to_int(d: &[u8], index: usize) -> Result<u32, String>{

    if d.len()-index >= 4 {
        let index = u32::from_be_bytes(
            d[index..index+4]
                .try_into()
                .expect("Failed to convert slice to array"),
        );
        Ok(index)
    }
    else{
        Err(String::from("Expected 4 bytes"))
    }
}

static mut ID_COUNTER: u32 = 1;
fn create_identifier() -> u32 {
    unsafe {
        let i = ID_COUNTER;
        ID_COUNTER += 1;
        i
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn u8_slice_to_int_returns_correct_int() {
        let i: u32 = 1;
        let bytes = i.to_be_bytes();
        let result = zero_copy_to_int(&bytes,0);
        assert_eq!(result,Ok(i));
    }

    #[test]

    fn u8_slice_to_int_small_slice_returns_err() {
        let small_buffer: [u8;4] = [0;4];
        
        let result = zero_copy_to_int(&small_buffer,1);
        assert_eq!(Err(String::from("Expected 4 bytes")),result);
    }

    #[test]
    fn u8_slice_to_int_big_slice_returns_correct() {
        let expected = Ok(0x01010101u32);
        let big_buffer: [u8;5] = [1;5];
        let result = zero_copy_to_int(&big_buffer,0);
        assert_eq!(expected,result);
    }

    #[test]
    fn u8_slice_to_float_returns_correct_float() {
        let expected: f32 = 1.0;
        let bytes = expected.to_be_bytes();
        let result = zero_copy_to_float(&bytes, 0);
        assert_eq!(expected,result);
    }

    #[test]
    fn u8_slice_to_float_to_few_bytes_returns_0(){
        let buffer: [u8;4] = [64;4];
        let expected: f32 = 0.0;
        let result = zero_copy_to_float(&buffer, 1);
        assert_eq!(expected,result);
    }

    #[test]
    fn create_identifiers_returns_1_2_from_two_calls() {
        let first_expected = 1;
        let second_expected = 2;
        let first_result = create_identifier();
        let second_result = create_identifier();
        assert_eq!(first_expected,first_result);
        assert_eq!(second_expected,second_result);
    }
}