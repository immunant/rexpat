use crate::stddef_h::size_t;
use crate::stdlib::uint64_t;
use libc::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siphash {
    pub v0: uint64_t,
    pub v1: uint64_t,
    pub v2: uint64_t,
    pub v3: uint64_t,
    pub buf: [c_uchar; 8],
    pub p: *mut c_uchar,
    pub c: uint64_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sipkey {
    pub k: [uint64_t; 2],
}

/* ==========================================================================
 * siphash.h - SipHash-2-4 in a single header file
 * --------------------------------------------------------------------------
 * Derived by William Ahern from the reference implementation[1] published[2]
 * by Jean-Philippe Aumasson and Daniel J. Berstein.
 * Minimal changes by Sebastian Pipping and Victor Stinner on top, see below.
 * Licensed under the CC0 Public Domain Dedication license.
 *
 * 1. https://www.131002.net/siphash/siphash24.c
 * 2. https://www.131002.net/siphash/
 * --------------------------------------------------------------------------
 * HISTORY:
 *
 * 2019-08-03  (Sebastian Pipping)
 *   - Mark part of sip24_valid as to be excluded from clang-format
 *   - Re-format code using clang-format 9
 *
 * 2018-07-08  (Anton Maklakov)
 *   - Add "fall through" markers for GCC's -Wimplicit-fallthrough
 *
 * 2017-11-03  (Sebastian Pipping)
 *   - Hide sip_tobin and sip_binof unless SIPHASH_TOBIN macro is defined
 *
 * 2017-07-25  (Vadim Zeitlin)
 *   - Fix use of SIPHASH_MAIN macro
 *
 * 2017-07-05  (Sebastian Pipping)
 *   - Use _SIP_ULL macro to not require a C++11 compiler if compiled as C++
 *   - Add const qualifiers at two places
 *   - Ensure <=80 characters line length (assuming tab width 4)
 *
 * 2017-06-23  (Victor Stinner)
 *   - Address Win64 compile warnings
 *
 * 2017-06-18  (Sebastian Pipping)
 *   - Clarify license note in the header
 *   - Address C89 issues:
 *     - Stop using inline keyword (and let compiler decide)
 *     - Replace _Bool by int
 *     - Turn macro siphash24 into a function
 *     - Address invalid conversion (void pointer) by explicit cast
 *   - Address lack of stdint.h for Visual Studio 2003 to 2008
 *   - Always expose sip24_valid (for self-tests)
 *
 * 2012-11-04 - Born.  (William Ahern)
 * --------------------------------------------------------------------------
 * USAGE:
 *
 * SipHash-2-4 takes as input two 64-bit words as the key, some number of
 * message bytes, and outputs a 64-bit word as the message digest. This
 * implementation employs two data structures: a struct sipkey for
 * representing the key, and a struct siphash for representing the hash
 * state.
 *
 * For converting a 16-byte unsigned char array to a key, use either the
 * macro sip_keyof or the routine sip_tokey. The former instantiates a
 * compound literal key, while the latter requires a key object as a
 * parameter.
 *
 * 	unsigned char secret[16];
 * 	arc4random_buf(secret, sizeof secret);
 * 	struct sipkey *key = sip_keyof(secret);
 *
 * For hashing a message, use either the convenience macro siphash24 or the
 * routines sip24_init, sip24_update, and sip24_final.
 *
 * 	struct siphash state;
 * 	void *msg;
 * 	size_t len;
 * 	uint64_t hash;
 *
 * 	sip24_init(&state, key);
 * 	sip24_update(&state, msg, len);
 * 	hash = sip24_final(&state);
 *
 * or
 *
 * 	hash = siphash24(msg, len, key);
 *
 * To convert the 64-bit hash value to a canonical 8-byte little-endian
 * binary representation, use either the macro sip_binof or the routine
 * sip_tobin. The former instantiates and returns a compound literal array,
 * while the latter requires an array object as a parameter.
 * --------------------------------------------------------------------------
 * NOTES:
 *
 * o Neither sip_keyof, sip_binof, nor siphash24 will work with compilers
 *   lacking compound literal support. Instead, you must use the lower-level
 *   interfaces which take as parameters the temporary state objects.
 *
 * o Uppercase macros may evaluate parameters more than once. Lowercase
 *   macros should not exhibit any such side effects.
 * ==========================================================================
 */
/* size_t */
/* uint64_t uint32_t uint8_t */
/*
 * Workaround to not require a C++11 compiler for using ULL suffix
 * if this code is included and compiled as C++; related GCC warning is:
 * warning: use of C++11 long long integer constant [-Wlong-long]
 */
/* struct siphash */
/* struct sipkey */
/* sip_tokey() */
/* SIPHASH_TOBIN */
/* sip_round() */
/* sip24_init() */

pub unsafe extern "C" fn sip24_update(
    mut H: *mut siphash,
    mut src: *const c_void,
    mut len: size_t,
) -> *mut siphash {
    let mut p: *const c_uchar = src as *const c_uchar;
    let mut pe: *const c_uchar = p.offset(len as isize);
    let mut m: uint64_t = 0;
    loop {
        while p < pe
            && (*H).p
                < &mut *(*H).buf.as_mut_ptr().offset(
                    (::std::mem::size_of::<[c_uchar; 8]>() as c_ulong)
                        .wrapping_div(::std::mem::size_of::<c_uchar>() as c_ulong)
                        as isize,
                ) as *mut c_uchar
        {
            let fresh0 = p;
            p = p.offset(1);
            let fresh1 = (*H).p;
            (*H).p = (*H).p.offset(1);
            *fresh1 = *fresh0
        }
        if (*H).p
            < &mut *(*H).buf.as_mut_ptr().offset(
                (::std::mem::size_of::<[c_uchar; 8]>() as c_ulong)
                    .wrapping_div(::std::mem::size_of::<c_uchar>() as c_ulong)
                    as isize,
            ) as *mut c_uchar
        {
            break;
        }
        m = ((*H).buf[0 as c_int as usize] as uint64_t) << 0 as c_int
            | ((*H).buf[1 as c_int as usize] as uint64_t) << 8 as c_int
            | ((*H).buf[2 as c_int as usize] as uint64_t) << 16 as c_int
            | ((*H).buf[3 as c_int as usize] as uint64_t) << 24 as c_int
            | ((*H).buf[4 as c_int as usize] as uint64_t) << 32 as c_int
            | ((*H).buf[5 as c_int as usize] as uint64_t) << 40 as c_int
            | ((*H).buf[6 as c_int as usize] as uint64_t) << 48 as c_int
            | ((*H).buf[7 as c_int as usize] as uint64_t) << 56 as c_int;
        (*H).v3 ^= m;
        sip_round(H, 2 as c_int);
        (*H).v0 ^= m;
        (*H).p = (*H).buf.as_mut_ptr();
        (*H).c = ((*H).c as c_ulong).wrapping_add(8 as c_int as c_ulong) as uint64_t as uint64_t;
        if !(p < pe) {
            break;
        }
    }
    return H;
}

pub unsafe extern "C" fn sip_round(mut H: *mut siphash, rounds: c_int) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < rounds {
        (*H).v0 = ((*H).v0 as c_ulong).wrapping_add((*H).v1) as uint64_t as uint64_t;
        (*H).v1 = (*H).v1 << 13 as c_int | (*H).v1 >> 64 as c_int - 13 as c_int;
        (*H).v1 ^= (*H).v0;
        (*H).v0 = (*H).v0 << 32 as c_int | (*H).v0 >> 64 as c_int - 32 as c_int;
        (*H).v2 = ((*H).v2 as c_ulong).wrapping_add((*H).v3) as uint64_t as uint64_t;
        (*H).v3 = (*H).v3 << 16 as c_int | (*H).v3 >> 64 as c_int - 16 as c_int;
        (*H).v3 ^= (*H).v2;
        (*H).v0 = ((*H).v0 as c_ulong).wrapping_add((*H).v3) as uint64_t as uint64_t;
        (*H).v3 = (*H).v3 << 21 as c_int | (*H).v3 >> 64 as c_int - 21 as c_int;
        (*H).v3 ^= (*H).v0;
        (*H).v2 = ((*H).v2 as c_ulong).wrapping_add((*H).v1) as uint64_t as uint64_t;
        (*H).v1 = (*H).v1 << 17 as c_int | (*H).v1 >> 64 as c_int - 17 as c_int;
        (*H).v1 ^= (*H).v2;
        (*H).v2 = (*H).v2 << 32 as c_int | (*H).v2 >> 64 as c_int - 32 as c_int;
        i += 1
    }
}
/* sip24_update() */

