// =============== BEGIN minicheck_h ================
pub const CK_VERBOSE: libc::c_int = 2 as libc::c_int;

pub type tcase_setup_function = Option<unsafe extern "C" fn() -> ()>;

pub type tcase_teardown_function = Option<unsafe extern "C" fn() -> ()>;

pub type tcase_test_function = Option<unsafe extern "C" fn() -> ()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SRunner {
    pub suite: *mut crate::src::tests::minicheck::Suite,
    pub nchecks: libc::c_int,
    pub nfailures: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Suite {
    pub name: *const libc::c_char,
    pub tests: *mut crate::src::tests::minicheck::TCase,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCase {
    pub name: *const libc::c_char,
    pub setup: crate::src::tests::minicheck::tcase_setup_function,
    pub teardown: crate::src::tests::minicheck::tcase_teardown_function,
    pub tests: *mut crate::src::tests::minicheck::tcase_test_function,
    pub ntests: libc::c_int,
    pub allocated: libc::c_int,
    pub next_tcase: *mut crate::src::tests::minicheck::TCase,
}

pub use crate::stddef_h::{size_t, NULL};
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __jmp_buf, __jmp_buf_tag, __off64_t,
    __off_t, __sigset_t, _setjmp, jmp_buf, longjmp, FILE, _IO_FILE,
};

use ::libc::{self};
/* Miniature re-implementation of the "check" library.

   This is intended to support just enough of check to run the Expat
   tests.  This interface is based entirely on the portion of the
   check library being used.
                            __  __            _
                         ___\ \/ /_ __   __ _| |_
                        / _ \\  /| '_ \ / _` | __|
                       |  __//  \| |_) | (_| | |_
                        \___/_/\_\ .__/ \__,_|\__|
                                 |_| XML parser

   Copyright (c) 1997-2000 Thai Open Source Software Center Ltd
   Copyright (c) 2000-2017 Expat development team
   Licensed under the MIT license:

   Permission is  hereby granted,  free of charge,  to any  person obtaining
   a  copy  of  this  software   and  associated  documentation  files  (the
   "Software"),  to  deal in  the  Software  without restriction,  including
   without  limitation the  rights  to use,  copy,  modify, merge,  publish,
   distribute, sublicense, and/or sell copies of the Software, and to permit
   persons  to whom  the Software  is  furnished to  do so,  subject to  the
   following conditions:

   The above copyright  notice and this permission notice  shall be included
   in all copies or substantial portions of the Software.

   THE  SOFTWARE  IS  PROVIDED  "AS  IS",  WITHOUT  WARRANTY  OF  ANY  KIND,
   EXPRESS  OR IMPLIED,  INCLUDING  BUT  NOT LIMITED  TO  THE WARRANTIES  OF
   MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN
   NO EVENT SHALL THE AUTHORS OR  COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
   DAMAGES OR  OTHER LIABILITY, WHETHER  IN AN  ACTION OF CONTRACT,  TORT OR
   OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
   USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
#[no_mangle]

pub unsafe extern "C" fn suite_create(
    mut name: *const libc::c_char,
) -> *mut crate::src::tests::minicheck::Suite {
    let mut suite: *mut crate::src::tests::minicheck::Suite = crate::stdlib::calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<crate::src::tests::minicheck::Suite>() as libc::c_ulong,
    )
        as *mut crate::src::tests::minicheck::Suite;
    if !suite.is_null() {
        (*suite).name = name
    }
    return suite;
}
#[no_mangle]

pub unsafe extern "C" fn tcase_create(
    mut name: *const libc::c_char,
) -> *mut crate::src::tests::minicheck::TCase {
    let mut tc: *mut crate::src::tests::minicheck::TCase = crate::stdlib::calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<crate::src::tests::minicheck::TCase>() as libc::c_ulong,
    )
        as *mut crate::src::tests::minicheck::TCase;
    if !tc.is_null() {
        (*tc).name = name
    }
    return tc;
}
#[no_mangle]

pub unsafe extern "C" fn suite_add_tcase(
    mut suite: *mut crate::src::tests::minicheck::Suite,
    mut tc: *mut crate::src::tests::minicheck::TCase,
) {
    if !suite.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"suite != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/minicheck.c\x00"
                as *const u8 as *const libc::c_char,
            66 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"void suite_add_tcase(Suite *, TCase *)\x00",
            ))
            .as_ptr(),
        );
    }
    if !tc.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"tc != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/minicheck.c\x00"
                as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"void suite_add_tcase(Suite *, TCase *)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*tc).next_tcase.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"tc->next_tcase == NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/minicheck.c\x00"
                as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"void suite_add_tcase(Suite *, TCase *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*tc).next_tcase = (*suite).tests;
    (*suite).tests = tc;
}
#[no_mangle]

pub unsafe extern "C" fn tcase_add_checked_fixture(
    mut tc: *mut crate::src::tests::minicheck::TCase,
    mut setup: crate::src::tests::minicheck::tcase_setup_function,
    mut teardown: crate::src::tests::minicheck::tcase_teardown_function,
) {
    if !tc.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"tc != NULL\x00" as *const u8 as *const libc::c_char,
                      b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/minicheck.c\x00"
                          as *const u8 as *const libc::c_char,
                      77 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 87],
                                                &[libc::c_char; 87]>(b"void tcase_add_checked_fixture(TCase *, tcase_setup_function, tcase_teardown_function)\x00")).as_ptr());
    }
    (*tc).setup = setup;
    (*tc).teardown = teardown;
}
#[no_mangle]

pub unsafe extern "C" fn tcase_add_test(
    mut tc: *mut crate::src::tests::minicheck::TCase,
    mut test: crate::src::tests::minicheck::tcase_test_function,
) {
    if !tc.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"tc != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/minicheck.c\x00"
                as *const u8 as *const libc::c_char,
            84 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void tcase_add_test(TCase *, tcase_test_function)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*tc).allocated == (*tc).ntests {
        let mut nalloc: libc::c_int = (*tc).allocated + 100 as libc::c_int;
        let mut new_size: crate::stddef_h::size_t = (::std::mem::size_of::<
            crate::src::tests::minicheck::tcase_test_function,
        >() as libc::c_ulong)
            .wrapping_mul(nalloc as libc::c_ulong);
        let mut new_tests: *mut crate::src::tests::minicheck::tcase_test_function =
            crate::stdlib::realloc((*tc).tests as *mut libc::c_void, new_size)
                as *mut crate::src::tests::minicheck::tcase_test_function;
        if !new_tests.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"new_tests != NULL\x00" as *const u8 as *const libc::c_char,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/minicheck.c\x00"
                    as *const u8 as *const libc::c_char,
                89 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                    b"void tcase_add_test(TCase *, tcase_test_function)\x00",
                ))
                .as_ptr(),
            );
        }
        (*tc).tests = new_tests;
        (*tc).allocated = nalloc
    }
    let ref mut fresh0 = *(*tc).tests.offset((*tc).ntests as isize);
    *fresh0 = test;
    (*tc).ntests += 1;
}

unsafe extern "C" fn tcase_free(mut tc: *mut crate::src::tests::minicheck::TCase) {
    if tc.is_null() {
        return;
    }
    ::libc::free((*tc).tests as *mut libc::c_void);
    ::libc::free(tc as *mut libc::c_void);
}

unsafe extern "C" fn suite_free(mut suite: *mut crate::src::tests::minicheck::Suite) {
    if suite.is_null() {
        return;
    }
    while !(*suite).tests.is_null() {
        let mut next: *mut crate::src::tests::minicheck::TCase = (*(*suite).tests).next_tcase;
        tcase_free((*suite).tests);
        (*suite).tests = next
    }
    ::libc::free(suite as *mut libc::c_void);
}
#[no_mangle]

pub unsafe extern "C" fn srunner_create(
    mut suite: *mut crate::src::tests::minicheck::Suite,
) -> *mut crate::src::tests::minicheck::SRunner {
    let mut runner: *mut crate::src::tests::minicheck::SRunner = crate::stdlib::calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<crate::src::tests::minicheck::SRunner>() as libc::c_ulong,
    )
        as *mut crate::src::tests::minicheck::SRunner;
    if !runner.is_null() {
        (*runner).suite = suite
    }
    return runner;
}

static mut env: crate::stdlib::jmp_buf = [crate::stdlib::__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: crate::stdlib::__sigset_t { __val: [0; 16] },
}; 1];

static mut _check_current_function: *const libc::c_char =
    crate::stddef_h::NULL as *const libc::c_char;

static mut _check_current_lineno: libc::c_int = -(1 as libc::c_int);

static mut _check_current_filename: *const libc::c_char =
    crate::stddef_h::NULL as *const libc::c_char;
/* Internal helper. */
#[no_mangle]

pub unsafe extern "C" fn _check_set_test_info(
    mut function: *const libc::c_char,
    mut filename: *const libc::c_char,
    mut lineno: libc::c_int,
) {
    _check_current_function = function;
    _check_current_lineno = lineno;
    _check_current_filename = filename;
}

unsafe extern "C" fn add_failure(
    mut runner: *mut crate::src::tests::minicheck::SRunner,
    mut verbosity: libc::c_int,
) {
    (*runner).nfailures += 1;
    if verbosity >= crate::src::tests::minicheck::CK_VERBOSE {
        ::libc::printf(
            b"%s:%d: %s\n\x00" as *const u8 as *const libc::c_char,
            _check_current_filename,
            _check_current_lineno,
            _check_current_function,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn srunner_run_all(
    mut runner: *mut crate::src::tests::minicheck::SRunner,
    mut verbosity: libc::c_int,
) {
    let mut suite: *mut crate::src::tests::minicheck::Suite =
        0 as *mut crate::src::tests::minicheck::Suite;
    let mut tc: *mut crate::src::tests::minicheck::TCase =
        0 as *mut crate::src::tests::minicheck::TCase;
    if !runner.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"runner != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/minicheck.c\x00"
                as *const u8 as *const libc::c_char,
            156 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"void srunner_run_all(SRunner *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    suite = (*runner).suite;
    ::std::ptr::write_volatile(
        &mut tc as *mut *mut crate::src::tests::minicheck::TCase,
        (*suite).tests,
    );
    while !tc.is_null() {
        let mut i: libc::c_int = 0;
        let mut current_block_10: u64;
        ::std::ptr::write_volatile(&mut i as *mut libc::c_int, 0 as libc::c_int);
        while i < (*tc).ntests {
            (*runner).nchecks += 1;
            if (*tc).setup.is_some() {
                /* setup */
                if crate::stdlib::_setjmp(env.as_mut_ptr()) != 0 {
                    add_failure(runner, verbosity);
                    current_block_10 = 11875828834189669668;
                } else {
                    (*tc).setup.expect("non-null function pointer")();
                    current_block_10 = 2979737022853876585;
                }
            } else {
                current_block_10 = 2979737022853876585;
            }
            match current_block_10 {
                2979737022853876585 =>
                /* test */
                {
                    if crate::stdlib::_setjmp(env.as_mut_ptr()) != 0 {
                        add_failure(runner, verbosity);
                    } else {
                        (*(*tc).tests.offset(i as isize)).expect("non-null function pointer")();
                        /* teardown */
                        if (*tc).teardown.is_some() {
                            if crate::stdlib::_setjmp(env.as_mut_ptr()) != 0 {
                                add_failure(runner, verbosity);
                            } else {
                                (*tc).teardown.expect("non-null function pointer")();
                            }
                        }
                    }
                }
                _ => {}
            }
            ::std::ptr::write_volatile(
                &mut i as *mut libc::c_int,
                ::std::ptr::read_volatile::<libc::c_int>(&i as *const libc::c_int) + 1,
            )
        }
        ::std::ptr::write_volatile(
            &mut tc as *mut *mut crate::src::tests::minicheck::TCase,
            (*tc).next_tcase,
        )
    }
    if verbosity != 0 {
        let mut passed: libc::c_int = (*runner).nchecks - (*runner).nfailures;
        let mut percentage: libc::c_double =
            passed as libc::c_double / (*runner).nchecks as libc::c_double;
        let mut display: libc::c_int =
            (percentage * 100 as libc::c_int as libc::c_double) as libc::c_int;
        ::libc::printf(
            b"%d%%: Checks: %d, Failed: %d\n\x00" as *const u8 as *const libc::c_char,
            display,
            (*runner).nchecks,
            (*runner).nfailures,
        );
    };
}
/*
 * Prototypes for the actual implementation.
 */
#[no_mangle]

pub unsafe extern "C" fn _fail_unless(
    mut _condition: libc::c_int,
    mut _file: *const libc::c_char,
    mut _line: libc::c_int,
    mut msg: *const libc::c_char,
) {
    /* Always print the error message so it isn't lost.  In this case,
       we have a failure, so there's no reason to be quiet about what
       it is.
    */
    if !msg.is_null() {
        let has_newline: libc::c_int = (*msg.offset(
            crate::stdlib::strlen(msg).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) as libc::c_int
            == '\n' as i32) as libc::c_int;
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"ERROR: %s%s\x00" as *const u8 as *const libc::c_char,
            msg,
            if has_newline != 0 {
                b"\x00" as *const u8 as *const libc::c_char
            } else {
                b"\n\x00" as *const u8 as *const libc::c_char
            },
        );
    }
    crate::stdlib::longjmp(env.as_mut_ptr(), 1 as libc::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn srunner_ntests_failed(
    mut runner: *mut crate::src::tests::minicheck::SRunner,
) -> libc::c_int {
    if !runner.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"runner != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/minicheck.c\x00"
                as *const u8 as *const libc::c_char,
            217 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"int srunner_ntests_failed(SRunner *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*runner).nfailures;
}
#[no_mangle]

pub unsafe extern "C" fn srunner_free(mut runner: *mut crate::src::tests::minicheck::SRunner) {
    if runner.is_null() {
        return;
    }
    suite_free((*runner).suite);
    ::libc::free(runner as *mut libc::c_void);
}
