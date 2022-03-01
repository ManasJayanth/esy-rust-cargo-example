external run_rust_fn: int -> unit = "caml_run"
external run_c_fn: int -> unit = "caml_exportedFunction"

let () = 
  run_rust_fn 30; print_newline (); run_c_fn 40