pub unsafe extern "C" fn sip24_final(mut H: *mut siphash) -> uint64_t {
    let left: c_char = (*H).p.wrapping_offset_from((*H).buf.as_mut_ptr()) as c_long as c_char;
    let mut b: uint64_t = (*H).c.wrapping_add(left as c_ulong) << 56 as c_int;
    let mut current_block_6: u64;
    match left as c_int {
        7 => {
            b |= ((*H).buf[6 as c_int as usize] as uint64_t) << 48 as c_int;
            current_block_6 = 15779782992241478193;
        }
        6 => {
            current_block_6 = 15779782992241478193;
        }
        5 => {
            current_block_6 = 17438419808098997991;
        }
        4 => {
            current_block_6 = 3917958908645891603;
        }
        3 => {
            current_block_6 = 14025370785517308083;
        }
        2 => {
            current_block_6 = 6645080409601096633;
        }
        1 => {
            current_block_6 = 15385381413911169552;
        }
        0 | _ => {
            current_block_6 = 17965632435239708295;
        }
    }
    match current_block_6 {
        15779782992241478193 =>
        /* fall through */
        {
            b |= ((*H).buf[5 as c_int as usize] as uint64_t) << 40 as c_int;
            current_block_6 = 17438419808098997991;
        }
        _ => {}
    }
    match current_block_6 {
        17438419808098997991 =>
        /* fall through */
        {
            b |= ((*H).buf[4 as c_int as usize] as uint64_t) << 32 as c_int;
            current_block_6 = 3917958908645891603;
        }
        _ => {}
    }
    match current_block_6 {
        3917958908645891603 =>
        /* fall through */
        {
            b |= ((*H).buf[3 as c_int as usize] as uint64_t) << 24 as c_int;
            current_block_6 = 14025370785517308083;
        }
        _ => {}
    }
    match current_block_6 {
        14025370785517308083 =>
        /* fall through */
        {
            b |= ((*H).buf[2 as c_int as usize] as uint64_t) << 16 as c_int;
            current_block_6 = 6645080409601096633;
        }
        _ => {}
    }
    match current_block_6 {
        6645080409601096633 =>
        /* fall through */
        {
            b |= ((*H).buf[1 as c_int as usize] as uint64_t) << 8 as c_int;
            current_block_6 = 15385381413911169552;
        }
        _ => {}
    }
    match current_block_6 {
        15385381413911169552 =>
        /* fall through */
        {
            b |= ((*H).buf[0 as c_int as usize] as uint64_t) << 0 as c_int
        }
        _ => {}
    }
    (*H).v3 ^= b;
    sip_round(H, 2 as c_int);
    (*H).v0 ^= b;
    (*H).v2 ^= 0xff as c_int as c_ulong;
    sip_round(H, 4 as c_int);
    return (*H).v0 ^ (*H).v1 ^ (*H).v2 ^ (*H).v3;
}

