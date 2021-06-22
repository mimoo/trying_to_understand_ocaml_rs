use std::rc::Rc;

//
// Pointer
//

mod caml_pointer {
    use super::*;
    use std::ops::{Deref, DerefMut};

    #[derive(std::fmt::Debug, Clone)]
    pub struct CamlPointer<T>(pub Rc<T>);

    impl<T> CamlPointer<T> {
        extern "C" fn caml_pointer_finalize(v: ocaml::Value) {
            let v: ocaml::Pointer<CamlPointer<T>> = ocaml::FromValue::from_value(v);
            // print the number of reference countn to see if we need to drop
            println!(
                "number of references before dropping: {}",
                Rc::strong_count(&v.as_ref().0)
            );
            /*
            // comment out the dropping, to see if we really need it
            unsafe {
                v.drop_in_place();
            }
            */
        }

        extern "C" fn caml_pointer_compare(_: ocaml::Value, _: ocaml::Value) -> i32 {
            // Always return equal. We can use this for sanity checks, and anything else using this
            // would be broken anyway.
            0
        }

        pub fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    ocaml::custom!(CamlPointer<T> {
        finalize: CamlPointer::<T>::caml_pointer_finalize,
        compare: CamlPointer::<T>::caml_pointer_compare,
    });

    unsafe impl<T> ocaml::FromValue for CamlPointer<T> {
        fn from_value(x: ocaml::Value) -> Self {
            let x = ocaml::Pointer::<Self>::from_value(x);
            CamlPointer(x.as_ref().0.clone())
        }
    }

    pub fn create<T>(x: T) -> CamlPointer<T> {
        CamlPointer(Rc::new(x))
    }

    impl<T> Deref for CamlPointer<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &*self.0
        }
    }

    impl<T> DerefMut for CamlPointer<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                // Wholely unsafe, Batman!
                // We would use [`get_mut_unchecked`] here, but it is nightly-only.
                // Instead, we get coerce our constant pointer to a mutable pointer, in the knowledge
                // that
                // * all of our mutations called from OCaml are blocking, so we won't have multiple
                // live mutable references live simultaneously, and
                // * the underlying pointer is in the correct state to be mutable, since we can call
                //   [`get_mut_unchecked`] in nightly, or can call [`get_mut`] and unwrap if this is
                //   the only live reference.
                &mut *(((&*self.0) as *const Self::Target) as *mut Self::Target)
            }
        }
    }
}

//
// Using CamlPointer
//

pub struct A {
    pub b: String,
}

pub type PointerA = caml_pointer::CamlPointer<Rc<A>>;

#[ocaml::func]
pub fn new_a(b: String) -> PointerA {
    caml_pointer::create(Rc::new(A { b }))
}

#[ocaml::func]
pub fn print_a(a: PointerA) {
    let a = &*a;
    println!("{}", a.b);
}

#[ocaml::func]
pub fn clone_a(a: PointerA) -> (PointerA, PointerA) {
    let other_a = a.clone();
    (a, other_a)
}
