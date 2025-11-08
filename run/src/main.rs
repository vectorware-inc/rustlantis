use cust::{
    launch,
    module::Module,
    stream::{Stream, StreamFlags},
};

fn main() {
    let _ctx = cust::quick_init().unwrap();
    let mdl = Module::from_file(std::env::args().nth(1).unwrap()).unwrap();
    let strm = Stream::new(StreamFlags::empty(), None).unwrap();
    unsafe { launch!(mdl.kernel_main<<<1,1,0,strm>>>()) }.unwrap();
    strm.synchronize().unwrap();
}