pub unsafe extern "C" fn sip_tokey(mut key: *mut sipkey, mut src: *const c_void) -> *mut sipkey {
    (*key).k[0 as c_int as usize] =
        (*(src as *const c_uchar).offset(0 as c_int as isize) as uint64_t) << 0 as c_int
            | (*(src as *const c_uchar).offset(1 as c_int as isize) as uint64_t) << 8 as c_int
            | (*(src as *const c_uchar).offset(2 as c_int as isize) as uint64_t) << 16 as c_int
            | (*(src as *const c_uchar).offset(3 as c_int as isize) as uint64_t) << 24 as c_int
            | (*(src as *const c_uchar).offset(4 as c_int as isize) as uint64_t) << 32 as c_int
            | (*(src as *const c_uchar).offset(5 as c_int as isize) as uint64_t) << 40 as c_int
            | (*(src as *const c_uchar).offset(6 as c_int as isize) as uint64_t) << 48 as c_int
            | (*(src as *const c_uchar).offset(7 as c_int as isize) as uint64_t) << 56 as c_int;
    (*key).k[1 as c_int as usize] = (*(src as *const c_uchar)
        .offset(8 as c_int as isize)
        .offset(0 as c_int as isize) as uint64_t)
        << 0 as c_int
        | (*(src as *const c_uchar)
            .offset(8 as c_int as isize)
            .offset(1 as c_int as isize) as uint64_t)
            << 8 as c_int
        | (*(src as *const c_uchar)
            .offset(8 as c_int as isize)
            .offset(2 as c_int as isize) as uint64_t)
            << 16 as c_int
        | (*(src as *const c_uchar)
            .offset(8 as c_int as isize)
            .offset(3 as c_int as isize) as uint64_t)
            << 24 as c_int
        | (*(src as *const c_uchar)
            .offset(8 as c_int as isize)
            .offset(4 as c_int as isize) as uint64_t)
            << 32 as c_int
        | (*(src as *const c_uchar)
            .offset(8 as c_int as isize)
            .offset(5 as c_int as isize) as uint64_t)
            << 40 as c_int
        | (*(src as *const c_uchar)
            .offset(8 as c_int as isize)
            .offset(6 as c_int as isize) as uint64_t)
            << 48 as c_int
        | (*(src as *const c_uchar)
            .offset(8 as c_int as isize)
            .offset(7 as c_int as isize) as uint64_t)
            << 56 as c_int;
    return key;
}
/* sip24_final() */
/* siphash24() */
/*
 * SipHash-2-4 output with
 * k = 00 01 02 ...
 * and
 * in = (empty string)
 * in = 00 (1 byte)
 * in = 00 01 (2 bytes)
 * in = 00 01 02 (3 bytes)
 * ...
 * in = 00 01 02 ... 3e (63 bytes)
 */

