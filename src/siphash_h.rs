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
        m = ((*H).buf[0usize] as uint64_t) << 0i32
            | ((*H).buf[1usize] as uint64_t) << 8i32
            | ((*H).buf[2usize] as uint64_t) << 16i32
            | ((*H).buf[3usize] as uint64_t) << 24i32
            | ((*H).buf[4usize] as uint64_t) << 32i32
            | ((*H).buf[5usize] as uint64_t) << 40i32
            | ((*H).buf[6usize] as uint64_t) << 48i32
            | ((*H).buf[7usize] as uint64_t) << 56i32;
        (*H).v3 ^= m;
        sip_round(H, 2i32);
        (*H).v0 ^= m;
        (*H).p = (*H).buf.as_mut_ptr();
        (*H).c =  ((*H).c).wrapping_add(8u64);
        if !(p < pe) {
            break;
        }
    }
    return H;
}

pub unsafe extern "C" fn sip_round(mut H: *mut siphash, rounds: c_int) {
    let mut i: c_int = 0;
    i = 0i32;
    while i < rounds {
        (*H).v0 =  ((*H).v0).wrapping_add((*H).v1);
        (*H).v1 = (*H).v1 << 13i32 | (*H).v1 >> 64i32 - 13i32;
        (*H).v1 ^= (*H).v0;
        (*H).v0 = (*H).v0 << 32i32 | (*H).v0 >> 64i32 - 32i32;
        (*H).v2 =  ((*H).v2).wrapping_add((*H).v3);
        (*H).v3 = (*H).v3 << 16i32 | (*H).v3 >> 64i32 - 16i32;
        (*H).v3 ^= (*H).v2;
        (*H).v0 =  ((*H).v0).wrapping_add((*H).v3);
        (*H).v3 = (*H).v3 << 21i32 | (*H).v3 >> 64i32 - 21i32;
        (*H).v3 ^= (*H).v0;
        (*H).v2 =  ((*H).v2).wrapping_add((*H).v1);
        (*H).v1 = (*H).v1 << 17i32 | (*H).v1 >> 64i32 - 17i32;
        (*H).v1 ^= (*H).v2;
        (*H).v2 = (*H).v2 << 32i32 | (*H).v2 >> 64i32 - 32i32;
        i += 1
    }
}
/* sip24_update() */

pub unsafe extern "C" fn sip24_final(mut H: *mut siphash) -> uint64_t {
    let left: c_char =  (*H).p.wrapping_offset_from((*H).buf.as_mut_ptr()) as c_char;
    let mut b: uint64_t = (*H).c.wrapping_add(left as c_ulong) << 56i32;
    let mut current_block_6: u64;
    match left as c_int {
        7 => {
            b |= ((*H).buf[6usize] as uint64_t) << 48i32;
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
            b |= ((*H).buf[5usize] as uint64_t) << 40i32;
            current_block_6 = 17438419808098997991;
        }
        _ => {}
    }
    match current_block_6 {
        17438419808098997991 =>
        /* fall through */
        {
            b |= ((*H).buf[4usize] as uint64_t) << 32i32;
            current_block_6 = 3917958908645891603;
        }
        _ => {}
    }
    match current_block_6 {
        3917958908645891603 =>
        /* fall through */
        {
            b |= ((*H).buf[3usize] as uint64_t) << 24i32;
            current_block_6 = 14025370785517308083;
        }
        _ => {}
    }
    match current_block_6 {
        14025370785517308083 =>
        /* fall through */
        {
            b |= ((*H).buf[2usize] as uint64_t) << 16i32;
            current_block_6 = 6645080409601096633;
        }
        _ => {}
    }
    match current_block_6 {
        6645080409601096633 =>
        /* fall through */
        {
            b |= ((*H).buf[1usize] as uint64_t) << 8i32;
            current_block_6 = 15385381413911169552;
        }
        _ => {}
    }
    match current_block_6 {
        15385381413911169552 =>
        /* fall through */
        {
            b |= ((*H).buf[0usize] as uint64_t) << 0i32
        }
        _ => {}
    }
    (*H).v3 ^= b;
    sip_round(H, 2i32);
    (*H).v0 ^= b;
    (*H).v2 ^= 0xffu64;
    sip_round(H, 4i32);
    return (*H).v0 ^ (*H).v1 ^ (*H).v2 ^ (*H).v3;
}

