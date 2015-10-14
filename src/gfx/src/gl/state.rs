

pub mod properties
{
    use gl::gl;
    use color::NormalizedRGBA;

    pub trait Property<T> : Copy {
        fn get(self) -> T;
        fn set(&mut self, val: T);

        fn original(self) -> T;
        fn get_unbuffered(self) -> T;
    }

    macro_rules! impl_color_property {
        ($name:ident, $valname:ident) => {
            #[derive(Copy,Clone)]
            pub struct $name {
                orig: NormalizedRGBA,
                cur: NormalizedRGBA,
            }

            impl Property<NormalizedRGBA> for $name {
                fn get(self) -> NormalizedRGBA { self.cur }
                fn set(&mut self, val: NormalizedRGBA) {
                    let NormalizedRGBA(r,g,b,a) = val;

                    unsafe {
                        gl::$name(r,g,b,a);
                    }
                }

                fn original(self) -> NormalizedRGBA { self.orig }
                fn get_unbuffered(self) -> NormalizedRGBA {
                    let mut vals: [f32; 4] = [0.0; 4];
                    let ptr = &mut vals[0] as *mut f32;

                    unsafe {
                        gl::GetFloatv(gl::$valname, ptr);
                    }
                    NormalizedRGBA(vals[0], vals[1],
                                   vals[2], vals[3])
                }
            }
        }
    }

    impl_color_property!(ClearColor, COLOR_CLEAR_VALUE);
}

#[allow(non_snake_case)]
pub struct State
{
    pub ClearColor: Option<properties::ClearColor>,
}

