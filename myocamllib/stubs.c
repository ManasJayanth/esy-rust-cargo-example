#include <foobar.h>
#include <myclib.h>
#include <caml/mlvalues.h>

CAMLprim value caml_run(value n) {
  run(Int_val(n));
  return Val_unit;
}

CAMLprim value caml_exportedFunction(value n) {
  exportedFunction(Int_val(n));
  return Val_unit;
}
