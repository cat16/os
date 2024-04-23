use core::{mem::MaybeUninit, ops::Deref};

pub struct LazyConst<T> {
    value: T,
    #[cfg(debug_assertions)]
    state: u8,
}

impl<T> Deref for LazyConst<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[cfg(debug_assertions)]
        if self.state == 0 {
            panic!("Lazy const for {} not assigned", core::any::type_name::<T>());
        }
        &self.value
    }
}

impl<T> LazyConst<T> {
    pub const fn new() -> Self {
        unsafe {
            Self {
                value: MaybeUninit::zeroed().assume_init(),
                #[cfg(debug_assertions)]
                state: 0,
            }
        }
    }

    pub unsafe fn init(&self, value: T) {
        as_mut(self).value = value;
        #[cfg(debug_assertions)]
        {
            as_mut(self).state = 1;
        }
    }
}

pub unsafe fn as_mut<T>(x: &T) -> &mut T {
    #[allow(invalid_reference_casting)]
    &mut *((x as *const T) as *mut T)
}
