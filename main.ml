module Amodule = struct
  type t 

  external new_a : string -> t = "new_a"
  external print_a : t -> unit = "print_a"
  external clone_a : t -> t * t = "clone_a"
end

let () =
  let a = Amodule.new_a "hey" in
  let a, b = Amodule.clone_a a in
  let () = Amodule.print_a a in
  let () = Amodule.print_a b in
  ()

let () =
  at_exit Gc.full_major
