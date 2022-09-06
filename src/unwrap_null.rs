pub trait UnwrapNull {
    type Return;

    fn unwrap_null(self) -> Self::Return;
}

impl<T> UnwrapNull for *mut T {
    type Return = *mut T;

    fn unwrap_null(self) -> Self::Return {
        if self.is_null() {
            panic!("Null Pointer");
        }
        self
    }
}

impl<T> UnwrapNull for *const T {
    type Return = *const T;

    fn unwrap_null(self) -> Self::Return {
        if self.is_null() {
            panic!("Null Pointer");
        }
        self
    }
}