pub unsafe extern "C" fn sip_tokey(mut key: *mut sipkey, mut src: *const c_void) -> *mut sipkey {
    (*key).k[0usize] =
        (*(src as *const c_uchar).offset(0isize) as uint64_t) << 0i32
            | (*(src as *const c_uchar).offset(1isize) as uint64_t) << 8i32
            | (*(src as *const c_uchar).offset(2isize) as uint64_t) << 16i32
            | (*(src as *const c_uchar).offset(3isize) as uint64_t) << 24i32
            | (*(src as *const c_uchar).offset(4isize) as uint64_t) << 32i32
            | (*(src as *const c_uchar).offset(5isize) as uint64_t) << 40i32
            | (*(src as *const c_uchar).offset(6isize) as uint64_t) << 48i32
            | (*(src as *const c_uchar).offset(7isize) as uint64_t) << 56i32;
    (*key).k[1usize] = (*(src as *const c_uchar)
        .offset(8isize)
        .offset(0isize) as uint64_t)
        << 0i32
        | (*(src as *const c_uchar)
            .offset(8isize)
            .offset(1isize) as uint64_t)
            << 8i32
        | (*(src as *const c_uchar)
            .offset(8isize)
            .offset(2isize) as uint64_t)
            << 16i32
        | (*(src as *const c_uchar)
            .offset(8isize)
            .offset(3isize) as uint64_t)
            << 24i32
        | (*(src as *const c_uchar)
            .offset(8isize)
            .offset(4isize) as uint64_t)
            << 32i32
        | (*(src as *const c_uchar)
            .offset(8isize)
            .offset(5isize) as uint64_t)
            << 40i32
        | (*(src as *const c_uchar)
            .offset(8isize)
            .offset(6isize) as uint64_t)
            << 48i32
        | (*(src as *const c_uchar)
            .offset(8isize)
            .offset(7isize) as uint64_t)
            << 56i32;
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
            0x31u8,
            0xeu8,
            0xeu8,
            0xddu8,
            0x47u8,
            0xdbu8,
            0x6fu8,
            0x72u8,
        ],
        [
            0xfdu8,
            0x67u8,
            0xdcu8,
            0x93u8,
            0xc5u8,
            0x39u8,
            0xf8u8,
            0x74u8,
        ],
        [
            0x5au8,
            0x4fu8,
            0xa9u8,
            0xd9u8,
            0x9u8,
            0x80u8,
            0x6cu8,
            0xdu8,
        ],
        [
            0x2du8,
            0x7eu8,
            0xfbu8,
            0xd7u8,
            0x96u8,
            0x66u8,
            0x67u8,
            0x85u8,
        ],
        [
            0xb7u8,
            0x87u8,
            0x71u8,
            0x27u8,
            0xe0u8,
            0x94u8,
            0x27u8,
            0xcfu8,
        ],
        [
            0x8du8,
            0xa6u8,
            0x99u8,
            0xcdu8,
            0x64u8,
            0x55u8,
            0x76u8,
            0x18u8,
        ],
        [
            0xceu8,
            0xe3u8,
            0xfeu8,
            0x58u8,
            0x6eu8,
            0x46u8,
            0xc9u8,
            0xcbu8,
        ],
        [
            0x37u8,
            0xd1u8,
            0x1u8,
            0x8bu8,
            0xf5u8,
            0u8,
            0x2u8,
            0xabu8,
        ],
        [
            0x62u8,
            0x24u8,
            0x93u8,
            0x9au8,
            0x79u8,
            0xf5u8,
            0xf5u8,
            0x93u8,
        ],
        [
            0xb0u8,
            0xe4u8,
            0xa9u8,
            0xbu8,
            0xdfu8,
            0x82u8,
            0u8,
            0x9eu8,
        ],
        [
            0xf3u8,
            0xb9u8,
            0xddu8,
            0x94u8,
            0xc5u8,
            0xbbu8,
            0x5du8,
            0x7au8,
        ],
        [
            0xa7u8,
            0xadu8,
            0x6bu8,
            0x22u8,
            0x46u8,
            0x2fu8,
            0xb3u8,
            0xf4u8,
        ],
        [
            0xfbu8,
            0xe5u8,
            0xeu8,
            0x86u8,
            0xbcu8,
            0x8fu8,
            0x1eu8,
            0x75u8,
        ],
        [
            0x90u8,
            0x3du8,
            0x84u8,
            0xc0u8,
            0x27u8,
            0x56u8,
            0xeau8,
            0x14u8,
        ],
        [
            0xeeu8,
            0xf2u8,
            0x7au8,
            0x8eu8,
            0x90u8,
            0xcau8,
            0x23u8,
            0xf7u8,
        ],
        [
            0xe5u8,
            0x45u8,
            0xbeu8,
            0x49u8,
            0x61u8,
            0xcau8,
            0x29u8,
            0xa1u8,
        ],
        [
            0xdbu8,
            0x9bu8,
            0xc2u8,
            0x57u8,
            0x7fu8,
            0xccu8,
            0x2au8,
            0x3fu8,
        ],
        [
            0x94u8,
            0x47u8,
            0xbeu8,
            0x2cu8,
            0xf5u8,
            0xe9u8,
            0x9au8,
            0x69u8,
        ],
        [
            0x9cu8,
            0xd3u8,
            0x8du8,
            0x96u8,
            0xf0u8,
            0xb3u8,
            0xc1u8,
            0x4bu8,
        ],
        [
            0xbdu8,
            0x61u8,
            0x79u8,
            0xa7u8,
            0x1du8,
            0xc9u8,
            0x6du8,
            0xbbu8,
        ],
        [
            0x98u8,
            0xeeu8,
            0xa2u8,
            0x1au8,
            0xf2u8,
            0x5cu8,
            0xd6u8,
            0xbeu8,
        ],
        [
            0xc7u8,
            0x67u8,
            0x3bu8,
            0x2eu8,
            0xb0u8,
            0xcbu8,
            0xf2u8,
            0xd0u8,
        ],
        [
            0x88u8,
            0x3eu8,
            0xa3u8,
            0xe3u8,
            0x95u8,
            0x67u8,
            0x53u8,
            0x93u8,
        ],
        [
            0xc8u8,
            0xceu8,
            0x5cu8,
            0xcdu8,
            0x8cu8,
            0x3u8,
            0xcu8,
            0xa8u8,
        ],
        [
            0x94u8,
            0xafu8,
            0x49u8,
            0xf6u8,
            0xc6u8,
            0x50u8,
            0xadu8,
            0xb8u8,
        ],
        [
            0xeau8,
            0xb8u8,
            0x85u8,
            0x8au8,
            0xdeu8,
            0x92u8,
            0xe1u8,
            0xbcu8,
        ],
        [
            0xf3u8,
            0x15u8,
            0xbbu8,
            0x5bu8,
            0xb8u8,
            0x35u8,
            0xd8u8,
            0x17u8,
        ],
        [
            0xadu8,
            0xcfu8,
            0x6bu8,
            0x7u8,
            0x63u8,
            0x61u8,
            0x2eu8,
            0x2fu8,
        ],
        [
            0xa5u8,
            0xc9u8,
            0x1du8,
            0xa7u8,
            0xacu8,
            0xaau8,
            0x4du8,
            0xdeu8,
        ],
        [
            0x71u8,
            0x65u8,
            0x95u8,
            0x87u8,
            0x66u8,
            0x50u8,
            0xa2u8,
            0xa6u8,
        ],
        [
            0x28u8,
            0xefu8,
            0x49u8,
            0x5cu8,
            0x53u8,
            0xa3u8,
            0x87u8,
            0xadu8,
        ],
        [
            0x42u8,
            0xc3u8,
            0x41u8,
            0xd8u8,
            0xfau8,
            0x92u8,
            0xd8u8,
            0x32u8,
        ],
        [
            0xceu8,
            0x7cu8,
            0xf2u8,
            0x72u8,
            0x2fu8,
            0x51u8,
            0x27u8,
            0x71u8,
        ],
        [
            0xe3u8,
            0x78u8,
            0x59u8,
            0xf9u8,
            0x46u8,
            0x23u8,
            0xf3u8,
            0xa7u8,
        ],
        [
            0x38u8,
            0x12u8,
            0x5u8,
            0xbbu8,
            0x1au8,
            0xb0u8,
            0xe0u8,
            0x12u8,
        ],
        [
            0xaeu8,
            0x97u8,
            0xa1u8,
            0xfu8,
            0xd4u8,
            0x34u8,
            0xe0u8,
            0x15u8,
        ],
        [
            0xb4u8,
            0xa3u8,
            0x15u8,
            0x8u8,
            0xbeu8,
            0xffu8,
            0x4du8,
            0x31u8,
        ],
        [
            0x81u8,
            0x39u8,
            0x62u8,
            0x29u8,
            0xf0u8,
            0x90u8,
            0x79u8,
            0x2u8,
        ],
        [
            0x4du8,
            0xcu8,
            0xf4u8,
            0x9eu8,
            0xe5u8,
            0xd4u8,
            0xdcu8,
            0xcau8,
        ],
        [
            0x5cu8,
            0x73u8,
            0x33u8,
            0x6au8,
            0x76u8,
            0xd8u8,
            0xbfu8,
            0x9au8,
        ],
        [
            0xd0u8,
            0xa7u8,
            0x4u8,
            0x53u8,
            0x6bu8,
            0xa9u8,
            0x3eu8,
            0xeu8,
        ],
        [
            0x92u8,
            0x59u8,
            0x58u8,
            0xfcu8,
            0xd6u8,
            0x42u8,
            0xcu8,
            0xadu8,
        ],
        [
            0xa9u8,
            0x15u8,
            0xc2u8,
            0x9bu8,
            0xc8u8,
            0x6u8,
            0x73u8,
            0x18u8,
        ],
        [
            0x95u8,
            0x2bu8,
            0x79u8,
            0xf3u8,
            0xbcu8,
            0xau8,
            0xa6u8,
            0xd4u8,
        ],
        [
            0xf2u8,
            0x1du8,
            0xf2u8,
            0xe4u8,
            0x1du8,
            0x45u8,
            0x35u8,
            0xf9u8,
        ],
        [
            0x87u8,
            0x57u8,
            0x75u8,
            0x19u8,
            0x4u8,
            0x8fu8,
            0x53u8,
            0xa9u8,
        ],
        [
            0x10u8,
            0xa5u8,
            0x6cu8,
            0xf5u8,
            0xdfu8,
            0xcdu8,
            0x9au8,
            0xdbu8,
        ],
        [
            0xebu8,
            0x75u8,
            0x9u8,
            0x5cu8,
            0xcdu8,
            0x98u8,
            0x6cu8,
            0xd0u8,
        ],
        [
            0x51u8,
            0xa9u8,
            0xcbu8,
            0x9eu8,
            0xcbu8,
            0xa3u8,
            0x12u8,
            0xe6u8,
        ],
        [
            0x96u8,
            0xafu8,
            0xadu8,
            0xfcu8,
            0x2cu8,
            0xe6u8,
            0x66u8,
            0xc7u8,
        ],
        [
            0x72u8,
            0xfeu8,
            0x52u8,
            0x97u8,
            0x5au8,
            0x43u8,
            0x64u8,
            0xeeu8,
        ],
        [
            0x5au8,
            0x16u8,
            0x45u8,
            0xb2u8,
            0x76u8,
            0xd5u8,
            0x92u8,
            0xa1u8,
        ],
        [
            0xb2u8,
            0x74u8,
            0xcbu8,
            0x8eu8,
            0xbfu8,
            0x87u8,
            0x87u8,
            0xau8,
        ],
        [
            0x6fu8,
            0x9bu8,
            0xb4u8,
            0x20u8,
            0x3du8,
            0xe7u8,
            0xb3u8,
            0x81u8,
        ],
        [
            0xeau8,
            0xecu8,
            0xb2u8,
            0xa3u8,
            0xbu8,
            0x22u8,
            0xa8u8,
            0x7fu8,
        ],
        [
            0x99u8,
            0x24u8,
            0xa4u8,
            0x3cu8,
            0xc1u8,
            0x31u8,
            0x57u8,
            0x24u8,
        ],
        [
            0xbdu8,
            0x83u8,
            0x8du8,
            0x3au8,
            0xafu8,
            0xbfu8,
            0x8du8,
            0xb7u8,
        ],
        [
            0xbu8,
            0x1au8,
            0x2au8,
            0x32u8,
            0x65u8,
            0xd5u8,
            0x1au8,
            0xeau8,
        ],
        [
            0x13u8,
            0x50u8,
            0x79u8,
            0xa3u8,
            0x23u8,
            0x1cu8,
            0xe6u8,
            0x60u8,
        ],
        [
            0x93u8,
            0x2bu8,
            0x28u8,
            0x46u8,
            0xe4u8,
            0xd7u8,
            0x6u8,
            0x66u8,
        ],
        [
            0xe1u8,
            0x91u8,
            0x5fu8,
            0x5cu8,
            0xb1u8,
            0xecu8,
            0xa4u8,
            0x6cu8,
        ],
        [
            0xf3u8,
            0x25u8,
            0x96u8,
            0x5cu8,
            0xa1u8,
            0x6du8,
            0x62u8,
            0x9fu8,
        ],
        [
            0x57u8,
            0x5fu8,
            0xf2u8,
            0x8eu8,
            0x60u8,
            0x38u8,
            0x1bu8,
            0xe5u8,
        ],
        [
            0x72u8,
            0x45u8,
            0x6u8,
            0xebu8,
            0x4cu8,
            0x32u8,
            0x8au8,
            0x95u8,
        ],
    ];
    /* clang-format on */
    let mut in_0: [c_uchar; 64] = [0; 64];
    let mut k: sipkey = sipkey { k: [0; 2] };
    let mut i: size_t = 0;
    sip_tokey(
        &mut k,
        
        b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x00"
            as *const  u8 as *const c_void,
    );
    i = 0u64;
    while i < ::std::mem::size_of::<[c_uchar; 64]>() as c_ulong {
        in_0[i as usize] = i as c_uchar;
        if siphash24(in_0.as_mut_ptr() as *const c_void, i, &mut k)
            != (vectors[i as usize][0usize] as uint64_t) << 0i32
                | (vectors[i as usize][1usize] as uint64_t) << 8i32
                | (vectors[i as usize][2usize] as uint64_t) << 16i32
                | (vectors[i as usize][3usize] as uint64_t) << 24i32
                | (vectors[i as usize][4usize] as uint64_t) << 32i32
                | (vectors[i as usize][5usize] as uint64_t) << 40i32
                | (vectors[i as usize][6usize] as uint64_t) << 48i32
                | (vectors[i as usize][7usize] as uint64_t) << 56i32
        {
            return 0i32;
        }
        i = i.wrapping_add(1)
    }
    return 1i32;
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
    (*H).v0 = ((0x736f6d65u64) << 32i32 | 0x70736575u64)
        ^ (*key).k[0usize];
    (*H).v1 = ((0x646f7261u64) << 32i32 | 0x6e646f6du64)
        ^ (*key).k[1usize];
    (*H).v2 = ((0x6c796765u64) << 32i32 | 0x6e657261u64)
        ^ (*key).k[0usize];
    (*H).v3 = ((0x74656462u64) << 32i32 | 0x79746573u64)
        ^ (*key).k[1usize];
    (*H).p = (*H).buf.as_mut_ptr();
    (*H).c = 0u64;
    return H;
}

/* SIPHASH_H */
/* SIPHASH_MAIN */
/* sip24_valid() */