pub unsafe extern "C" fn sip24_valid() -> c_int {
    /* clang-format off */
    pub static mut vectors: [[c_uchar; 8]; 64] = [
        [
            0x31 as c_int as c_uchar,
            0xe as c_int as c_uchar,
            0xe as c_int as c_uchar,
            0xdd as c_int as c_uchar,
            0x47 as c_int as c_uchar,
            0xdb as c_int as c_uchar,
            0x6f as c_int as c_uchar,
            0x72 as c_int as c_uchar,
        ],
        [
            0xfd as c_int as c_uchar,
            0x67 as c_int as c_uchar,
            0xdc as c_int as c_uchar,
            0x93 as c_int as c_uchar,
            0xc5 as c_int as c_uchar,
            0x39 as c_int as c_uchar,
            0xf8 as c_int as c_uchar,
            0x74 as c_int as c_uchar,
        ],
        [
            0x5a as c_int as c_uchar,
            0x4f as c_int as c_uchar,
            0xa9 as c_int as c_uchar,
            0xd9 as c_int as c_uchar,
            0x9 as c_int as c_uchar,
            0x80 as c_int as c_uchar,
            0x6c as c_int as c_uchar,
            0xd as c_int as c_uchar,
        ],
        [
            0x2d as c_int as c_uchar,
            0x7e as c_int as c_uchar,
            0xfb as c_int as c_uchar,
            0xd7 as c_int as c_uchar,
            0x96 as c_int as c_uchar,
            0x66 as c_int as c_uchar,
            0x67 as c_int as c_uchar,
            0x85 as c_int as c_uchar,
        ],
        [
            0xb7 as c_int as c_uchar,
            0x87 as c_int as c_uchar,
            0x71 as c_int as c_uchar,
            0x27 as c_int as c_uchar,
            0xe0 as c_int as c_uchar,
            0x94 as c_int as c_uchar,
            0x27 as c_int as c_uchar,
            0xcf as c_int as c_uchar,
        ],
        [
            0x8d as c_int as c_uchar,
            0xa6 as c_int as c_uchar,
            0x99 as c_int as c_uchar,
            0xcd as c_int as c_uchar,
            0x64 as c_int as c_uchar,
            0x55 as c_int as c_uchar,
            0x76 as c_int as c_uchar,
            0x18 as c_int as c_uchar,
        ],
        [
            0xce as c_int as c_uchar,
            0xe3 as c_int as c_uchar,
            0xfe as c_int as c_uchar,
            0x58 as c_int as c_uchar,
            0x6e as c_int as c_uchar,
            0x46 as c_int as c_uchar,
            0xc9 as c_int as c_uchar,
            0xcb as c_int as c_uchar,
        ],
        [
            0x37 as c_int as c_uchar,
            0xd1 as c_int as c_uchar,
            0x1 as c_int as c_uchar,
            0x8b as c_int as c_uchar,
            0xf5 as c_int as c_uchar,
            0 as c_int as c_uchar,
            0x2 as c_int as c_uchar,
            0xab as c_int as c_uchar,
        ],
        [
            0x62 as c_int as c_uchar,
            0x24 as c_int as c_uchar,
            0x93 as c_int as c_uchar,
            0x9a as c_int as c_uchar,
            0x79 as c_int as c_uchar,
            0xf5 as c_int as c_uchar,
            0xf5 as c_int as c_uchar,
            0x93 as c_int as c_uchar,
        ],
        [
            0xb0 as c_int as c_uchar,
            0xe4 as c_int as c_uchar,
            0xa9 as c_int as c_uchar,
            0xb as c_int as c_uchar,
            0xdf as c_int as c_uchar,
            0x82 as c_int as c_uchar,
            0 as c_int as c_uchar,
            0x9e as c_int as c_uchar,
        ],
        [
            0xf3 as c_int as c_uchar,
            0xb9 as c_int as c_uchar,
            0xdd as c_int as c_uchar,
            0x94 as c_int as c_uchar,
            0xc5 as c_int as c_uchar,
            0xbb as c_int as c_uchar,
            0x5d as c_int as c_uchar,
            0x7a as c_int as c_uchar,
        ],
        [
            0xa7 as c_int as c_uchar,
            0xad as c_int as c_uchar,
            0x6b as c_int as c_uchar,
            0x22 as c_int as c_uchar,
            0x46 as c_int as c_uchar,
            0x2f as c_int as c_uchar,
            0xb3 as c_int as c_uchar,
            0xf4 as c_int as c_uchar,
        ],
        [
            0xfb as c_int as c_uchar,
            0xe5 as c_int as c_uchar,
            0xe as c_int as c_uchar,
            0x86 as c_int as c_uchar,
            0xbc as c_int as c_uchar,
            0x8f as c_int as c_uchar,
            0x1e as c_int as c_uchar,
            0x75 as c_int as c_uchar,
        ],
        [
            0x90 as c_int as c_uchar,
            0x3d as c_int as c_uchar,
            0x84 as c_int as c_uchar,
            0xc0 as c_int as c_uchar,
            0x27 as c_int as c_uchar,
            0x56 as c_int as c_uchar,
            0xea as c_int as c_uchar,
            0x14 as c_int as c_uchar,
        ],
        [
            0xee as c_int as c_uchar,
            0xf2 as c_int as c_uchar,
            0x7a as c_int as c_uchar,
            0x8e as c_int as c_uchar,
            0x90 as c_int as c_uchar,
            0xca as c_int as c_uchar,
            0x23 as c_int as c_uchar,
            0xf7 as c_int as c_uchar,
        ],
        [
            0xe5 as c_int as c_uchar,
            0x45 as c_int as c_uchar,
            0xbe as c_int as c_uchar,
            0x49 as c_int as c_uchar,
            0x61 as c_int as c_uchar,
            0xca as c_int as c_uchar,
            0x29 as c_int as c_uchar,
            0xa1 as c_int as c_uchar,
        ],
        [
            0xdb as c_int as c_uchar,
            0x9b as c_int as c_uchar,
            0xc2 as c_int as c_uchar,
            0x57 as c_int as c_uchar,
            0x7f as c_int as c_uchar,
            0xcc as c_int as c_uchar,
            0x2a as c_int as c_uchar,
            0x3f as c_int as c_uchar,
        ],
        [
            0x94 as c_int as c_uchar,
            0x47 as c_int as c_uchar,
            0xbe as c_int as c_uchar,
            0x2c as c_int as c_uchar,
            0xf5 as c_int as c_uchar,
            0xe9 as c_int as c_uchar,
            0x9a as c_int as c_uchar,
            0x69 as c_int as c_uchar,
        ],
        [
            0x9c as c_int as c_uchar,
            0xd3 as c_int as c_uchar,
            0x8d as c_int as c_uchar,
            0x96 as c_int as c_uchar,
            0xf0 as c_int as c_uchar,
            0xb3 as c_int as c_uchar,
            0xc1 as c_int as c_uchar,
            0x4b as c_int as c_uchar,
        ],
        [
            0xbd as c_int as c_uchar,
            0x61 as c_int as c_uchar,
            0x79 as c_int as c_uchar,
            0xa7 as c_int as c_uchar,
            0x1d as c_int as c_uchar,
            0xc9 as c_int as c_uchar,
            0x6d as c_int as c_uchar,
            0xbb as c_int as c_uchar,
        ],
        [
            0x98 as c_int as c_uchar,
            0xee as c_int as c_uchar,
            0xa2 as c_int as c_uchar,
            0x1a as c_int as c_uchar,
            0xf2 as c_int as c_uchar,
            0x5c as c_int as c_uchar,
            0xd6 as c_int as c_uchar,
            0xbe as c_int as c_uchar,
        ],
        [
            0xc7 as c_int as c_uchar,
            0x67 as c_int as c_uchar,
            0x3b as c_int as c_uchar,
            0x2e as c_int as c_uchar,
            0xb0 as c_int as c_uchar,
            0xcb as c_int as c_uchar,
            0xf2 as c_int as c_uchar,
            0xd0 as c_int as c_uchar,
        ],
        [
            0x88 as c_int as c_uchar,
            0x3e as c_int as c_uchar,
            0xa3 as c_int as c_uchar,
            0xe3 as c_int as c_uchar,
            0x95 as c_int as c_uchar,
            0x67 as c_int as c_uchar,
            0x53 as c_int as c_uchar,
            0x93 as c_int as c_uchar,
        ],
        [
            0xc8 as c_int as c_uchar,
            0xce as c_int as c_uchar,
            0x5c as c_int as c_uchar,
            0xcd as c_int as c_uchar,
            0x8c as c_int as c_uchar,
            0x3 as c_int as c_uchar,
            0xc as c_int as c_uchar,
            0xa8 as c_int as c_uchar,
        ],
        [
            0x94 as c_int as c_uchar,
            0xaf as c_int as c_uchar,
            0x49 as c_int as c_uchar,
            0xf6 as c_int as c_uchar,
            0xc6 as c_int as c_uchar,
            0x50 as c_int as c_uchar,
            0xad as c_int as c_uchar,
            0xb8 as c_int as c_uchar,
        ],
        [
            0xea as c_int as c_uchar,
            0xb8 as c_int as c_uchar,
            0x85 as c_int as c_uchar,
            0x8a as c_int as c_uchar,
            0xde as c_int as c_uchar,
            0x92 as c_int as c_uchar,
            0xe1 as c_int as c_uchar,
            0xbc as c_int as c_uchar,
        ],
        [
            0xf3 as c_int as c_uchar,
            0x15 as c_int as c_uchar,
            0xbb as c_int as c_uchar,
            0x5b as c_int as c_uchar,
            0xb8 as c_int as c_uchar,
            0x35 as c_int as c_uchar,
            0xd8 as c_int as c_uchar,
            0x17 as c_int as c_uchar,
        ],
        [
            0xad as c_int as c_uchar,
            0xcf as c_int as c_uchar,
            0x6b as c_int as c_uchar,
            0x7 as c_int as c_uchar,
            0x63 as c_int as c_uchar,
            0x61 as c_int as c_uchar,
            0x2e as c_int as c_uchar,
            0x2f as c_int as c_uchar,
        ],
        [
            0xa5 as c_int as c_uchar,
            0xc9 as c_int as c_uchar,
            0x1d as c_int as c_uchar,
            0xa7 as c_int as c_uchar,
            0xac as c_int as c_uchar,
            0xaa as c_int as c_uchar,
            0x4d as c_int as c_uchar,
            0xde as c_int as c_uchar,
        ],
        [
            0x71 as c_int as c_uchar,
            0x65 as c_int as c_uchar,
            0x95 as c_int as c_uchar,
            0x87 as c_int as c_uchar,
            0x66 as c_int as c_uchar,
            0x50 as c_int as c_uchar,
            0xa2 as c_int as c_uchar,
            0xa6 as c_int as c_uchar,
        ],
        [
            0x28 as c_int as c_uchar,
            0xef as c_int as c_uchar,
            0x49 as c_int as c_uchar,
            0x5c as c_int as c_uchar,
            0x53 as c_int as c_uchar,
            0xa3 as c_int as c_uchar,
            0x87 as c_int as c_uchar,
            0xad as c_int as c_uchar,
        ],
        [
            0x42 as c_int as c_uchar,
            0xc3 as c_int as c_uchar,
            0x41 as c_int as c_uchar,
            0xd8 as c_int as c_uchar,
            0xfa as c_int as c_uchar,
            0x92 as c_int as c_uchar,
            0xd8 as c_int as c_uchar,
            0x32 as c_int as c_uchar,
        ],
        [
            0xce as c_int as c_uchar,
            0x7c as c_int as c_uchar,
            0xf2 as c_int as c_uchar,
            0x72 as c_int as c_uchar,
            0x2f as c_int as c_uchar,
            0x51 as c_int as c_uchar,
            0x27 as c_int as c_uchar,
            0x71 as c_int as c_uchar,
        ],
        [
            0xe3 as c_int as c_uchar,
            0x78 as c_int as c_uchar,
            0x59 as c_int as c_uchar,
            0xf9 as c_int as c_uchar,
            0x46 as c_int as c_uchar,
            0x23 as c_int as c_uchar,
            0xf3 as c_int as c_uchar,
            0xa7 as c_int as c_uchar,
        ],
        [
            0x38 as c_int as c_uchar,
            0x12 as c_int as c_uchar,
            0x5 as c_int as c_uchar,
            0xbb as c_int as c_uchar,
            0x1a as c_int as c_uchar,
            0xb0 as c_int as c_uchar,
            0xe0 as c_int as c_uchar,
            0x12 as c_int as c_uchar,
        ],
        [
            0xae as c_int as c_uchar,
            0x97 as c_int as c_uchar,
            0xa1 as c_int as c_uchar,
            0xf as c_int as c_uchar,
            0xd4 as c_int as c_uchar,
            0x34 as c_int as c_uchar,
            0xe0 as c_int as c_uchar,
            0x15 as c_int as c_uchar,
        ],
        [
            0xb4 as c_int as c_uchar,
            0xa3 as c_int as c_uchar,
            0x15 as c_int as c_uchar,
            0x8 as c_int as c_uchar,
            0xbe as c_int as c_uchar,
            0xff as c_int as c_uchar,
            0x4d as c_int as c_uchar,
            0x31 as c_int as c_uchar,
        ],
        [
            0x81 as c_int as c_uchar,
            0x39 as c_int as c_uchar,
            0x62 as c_int as c_uchar,
            0x29 as c_int as c_uchar,
            0xf0 as c_int as c_uchar,
            0x90 as c_int as c_uchar,
            0x79 as c_int as c_uchar,
            0x2 as c_int as c_uchar,
        ],
        [
            0x4d as c_int as c_uchar,
            0xc as c_int as c_uchar,
            0xf4 as c_int as c_uchar,
            0x9e as c_int as c_uchar,
            0xe5 as c_int as c_uchar,
            0xd4 as c_int as c_uchar,
            0xdc as c_int as c_uchar,
            0xca as c_int as c_uchar,
        ],
        [
            0x5c as c_int as c_uchar,
            0x73 as c_int as c_uchar,
            0x33 as c_int as c_uchar,
            0x6a as c_int as c_uchar,
            0x76 as c_int as c_uchar,
            0xd8 as c_int as c_uchar,
            0xbf as c_int as c_uchar,
            0x9a as c_int as c_uchar,
        ],
        [
            0xd0 as c_int as c_uchar,
            0xa7 as c_int as c_uchar,
            0x4 as c_int as c_uchar,
            0x53 as c_int as c_uchar,
            0x6b as c_int as c_uchar,
            0xa9 as c_int as c_uchar,
            0x3e as c_int as c_uchar,
            0xe as c_int as c_uchar,
        ],
        [
            0x92 as c_int as c_uchar,
            0x59 as c_int as c_uchar,
            0x58 as c_int as c_uchar,
            0xfc as c_int as c_uchar,
            0xd6 as c_int as c_uchar,
            0x42 as c_int as c_uchar,
            0xc as c_int as c_uchar,
            0xad as c_int as c_uchar,
        ],
        [
            0xa9 as c_int as c_uchar,
            0x15 as c_int as c_uchar,
            0xc2 as c_int as c_uchar,
            0x9b as c_int as c_uchar,
            0xc8 as c_int as c_uchar,
            0x6 as c_int as c_uchar,
            0x73 as c_int as c_uchar,
            0x18 as c_int as c_uchar,
        ],
        [
            0x95 as c_int as c_uchar,
            0x2b as c_int as c_uchar,
            0x79 as c_int as c_uchar,
            0xf3 as c_int as c_uchar,
            0xbc as c_int as c_uchar,
            0xa as c_int as c_uchar,
            0xa6 as c_int as c_uchar,
            0xd4 as c_int as c_uchar,
        ],
        [
            0xf2 as c_int as c_uchar,
            0x1d as c_int as c_uchar,
            0xf2 as c_int as c_uchar,
            0xe4 as c_int as c_uchar,
            0x1d as c_int as c_uchar,
            0x45 as c_int as c_uchar,
            0x35 as c_int as c_uchar,
            0xf9 as c_int as c_uchar,
        ],
        [
            0x87 as c_int as c_uchar,
            0x57 as c_int as c_uchar,
            0x75 as c_int as c_uchar,
            0x19 as c_int as c_uchar,
            0x4 as c_int as c_uchar,
            0x8f as c_int as c_uchar,
            0x53 as c_int as c_uchar,
            0xa9 as c_int as c_uchar,
        ],
        [
            0x10 as c_int as c_uchar,
            0xa5 as c_int as c_uchar,
            0x6c as c_int as c_uchar,
            0xf5 as c_int as c_uchar,
            0xdf as c_int as c_uchar,
            0xcd as c_int as c_uchar,
            0x9a as c_int as c_uchar,
            0xdb as c_int as c_uchar,
        ],
        [
            0xeb as c_int as c_uchar,
            0x75 as c_int as c_uchar,
            0x9 as c_int as c_uchar,
            0x5c as c_int as c_uchar,
            0xcd as c_int as c_uchar,
            0x98 as c_int as c_uchar,
            0x6c as c_int as c_uchar,
            0xd0 as c_int as c_uchar,
        ],
        [
            0x51 as c_int as c_uchar,
            0xa9 as c_int as c_uchar,
            0xcb as c_int as c_uchar,
            0x9e as c_int as c_uchar,
            0xcb as c_int as c_uchar,
            0xa3 as c_int as c_uchar,
            0x12 as c_int as c_uchar,
            0xe6 as c_int as c_uchar,
        ],
        [
            0x96 as c_int as c_uchar,
            0xaf as c_int as c_uchar,
            0xad as c_int as c_uchar,
            0xfc as c_int as c_uchar,
            0x2c as c_int as c_uchar,
            0xe6 as c_int as c_uchar,
            0x66 as c_int as c_uchar,
            0xc7 as c_int as c_uchar,
        ],
        [
            0x72 as c_int as c_uchar,
            0xfe as c_int as c_uchar,
            0x52 as c_int as c_uchar,
            0x97 as c_int as c_uchar,
            0x5a as c_int as c_uchar,
            0x43 as c_int as c_uchar,
            0x64 as c_int as c_uchar,
            0xee as c_int as c_uchar,
        ],
        [
            0x5a as c_int as c_uchar,
            0x16 as c_int as c_uchar,
            0x45 as c_int as c_uchar,
            0xb2 as c_int as c_uchar,
            0x76 as c_int as c_uchar,
            0xd5 as c_int as c_uchar,
            0x92 as c_int as c_uchar,
            0xa1 as c_int as c_uchar,
        ],
        [
            0xb2 as c_int as c_uchar,
            0x74 as c_int as c_uchar,
            0xcb as c_int as c_uchar,
            0x8e as c_int as c_uchar,
            0xbf as c_int as c_uchar,
            0x87 as c_int as c_uchar,
            0x87 as c_int as c_uchar,
            0xa as c_int as c_uchar,
        ],
        [
            0x6f as c_int as c_uchar,
            0x9b as c_int as c_uchar,
            0xb4 as c_int as c_uchar,
            0x20 as c_int as c_uchar,
            0x3d as c_int as c_uchar,
            0xe7 as c_int as c_uchar,
            0xb3 as c_int as c_uchar,
            0x81 as c_int as c_uchar,
        ],
        [
            0xea as c_int as c_uchar,
            0xec as c_int as c_uchar,
            0xb2 as c_int as c_uchar,
            0xa3 as c_int as c_uchar,
            0xb as c_int as c_uchar,
            0x22 as c_int as c_uchar,
            0xa8 as c_int as c_uchar,
            0x7f as c_int as c_uchar,
        ],
        [
            0x99 as c_int as c_uchar,
            0x24 as c_int as c_uchar,
            0xa4 as c_int as c_uchar,
            0x3c as c_int as c_uchar,
            0xc1 as c_int as c_uchar,
            0x31 as c_int as c_uchar,
            0x57 as c_int as c_uchar,
            0x24 as c_int as c_uchar,
        ],
        [
            0xbd as c_int as c_uchar,
            0x83 as c_int as c_uchar,
            0x8d as c_int as c_uchar,
            0x3a as c_int as c_uchar,
            0xaf as c_int as c_uchar,
            0xbf as c_int as c_uchar,
            0x8d as c_int as c_uchar,
            0xb7 as c_int as c_uchar,
        ],
        [
            0xb as c_int as c_uchar,
            0x1a as c_int as c_uchar,
            0x2a as c_int as c_uchar,
            0x32 as c_int as c_uchar,
            0x65 as c_int as c_uchar,
            0xd5 as c_int as c_uchar,
            0x1a as c_int as c_uchar,
            0xea as c_int as c_uchar,
        ],
        [
            0x13 as c_int as c_uchar,
            0x50 as c_int as c_uchar,
            0x79 as c_int as c_uchar,
            0xa3 as c_int as c_uchar,
            0x23 as c_int as c_uchar,
            0x1c as c_int as c_uchar,
            0xe6 as c_int as c_uchar,
            0x60 as c_int as c_uchar,
        ],
        [
            0x93 as c_int as c_uchar,
            0x2b as c_int as c_uchar,
            0x28 as c_int as c_uchar,
            0x46 as c_int as c_uchar,
            0xe4 as c_int as c_uchar,
            0xd7 as c_int as c_uchar,
            0x6 as c_int as c_uchar,
            0x66 as c_int as c_uchar,
        ],
        [
            0xe1 as c_int as c_uchar,
            0x91 as c_int as c_uchar,
            0x5f as c_int as c_uchar,
            0x5c as c_int as c_uchar,
            0xb1 as c_int as c_uchar,
            0xec as c_int as c_uchar,
            0xa4 as c_int as c_uchar,
            0x6c as c_int as c_uchar,
        ],
        [
            0xf3 as c_int as c_uchar,
            0x25 as c_int as c_uchar,
            0x96 as c_int as c_uchar,
            0x5c as c_int as c_uchar,
            0xa1 as c_int as c_uchar,
            0x6d as c_int as c_uchar,
            0x62 as c_int as c_uchar,
            0x9f as c_int as c_uchar,
        ],
        [
            0x57 as c_int as c_uchar,
            0x5f as c_int as c_uchar,
            0xf2 as c_int as c_uchar,
            0x8e as c_int as c_uchar,
            0x60 as c_int as c_uchar,
            0x38 as c_int as c_uchar,
            0x1b as c_int as c_uchar,
            0xe5 as c_int as c_uchar,
        ],
        [
            0x72 as c_int as c_uchar,
            0x45 as c_int as c_uchar,
            0x6 as c_int as c_uchar,
            0xeb as c_int as c_uchar,
            0x4c as c_int as c_uchar,
            0x32 as c_int as c_uchar,
            0x8a as c_int as c_uchar,
            0x95 as c_int as c_uchar,
        ],
    ];
    /* clang-format on */
    let mut in_0: [c_uchar; 64] = [0; 64];
    let mut k: sipkey = sipkey { k: [0; 2] };
    let mut i: size_t = 0;
    sip_tokey(
        &mut k,
        b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x00" as *const u8
            as *const c_char as *const c_void,
    );
    i = 0 as c_int as size_t;
    while i < ::std::mem::size_of::<[c_uchar; 64]>() as c_ulong {
        in_0[i as usize] = i as c_uchar;
        if siphash24(in_0.as_mut_ptr() as *const c_void, i, &mut k)
            != (vectors[i as usize][0 as c_int as usize] as uint64_t) << 0 as c_int
                | (vectors[i as usize][1 as c_int as usize] as uint64_t) << 8 as c_int
                | (vectors[i as usize][2 as c_int as usize] as uint64_t) << 16 as c_int
                | (vectors[i as usize][3 as c_int as usize] as uint64_t) << 24 as c_int
                | (vectors[i as usize][4 as c_int as usize] as uint64_t) << 32 as c_int
                | (vectors[i as usize][5 as c_int as usize] as uint64_t) << 40 as c_int
                | (vectors[i as usize][6 as c_int as usize] as uint64_t) << 48 as c_int
                | (vectors[i as usize][7 as c_int as usize] as uint64_t) << 56 as c_int
        {
            return 0 as c_int;
        }
        i = i.wrapping_add(1)
    }
    return 1 as c_int;
}

