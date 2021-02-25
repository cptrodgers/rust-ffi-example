use std::intrinsics::transmute;
/// [no_mangle] = Don't change name of function

#[no_mangle]
pub extern "C" fn simple_hello() {
    println!("Hello World!"); // Println also works when you call this fn in C#
}

/// You can transer scalar value such as i32, bool

#[no_mangle]
pub extern "C" fn transfer_simple_variable(number: i32, boolean: bool) {
    println!("{}", number);
    println!("{}", boolean);
}

/// You can also transfer struct with C layout
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SimpleStruct {
    pub inner: i32,
}

impl SimpleStruct {
    pub fn from_ptr(ptr: *mut SimpleStruct) -> SimpleStruct {
        unsafe { *ptr }
    }

    pub fn to_ptr(self) -> *mut SimpleStruct {
        unsafe { transmute(Box::new(self)) }
    }
}

#[no_mangle]
pub extern "C" fn transfer_simple_struct(input: *mut SimpleStruct) -> *mut SimpleStruct {
    if input.is_null(){ 
        SimpleStruct { inner: 0 }.to_ptr()
    } else {
        let input = SimpleStruct::from_ptr(input);
        SimpleStruct { inner: input.inner * 2 }.to_ptr()
    }
}


/// For the complicated struct, For example: Inner property doesn't support FFI layout
#[derive(Debug, Clone, Copy)]
pub struct ComplicatedStruct {
    pub inner: i32,
}

impl ComplicatedStruct {
    pub fn from_ptr(ptr: *mut ComplicatedStruct) -> ComplicatedStruct {
        unsafe { *ptr }
    }

    pub fn to_ptr(self) -> *mut ComplicatedStruct {
        unsafe { transmute(Box::new(self)) }
    }
}

#[no_mangle]
pub extern "C" fn init_complicated_struct(ptr: *mut *const ComplicatedStruct, inner: i32) {
    let c_struct = ComplicatedStruct { inner };
    unsafe { *ptr = c_struct.to_ptr() }
}

#[no_mangle]
pub extern "C" fn get_inner(ptr: *mut ComplicatedStruct) -> i32 {
    if ptr.is_null() {
        0
    } else {
        ComplicatedStruct::from_ptr(ptr).inner
    }
}
