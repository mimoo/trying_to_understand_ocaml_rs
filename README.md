# Trying to understand OCaml-rs

When are we supposed to drop custom types?

try: 

```console
$ dune exec ./main.exe
hey
hey
number of references before dropping: 3
number of references before dropping: 3
number of references before dropping: 3
```

If you uncomment the `drop_in_place` in:

```rust=
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
```

you will see that Rc count goes down:

```console
$ dune exec ./main.exe
hey
hey
number of references before dropping: 3
number of references before dropping: 2
number of references before dropping: 1
```

Why do we have to manually drop it though? It looks like Rust never leave control of the value, and ocaml-rs just moves the value on the ocaml heap: https://github.com/zshipko/ocaml-rs/blob/master/src/types.rs#L80

so it should get dropped when this gets garbage collected no?
