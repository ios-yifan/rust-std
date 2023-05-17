#[cxx::bridge(namespace = "cn::rongcloud")]
mod ffi {
    extern "Rust" {
        unsafe fn next_chunk(func: &MyFunction);
    }

    unsafe extern "C++" {
        include!("engine.h");
        type MyFunction;

        #[cxx_name = "operatorINVOKE"]
        fn call(self: &MyFunction);
    }
}

    pub fn next_chunk(function: &ffi::MyFunction) {
        function.call();
    }
