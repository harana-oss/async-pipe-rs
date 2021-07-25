use std::task::Waker;

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) reader_waker: Option<Waker>,
    pub(crate) writer_waker: Option<Waker>,
    pub(crate) data: Option<Data>,
    pub(crate) done_reading: bool,
    pub(crate) read: usize,
    pub(crate) done_cycle: bool,
    pub(crate) closed: bool,
}

#[derive(Clone)]
pub(crate) struct Data {
    pub(crate) ptr: *const u8,
    pub(crate) len: usize,
}

unsafe impl Send for Data {}
