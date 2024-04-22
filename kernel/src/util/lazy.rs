use core::{mem::zeroed, ops::Deref};

pub struct LazyConst<T> {
    #[cfg(not(debug_assertions))]
    value: T,
    #[cfg(debug_assertions)]
    value: Option<T>,
}

impl<T> Deref for LazyConst<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[cfg(not(debug_assertions))]
        {
            &self.value
        }
        #[cfg(debug_assertions)]
        {
            self.value.as_ref().expect("Value was not assigned")
        }
    }
}

impl<T> LazyConst<T> {
    pub const fn new() -> Self {
        unsafe { Self { value: zeroed() } }
    }

    pub fn init(&self, value: T) {
        #[cfg(not(debug_assertions))]
        unsafe {
            as_mut(self).value = value
        }
        #[cfg(debug_assertions)]
        unsafe {
            as_mut(self).value = Some(value)
        }
    }
}

pub unsafe fn as_mut<T>(x: &T) -> &mut T {
    #[allow(invalid_reference_casting)]
    &mut *((x as *const T) as *mut T)
}
