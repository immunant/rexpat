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
        m = ((*H).buf[0] as uint64_t) << 0
            | ((*H).buf[1] as uint64_t) << 8
            | ((*H).buf[2] as uint64_t) << 16
            | ((*H).buf[3] as uint64_t) << 24
            | ((*H).buf[4] as uint64_t) << 32
            | ((*H).buf[5] as uint64_t) << 40
            | ((*H).buf[6] as uint64_t) << 48
            | ((*H).buf[7] as uint64_t) << 56;
        (*H).v3 ^= m;
        sip_round(H, 2);
        (*H).v0 ^= m;
        (*H).p = (*H).buf.as_mut_ptr();
        (*H).c = ((*H).c).wrapping_add(8u64);
        if !(p < pe) {
            break;
        }
    }
    return H;
}

pub unsafe extern "C" fn sip_round(mut H: *mut siphash, rounds: c_int) {
    let mut i: c_int = 0;
    i = 0;
    while i < rounds {
        (*H).v0 = ((*H).v0).wrapping_add((*H).v1);
        (*H).v1 = (*H).v1 << 13 | (*H).v1 >> 64 - 13;
        (*H).v1 ^= (*H).v0;
        (*H).v0 = (*H).v0 << 32 | (*H).v0 >> 64 - 32;
        (*H).v2 = ((*H).v2).wrapping_add((*H).v3);
        (*H).v3 = (*H).v3 << 16 | (*H).v3 >> 64 - 16;
        (*H).v3 ^= (*H).v2;
        (*H).v0 = ((*H).v0).wrapping_add((*H).v3);
        (*H).v3 = (*H).v3 << 21 | (*H).v3 >> 64 - 21;
        (*H).v3 ^= (*H).v0;
        (*H).v2 = ((*H).v2).wrapping_add((*H).v1);
        (*H).v1 = (*H).v1 << 17 | (*H).v1 >> 64 - 17;
        (*H).v1 ^= (*H).v2;
        (*H).v2 = (*H).v2 << 32 | (*H).v2 >> 64 - 32;
        i += 1
    }
}
/* sip24_update() */

pub unsafe extern "C" fn sip24_final(mut H: *mut siphash) -> uint64_t {
    let left: c_char = (*H).p.wrapping_offset_from((*H).buf.as_mut_ptr()) as c_char;
    let mut b: uint64_t = (*H).c.wrapping_add(left as c_ulong) << 56;
    let mut current_block_6: u64;
    match left as c_int {
        7 => {
            b |= ((*H).buf[6] as uint64_t) << 48;
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
            b |= ((*H).buf[5] as uint64_t) << 40;
            current_block_6 = 17438419808098997991;
        }
        _ => {}
    }
    match current_block_6 {
        17438419808098997991 =>
        /* fall through */
        {
            b |= ((*H).buf[4] as uint64_t) << 32;
            current_block_6 = 3917958908645891603;
        }
        _ => {}
    }
    match current_block_6 {
        3917958908645891603 =>
        /* fall through */
        {
            b |= ((*H).buf[3] as uint64_t) << 24;
            current_block_6 = 14025370785517308083;
        }
        _ => {}
    }
    match current_block_6 {
        14025370785517308083 =>
        /* fall through */
        {
            b |= ((*H).buf[2] as uint64_t) << 16;
            current_block_6 = 6645080409601096633;
        }
        _ => {}
    }
    match current_block_6 {
        6645080409601096633 =>
        /* fall through */
        {
            b |= ((*H).buf[1] as uint64_t) << 8;
            current_block_6 = 15385381413911169552;
        }
        _ => {}
    }
    match current_block_6 {
        15385381413911169552 =>
        /* fall through */
        {
            b |= ((*H).buf[0] as uint64_t) << 0
        }
        _ => {}
    }
    (*H).v3 ^= b;
    sip_round(H, 2);
    (*H).v0 ^= b;
    (*H).v2 ^= 0xffu64;
    sip_round(H, 4);
    return (*H).v0 ^ (*H).v1 ^ (*H).v2 ^ (*H).v3;
}

