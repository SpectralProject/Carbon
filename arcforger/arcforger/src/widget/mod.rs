pub trait Widget {
    pub desc: String

}

pub struct Window impl Widget {
    // returns handle to window
    pub fn open() -> i32;
    pub fn close() -> i32;
    pub fn minimize() -> i32;
}

impl Window {
    
}