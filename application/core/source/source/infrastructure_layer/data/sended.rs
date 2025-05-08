use std::mem::ManuallyDrop;
#[derive(Clone)]
pub struct Sended<T>(*mut T);
impl<T> Sended<T> {
    pub fn new(pointer: *mut T) -> Self {
        return Self(pointer);
    }
    pub unsafe fn write(self, value: T) -> () {
        unsafe { self.0.write(value); }
        return ();
    }
    pub unsafe fn read(self) -> ManuallyDrop<T> {
        return unsafe { ManuallyDrop::new(self.0.read()) };
    }
    pub unsafe fn read_<'a>(self) -> &'a T {
        return unsafe { &*self.0 };
    }
}
unsafe impl<T> Send for Sended<T> {}
unsafe impl<T> Sync for Sended<T> {}
impl<T> Copy for Sended<T>
where
    Self: Clone
{}
#[derive(Clone)]
pub struct Sended_<T>(*const T);
impl<T> Sended_<T> {
    pub fn new(pointer: *const T) -> Self {
        return Self(pointer);
    }
    pub unsafe fn read(self) -> ManuallyDrop<T> {
        return unsafe { ManuallyDrop::new(self.0.read()) };
    }
    pub unsafe fn read_<'a>(self) -> &'a T {
        return unsafe { &*self.0 };
    }
}
unsafe impl<T> Send for Sended_<T> {}
unsafe impl<T> Sync for Sended_<T> {}
impl<T> Copy for Sended_<T>
where
    Self: Clone
{}