pub unsafe extern "C" fn sip_tokey(mut key: *mut sipkey, mut src: *const c_void) -> *mut sipkey {
    (*key).k[0] = (*(src as *const c_uchar).offset(0) as uint64_t) << 0
        | (*(src as *const c_uchar).offset(1) as uint64_t) << 8
        | (*(src as *const c_uchar).offset(2) as uint64_t) << 16
        | (*(src as *const c_uchar).offset(3) as uint64_t) << 24
        | (*(src as *const c_uchar).offset(4) as uint64_t) << 32
        | (*(src as *const c_uchar).offset(5) as uint64_t) << 40
        | (*(src as *const c_uchar).offset(6) as uint64_t) << 48
        | (*(src as *const c_uchar).offset(7) as uint64_t) << 56;
    (*key).k[1] = (*(src as *const c_uchar).offset(8).offset(0) as uint64_t) << 0
        | (*(src as *const c_uchar).offset(8).offset(1) as uint64_t) << 8
        | (*(src as *const c_uchar).offset(8).offset(2) as uint64_t) << 16
        | (*(src as *const c_uchar).offset(8).offset(3) as uint64_t) << 24
        | (*(src as *const c_uchar).offset(8).offset(4) as uint64_t) << 32
        | (*(src as *const c_uchar).offset(8).offset(5) as uint64_t) << 40
        | (*(src as *const c_uchar).offset(8).offset(6) as uint64_t) << 48
        | (*(src as *const c_uchar).offset(8).offset(7) as uint64_t) << 56;
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
        [0x31, 0xe, 0xe, 0xdd, 0x47, 0xdb, 0x6f, 0x72],
        [0xfd, 0x67, 0xdc, 0x93, 0xc5, 0x39, 0xf8, 0x74],
        [0x5a, 0x4f, 0xa9, 0xd9, 0x9, 0x80, 0x6c, 0xd],
        [0x2d, 0x7e, 0xfb, 0xd7, 0x96, 0x66, 0x67, 0x85],
        [0xb7, 0x87, 0x71, 0x27, 0xe0, 0x94, 0x27, 0xcf],
        [0x8d, 0xa6, 0x99, 0xcd, 0x64, 0x55, 0x76, 0x18],
        [0xce, 0xe3, 0xfe, 0x58, 0x6e, 0x46, 0xc9, 0xcb],
        [0x37, 0xd1, 0x1, 0x8b, 0xf5, 0, 0x2, 0xab],
        [0x62, 0x24, 0x93, 0x9a, 0x79, 0xf5, 0xf5, 0x93],
        [0xb0, 0xe4, 0xa9, 0xb, 0xdf, 0x82, 0, 0x9e],
        [0xf3, 0xb9, 0xdd, 0x94, 0xc5, 0xbb, 0x5d, 0x7a],
        [0xa7, 0xad, 0x6b, 0x22, 0x46, 0x2f, 0xb3, 0xf4],
        [0xfb, 0xe5, 0xe, 0x86, 0xbc, 0x8f, 0x1e, 0x75],
        [0x90, 0x3d, 0x84, 0xc0, 0x27, 0x56, 0xea, 0x14],
        [0xee, 0xf2, 0x7a, 0x8e, 0x90, 0xca, 0x23, 0xf7],
        [0xe5, 0x45, 0xbe, 0x49, 0x61, 0xca, 0x29, 0xa1],
        [0xdb, 0x9b, 0xc2, 0x57, 0x7f, 0xcc, 0x2a, 0x3f],
        [0x94, 0x47, 0xbe, 0x2c, 0xf5, 0xe9, 0x9a, 0x69],
        [0x9c, 0xd3, 0x8d, 0x96, 0xf0, 0xb3, 0xc1, 0x4b],
        [0xbd, 0x61, 0x79, 0xa7, 0x1d, 0xc9, 0x6d, 0xbb],
        [0x98, 0xee, 0xa2, 0x1a, 0xf2, 0x5c, 0xd6, 0xbe],
        [0xc7, 0x67, 0x3b, 0x2e, 0xb0, 0xcb, 0xf2, 0xd0],
        [0x88, 0x3e, 0xa3, 0xe3, 0x95, 0x67, 0x53, 0x93],
        [0xc8, 0xce, 0x5c, 0xcd, 0x8c, 0x3, 0xc, 0xa8],
        [0x94, 0xaf, 0x49, 0xf6, 0xc6, 0x50, 0xad, 0xb8],
        [0xea, 0xb8, 0x85, 0x8a, 0xde, 0x92, 0xe1, 0xbc],
        [0xf3, 0x15, 0xbb, 0x5b, 0xb8, 0x35, 0xd8, 0x17],
        [0xad, 0xcf, 0x6b, 0x7, 0x63, 0x61, 0x2e, 0x2f],
        [0xa5, 0xc9, 0x1d, 0xa7, 0xac, 0xaa, 0x4d, 0xde],
        [0x71, 0x65, 0x95, 0x87, 0x66, 0x50, 0xa2, 0xa6],
        [0x28, 0xef, 0x49, 0x5c, 0x53, 0xa3, 0x87, 0xad],
        [0x42, 0xc3, 0x41, 0xd8, 0xfa, 0x92, 0xd8, 0x32],
        [0xce, 0x7c, 0xf2, 0x72, 0x2f, 0x51, 0x27, 0x71],
        [0xe3, 0x78, 0x59, 0xf9, 0x46, 0x23, 0xf3, 0xa7],
        [0x38, 0x12, 0x5, 0xbb, 0x1a, 0xb0, 0xe0, 0x12],
        [0xae, 0x97, 0xa1, 0xf, 0xd4, 0x34, 0xe0, 0x15],
        [0xb4, 0xa3, 0x15, 0x8, 0xbe, 0xff, 0x4d, 0x31],
        [0x81, 0x39, 0x62, 0x29, 0xf0, 0x90, 0x79, 0x2],
        [0x4d, 0xc, 0xf4, 0x9e, 0xe5, 0xd4, 0xdc, 0xca],
        [0x5c, 0x73, 0x33, 0x6a, 0x76, 0xd8, 0xbf, 0x9a],
        [0xd0, 0xa7, 0x4, 0x53, 0x6b, 0xa9, 0x3e, 0xe],
        [0x92, 0x59, 0x58, 0xfc, 0xd6, 0x42, 0xc, 0xad],
        [0xa9, 0x15, 0xc2, 0x9b, 0xc8, 0x6, 0x73, 0x18],
        [0x95, 0x2b, 0x79, 0xf3, 0xbc, 0xa, 0xa6, 0xd4],
        [0xf2, 0x1d, 0xf2, 0xe4, 0x1d, 0x45, 0x35, 0xf9],
        [0x87, 0x57, 0x75, 0x19, 0x4, 0x8f, 0x53, 0xa9],
        [0x10, 0xa5, 0x6c, 0xf5, 0xdf, 0xcd, 0x9a, 0xdb],
        [0xeb, 0x75, 0x9, 0x5c, 0xcd, 0x98, 0x6c, 0xd0],
        [0x51, 0xa9, 0xcb, 0x9e, 0xcb, 0xa3, 0x12, 0xe6],
        [0x96, 0xaf, 0xad, 0xfc, 0x2c, 0xe6, 0x66, 0xc7],
        [0x72, 0xfe, 0x52, 0x97, 0x5a, 0x43, 0x64, 0xee],
        [0x5a, 0x16, 0x45, 0xb2, 0x76, 0xd5, 0x92, 0xa1],
        [0xb2, 0x74, 0xcb, 0x8e, 0xbf, 0x87, 0x87, 0xa],
        [0x6f, 0x9b, 0xb4, 0x20, 0x3d, 0xe7, 0xb3, 0x81],
        [0xea, 0xec, 0xb2, 0xa3, 0xb, 0x22, 0xa8, 0x7f],
        [0x99, 0x24, 0xa4, 0x3c, 0xc1, 0x31, 0x57, 0x24],
        [0xbd, 0x83, 0x8d, 0x3a, 0xaf, 0xbf, 0x8d, 0xb7],
        [0xb, 0x1a, 0x2a, 0x32, 0x65, 0xd5, 0x1a, 0xea],
        [0x13, 0x50, 0x79, 0xa3, 0x23, 0x1c, 0xe6, 0x60],
        [0x93, 0x2b, 0x28, 0x46, 0xe4, 0xd7, 0x6, 0x66],
        [0xe1, 0x91, 0x5f, 0x5c, 0xb1, 0xec, 0xa4, 0x6c],
        [0xf3, 0x25, 0x96, 0x5c, 0xa1, 0x6d, 0x62, 0x9f],
        [0x57, 0x5f, 0xf2, 0x8e, 0x60, 0x38, 0x1b, 0xe5],
        [0x72, 0x45, 0x6, 0xeb, 0x4c, 0x32, 0x8a, 0x95],
    ];
    /* clang-format on */
    let mut in_0: [c_uchar; 64] = [0; 64];
    let mut k: sipkey = sipkey { k: [0; 2] };
    let mut i: size_t = 0;
    sip_tokey(
        &mut k,
        b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x00".as_ptr() as *const c_void,
    );
    i = 0;
    while i < ::std::mem::size_of::<[c_uchar; 64]>() as c_ulong {
        in_0[i as usize] = i as c_uchar;
        if siphash24(in_0.as_mut_ptr() as *const c_void, i, &mut k)
            != (vectors[i as usize][0] as uint64_t) << 0
                | (vectors[i as usize][1] as uint64_t) << 8
                | (vectors[i as usize][2] as uint64_t) << 16
                | (vectors[i as usize][3] as uint64_t) << 24
                | (vectors[i as usize][4] as uint64_t) << 32
                | (vectors[i as usize][5] as uint64_t) << 40
                | (vectors[i as usize][6] as uint64_t) << 48
                | (vectors[i as usize][7] as uint64_t) << 56
        {
            return 0i32;
        }
        i = i.wrapping_add(1)
    }
    return 1;
}

