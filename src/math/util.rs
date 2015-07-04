
use std::marker::PhantomData;
use std;

#[derive(Copy,Clone)]
#[allow(raw_pointer_derive)]
pub struct Components<'a, T:'a + Copy>
{
    cur: *const T,
    end: *const T,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: Copy> Components<'a, T>
{
    pub fn new(start: *const T,
               end: *const T) -> Self {
        Components {
            cur: start,
            end: end,
            phantom: PhantomData,
        }
    }

    pub fn with_length(start: *const T,
                       size: usize) -> Self {
        let end = (start as usize + (size * std::mem::size_of::<T>())) as *const T;

        Components::new(start, end)
    }
}

impl<'a, T: Copy> Iterator for Components<'a, T>
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.cur != self.end {
            let item = unsafe { *self.cur };

            self.cur = (self.cur as usize + std::mem::size_of::<T>()) as *const T;

            Some(item)

        } else { // end of components
            None
        }
    }
}