pub unsafe extern "C" fn siphash24(
    mut src: *const c_void,
    mut len: size_t,
    mut key: *const sipkey,
) -> uint64_t {
    let mut state: siphash = {
        let mut init = siphash {
            v0: 0 as c_int as uint64_t,
            v1: 0 as c_int as uint64_t,
            v2: 0 as c_int as uint64_t,
            v3: 0 as c_int as uint64_t,
            buf: [0 as c_int as c_uchar, 0, 0, 0, 0, 0, 0, 0],
            p: 0 as *mut c_uchar,
            c: 0 as c_int as uint64_t,
        };
        init
    };
    return sip24_final(sip24_update(sip24_init(&mut state, key), src, len));
}

pub unsafe extern "C" fn sip24_init(mut H: *mut siphash, mut key: *const sipkey) -> *mut siphash {
    (*H).v0 = ((0x736f6d65 as c_uint as uint64_t) << 32 as c_int | 0x70736575 as c_uint as c_ulong)
        ^ (*key).k[0 as c_int as usize];
    (*H).v1 = ((0x646f7261 as c_uint as uint64_t) << 32 as c_int | 0x6e646f6d as c_uint as c_ulong)
        ^ (*key).k[1 as c_int as usize];
    (*H).v2 = ((0x6c796765 as c_uint as uint64_t) << 32 as c_int | 0x6e657261 as c_uint as c_ulong)
        ^ (*key).k[0 as c_int as usize];
    (*H).v3 = ((0x74656462 as c_uint as uint64_t) << 32 as c_int | 0x79746573 as c_uint as c_ulong)
        ^ (*key).k[1 as c_int as usize];
    (*H).p = (*H).buf.as_mut_ptr();
    (*H).c = 0 as c_int as uint64_t;
    return H;
}

/* SIPHASH_H */
/* SIPHASH_MAIN */
/* sip24_valid() */