pub unsafe extern "C" fn siphash24(
    mut src: *const c_void,
    mut len: size_t,
    mut key: *const sipkey,
) -> uint64_t {
    let mut state: siphash = {
        let mut init = siphash {
            v0: 0u64,
            v1: 0u64,
            v2: 0u64,
            v3: 0u64,
            buf: [0u8, 0, 0, 0, 0, 0, 0, 0],
            p: 0 as *mut c_uchar,
            c: 0u64,
        };
        init
    };
    return sip24_final(sip24_update(sip24_init(&mut state, key), src, len));
}

pub unsafe extern "C" fn sip24_init(mut H: *mut siphash, mut key: *const sipkey) -> *mut siphash {
    (*H).v0 = ((0x736f6d65u64) << 32 | 0x70736575) ^ (*key).k[0];
    (*H).v1 = ((0x646f7261u64) << 32 | 0x6e646f6d) ^ (*key).k[1];
    (*H).v2 = ((0x6c796765u64) << 32 | 0x6e657261) ^ (*key).k[0];
    (*H).v3 = ((0x74656462u64) << 32 | 0x79746573) ^ (*key).k[1];
    (*H).p = (*H).buf.as_mut_ptr();
    (*H).c = 0u64;
    return H;
}

/* SIPHASH_H */
/* SIPHASH_MAIN */
/* sip24_valid() */
