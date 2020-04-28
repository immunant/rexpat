// =============== BEGIN minicheck_h ================
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
   Portions copyright (c) 2020 Immunant, Inc.
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
use crate::stdlib::{stderr};
use ::libc::{free, printf, fprintf};
use ::std::ptr::write_volatile;
use libc::{c_char, c_double, c_int, realloc, c_void, calloc, strlen};
pub const CK_VERBOSE: c_int = 2;

pub type tcase_setup_function = Option<unsafe extern "C" fn() -> ()>;

pub type tcase_teardown_function = Option<unsafe extern "C" fn() -> ()>;

pub type tcase_test_function = Option<unsafe extern "C" fn() -> ()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SRunner {
    pub suite: *mut Suite,
    pub nchecks: c_int,
    pub nfailures: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Suite {
    pub name: *const c_char,
    pub tests: *mut TCase,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCase {
    pub name: *const c_char,
    pub setup: tcase_setup_function,
    pub teardown: tcase_teardown_function,
    pub tests: *mut tcase_test_function,
    pub ntests: c_int,
    pub allocated: c_int,
    pub next_tcase: *mut TCase,
}

pub use crate::stdlib::{
    __jmp_buf, __jmp_buf_tag,
    __sigset_t, _setjmp, jmp_buf, longjmp,
};
use ::libc;

#[no_mangle]
pub unsafe extern "C" fn suite_create(mut name: *const c_char) -> *mut Suite {
    let mut suite: *mut Suite = calloc(1, ::std::mem::size_of::<Suite>()) as *mut Suite;
    if !suite.is_null() {
        (*suite).name = name
    }
    return suite;
}
#[no_mangle]
pub unsafe extern "C" fn tcase_create(mut name: *const c_char) -> *mut TCase {
    let mut tc: *mut TCase = calloc(1, ::std::mem::size_of::<TCase>()) as *mut TCase;
    if !tc.is_null() {
        (*tc).name = name
    }
    return tc;
}
#[no_mangle]
pub unsafe extern "C" fn suite_add_tcase(mut suite: *mut Suite, mut tc: *mut TCase) {
    assert!(!suite.is_null());
    assert!(!tc.is_null());
    assert!((*tc).next_tcase.is_null());
    (*tc).next_tcase = (*suite).tests;
    (*suite).tests = tc;
}
#[no_mangle]

pub unsafe extern "C" fn tcase_add_checked_fixture(
    mut tc: *mut TCase,
    mut setup: tcase_setup_function,
    mut teardown: tcase_teardown_function,
) {
    assert!(!tc.is_null());
    (*tc).setup = setup;
    (*tc).teardown = teardown;
}
#[no_mangle]

pub unsafe extern "C" fn tcase_add_test(mut tc: *mut TCase, mut test: tcase_test_function) {
    assert!(!tc.is_null());
    if (*tc).allocated == (*tc).ntests {
        let mut nalloc: c_int = (*tc).allocated + 100;
        let mut new_size = ::std::mem::size_of::<tcase_test_function>()
            .wrapping_mul(nalloc as usize);
        let mut new_tests: *mut tcase_test_function =
            realloc((*tc).tests as *mut c_void, new_size) as *mut tcase_test_function;
        assert!(!new_tests.is_null());
        (*tc).tests = new_tests;
        (*tc).allocated = nalloc
    }
    let ref mut fresh0 = *(*tc).tests.offset((*tc).ntests as isize);
    *fresh0 = test;
    (*tc).ntests += 1;
}

unsafe extern "C" fn tcase_free(mut tc: *mut TCase) {
    if tc.is_null() {
        return;
    }
    free((*tc).tests as *mut c_void);
    free(tc as *mut c_void);
}

unsafe extern "C" fn suite_free(mut suite: *mut Suite) {
    if suite.is_null() {
        return;
    }
    while !(*suite).tests.is_null() {
        let mut next: *mut TCase = (*(*suite).tests).next_tcase;
        tcase_free((*suite).tests);
        (*suite).tests = next
    }
    free(suite as *mut c_void);
}
#[no_mangle]

pub unsafe extern "C" fn srunner_create(mut suite: *mut Suite) -> *mut SRunner {
    let mut runner: *mut SRunner =
        calloc(1, ::std::mem::size_of::<SRunner>()) as *mut SRunner;
    if !runner.is_null() {
        (*runner).suite = suite
    }
    return runner;
}

static mut env: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];

static mut _check_current_function: *const c_char = std::ptr::null();

static mut _check_current_lineno: c_int = -(1);

static mut _check_current_filename: *const c_char = std::ptr::null();
/* Internal helper. */
#[no_mangle]

pub unsafe extern "C" fn _check_set_test_info(
    mut function: *const c_char,
    mut filename: *const c_char,
    mut lineno: c_int,
) {
    _check_current_function = function;
    _check_current_lineno = lineno;
    _check_current_filename = filename;
}

unsafe extern "C" fn add_failure(mut runner: *mut SRunner, mut verbosity: c_int) {
    (*runner).nfailures += 1;
    if verbosity >= CK_VERBOSE {
        printf(
            b"%s:%d: %s\n\x00".as_ptr() as *const c_char,
            _check_current_filename,
            _check_current_lineno,
            _check_current_function,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn srunner_run_all(mut runner: *mut SRunner, mut verbosity: c_int) {
    let mut suite: *mut Suite = 0 as *mut Suite;
    let mut tc: *mut TCase = 0 as *mut TCase;
    assert!(!runner.is_null());
    suite = (*runner).suite;
    write_volatile(&mut tc as *mut *mut TCase, (*suite).tests);
    while !tc.is_null() {
        let mut i: c_int = 0;
        let mut current_block_10: u64;
        write_volatile(&mut i as *mut c_int, 0);
        while i < (*tc).ntests {
            (*runner).nchecks += 1;
            if (*tc).setup.is_some() {
                /* setup */
                if _setjmp(env.as_mut_ptr()) != 0 {
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
                    if _setjmp(env.as_mut_ptr()) != 0 {
                        add_failure(runner, verbosity);
                    } else {
                        (*(*tc).tests.offset(i as isize)).expect("non-null function pointer")();
                        /* teardown */
                        if (*tc).teardown.is_some() {
                            if _setjmp(env.as_mut_ptr()) != 0 {
                                add_failure(runner, verbosity);
                            } else {
                                (*tc).teardown.expect("non-null function pointer")();
                            }
                        }
                    }
                }
                _ => {}
            }
            write_volatile(
                &mut i as *mut c_int,
                ::std::ptr::read_volatile::<c_int>(&i as *const c_int) + 1,
            )
        }
        write_volatile(&mut tc as *mut *mut TCase, (*tc).next_tcase)
    }
    if verbosity != 0 {
        let mut passed: c_int = (*runner).nchecks - (*runner).nfailures;
        let mut percentage: c_double = passed as c_double / (*runner).nchecks as c_double;
        let mut display: c_int = (percentage * 100f64) as c_int;
        printf(
            b"%d%%: Checks: %d, Failed: %d\n\x00".as_ptr() as *const c_char,
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
    mut _condition: c_int,
    mut _file: *const c_char,
    mut _line: c_int,
    mut msg: *const c_char,
) {
    /* Always print the error message so it isn't lost.  In this case,
       we have a failure, so there's no reason to be quiet about what
       it is.
    */
    if !msg.is_null() {
        let has_newline: c_int =
            (*msg.offset(strlen(msg).wrapping_sub(1) as isize) as c_int == '\n' as i32) as c_int;
        fprintf(
            stderr,
            b"ERROR: %s%s\x00".as_ptr() as *const c_char,
            msg,
            if has_newline != 0 {
                b"\x00".as_ptr() as *const c_char
            } else {
                b"\n\x00".as_ptr() as *const c_char
            },
        );
    }
    longjmp(env.as_mut_ptr(), 1);
}
#[no_mangle]

pub unsafe extern "C" fn srunner_ntests_failed(mut runner: *mut SRunner) -> c_int {
    assert!(!runner.is_null());
    return (*runner).nfailures;
}
#[no_mangle]

pub unsafe extern "C" fn srunner_free(mut runner: *mut SRunner) {
    if runner.is_null() {
        return;
    }
    suite_free((*runner).suite);
    free(runner as *mut c_void);
}
