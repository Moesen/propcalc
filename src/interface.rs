use inquire::{InquireError, Select};

pub trait MenuInteraction {    
}

pub trait Menu {
    fn functions(&self) -> str;
}

impl<T> MenuInteraction for T where T: Menu {
}
