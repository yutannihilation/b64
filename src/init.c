
#include <stdint.h>
#include <Rinternals.h>
#include <R_ext/Parse.h>

#include "rust/api.h"

static uintptr_t TAGGED_POINTER_MASK = (uintptr_t)1;

SEXP handle_result(SEXP res_) {
    uintptr_t res = (uintptr_t)res_;

    // An error is indicated by tag.
    if ((res & TAGGED_POINTER_MASK) == 1) {
        // Remove tag
        SEXP res_aligned = (SEXP)(res & ~TAGGED_POINTER_MASK);

        // Currently, there are two types of error cases:
        //
        //   1. Error from Rust code
        //   2. Error from R's C API, which is caught by R_UnwindProtect()
        //
        if (TYPEOF(res_aligned) == CHARSXP) {
            // In case 1, the result is an error message that can be passed to
            // Rf_errorcall() directly.
            Rf_errorcall(R_NilValue, "%s", CHAR(res_aligned));
        } else {
            // In case 2, the result is the token to restart the
            // cleanup process on R's side.
            R_ContinueUnwind(res_aligned);
        }
    }

    return (SEXP)res;
}

SEXP savvy_alphabet___impl(SEXP c_arg__which) {
    SEXP res = savvy_alphabet___ffi(c_arg__which);
    return handle_result(res);
}

SEXP savvy_b64_chunk__impl(SEXP c_arg__encoded, SEXP c_arg__width) {
    SEXP res = savvy_b64_chunk__ffi(c_arg__encoded, c_arg__width);
    return handle_result(res);
}

SEXP savvy_b64_wrap__impl(SEXP c_arg__chunks, SEXP c_arg__newline) {
    SEXP res = savvy_b64_wrap__ffi(c_arg__chunks, c_arg__newline);
    return handle_result(res);
}

SEXP savvy_decode___impl(SEXP c_arg__input, SEXP c_arg__engine) {
    SEXP res = savvy_decode___ffi(c_arg__input, c_arg__engine);
    return handle_result(res);
}

SEXP savvy_decode_as_string___impl(SEXP c_arg__what, SEXP c_arg__engine, SEXP c_arg__split) {
    SEXP res = savvy_decode_as_string___ffi(c_arg__what, c_arg__engine, c_arg__split);
    return handle_result(res);
}

SEXP savvy_decode_file___impl(SEXP c_arg__path, SEXP c_arg__engine) {
    SEXP res = savvy_decode_file___ffi(c_arg__path, c_arg__engine);
    return handle_result(res);
}

SEXP savvy_decode_vectorized___impl(SEXP c_arg__what, SEXP c_arg__engine) {
    SEXP res = savvy_decode_vectorized___ffi(c_arg__what, c_arg__engine);
    return handle_result(res);
}

SEXP savvy_encode___impl(SEXP c_arg__what, SEXP c_arg__engine) {
    SEXP res = savvy_encode___ffi(c_arg__what, c_arg__engine);
    return handle_result(res);
}

SEXP savvy_encode_file___impl(SEXP c_arg__path, SEXP c_arg__engine) {
    SEXP res = savvy_encode_file___ffi(c_arg__path, c_arg__engine);
    return handle_result(res);
}

SEXP savvy_encode_vectorized___impl(SEXP c_arg__what, SEXP c_arg__engine) {
    SEXP res = savvy_encode_vectorized___ffi(c_arg__what, c_arg__engine);
    return handle_result(res);
}

SEXP savvy_engine___impl(SEXP c_arg__which) {
    SEXP res = savvy_engine___ffi(c_arg__which);
    return handle_result(res);
}

SEXP savvy_get_alphabet___impl(SEXP c_arg__alphabet) {
    SEXP res = savvy_get_alphabet___ffi(c_arg__alphabet);
    return handle_result(res);
}

SEXP savvy_new_alphabet___impl(SEXP c_arg__chars) {
    SEXP res = savvy_new_alphabet___ffi(c_arg__chars);
    return handle_result(res);
}

SEXP savvy_new_config___impl(SEXP c_arg__encode_padding, SEXP c_arg__decode_padding_trailing_bits, SEXP c_arg__decode_padding_mode) {
    SEXP res = savvy_new_config___ffi(c_arg__encode_padding, c_arg__decode_padding_trailing_bits, c_arg__decode_padding_mode);
    return handle_result(res);
}

SEXP savvy_new_engine___impl(SEXP c_arg__alphabet, SEXP c_arg__config) {
    SEXP res = savvy_new_engine___ffi(c_arg__alphabet, c_arg__config);
    return handle_result(res);
}

SEXP savvy_print_config___impl(SEXP c_arg__config) {
    SEXP res = savvy_print_config___ffi(c_arg__config);
    return handle_result(res);
}

SEXP savvy_print_engine___impl(SEXP c_arg__engine) {
    SEXP res = savvy_print_engine___ffi(c_arg__engine);
    return handle_result(res);
}





static const R_CallMethodDef CallEntries[] = {
    {"savvy_alphabet___impl", (DL_FUNC) &savvy_alphabet___impl, 1},
    {"savvy_b64_chunk__impl", (DL_FUNC) &savvy_b64_chunk__impl, 2},
    {"savvy_b64_wrap__impl", (DL_FUNC) &savvy_b64_wrap__impl, 2},
    {"savvy_decode___impl", (DL_FUNC) &savvy_decode___impl, 2},
    {"savvy_decode_as_string___impl", (DL_FUNC) &savvy_decode_as_string___impl, 3},
    {"savvy_decode_file___impl", (DL_FUNC) &savvy_decode_file___impl, 2},
    {"savvy_decode_vectorized___impl", (DL_FUNC) &savvy_decode_vectorized___impl, 2},
    {"savvy_encode___impl", (DL_FUNC) &savvy_encode___impl, 2},
    {"savvy_encode_file___impl", (DL_FUNC) &savvy_encode_file___impl, 2},
    {"savvy_encode_vectorized___impl", (DL_FUNC) &savvy_encode_vectorized___impl, 2},
    {"savvy_engine___impl", (DL_FUNC) &savvy_engine___impl, 1},
    {"savvy_get_alphabet___impl", (DL_FUNC) &savvy_get_alphabet___impl, 1},
    {"savvy_new_alphabet___impl", (DL_FUNC) &savvy_new_alphabet___impl, 1},
    {"savvy_new_config___impl", (DL_FUNC) &savvy_new_config___impl, 3},
    {"savvy_new_engine___impl", (DL_FUNC) &savvy_new_engine___impl, 2},
    {"savvy_print_config___impl", (DL_FUNC) &savvy_print_config___impl, 1},
    {"savvy_print_engine___impl", (DL_FUNC) &savvy_print_engine___impl, 1},



    {NULL, NULL, 0}
};

void R_init_b64(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);

    // Functions for initialzation, if any.

}
