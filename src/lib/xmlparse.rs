use ::libc;

pub mod siphash_h {

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
        mut H: *mut crate::siphash_h::siphash,
        mut src: *const libc::c_void,
        mut len: crate::stddef_h::size_t,
    ) -> *mut crate::siphash_h::siphash {
        let mut p: *const libc::c_uchar = src as *const libc::c_uchar;
        let mut pe: *const libc::c_uchar = p.offset(len as isize);
        let mut m: crate::stdlib::uint64_t = 0;
        loop {
            while p < pe
                && (*H).p
                    < &mut *(*H).buf.as_mut_ptr().offset(
                        (::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong)
                            .wrapping_div(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_uchar
            {
                let fresh0 = p;
                p = p.offset(1);
                let fresh1 = (*H).p;
                (*H).p = (*H).p.offset(1);
                *fresh1 = *fresh0
            }
            if (*H).p
                < &mut *(*H).buf.as_mut_ptr().offset(
                    (::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_uchar
            {
                break;
            }
            m = ((*H).buf[0 as libc::c_int as usize] as crate::stdlib::uint64_t)
                << 0 as libc::c_int
                | ((*H).buf[1 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 8 as libc::c_int
                | ((*H).buf[2 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 16 as libc::c_int
                | ((*H).buf[3 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 24 as libc::c_int
                | ((*H).buf[4 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 32 as libc::c_int
                | ((*H).buf[5 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 40 as libc::c_int
                | ((*H).buf[6 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 48 as libc::c_int
                | ((*H).buf[7 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 56 as libc::c_int;
            (*H).v3 ^= m;
            sip_round(H, 2 as libc::c_int);
            (*H).v0 ^= m;
            (*H).p = (*H).buf.as_mut_ptr();
            (*H).c = ((*H).c as libc::c_ulong).wrapping_add(8 as libc::c_int as libc::c_ulong)
                as crate::stdlib::uint64_t as crate::stdlib::uint64_t;
            if !(p < pe) {
                break;
            }
        }
        return H;
    }

    pub unsafe extern "C" fn sip_round(mut H: *mut crate::siphash_h::siphash, rounds: libc::c_int) {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < rounds {
            (*H).v0 = ((*H).v0 as libc::c_ulong).wrapping_add((*H).v1) as crate::stdlib::uint64_t
                as crate::stdlib::uint64_t;
            (*H).v1 =
                (*H).v1 << 13 as libc::c_int | (*H).v1 >> 64 as libc::c_int - 13 as libc::c_int;
            (*H).v1 ^= (*H).v0;
            (*H).v0 =
                (*H).v0 << 32 as libc::c_int | (*H).v0 >> 64 as libc::c_int - 32 as libc::c_int;
            (*H).v2 = ((*H).v2 as libc::c_ulong).wrapping_add((*H).v3) as crate::stdlib::uint64_t
                as crate::stdlib::uint64_t;
            (*H).v3 =
                (*H).v3 << 16 as libc::c_int | (*H).v3 >> 64 as libc::c_int - 16 as libc::c_int;
            (*H).v3 ^= (*H).v2;
            (*H).v0 = ((*H).v0 as libc::c_ulong).wrapping_add((*H).v3) as crate::stdlib::uint64_t
                as crate::stdlib::uint64_t;
            (*H).v3 =
                (*H).v3 << 21 as libc::c_int | (*H).v3 >> 64 as libc::c_int - 21 as libc::c_int;
            (*H).v3 ^= (*H).v0;
            (*H).v2 = ((*H).v2 as libc::c_ulong).wrapping_add((*H).v1) as crate::stdlib::uint64_t
                as crate::stdlib::uint64_t;
            (*H).v1 =
                (*H).v1 << 17 as libc::c_int | (*H).v1 >> 64 as libc::c_int - 17 as libc::c_int;
            (*H).v1 ^= (*H).v2;
            (*H).v2 =
                (*H).v2 << 32 as libc::c_int | (*H).v2 >> 64 as libc::c_int - 32 as libc::c_int;
            i += 1
        }
    }
    /* sip24_update() */

    pub unsafe extern "C" fn sip24_final(
        mut H: *mut crate::siphash_h::siphash,
    ) -> crate::stdlib::uint64_t {
        let left: libc::c_char =
            (*H).p.wrapping_offset_from((*H).buf.as_mut_ptr()) as libc::c_long as libc::c_char;
        let mut b: crate::stdlib::uint64_t =
            (*H).c.wrapping_add(left as libc::c_ulong) << 56 as libc::c_int;
        let mut current_block_6: u64;
        match left as libc::c_int {
            7 => {
                b |= ((*H).buf[6 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 48 as libc::c_int;
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
                b |= ((*H).buf[5 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 40 as libc::c_int;
                current_block_6 = 17438419808098997991;
            }
            _ => {}
        }
        match current_block_6 {
            17438419808098997991 =>
            /* fall through */
            {
                b |= ((*H).buf[4 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 32 as libc::c_int;
                current_block_6 = 3917958908645891603;
            }
            _ => {}
        }
        match current_block_6 {
            3917958908645891603 =>
            /* fall through */
            {
                b |= ((*H).buf[3 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 24 as libc::c_int;
                current_block_6 = 14025370785517308083;
            }
            _ => {}
        }
        match current_block_6 {
            14025370785517308083 =>
            /* fall through */
            {
                b |= ((*H).buf[2 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 16 as libc::c_int;
                current_block_6 = 6645080409601096633;
            }
            _ => {}
        }
        match current_block_6 {
            6645080409601096633 =>
            /* fall through */
            {
                b |= ((*H).buf[1 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 8 as libc::c_int;
                current_block_6 = 15385381413911169552;
            }
            _ => {}
        }
        match current_block_6 {
            15385381413911169552 =>
            /* fall through */
            {
                b |= ((*H).buf[0 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 0 as libc::c_int
            }
            _ => {}
        }
        (*H).v3 ^= b;
        sip_round(H, 2 as libc::c_int);
        (*H).v0 ^= b;
        (*H).v2 ^= 0xff as libc::c_int as libc::c_ulong;
        sip_round(H, 4 as libc::c_int);
        return (*H).v0 ^ (*H).v1 ^ (*H).v2 ^ (*H).v3;
    }

    pub unsafe extern "C" fn sip_tokey(
        mut key: *mut crate::siphash_h::sipkey,
        mut src: *const libc::c_void,
    ) -> *mut crate::siphash_h::sipkey {
        (*key).k[0 as libc::c_int as usize] = (*(src as *const libc::c_uchar)
            .offset(0 as libc::c_int as isize)
            as crate::stdlib::uint64_t)
            << 0 as libc::c_int
            | (*(src as *const libc::c_uchar).offset(1 as libc::c_int as isize)
                as crate::stdlib::uint64_t)
                << 8 as libc::c_int
            | (*(src as *const libc::c_uchar).offset(2 as libc::c_int as isize)
                as crate::stdlib::uint64_t)
                << 16 as libc::c_int
            | (*(src as *const libc::c_uchar).offset(3 as libc::c_int as isize)
                as crate::stdlib::uint64_t)
                << 24 as libc::c_int
            | (*(src as *const libc::c_uchar).offset(4 as libc::c_int as isize)
                as crate::stdlib::uint64_t)
                << 32 as libc::c_int
            | (*(src as *const libc::c_uchar).offset(5 as libc::c_int as isize)
                as crate::stdlib::uint64_t)
                << 40 as libc::c_int
            | (*(src as *const libc::c_uchar).offset(6 as libc::c_int as isize)
                as crate::stdlib::uint64_t)
                << 48 as libc::c_int
            | (*(src as *const libc::c_uchar).offset(7 as libc::c_int as isize)
                as crate::stdlib::uint64_t)
                << 56 as libc::c_int;
        (*key).k[1 as libc::c_int as usize] = (*(src as *const libc::c_uchar)
            .offset(8 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize)
            as crate::stdlib::uint64_t)
            << 0 as libc::c_int
            | (*(src as *const libc::c_uchar)
                .offset(8 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as crate::stdlib::uint64_t)
                << 8 as libc::c_int
            | (*(src as *const libc::c_uchar)
                .offset(8 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as crate::stdlib::uint64_t)
                << 16 as libc::c_int
            | (*(src as *const libc::c_uchar)
                .offset(8 as libc::c_int as isize)
                .offset(3 as libc::c_int as isize) as crate::stdlib::uint64_t)
                << 24 as libc::c_int
            | (*(src as *const libc::c_uchar)
                .offset(8 as libc::c_int as isize)
                .offset(4 as libc::c_int as isize) as crate::stdlib::uint64_t)
                << 32 as libc::c_int
            | (*(src as *const libc::c_uchar)
                .offset(8 as libc::c_int as isize)
                .offset(5 as libc::c_int as isize) as crate::stdlib::uint64_t)
                << 40 as libc::c_int
            | (*(src as *const libc::c_uchar)
                .offset(8 as libc::c_int as isize)
                .offset(6 as libc::c_int as isize) as crate::stdlib::uint64_t)
                << 48 as libc::c_int
            | (*(src as *const libc::c_uchar)
                .offset(8 as libc::c_int as isize)
                .offset(7 as libc::c_int as isize) as crate::stdlib::uint64_t)
                << 56 as libc::c_int;
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

    pub unsafe extern "C" fn sip24_valid() -> libc::c_int {
        /* clang-format off */
        pub static mut vectors: [[libc::c_uchar; 8]; 64] = [
            [
                0x31 as libc::c_int as libc::c_uchar,
                0xe as libc::c_int as libc::c_uchar,
                0xe as libc::c_int as libc::c_uchar,
                0xdd as libc::c_int as libc::c_uchar,
                0x47 as libc::c_int as libc::c_uchar,
                0xdb as libc::c_int as libc::c_uchar,
                0x6f as libc::c_int as libc::c_uchar,
                0x72 as libc::c_int as libc::c_uchar,
            ],
            [
                0xfd as libc::c_int as libc::c_uchar,
                0x67 as libc::c_int as libc::c_uchar,
                0xdc as libc::c_int as libc::c_uchar,
                0x93 as libc::c_int as libc::c_uchar,
                0xc5 as libc::c_int as libc::c_uchar,
                0x39 as libc::c_int as libc::c_uchar,
                0xf8 as libc::c_int as libc::c_uchar,
                0x74 as libc::c_int as libc::c_uchar,
            ],
            [
                0x5a as libc::c_int as libc::c_uchar,
                0x4f as libc::c_int as libc::c_uchar,
                0xa9 as libc::c_int as libc::c_uchar,
                0xd9 as libc::c_int as libc::c_uchar,
                0x9 as libc::c_int as libc::c_uchar,
                0x80 as libc::c_int as libc::c_uchar,
                0x6c as libc::c_int as libc::c_uchar,
                0xd as libc::c_int as libc::c_uchar,
            ],
            [
                0x2d as libc::c_int as libc::c_uchar,
                0x7e as libc::c_int as libc::c_uchar,
                0xfb as libc::c_int as libc::c_uchar,
                0xd7 as libc::c_int as libc::c_uchar,
                0x96 as libc::c_int as libc::c_uchar,
                0x66 as libc::c_int as libc::c_uchar,
                0x67 as libc::c_int as libc::c_uchar,
                0x85 as libc::c_int as libc::c_uchar,
            ],
            [
                0xb7 as libc::c_int as libc::c_uchar,
                0x87 as libc::c_int as libc::c_uchar,
                0x71 as libc::c_int as libc::c_uchar,
                0x27 as libc::c_int as libc::c_uchar,
                0xe0 as libc::c_int as libc::c_uchar,
                0x94 as libc::c_int as libc::c_uchar,
                0x27 as libc::c_int as libc::c_uchar,
                0xcf as libc::c_int as libc::c_uchar,
            ],
            [
                0x8d as libc::c_int as libc::c_uchar,
                0xa6 as libc::c_int as libc::c_uchar,
                0x99 as libc::c_int as libc::c_uchar,
                0xcd as libc::c_int as libc::c_uchar,
                0x64 as libc::c_int as libc::c_uchar,
                0x55 as libc::c_int as libc::c_uchar,
                0x76 as libc::c_int as libc::c_uchar,
                0x18 as libc::c_int as libc::c_uchar,
            ],
            [
                0xce as libc::c_int as libc::c_uchar,
                0xe3 as libc::c_int as libc::c_uchar,
                0xfe as libc::c_int as libc::c_uchar,
                0x58 as libc::c_int as libc::c_uchar,
                0x6e as libc::c_int as libc::c_uchar,
                0x46 as libc::c_int as libc::c_uchar,
                0xc9 as libc::c_int as libc::c_uchar,
                0xcb as libc::c_int as libc::c_uchar,
            ],
            [
                0x37 as libc::c_int as libc::c_uchar,
                0xd1 as libc::c_int as libc::c_uchar,
                0x1 as libc::c_int as libc::c_uchar,
                0x8b as libc::c_int as libc::c_uchar,
                0xf5 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0x2 as libc::c_int as libc::c_uchar,
                0xab as libc::c_int as libc::c_uchar,
            ],
            [
                0x62 as libc::c_int as libc::c_uchar,
                0x24 as libc::c_int as libc::c_uchar,
                0x93 as libc::c_int as libc::c_uchar,
                0x9a as libc::c_int as libc::c_uchar,
                0x79 as libc::c_int as libc::c_uchar,
                0xf5 as libc::c_int as libc::c_uchar,
                0xf5 as libc::c_int as libc::c_uchar,
                0x93 as libc::c_int as libc::c_uchar,
            ],
            [
                0xb0 as libc::c_int as libc::c_uchar,
                0xe4 as libc::c_int as libc::c_uchar,
                0xa9 as libc::c_int as libc::c_uchar,
                0xb as libc::c_int as libc::c_uchar,
                0xdf as libc::c_int as libc::c_uchar,
                0x82 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0x9e as libc::c_int as libc::c_uchar,
            ],
            [
                0xf3 as libc::c_int as libc::c_uchar,
                0xb9 as libc::c_int as libc::c_uchar,
                0xdd as libc::c_int as libc::c_uchar,
                0x94 as libc::c_int as libc::c_uchar,
                0xc5 as libc::c_int as libc::c_uchar,
                0xbb as libc::c_int as libc::c_uchar,
                0x5d as libc::c_int as libc::c_uchar,
                0x7a as libc::c_int as libc::c_uchar,
            ],
            [
                0xa7 as libc::c_int as libc::c_uchar,
                0xad as libc::c_int as libc::c_uchar,
                0x6b as libc::c_int as libc::c_uchar,
                0x22 as libc::c_int as libc::c_uchar,
                0x46 as libc::c_int as libc::c_uchar,
                0x2f as libc::c_int as libc::c_uchar,
                0xb3 as libc::c_int as libc::c_uchar,
                0xf4 as libc::c_int as libc::c_uchar,
            ],
            [
                0xfb as libc::c_int as libc::c_uchar,
                0xe5 as libc::c_int as libc::c_uchar,
                0xe as libc::c_int as libc::c_uchar,
                0x86 as libc::c_int as libc::c_uchar,
                0xbc as libc::c_int as libc::c_uchar,
                0x8f as libc::c_int as libc::c_uchar,
                0x1e as libc::c_int as libc::c_uchar,
                0x75 as libc::c_int as libc::c_uchar,
            ],
            [
                0x90 as libc::c_int as libc::c_uchar,
                0x3d as libc::c_int as libc::c_uchar,
                0x84 as libc::c_int as libc::c_uchar,
                0xc0 as libc::c_int as libc::c_uchar,
                0x27 as libc::c_int as libc::c_uchar,
                0x56 as libc::c_int as libc::c_uchar,
                0xea as libc::c_int as libc::c_uchar,
                0x14 as libc::c_int as libc::c_uchar,
            ],
            [
                0xee as libc::c_int as libc::c_uchar,
                0xf2 as libc::c_int as libc::c_uchar,
                0x7a as libc::c_int as libc::c_uchar,
                0x8e as libc::c_int as libc::c_uchar,
                0x90 as libc::c_int as libc::c_uchar,
                0xca as libc::c_int as libc::c_uchar,
                0x23 as libc::c_int as libc::c_uchar,
                0xf7 as libc::c_int as libc::c_uchar,
            ],
            [
                0xe5 as libc::c_int as libc::c_uchar,
                0x45 as libc::c_int as libc::c_uchar,
                0xbe as libc::c_int as libc::c_uchar,
                0x49 as libc::c_int as libc::c_uchar,
                0x61 as libc::c_int as libc::c_uchar,
                0xca as libc::c_int as libc::c_uchar,
                0x29 as libc::c_int as libc::c_uchar,
                0xa1 as libc::c_int as libc::c_uchar,
            ],
            [
                0xdb as libc::c_int as libc::c_uchar,
                0x9b as libc::c_int as libc::c_uchar,
                0xc2 as libc::c_int as libc::c_uchar,
                0x57 as libc::c_int as libc::c_uchar,
                0x7f as libc::c_int as libc::c_uchar,
                0xcc as libc::c_int as libc::c_uchar,
                0x2a as libc::c_int as libc::c_uchar,
                0x3f as libc::c_int as libc::c_uchar,
            ],
            [
                0x94 as libc::c_int as libc::c_uchar,
                0x47 as libc::c_int as libc::c_uchar,
                0xbe as libc::c_int as libc::c_uchar,
                0x2c as libc::c_int as libc::c_uchar,
                0xf5 as libc::c_int as libc::c_uchar,
                0xe9 as libc::c_int as libc::c_uchar,
                0x9a as libc::c_int as libc::c_uchar,
                0x69 as libc::c_int as libc::c_uchar,
            ],
            [
                0x9c as libc::c_int as libc::c_uchar,
                0xd3 as libc::c_int as libc::c_uchar,
                0x8d as libc::c_int as libc::c_uchar,
                0x96 as libc::c_int as libc::c_uchar,
                0xf0 as libc::c_int as libc::c_uchar,
                0xb3 as libc::c_int as libc::c_uchar,
                0xc1 as libc::c_int as libc::c_uchar,
                0x4b as libc::c_int as libc::c_uchar,
            ],
            [
                0xbd as libc::c_int as libc::c_uchar,
                0x61 as libc::c_int as libc::c_uchar,
                0x79 as libc::c_int as libc::c_uchar,
                0xa7 as libc::c_int as libc::c_uchar,
                0x1d as libc::c_int as libc::c_uchar,
                0xc9 as libc::c_int as libc::c_uchar,
                0x6d as libc::c_int as libc::c_uchar,
                0xbb as libc::c_int as libc::c_uchar,
            ],
            [
                0x98 as libc::c_int as libc::c_uchar,
                0xee as libc::c_int as libc::c_uchar,
                0xa2 as libc::c_int as libc::c_uchar,
                0x1a as libc::c_int as libc::c_uchar,
                0xf2 as libc::c_int as libc::c_uchar,
                0x5c as libc::c_int as libc::c_uchar,
                0xd6 as libc::c_int as libc::c_uchar,
                0xbe as libc::c_int as libc::c_uchar,
            ],
            [
                0xc7 as libc::c_int as libc::c_uchar,
                0x67 as libc::c_int as libc::c_uchar,
                0x3b as libc::c_int as libc::c_uchar,
                0x2e as libc::c_int as libc::c_uchar,
                0xb0 as libc::c_int as libc::c_uchar,
                0xcb as libc::c_int as libc::c_uchar,
                0xf2 as libc::c_int as libc::c_uchar,
                0xd0 as libc::c_int as libc::c_uchar,
            ],
            [
                0x88 as libc::c_int as libc::c_uchar,
                0x3e as libc::c_int as libc::c_uchar,
                0xa3 as libc::c_int as libc::c_uchar,
                0xe3 as libc::c_int as libc::c_uchar,
                0x95 as libc::c_int as libc::c_uchar,
                0x67 as libc::c_int as libc::c_uchar,
                0x53 as libc::c_int as libc::c_uchar,
                0x93 as libc::c_int as libc::c_uchar,
            ],
            [
                0xc8 as libc::c_int as libc::c_uchar,
                0xce as libc::c_int as libc::c_uchar,
                0x5c as libc::c_int as libc::c_uchar,
                0xcd as libc::c_int as libc::c_uchar,
                0x8c as libc::c_int as libc::c_uchar,
                0x3 as libc::c_int as libc::c_uchar,
                0xc as libc::c_int as libc::c_uchar,
                0xa8 as libc::c_int as libc::c_uchar,
            ],
            [
                0x94 as libc::c_int as libc::c_uchar,
                0xaf as libc::c_int as libc::c_uchar,
                0x49 as libc::c_int as libc::c_uchar,
                0xf6 as libc::c_int as libc::c_uchar,
                0xc6 as libc::c_int as libc::c_uchar,
                0x50 as libc::c_int as libc::c_uchar,
                0xad as libc::c_int as libc::c_uchar,
                0xb8 as libc::c_int as libc::c_uchar,
            ],
            [
                0xea as libc::c_int as libc::c_uchar,
                0xb8 as libc::c_int as libc::c_uchar,
                0x85 as libc::c_int as libc::c_uchar,
                0x8a as libc::c_int as libc::c_uchar,
                0xde as libc::c_int as libc::c_uchar,
                0x92 as libc::c_int as libc::c_uchar,
                0xe1 as libc::c_int as libc::c_uchar,
                0xbc as libc::c_int as libc::c_uchar,
            ],
            [
                0xf3 as libc::c_int as libc::c_uchar,
                0x15 as libc::c_int as libc::c_uchar,
                0xbb as libc::c_int as libc::c_uchar,
                0x5b as libc::c_int as libc::c_uchar,
                0xb8 as libc::c_int as libc::c_uchar,
                0x35 as libc::c_int as libc::c_uchar,
                0xd8 as libc::c_int as libc::c_uchar,
                0x17 as libc::c_int as libc::c_uchar,
            ],
            [
                0xad as libc::c_int as libc::c_uchar,
                0xcf as libc::c_int as libc::c_uchar,
                0x6b as libc::c_int as libc::c_uchar,
                0x7 as libc::c_int as libc::c_uchar,
                0x63 as libc::c_int as libc::c_uchar,
                0x61 as libc::c_int as libc::c_uchar,
                0x2e as libc::c_int as libc::c_uchar,
                0x2f as libc::c_int as libc::c_uchar,
            ],
            [
                0xa5 as libc::c_int as libc::c_uchar,
                0xc9 as libc::c_int as libc::c_uchar,
                0x1d as libc::c_int as libc::c_uchar,
                0xa7 as libc::c_int as libc::c_uchar,
                0xac as libc::c_int as libc::c_uchar,
                0xaa as libc::c_int as libc::c_uchar,
                0x4d as libc::c_int as libc::c_uchar,
                0xde as libc::c_int as libc::c_uchar,
            ],
            [
                0x71 as libc::c_int as libc::c_uchar,
                0x65 as libc::c_int as libc::c_uchar,
                0x95 as libc::c_int as libc::c_uchar,
                0x87 as libc::c_int as libc::c_uchar,
                0x66 as libc::c_int as libc::c_uchar,
                0x50 as libc::c_int as libc::c_uchar,
                0xa2 as libc::c_int as libc::c_uchar,
                0xa6 as libc::c_int as libc::c_uchar,
            ],
            [
                0x28 as libc::c_int as libc::c_uchar,
                0xef as libc::c_int as libc::c_uchar,
                0x49 as libc::c_int as libc::c_uchar,
                0x5c as libc::c_int as libc::c_uchar,
                0x53 as libc::c_int as libc::c_uchar,
                0xa3 as libc::c_int as libc::c_uchar,
                0x87 as libc::c_int as libc::c_uchar,
                0xad as libc::c_int as libc::c_uchar,
            ],
            [
                0x42 as libc::c_int as libc::c_uchar,
                0xc3 as libc::c_int as libc::c_uchar,
                0x41 as libc::c_int as libc::c_uchar,
                0xd8 as libc::c_int as libc::c_uchar,
                0xfa as libc::c_int as libc::c_uchar,
                0x92 as libc::c_int as libc::c_uchar,
                0xd8 as libc::c_int as libc::c_uchar,
                0x32 as libc::c_int as libc::c_uchar,
            ],
            [
                0xce as libc::c_int as libc::c_uchar,
                0x7c as libc::c_int as libc::c_uchar,
                0xf2 as libc::c_int as libc::c_uchar,
                0x72 as libc::c_int as libc::c_uchar,
                0x2f as libc::c_int as libc::c_uchar,
                0x51 as libc::c_int as libc::c_uchar,
                0x27 as libc::c_int as libc::c_uchar,
                0x71 as libc::c_int as libc::c_uchar,
            ],
            [
                0xe3 as libc::c_int as libc::c_uchar,
                0x78 as libc::c_int as libc::c_uchar,
                0x59 as libc::c_int as libc::c_uchar,
                0xf9 as libc::c_int as libc::c_uchar,
                0x46 as libc::c_int as libc::c_uchar,
                0x23 as libc::c_int as libc::c_uchar,
                0xf3 as libc::c_int as libc::c_uchar,
                0xa7 as libc::c_int as libc::c_uchar,
            ],
            [
                0x38 as libc::c_int as libc::c_uchar,
                0x12 as libc::c_int as libc::c_uchar,
                0x5 as libc::c_int as libc::c_uchar,
                0xbb as libc::c_int as libc::c_uchar,
                0x1a as libc::c_int as libc::c_uchar,
                0xb0 as libc::c_int as libc::c_uchar,
                0xe0 as libc::c_int as libc::c_uchar,
                0x12 as libc::c_int as libc::c_uchar,
            ],
            [
                0xae as libc::c_int as libc::c_uchar,
                0x97 as libc::c_int as libc::c_uchar,
                0xa1 as libc::c_int as libc::c_uchar,
                0xf as libc::c_int as libc::c_uchar,
                0xd4 as libc::c_int as libc::c_uchar,
                0x34 as libc::c_int as libc::c_uchar,
                0xe0 as libc::c_int as libc::c_uchar,
                0x15 as libc::c_int as libc::c_uchar,
            ],
            [
                0xb4 as libc::c_int as libc::c_uchar,
                0xa3 as libc::c_int as libc::c_uchar,
                0x15 as libc::c_int as libc::c_uchar,
                0x8 as libc::c_int as libc::c_uchar,
                0xbe as libc::c_int as libc::c_uchar,
                0xff as libc::c_int as libc::c_uchar,
                0x4d as libc::c_int as libc::c_uchar,
                0x31 as libc::c_int as libc::c_uchar,
            ],
            [
                0x81 as libc::c_int as libc::c_uchar,
                0x39 as libc::c_int as libc::c_uchar,
                0x62 as libc::c_int as libc::c_uchar,
                0x29 as libc::c_int as libc::c_uchar,
                0xf0 as libc::c_int as libc::c_uchar,
                0x90 as libc::c_int as libc::c_uchar,
                0x79 as libc::c_int as libc::c_uchar,
                0x2 as libc::c_int as libc::c_uchar,
            ],
            [
                0x4d as libc::c_int as libc::c_uchar,
                0xc as libc::c_int as libc::c_uchar,
                0xf4 as libc::c_int as libc::c_uchar,
                0x9e as libc::c_int as libc::c_uchar,
                0xe5 as libc::c_int as libc::c_uchar,
                0xd4 as libc::c_int as libc::c_uchar,
                0xdc as libc::c_int as libc::c_uchar,
                0xca as libc::c_int as libc::c_uchar,
            ],
            [
                0x5c as libc::c_int as libc::c_uchar,
                0x73 as libc::c_int as libc::c_uchar,
                0x33 as libc::c_int as libc::c_uchar,
                0x6a as libc::c_int as libc::c_uchar,
                0x76 as libc::c_int as libc::c_uchar,
                0xd8 as libc::c_int as libc::c_uchar,
                0xbf as libc::c_int as libc::c_uchar,
                0x9a as libc::c_int as libc::c_uchar,
            ],
            [
                0xd0 as libc::c_int as libc::c_uchar,
                0xa7 as libc::c_int as libc::c_uchar,
                0x4 as libc::c_int as libc::c_uchar,
                0x53 as libc::c_int as libc::c_uchar,
                0x6b as libc::c_int as libc::c_uchar,
                0xa9 as libc::c_int as libc::c_uchar,
                0x3e as libc::c_int as libc::c_uchar,
                0xe as libc::c_int as libc::c_uchar,
            ],
            [
                0x92 as libc::c_int as libc::c_uchar,
                0x59 as libc::c_int as libc::c_uchar,
                0x58 as libc::c_int as libc::c_uchar,
                0xfc as libc::c_int as libc::c_uchar,
                0xd6 as libc::c_int as libc::c_uchar,
                0x42 as libc::c_int as libc::c_uchar,
                0xc as libc::c_int as libc::c_uchar,
                0xad as libc::c_int as libc::c_uchar,
            ],
            [
                0xa9 as libc::c_int as libc::c_uchar,
                0x15 as libc::c_int as libc::c_uchar,
                0xc2 as libc::c_int as libc::c_uchar,
                0x9b as libc::c_int as libc::c_uchar,
                0xc8 as libc::c_int as libc::c_uchar,
                0x6 as libc::c_int as libc::c_uchar,
                0x73 as libc::c_int as libc::c_uchar,
                0x18 as libc::c_int as libc::c_uchar,
            ],
            [
                0x95 as libc::c_int as libc::c_uchar,
                0x2b as libc::c_int as libc::c_uchar,
                0x79 as libc::c_int as libc::c_uchar,
                0xf3 as libc::c_int as libc::c_uchar,
                0xbc as libc::c_int as libc::c_uchar,
                0xa as libc::c_int as libc::c_uchar,
                0xa6 as libc::c_int as libc::c_uchar,
                0xd4 as libc::c_int as libc::c_uchar,
            ],
            [
                0xf2 as libc::c_int as libc::c_uchar,
                0x1d as libc::c_int as libc::c_uchar,
                0xf2 as libc::c_int as libc::c_uchar,
                0xe4 as libc::c_int as libc::c_uchar,
                0x1d as libc::c_int as libc::c_uchar,
                0x45 as libc::c_int as libc::c_uchar,
                0x35 as libc::c_int as libc::c_uchar,
                0xf9 as libc::c_int as libc::c_uchar,
            ],
            [
                0x87 as libc::c_int as libc::c_uchar,
                0x57 as libc::c_int as libc::c_uchar,
                0x75 as libc::c_int as libc::c_uchar,
                0x19 as libc::c_int as libc::c_uchar,
                0x4 as libc::c_int as libc::c_uchar,
                0x8f as libc::c_int as libc::c_uchar,
                0x53 as libc::c_int as libc::c_uchar,
                0xa9 as libc::c_int as libc::c_uchar,
            ],
            [
                0x10 as libc::c_int as libc::c_uchar,
                0xa5 as libc::c_int as libc::c_uchar,
                0x6c as libc::c_int as libc::c_uchar,
                0xf5 as libc::c_int as libc::c_uchar,
                0xdf as libc::c_int as libc::c_uchar,
                0xcd as libc::c_int as libc::c_uchar,
                0x9a as libc::c_int as libc::c_uchar,
                0xdb as libc::c_int as libc::c_uchar,
            ],
            [
                0xeb as libc::c_int as libc::c_uchar,
                0x75 as libc::c_int as libc::c_uchar,
                0x9 as libc::c_int as libc::c_uchar,
                0x5c as libc::c_int as libc::c_uchar,
                0xcd as libc::c_int as libc::c_uchar,
                0x98 as libc::c_int as libc::c_uchar,
                0x6c as libc::c_int as libc::c_uchar,
                0xd0 as libc::c_int as libc::c_uchar,
            ],
            [
                0x51 as libc::c_int as libc::c_uchar,
                0xa9 as libc::c_int as libc::c_uchar,
                0xcb as libc::c_int as libc::c_uchar,
                0x9e as libc::c_int as libc::c_uchar,
                0xcb as libc::c_int as libc::c_uchar,
                0xa3 as libc::c_int as libc::c_uchar,
                0x12 as libc::c_int as libc::c_uchar,
                0xe6 as libc::c_int as libc::c_uchar,
            ],
            [
                0x96 as libc::c_int as libc::c_uchar,
                0xaf as libc::c_int as libc::c_uchar,
                0xad as libc::c_int as libc::c_uchar,
                0xfc as libc::c_int as libc::c_uchar,
                0x2c as libc::c_int as libc::c_uchar,
                0xe6 as libc::c_int as libc::c_uchar,
                0x66 as libc::c_int as libc::c_uchar,
                0xc7 as libc::c_int as libc::c_uchar,
            ],
            [
                0x72 as libc::c_int as libc::c_uchar,
                0xfe as libc::c_int as libc::c_uchar,
                0x52 as libc::c_int as libc::c_uchar,
                0x97 as libc::c_int as libc::c_uchar,
                0x5a as libc::c_int as libc::c_uchar,
                0x43 as libc::c_int as libc::c_uchar,
                0x64 as libc::c_int as libc::c_uchar,
                0xee as libc::c_int as libc::c_uchar,
            ],
            [
                0x5a as libc::c_int as libc::c_uchar,
                0x16 as libc::c_int as libc::c_uchar,
                0x45 as libc::c_int as libc::c_uchar,
                0xb2 as libc::c_int as libc::c_uchar,
                0x76 as libc::c_int as libc::c_uchar,
                0xd5 as libc::c_int as libc::c_uchar,
                0x92 as libc::c_int as libc::c_uchar,
                0xa1 as libc::c_int as libc::c_uchar,
            ],
            [
                0xb2 as libc::c_int as libc::c_uchar,
                0x74 as libc::c_int as libc::c_uchar,
                0xcb as libc::c_int as libc::c_uchar,
                0x8e as libc::c_int as libc::c_uchar,
                0xbf as libc::c_int as libc::c_uchar,
                0x87 as libc::c_int as libc::c_uchar,
                0x87 as libc::c_int as libc::c_uchar,
                0xa as libc::c_int as libc::c_uchar,
            ],
            [
                0x6f as libc::c_int as libc::c_uchar,
                0x9b as libc::c_int as libc::c_uchar,
                0xb4 as libc::c_int as libc::c_uchar,
                0x20 as libc::c_int as libc::c_uchar,
                0x3d as libc::c_int as libc::c_uchar,
                0xe7 as libc::c_int as libc::c_uchar,
                0xb3 as libc::c_int as libc::c_uchar,
                0x81 as libc::c_int as libc::c_uchar,
            ],
            [
                0xea as libc::c_int as libc::c_uchar,
                0xec as libc::c_int as libc::c_uchar,
                0xb2 as libc::c_int as libc::c_uchar,
                0xa3 as libc::c_int as libc::c_uchar,
                0xb as libc::c_int as libc::c_uchar,
                0x22 as libc::c_int as libc::c_uchar,
                0xa8 as libc::c_int as libc::c_uchar,
                0x7f as libc::c_int as libc::c_uchar,
            ],
            [
                0x99 as libc::c_int as libc::c_uchar,
                0x24 as libc::c_int as libc::c_uchar,
                0xa4 as libc::c_int as libc::c_uchar,
                0x3c as libc::c_int as libc::c_uchar,
                0xc1 as libc::c_int as libc::c_uchar,
                0x31 as libc::c_int as libc::c_uchar,
                0x57 as libc::c_int as libc::c_uchar,
                0x24 as libc::c_int as libc::c_uchar,
            ],
            [
                0xbd as libc::c_int as libc::c_uchar,
                0x83 as libc::c_int as libc::c_uchar,
                0x8d as libc::c_int as libc::c_uchar,
                0x3a as libc::c_int as libc::c_uchar,
                0xaf as libc::c_int as libc::c_uchar,
                0xbf as libc::c_int as libc::c_uchar,
                0x8d as libc::c_int as libc::c_uchar,
                0xb7 as libc::c_int as libc::c_uchar,
            ],
            [
                0xb as libc::c_int as libc::c_uchar,
                0x1a as libc::c_int as libc::c_uchar,
                0x2a as libc::c_int as libc::c_uchar,
                0x32 as libc::c_int as libc::c_uchar,
                0x65 as libc::c_int as libc::c_uchar,
                0xd5 as libc::c_int as libc::c_uchar,
                0x1a as libc::c_int as libc::c_uchar,
                0xea as libc::c_int as libc::c_uchar,
            ],
            [
                0x13 as libc::c_int as libc::c_uchar,
                0x50 as libc::c_int as libc::c_uchar,
                0x79 as libc::c_int as libc::c_uchar,
                0xa3 as libc::c_int as libc::c_uchar,
                0x23 as libc::c_int as libc::c_uchar,
                0x1c as libc::c_int as libc::c_uchar,
                0xe6 as libc::c_int as libc::c_uchar,
                0x60 as libc::c_int as libc::c_uchar,
            ],
            [
                0x93 as libc::c_int as libc::c_uchar,
                0x2b as libc::c_int as libc::c_uchar,
                0x28 as libc::c_int as libc::c_uchar,
                0x46 as libc::c_int as libc::c_uchar,
                0xe4 as libc::c_int as libc::c_uchar,
                0xd7 as libc::c_int as libc::c_uchar,
                0x6 as libc::c_int as libc::c_uchar,
                0x66 as libc::c_int as libc::c_uchar,
            ],
            [
                0xe1 as libc::c_int as libc::c_uchar,
                0x91 as libc::c_int as libc::c_uchar,
                0x5f as libc::c_int as libc::c_uchar,
                0x5c as libc::c_int as libc::c_uchar,
                0xb1 as libc::c_int as libc::c_uchar,
                0xec as libc::c_int as libc::c_uchar,
                0xa4 as libc::c_int as libc::c_uchar,
                0x6c as libc::c_int as libc::c_uchar,
            ],
            [
                0xf3 as libc::c_int as libc::c_uchar,
                0x25 as libc::c_int as libc::c_uchar,
                0x96 as libc::c_int as libc::c_uchar,
                0x5c as libc::c_int as libc::c_uchar,
                0xa1 as libc::c_int as libc::c_uchar,
                0x6d as libc::c_int as libc::c_uchar,
                0x62 as libc::c_int as libc::c_uchar,
                0x9f as libc::c_int as libc::c_uchar,
            ],
            [
                0x57 as libc::c_int as libc::c_uchar,
                0x5f as libc::c_int as libc::c_uchar,
                0xf2 as libc::c_int as libc::c_uchar,
                0x8e as libc::c_int as libc::c_uchar,
                0x60 as libc::c_int as libc::c_uchar,
                0x38 as libc::c_int as libc::c_uchar,
                0x1b as libc::c_int as libc::c_uchar,
                0xe5 as libc::c_int as libc::c_uchar,
            ],
            [
                0x72 as libc::c_int as libc::c_uchar,
                0x45 as libc::c_int as libc::c_uchar,
                0x6 as libc::c_int as libc::c_uchar,
                0xeb as libc::c_int as libc::c_uchar,
                0x4c as libc::c_int as libc::c_uchar,
                0x32 as libc::c_int as libc::c_uchar,
                0x8a as libc::c_int as libc::c_uchar,
                0x95 as libc::c_int as libc::c_uchar,
            ],
        ];
        /* clang-format on */
        let mut in_0: [libc::c_uchar; 64] = [0; 64];
        let mut k: crate::siphash_h::sipkey = crate::siphash_h::sipkey { k: [0; 2] };
        let mut i: crate::stddef_h::size_t = 0;
        sip_tokey(
            &mut k,
            b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x00" as *const u8
                as *const libc::c_char as *const libc::c_void,
        );
        i = 0 as libc::c_int as crate::stddef_h::size_t;
        while i < ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong {
            in_0[i as usize] = i as libc::c_uchar;
            if siphash24(in_0.as_mut_ptr() as *const libc::c_void, i, &mut k)
                != (vectors[i as usize][0 as libc::c_int as usize] as crate::stdlib::uint64_t)
                    << 0 as libc::c_int
                    | (vectors[i as usize][1 as libc::c_int as usize] as crate::stdlib::uint64_t)
                        << 8 as libc::c_int
                    | (vectors[i as usize][2 as libc::c_int as usize] as crate::stdlib::uint64_t)
                        << 16 as libc::c_int
                    | (vectors[i as usize][3 as libc::c_int as usize] as crate::stdlib::uint64_t)
                        << 24 as libc::c_int
                    | (vectors[i as usize][4 as libc::c_int as usize] as crate::stdlib::uint64_t)
                        << 32 as libc::c_int
                    | (vectors[i as usize][5 as libc::c_int as usize] as crate::stdlib::uint64_t)
                        << 40 as libc::c_int
                    | (vectors[i as usize][6 as libc::c_int as usize] as crate::stdlib::uint64_t)
                        << 48 as libc::c_int
                    | (vectors[i as usize][7 as libc::c_int as usize] as crate::stdlib::uint64_t)
                        << 56 as libc::c_int
            {
                return 0 as libc::c_int;
            }
            i = i.wrapping_add(1)
        }
        return 1 as libc::c_int;
    }

    pub unsafe extern "C" fn siphash24(
        mut src: *const libc::c_void,
        mut len: crate::stddef_h::size_t,
        mut key: *const crate::siphash_h::sipkey,
    ) -> crate::stdlib::uint64_t {
        let mut state: crate::siphash_h::siphash = {
            let mut init = crate::siphash_h::siphash {
                v0: 0 as libc::c_int as crate::stdlib::uint64_t,
                v1: 0 as libc::c_int as crate::stdlib::uint64_t,
                v2: 0 as libc::c_int as crate::stdlib::uint64_t,
                v3: 0 as libc::c_int as crate::stdlib::uint64_t,
                buf: [0 as libc::c_int as libc::c_uchar, 0, 0, 0, 0, 0, 0, 0],
                p: 0 as *mut libc::c_uchar,
                c: 0 as libc::c_int as crate::stdlib::uint64_t,
            };
            init
        };
        return sip24_final(sip24_update(sip24_init(&mut state, key), src, len));
    }

    pub unsafe extern "C" fn sip24_init(
        mut H: *mut crate::siphash_h::siphash,
        mut key: *const crate::siphash_h::sipkey,
    ) -> *mut crate::siphash_h::siphash {
        (*H).v0 = ((0x736f6d65 as libc::c_uint as crate::stdlib::uint64_t) << 32 as libc::c_int
            | 0x70736575 as libc::c_uint as libc::c_ulong)
            ^ (*key).k[0 as libc::c_int as usize];
        (*H).v1 = ((0x646f7261 as libc::c_uint as crate::stdlib::uint64_t) << 32 as libc::c_int
            | 0x6e646f6d as libc::c_uint as libc::c_ulong)
            ^ (*key).k[1 as libc::c_int as usize];
        (*H).v2 = ((0x6c796765 as libc::c_uint as crate::stdlib::uint64_t) << 32 as libc::c_int
            | 0x6e657261 as libc::c_uint as libc::c_ulong)
            ^ (*key).k[0 as libc::c_int as usize];
        (*H).v3 = ((0x74656462 as libc::c_uint as crate::stdlib::uint64_t) << 32 as libc::c_int
            | 0x79746573 as libc::c_uint as libc::c_ulong)
            ^ (*key).k[1 as libc::c_int as usize];
        (*H).p = (*H).buf.as_mut_ptr();
        (*H).c = 0 as libc::c_int as crate::stdlib::uint64_t;
        return H;
    }

    use crate::stddef_h::size_t;
    use crate::stdlib::uint64_t;
    /* SIPHASH_H */
    /* SIPHASH_MAIN */
    /* sip24_valid() */
}

pub use crate::ascii_h::{
    ASCII_a, ASCII_c, ASCII_e, ASCII_g, ASCII_h, ASCII_l, ASCII_m, ASCII_n, ASCII_o, ASCII_p,
    ASCII_r, ASCII_s, ASCII_t, ASCII_w, ASCII_x, ASCII_0, ASCII_1, ASCII_2, ASCII_3, ASCII_8,
    ASCII_9, ASCII_A, ASCII_C, ASCII_COLON, ASCII_COMMA, ASCII_D, ASCII_E, ASCII_EQUALS,
    ASCII_EXCL, ASCII_F, ASCII_HASH, ASCII_I, ASCII_K, ASCII_L, ASCII_LPAREN, ASCII_M, ASCII_N,
    ASCII_O, ASCII_PERIOD, ASCII_PIPE, ASCII_R, ASCII_S, ASCII_SLASH, ASCII_T, ASCII_X, ASCII_Y,
};
pub use crate::expat_config_h::XML_CONTEXT_BYTES;
pub use crate::expat_external_h::{XML_Char, XML_Index, XML_LChar, XML_Size};
pub use crate::expat_h::{
    XML_AttlistDeclHandler, XML_Bool, XML_CharacterDataHandler, XML_CommentHandler, XML_Content,
    XML_Content_Quant, XML_Content_Type, XML_DefaultHandler, XML_ElementDeclHandler, XML_Encoding,
    XML_EndCdataSectionHandler, XML_EndDoctypeDeclHandler, XML_EndElementHandler,
    XML_EndNamespaceDeclHandler, XML_EntityDeclHandler, XML_Error, XML_Expat_Version,
    XML_ExternalEntityRefHandler, XML_Feature, XML_FeatureEnum, XML_Memory_Handling_Suite,
    XML_NotStandaloneHandler, XML_NotationDeclHandler, XML_ParamEntityParsing, XML_Parser,
    XML_Parsing, XML_ParsingStatus, XML_ProcessingInstructionHandler, XML_SkippedEntityHandler,
    XML_StartCdataSectionHandler, XML_StartDoctypeDeclHandler, XML_StartElementHandler,
    XML_StartNamespaceDeclHandler, XML_Status, XML_UnknownEncodingHandler,
    XML_UnparsedEntityDeclHandler, XML_XmlDeclHandler, XML_cp, XML_CQUANT_NONE, XML_CQUANT_OPT,
    XML_CQUANT_PLUS, XML_CQUANT_REP, XML_CTYPE_ANY, XML_CTYPE_CHOICE, XML_CTYPE_EMPTY,
    XML_CTYPE_MIXED, XML_CTYPE_NAME, XML_CTYPE_SEQ, XML_ERROR_ABORTED, XML_ERROR_ASYNC_ENTITY,
    XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF, XML_ERROR_BAD_CHAR_REF, XML_ERROR_BINARY_ENTITY_REF,
    XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING, XML_ERROR_DUPLICATE_ATTRIBUTE,
    XML_ERROR_ENTITY_DECLARED_IN_PE, XML_ERROR_EXTERNAL_ENTITY_HANDLING,
    XML_ERROR_FEATURE_REQUIRES_XML_DTD, XML_ERROR_FINISHED, XML_ERROR_INCOMPLETE_PE,
    XML_ERROR_INCORRECT_ENCODING, XML_ERROR_INVALID_ARGUMENT, XML_ERROR_INVALID_TOKEN,
    XML_ERROR_JUNK_AFTER_DOC_ELEMENT, XML_ERROR_MISPLACED_XML_PI, XML_ERROR_NONE,
    XML_ERROR_NOT_STANDALONE, XML_ERROR_NOT_SUSPENDED, XML_ERROR_NO_ELEMENTS, XML_ERROR_NO_MEMORY,
    XML_ERROR_PARAM_ENTITY_REF, XML_ERROR_PARTIAL_CHAR, XML_ERROR_PUBLICID,
    XML_ERROR_RECURSIVE_ENTITY_REF, XML_ERROR_RESERVED_NAMESPACE_URI,
    XML_ERROR_RESERVED_PREFIX_XML, XML_ERROR_RESERVED_PREFIX_XMLNS, XML_ERROR_SUSPENDED,
    XML_ERROR_SUSPEND_PE, XML_ERROR_SYNTAX, XML_ERROR_TAG_MISMATCH, XML_ERROR_TEXT_DECL,
    XML_ERROR_UNBOUND_PREFIX, XML_ERROR_UNCLOSED_CDATA_SECTION, XML_ERROR_UNCLOSED_TOKEN,
    XML_ERROR_UNDECLARING_PREFIX, XML_ERROR_UNDEFINED_ENTITY, XML_ERROR_UNEXPECTED_STATE,
    XML_ERROR_UNKNOWN_ENCODING, XML_ERROR_XML_DECL, XML_FALSE, XML_FEATURE_ATTR_INFO,
    XML_FEATURE_CONTEXT_BYTES, XML_FEATURE_DTD, XML_FEATURE_END, XML_FEATURE_LARGE_SIZE,
    XML_FEATURE_MIN_SIZE, XML_FEATURE_NS, XML_FEATURE_SIZEOF_XML_CHAR,
    XML_FEATURE_SIZEOF_XML_LCHAR, XML_FEATURE_UNICODE, XML_FEATURE_UNICODE_WCHAR_T, XML_FINISHED,
    XML_INITIALIZED, XML_MAJOR_VERSION, XML_MICRO_VERSION, XML_MINOR_VERSION,
    XML_PARAM_ENTITY_PARSING_ALWAYS, XML_PARAM_ENTITY_PARSING_NEVER,
    XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE, XML_PARSING, XML_STATUS_ERROR, XML_STATUS_ERROR_0,
    XML_STATUS_OK, XML_STATUS_OK_0, XML_STATUS_SUSPENDED, XML_STATUS_SUSPENDED_0, XML_SUSPENDED,
    XML_TRUE,
};
pub use crate::internal::__INT_MAX__;
pub use crate::siphash_h::{siphash, sipkey};
pub use crate::src::lib::xmlparse::siphash_h::{
    sip24_final, sip24_init, sip24_update, sip24_valid, sip_round, sip_tokey, siphash24,
};
pub use crate::src::lib::xmlrole::{
    prolog_state, C2RustUnnamed_0, XmlPrologStateInit, XmlPrologStateInitExternalEntity,
    PROLOG_STATE, XML_ROLE_ATTLIST_ELEMENT_NAME, XML_ROLE_ATTLIST_NONE,
    XML_ROLE_ATTRIBUTE_ENUM_VALUE, XML_ROLE_ATTRIBUTE_NAME, XML_ROLE_ATTRIBUTE_NOTATION_VALUE,
    XML_ROLE_ATTRIBUTE_TYPE_CDATA, XML_ROLE_ATTRIBUTE_TYPE_ENTITIES,
    XML_ROLE_ATTRIBUTE_TYPE_ENTITY, XML_ROLE_ATTRIBUTE_TYPE_ID, XML_ROLE_ATTRIBUTE_TYPE_IDREF,
    XML_ROLE_ATTRIBUTE_TYPE_IDREFS, XML_ROLE_ATTRIBUTE_TYPE_NMTOKEN,
    XML_ROLE_ATTRIBUTE_TYPE_NMTOKENS, XML_ROLE_COMMENT, XML_ROLE_CONTENT_ANY,
    XML_ROLE_CONTENT_ELEMENT, XML_ROLE_CONTENT_ELEMENT_OPT, XML_ROLE_CONTENT_ELEMENT_PLUS,
    XML_ROLE_CONTENT_ELEMENT_REP, XML_ROLE_CONTENT_EMPTY, XML_ROLE_CONTENT_PCDATA,
    XML_ROLE_DEFAULT_ATTRIBUTE_VALUE, XML_ROLE_DOCTYPE_CLOSE, XML_ROLE_DOCTYPE_INTERNAL_SUBSET,
    XML_ROLE_DOCTYPE_NAME, XML_ROLE_DOCTYPE_NONE, XML_ROLE_DOCTYPE_PUBLIC_ID,
    XML_ROLE_DOCTYPE_SYSTEM_ID, XML_ROLE_ELEMENT_NAME, XML_ROLE_ELEMENT_NONE,
    XML_ROLE_ENTITY_COMPLETE, XML_ROLE_ENTITY_NONE, XML_ROLE_ENTITY_NOTATION_NAME,
    XML_ROLE_ENTITY_PUBLIC_ID, XML_ROLE_ENTITY_SYSTEM_ID, XML_ROLE_ENTITY_VALUE, XML_ROLE_ERROR,
    XML_ROLE_FIXED_ATTRIBUTE_VALUE, XML_ROLE_GENERAL_ENTITY_NAME, XML_ROLE_GROUP_CHOICE,
    XML_ROLE_GROUP_CLOSE, XML_ROLE_GROUP_CLOSE_OPT, XML_ROLE_GROUP_CLOSE_PLUS,
    XML_ROLE_GROUP_CLOSE_REP, XML_ROLE_GROUP_OPEN, XML_ROLE_GROUP_SEQUENCE, XML_ROLE_IGNORE_SECT,
    XML_ROLE_IMPLIED_ATTRIBUTE_VALUE, XML_ROLE_INNER_PARAM_ENTITY_REF, XML_ROLE_INSTANCE_START,
    XML_ROLE_NONE, XML_ROLE_NOTATION_NAME, XML_ROLE_NOTATION_NONE, XML_ROLE_NOTATION_NO_SYSTEM_ID,
    XML_ROLE_NOTATION_PUBLIC_ID, XML_ROLE_NOTATION_SYSTEM_ID, XML_ROLE_PARAM_ENTITY_NAME,
    XML_ROLE_PARAM_ENTITY_REF, XML_ROLE_PI, XML_ROLE_REQUIRED_ATTRIBUTE_VALUE, XML_ROLE_TEXT_DECL,
    XML_ROLE_XML_DECL,
};
pub use crate::src::lib::xmltok::xmltok_ns_c::{
    XmlGetUtf8InternalEncoding, XmlGetUtf8InternalEncodingNS, XmlInitEncoding, XmlInitEncodingNS,
    XmlParseXmlDecl, XmlParseXmlDeclNS,
};
pub use crate::src::lib::xmltok::{
    encoding, position, XML_Convert_Result, XmlInitUnknownEncoding, XmlInitUnknownEncodingNS,
    XmlSizeOfUnknownEncoding, XmlUtf8Encode, ATTRIBUTE, CONVERTER, ENCODING, INIT_ENCODING,
    POSITION, SCANNER, XML_CONVERT_COMPLETED, XML_CONVERT_INPUT_INCOMPLETE,
    XML_CONVERT_OUTPUT_EXHAUSTED, XML_TOK_ATTRIBUTE_VALUE_S, XML_TOK_BOM, XML_TOK_CDATA_SECT_CLOSE,
    XML_TOK_CDATA_SECT_OPEN, XML_TOK_CHAR_REF, XML_TOK_COMMENT, XML_TOK_DATA_CHARS,
    XML_TOK_DATA_NEWLINE, XML_TOK_EMPTY_ELEMENT_NO_ATTS, XML_TOK_EMPTY_ELEMENT_WITH_ATTS,
    XML_TOK_END_TAG, XML_TOK_ENTITY_REF, XML_TOK_IGNORE_SECT, XML_TOK_INSTANCE_START,
    XML_TOK_INVALID, XML_TOK_NONE, XML_TOK_PARAM_ENTITY_REF, XML_TOK_PARTIAL, XML_TOK_PARTIAL_CHAR,
    XML_TOK_PI, XML_TOK_PROLOG_S, XML_TOK_START_TAG_NO_ATTS, XML_TOK_START_TAG_WITH_ATTS,
    XML_TOK_TRAILING_CR, XML_TOK_TRAILING_RSQB, XML_TOK_XML_DECL,
};
pub use crate::stddef_h::{ptrdiff_t, size_t, NULL};
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __off64_t, __off_t, __pid_t, __ssize_t,
    __suseconds_t, __time_t, __timezone_ptr_t, __uint64_t, fprintf, getrandom, gettimeofday,
    ssize_t, stderr, timezone, uint64_t, FILE, GRND_NONBLOCK, _IO_FILE,
};
use crate::stdlib::{__assert_fail, malloc, memcmp, memcpy, memmove, memset, read, realloc};
use ::libc::{__errno_location, close, free, getenv, getpid, open, strcmp};
pub use ::libc::{timeval, EINTR, INT_MAX, O_RDONLY};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XML_ParserStruct {
    pub m_userData: *mut libc::c_void,
    pub m_handlerArg: *mut libc::c_void,
    pub m_buffer: *mut libc::c_char,
    pub m_mem: crate::expat_h::XML_Memory_Handling_Suite,
    pub m_bufferPtr: *const libc::c_char,
    pub m_bufferEnd: *mut libc::c_char,
    pub m_bufferLim: *const libc::c_char,
    pub m_parseEndByteIndex: crate::expat_external_h::XML_Index,
    pub m_parseEndPtr: *const libc::c_char,
    pub m_dataBuf: *mut crate::expat_external_h::XML_Char,
    pub m_dataBufEnd: *mut crate::expat_external_h::XML_Char,
    pub m_startElementHandler: crate::expat_h::XML_StartElementHandler,
    pub m_endElementHandler: crate::expat_h::XML_EndElementHandler,
    pub m_characterDataHandler: crate::expat_h::XML_CharacterDataHandler,
    pub m_processingInstructionHandler: crate::expat_h::XML_ProcessingInstructionHandler,
    pub m_commentHandler: crate::expat_h::XML_CommentHandler,
    pub m_startCdataSectionHandler: crate::expat_h::XML_StartCdataSectionHandler,
    pub m_endCdataSectionHandler: crate::expat_h::XML_EndCdataSectionHandler,
    pub m_defaultHandler: crate::expat_h::XML_DefaultHandler,
    pub m_startDoctypeDeclHandler: crate::expat_h::XML_StartDoctypeDeclHandler,
    pub m_endDoctypeDeclHandler: crate::expat_h::XML_EndDoctypeDeclHandler,
    pub m_unparsedEntityDeclHandler: crate::expat_h::XML_UnparsedEntityDeclHandler,
    pub m_notationDeclHandler: crate::expat_h::XML_NotationDeclHandler,
    pub m_startNamespaceDeclHandler: crate::expat_h::XML_StartNamespaceDeclHandler,
    pub m_endNamespaceDeclHandler: crate::expat_h::XML_EndNamespaceDeclHandler,
    pub m_notStandaloneHandler: crate::expat_h::XML_NotStandaloneHandler,
    pub m_externalEntityRefHandler: crate::expat_h::XML_ExternalEntityRefHandler,
    pub m_externalEntityRefHandlerArg: crate::expat_h::XML_Parser,
    pub m_skippedEntityHandler: crate::expat_h::XML_SkippedEntityHandler,
    pub m_unknownEncodingHandler: crate::expat_h::XML_UnknownEncodingHandler,
    pub m_elementDeclHandler: crate::expat_h::XML_ElementDeclHandler,
    pub m_attlistDeclHandler: crate::expat_h::XML_AttlistDeclHandler,
    pub m_entityDeclHandler: crate::expat_h::XML_EntityDeclHandler,
    pub m_xmlDeclHandler: crate::expat_h::XML_XmlDeclHandler,
    pub m_encoding: *const crate::src::lib::xmltok::ENCODING,
    pub m_initEncoding: crate::src::lib::xmltok::INIT_ENCODING,
    pub m_internalEncoding: *const crate::src::lib::xmltok::ENCODING,
    pub m_protocolEncodingName: *const crate::expat_external_h::XML_Char,
    pub m_ns: crate::expat_h::XML_Bool,
    pub m_ns_triplets: crate::expat_h::XML_Bool,
    pub m_unknownEncodingMem: *mut libc::c_void,
    pub m_unknownEncodingData: *mut libc::c_void,
    pub m_unknownEncodingHandlerData: *mut libc::c_void,
    pub m_unknownEncodingRelease: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub m_prologState: crate::src::lib::xmlrole::PROLOG_STATE,
    pub m_processor: Option<Processor>,
    pub m_errorCode: crate::expat_h::XML_Error,
    pub m_eventPtr: *const libc::c_char,
    pub m_eventEndPtr: *const libc::c_char,
    pub m_positionPtr: *const libc::c_char,
    pub m_openInternalEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_freeInternalEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_defaultExpandInternalEntities: crate::expat_h::XML_Bool,
    pub m_tagLevel: libc::c_int,
    pub m_declEntity: *mut ENTITY,
    pub m_doctypeName: *const crate::expat_external_h::XML_Char,
    pub m_doctypeSysid: *const crate::expat_external_h::XML_Char,
    pub m_doctypePubid: *const crate::expat_external_h::XML_Char,
    pub m_declAttributeType: *const crate::expat_external_h::XML_Char,
    pub m_declNotationName: *const crate::expat_external_h::XML_Char,
    pub m_declNotationPublicId: *const crate::expat_external_h::XML_Char,
    pub m_declElementType: *mut ELEMENT_TYPE,
    pub m_declAttributeId: *mut ATTRIBUTE_ID,
    pub m_declAttributeIsCdata: crate::expat_h::XML_Bool,
    pub m_declAttributeIsId: crate::expat_h::XML_Bool,
    pub m_dtd: *mut DTD,
    pub m_curBase: *const crate::expat_external_h::XML_Char,
    pub m_tagStack: *mut TAG,
    pub m_freeTagList: *mut TAG,
    pub m_inheritedBindings: *mut BINDING,
    pub m_freeBindingList: *mut BINDING,
    pub m_attsSize: libc::c_int,
    pub m_nSpecifiedAtts: libc::c_int,
    pub m_idAttIndex: libc::c_int,
    pub m_atts: *mut crate::src::lib::xmltok::ATTRIBUTE,
    pub m_nsAtts: *mut NS_ATT,
    pub m_nsAttsVersion: libc::c_ulong,
    pub m_nsAttsPower: libc::c_uchar,
    pub m_position: crate::src::lib::xmltok::POSITION,
    pub m_tempPool: STRING_POOL,
    pub m_temp2Pool: STRING_POOL,
    pub m_groupConnector: *mut libc::c_char,
    pub m_groupSize: libc::c_uint,
    pub m_namespaceSeparator: crate::expat_external_h::XML_Char,
    pub m_parentParser: crate::expat_h::XML_Parser,
    pub m_parsingStatus: crate::expat_h::XML_ParsingStatus,
    pub m_isParamEntity: crate::expat_h::XML_Bool,
    pub m_useForeignDTD: crate::expat_h::XML_Bool,
    pub m_paramEntityParsing: crate::expat_h::XML_ParamEntityParsing,
    pub m_hash_secret_salt: libc::c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct STRING_POOL {
    pub blocks: *mut BLOCK,
    pub freeBlocks: *mut BLOCK,
    pub end: *const crate::expat_external_h::XML_Char,
    pub ptr: *mut crate::expat_external_h::XML_Char,
    pub start: *mut crate::expat_external_h::XML_Char,
    pub mem: *const crate::expat_h::XML_Memory_Handling_Suite,
}

pub type BLOCK = block;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct block {
    pub next: *mut block,
    pub size: libc::c_int,
    pub s: [crate::expat_external_h::XML_Char; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NS_ATT {
    pub version: libc::c_ulong,
    pub hash: libc::c_ulong,
    pub uriName: *const crate::expat_external_h::XML_Char,
}

pub type BINDING = binding;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binding {
    pub prefix: *mut prefix,
    pub nextTagBinding: *mut binding,
    pub prevPrefixBinding: *mut binding,
    pub attId: *const attribute_id,
    pub uri: *mut crate::expat_external_h::XML_Char,
    pub uriLen: libc::c_int,
    pub uriAlloc: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct attribute_id {
    pub name: *mut crate::expat_external_h::XML_Char,
    pub prefix: *mut PREFIX,
    pub maybeTokenized: crate::expat_h::XML_Bool,
    pub xmlns: crate::expat_h::XML_Bool,
}

pub type PREFIX = prefix;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prefix {
    pub name: *const crate::expat_external_h::XML_Char,
    pub binding: *mut BINDING,
}
/* TAG represents an open element.
   The name of the element is stored in both the document and API
   encodings.  The memory buffer 'buf' is a separately-allocated
   memory area which stores the name.  During the XML_Parse()/
   XMLParseBuffer() when the element is open, the memory for the 'raw'
   version of the name (in the document encoding) is shared with the
   document buffer.  If the element is open across calls to
   XML_Parse()/XML_ParseBuffer(), the buffer is re-allocated to
   contain the 'raw' name as well.

   A parser re-uses these structures, maintaining a list of allocated
   TAG objects in a free list.
*/

pub type TAG = tag;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag {
    pub parent: *mut tag,
    pub rawName: *const libc::c_char,
    pub rawNameLength: libc::c_int,
    pub name: TAG_NAME,
    pub buf: *mut libc::c_char,
    pub bufEnd: *mut libc::c_char,
    pub bindings: *mut BINDING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TAG_NAME {
    pub str_0: *const crate::expat_external_h::XML_Char,
    pub localPart: *const crate::expat_external_h::XML_Char,
    pub prefix: *const crate::expat_external_h::XML_Char,
    pub strLen: libc::c_int,
    pub uriLen: libc::c_int,
    pub prefixLen: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DTD {
    pub generalEntities: HASH_TABLE,
    pub elementTypes: HASH_TABLE,
    pub attributeIds: HASH_TABLE,
    pub prefixes: HASH_TABLE,
    pub pool: STRING_POOL,
    pub entityValuePool: STRING_POOL,
    pub keepProcessing: crate::expat_h::XML_Bool,
    pub hasParamEntityRefs: crate::expat_h::XML_Bool,
    pub standalone: crate::expat_h::XML_Bool,
    pub paramEntityRead: crate::expat_h::XML_Bool,
    pub paramEntities: HASH_TABLE,
    pub defaultPrefix: PREFIX,
    pub in_eldecl: crate::expat_h::XML_Bool,
    pub scaffold: *mut CONTENT_SCAFFOLD,
    pub contentStringLen: libc::c_uint,
    pub scaffSize: libc::c_uint,
    pub scaffCount: libc::c_uint,
    pub scaffLevel: libc::c_int,
    pub scaffIndex: *mut libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CONTENT_SCAFFOLD {
    pub type_0: crate::expat_h::XML_Content_Type,
    pub quant: crate::expat_h::XML_Content_Quant,
    pub name: *const crate::expat_external_h::XML_Char,
    pub firstchild: libc::c_int,
    pub lastchild: libc::c_int,
    pub childcnt: libc::c_int,
    pub nextsib: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HASH_TABLE {
    pub v: *mut *mut NAMED,
    pub power: libc::c_uchar,
    pub size: crate::stddef_h::size_t,
    pub used: crate::stddef_h::size_t,
    pub mem: *const crate::expat_h::XML_Memory_Handling_Suite,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NAMED {
    pub name: KEY,
}
/* parent of this element */
/* tagName in the original encoding */
/* tagName in the API encoding */
/* buffer for name components */
/* end of the buffer */
/* Round up n to be a multiple of sz, where sz is a power of 2. */
/* Do safe (NULL-aware) pointer arithmetic */

pub type KEY = *const crate::expat_external_h::XML_Char;
/* The XML_Char before the name is used to determine whether
an attribute has been specified. */

pub type ATTRIBUTE_ID = attribute_id;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ELEMENT_TYPE {
    pub name: *const crate::expat_external_h::XML_Char,
    pub prefix: *mut PREFIX,
    pub idAtt: *const ATTRIBUTE_ID,
    pub nDefaultAtts: libc::c_int,
    pub allocDefaultAtts: libc::c_int,
    pub defaultAtts: *mut DEFAULT_ATTRIBUTE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DEFAULT_ATTRIBUTE {
    pub id: *const ATTRIBUTE_ID,
    pub isCdata: crate::expat_h::XML_Bool,
    pub value: *const crate::expat_external_h::XML_Char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ENTITY {
    pub name: *const crate::expat_external_h::XML_Char,
    pub textPtr: *const crate::expat_external_h::XML_Char,
    pub textLen: libc::c_int,
    pub processed: libc::c_int,
    pub systemId: *const crate::expat_external_h::XML_Char,
    pub base: *const crate::expat_external_h::XML_Char,
    pub publicId: *const crate::expat_external_h::XML_Char,
    pub notation: *const crate::expat_external_h::XML_Char,
    pub open: crate::expat_h::XML_Bool,
    pub is_param: crate::expat_h::XML_Bool,
    pub is_internal: crate::expat_h::XML_Bool,
}

pub type OPEN_INTERNAL_ENTITY = open_internal_entity;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct open_internal_entity {
    pub internalEventPtr: *const libc::c_char,
    pub internalEventEndPtr: *const libc::c_char,
    pub next: *mut open_internal_entity,
    pub entity: *mut ENTITY,
    pub startTagLevel: libc::c_int,
    pub betweenDecl: crate::expat_h::XML_Bool,
}

pub type Processor = unsafe extern "C" fn(
    _: crate::expat_h::XML_Parser,
    _: *const libc::c_char,
    _: *const libc::c_char,
    _: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HASH_TABLE_ITER {
    pub p: *mut *mut NAMED,
    pub end: *mut *mut NAMED,
}

pub type ICHAR = libc::c_char;
/* WFC: PE Between Declarations */
/* f519f27c7c3b79fee55aeb8b1e53b7384b079d9118bf3a62eb3a60986a6742f2 (2.2.9+)
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
/* syscall prototype */
/* memset(), memcpy() */
/* UINT_MAX */
/* fprintf */
/* getenv, rand_s */
/* gettimeofday() */
/* getpid() */
/* getpid() */
/* O_RDONLY */
/* ndef _WIN32 */
/* getrandom */
/* defined(GRND_NONBLOCK) */
/* defined(HAVE_GETRANDOM) || defined(HAVE_SYSCALL_GETRANDOM) */

pub const XmlGetInternalEncoding: unsafe extern "C" fn()
    -> *const crate::src::lib::xmltok::ENCODING =
    crate::src::lib::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncoding;

pub const XmlGetInternalEncodingNS:
    unsafe extern "C" fn() -> *const crate::src::lib::xmltok::ENCODING =
    crate::src::lib::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncodingNS;

pub const XmlEncode: unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char) -> libc::c_int =
    crate::src::lib::xmltok::XmlUtf8Encode;

pub const INIT_TAG_BUF_SIZE: libc::c_int = 32 as libc::c_int;
/* must be a multiple of sizeof(XML_Char) */

pub const INIT_DATA_BUF_SIZE: libc::c_int = 1024 as libc::c_int;

pub const INIT_ATTS_SIZE: libc::c_int = 16 as libc::c_int;

pub const INIT_ATTS_VERSION: libc::c_uint = 0xffffffff as libc::c_uint;

pub const INIT_BLOCK_SIZE: libc::c_int = 1024 as libc::c_int;

pub const INIT_BUFFER_SIZE: libc::c_int = 1024 as libc::c_int;

pub const EXPAND_SPARE: libc::c_int = 24 as libc::c_int;

pub const INIT_SCAFFOLD_ELEMENTS: libc::c_int = 32 as libc::c_int;
/* Constructs a new parser; encoding is the encoding specified by the
   external protocol or NULL if there is none specified.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_ParserCreate(
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    return XML_ParserCreate_MM(
        encodingName,
        crate::stddef_h::NULL as *const crate::expat_h::XML_Memory_Handling_Suite,
        crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char,
    );
}
/* Constructs a new parser and namespace processor.  Element type
   names and attribute names that belong to a namespace will be
   expanded; unprefixed attribute names are never expanded; unprefixed
   element type names are expanded only if there is a default
   namespace. The expanded name is the concatenation of the namespace
   URI, the namespace separator character, and the local part of the
   name.  If the namespace separator is '\0' then the namespace URI
   and the local part will be concatenated without any separator.
   It is a programming error to use the separator '\0' with namespace
   triplets (see XML_SetReturnNSTriplet).
*/
#[no_mangle]

pub unsafe extern "C" fn XML_ParserCreateNS(
    mut encodingName: *const crate::expat_external_h::XML_Char,
    mut nsSep: crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    let mut tmp: [crate::expat_external_h::XML_Char; 2] = [0; 2];
    *tmp.as_mut_ptr() = nsSep;
    return XML_ParserCreate_MM(
        encodingName,
        crate::stddef_h::NULL as *const crate::expat_h::XML_Memory_Handling_Suite,
        tmp.as_mut_ptr(),
    );
}

static mut implicitContext: [crate::expat_external_h::XML_Char; 41] = [
    crate::ascii_h::ASCII_x as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_m as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_l as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_EQUALS as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_h as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_t as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_t as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_p as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_COLON as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_PERIOD as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_3 as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_PERIOD as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_o as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_r as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_g as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_X as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_M as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_L as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_1 as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_9 as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_9 as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_8 as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_n as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_a as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_m as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_e as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_s as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_p as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_a as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_c as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_e as crate::expat_external_h::XML_Char,
    '\u{0}' as i32 as crate::expat_external_h::XML_Char,
];
/* To avoid warnings about unused functions: */
/* Obtain entropy on Linux 3.17+ */

unsafe extern "C" fn writeRandomBytes_getrandom_nonblock(
    mut target: *mut libc::c_void,
    mut count: crate::stddef_h::size_t,
) -> libc::c_int {
    let mut success: libc::c_int = 0 as libc::c_int; /* full count bytes written? */
    let mut bytesWrittenTotal: crate::stddef_h::size_t =
        0 as libc::c_int as crate::stddef_h::size_t;
    let getrandomFlags: libc::c_uint = crate::stdlib::GRND_NONBLOCK as libc::c_uint;
    loop {
        let currentTarget: *mut libc::c_void =
            (target as *mut libc::c_char).offset(bytesWrittenTotal as isize) as *mut libc::c_void;
        let bytesToWrite: crate::stddef_h::size_t = count.wrapping_sub(bytesWrittenTotal);
        let bytesWrittenMore: libc::c_int =
            crate::stdlib::getrandom(currentTarget, bytesToWrite, getrandomFlags) as libc::c_int;
        if bytesWrittenMore > 0 as libc::c_int {
            bytesWrittenTotal = (bytesWrittenTotal as libc::c_ulong)
                .wrapping_add(bytesWrittenMore as libc::c_ulong)
                as crate::stddef_h::size_t
                as crate::stddef_h::size_t;
            if bytesWrittenTotal >= count {
                success = 1 as libc::c_int
            }
        }
        if !(success == 0 && *::libc::__errno_location() == ::libc::EINTR) {
            break;
        }
    }
    return success;
}
/* defined(HAVE_GETRANDOM) || defined(HAVE_SYSCALL_GETRANDOM) */
/* Extract entropy from /dev/urandom */

unsafe extern "C" fn writeRandomBytes_dev_urandom(
    mut target: *mut libc::c_void,
    mut count: crate::stddef_h::size_t,
) -> libc::c_int {
    let mut success: libc::c_int = 0 as libc::c_int; /* full count bytes written? */
    let mut bytesWrittenTotal: crate::stddef_h::size_t =
        0 as libc::c_int as crate::stddef_h::size_t;
    let fd: libc::c_int = ::libc::open(
        b"/dev/urandom\x00" as *const u8 as *const libc::c_char,
        ::libc::O_RDONLY,
    );
    if fd < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    loop {
        let currentTarget: *mut libc::c_void =
            (target as *mut libc::c_char).offset(bytesWrittenTotal as isize) as *mut libc::c_void;
        let bytesToWrite: crate::stddef_h::size_t = count.wrapping_sub(bytesWrittenTotal);
        let bytesWrittenMore: crate::stdlib::ssize_t =
            crate::stdlib::read(fd, currentTarget, bytesToWrite);
        if bytesWrittenMore > 0 as libc::c_int as libc::c_long {
            bytesWrittenTotal = (bytesWrittenTotal as libc::c_ulong)
                .wrapping_add(bytesWrittenMore as libc::c_ulong)
                as crate::stddef_h::size_t
                as crate::stddef_h::size_t;
            if bytesWrittenTotal >= count {
                success = 1 as libc::c_int
            }
        }
        if !(success == 0 && *::libc::__errno_location() == ::libc::EINTR) {
            break;
        }
    }
    ::libc::close(fd);
    return success;
}
/* ! defined(_WIN32) && defined(XML_DEV_URANDOM) */
/* ! defined(HAVE_ARC4RANDOM_BUF) && ! defined(HAVE_ARC4RANDOM) */
/* defined(HAVE_ARC4RANDOM) && ! defined(HAVE_ARC4RANDOM_BUF) */
/* _WIN32 */

unsafe extern "C" fn gather_time_entropy() -> libc::c_ulong {
    let mut tv: ::libc::timeval = ::libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut gettimeofday_res: libc::c_int = 0;
    gettimeofday_res = crate::stdlib::gettimeofday(
        &mut tv,
        crate::stddef_h::NULL as *mut crate::stdlib::timezone,
    );
    if gettimeofday_res == 0 as libc::c_int {
    } else {
        crate::stdlib::__assert_fail(
            b"gettimeofday_res == 0\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/lib/xmlparse.c\x00" as *const u8
                as *const libc::c_char,
            782 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"unsigned long gather_time_entropy(void)\x00",
            ))
            .as_ptr(),
        );
    }
    /* defined(NDEBUG) */
    /* Microseconds time is <20 bits entropy */
    return tv.tv_usec as libc::c_ulong;
}
/* ! defined(HAVE_ARC4RANDOM_BUF) && ! defined(HAVE_ARC4RANDOM) */

unsafe extern "C" fn ENTROPY_DEBUG(
    mut label: *const libc::c_char,
    mut entropy: libc::c_ulong,
) -> libc::c_ulong {
    let EXPAT_ENTROPY_DEBUG: *const libc::c_char =
        ::libc::getenv(b"EXPAT_ENTROPY_DEBUG\x00" as *const u8 as *const libc::c_char);
    if !EXPAT_ENTROPY_DEBUG.is_null()
        && ::libc::strcmp(
            EXPAT_ENTROPY_DEBUG,
            b"1\x00" as *const u8 as *const libc::c_char,
        ) == 0
    {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"Entropy: %s --> 0x%0*lx (%lu bytes)\n\x00" as *const u8 as *const libc::c_char,
            label,
            ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as libc::c_int
                * 2 as libc::c_int,
            entropy,
            ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
        );
    }
    return entropy;
}

unsafe extern "C" fn generate_hash_secret_salt(
    mut parser: crate::expat_h::XML_Parser,
) -> libc::c_ulong {
    let mut entropy: libc::c_ulong = 0;
    /* "Failproof" high quality providers: */
    /* Try high quality providers first .. */
    if writeRandomBytes_getrandom_nonblock(
        &mut entropy as *mut libc::c_ulong as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
    ) != 0
    {
        return ENTROPY_DEBUG(
            b"getrandom\x00" as *const u8 as *const libc::c_char,
            entropy,
        );
    }
    if writeRandomBytes_dev_urandom(
        &mut entropy as *mut libc::c_ulong as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
    ) != 0
    {
        return ENTROPY_DEBUG(
            b"/dev/urandom\x00" as *const u8 as *const libc::c_char,
            entropy,
        );
    }
    /* ! defined(_WIN32) && defined(XML_DEV_URANDOM) */
    /* .. and self-made low quality for backup: */
    /* Process ID is 0 bits entropy if attacker has local access */
    entropy = gather_time_entropy() ^ ::libc::getpid() as libc::c_ulong;
    /* Factors are 2^31-1 and 2^61-1 (Mersenne primes M31 and M61) */
    if ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong == 4 as libc::c_int as libc::c_ulong
    {
        return ENTROPY_DEBUG(
            b"fallback(4)\x00" as *const u8 as *const libc::c_char,
            entropy.wrapping_mul(2147483647 as libc::c_int as libc::c_ulong),
        );
    } else {
        return ENTROPY_DEBUG(
            b"fallback(8)\x00" as *const u8 as *const libc::c_char,
            entropy.wrapping_mul(2305843009213693951 as libc::c_ulonglong as libc::c_ulong),
        );
    };
}

unsafe extern "C" fn get_hash_secret_salt(mut parser: crate::expat_h::XML_Parser) -> libc::c_ulong {
    if !(*parser).m_parentParser.is_null() {
        return get_hash_secret_salt((*parser).m_parentParser);
    }
    return (*parser).m_hash_secret_salt;
}

unsafe extern "C" fn startParsing(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Bool {
    /* hash functions must be initialized before setContext() is called */
    if (*parser).m_hash_secret_salt == 0 as libc::c_int as libc::c_ulong {
        (*parser).m_hash_secret_salt = generate_hash_secret_salt(parser)
    }
    if (*parser).m_ns != 0 {
        /* implicit context only set for root parser, since child
           parsers (i.e. external entity parsers) will inherit it
        */
        return setContext(parser, implicitContext.as_ptr());
    }
    return crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
}
/* Constructs a new parser using the memory management suite referred to
   by memsuite. If memsuite is NULL, then use the standard library memory
   suite. If namespaceSeparator is non-NULL it creates a parser with
   namespace processing as described above. The character pointed at
   will serve as the namespace separator.

   All further memory operations used for the created parser will come from
   the given suite.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_ParserCreate_MM(
    mut encodingName: *const crate::expat_external_h::XML_Char,
    mut memsuite: *const crate::expat_h::XML_Memory_Handling_Suite,
    mut nameSep: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    return parserCreate(
        encodingName,
        memsuite,
        nameSep,
        crate::stddef_h::NULL as *mut DTD,
    );
}

unsafe extern "C" fn parserCreate(
    mut encodingName: *const crate::expat_external_h::XML_Char,
    mut memsuite: *const crate::expat_h::XML_Memory_Handling_Suite,
    mut nameSep: *const crate::expat_external_h::XML_Char,
    mut dtd: *mut DTD,
) -> crate::expat_h::XML_Parser {
    let mut parser: crate::expat_h::XML_Parser = 0 as *mut XML_ParserStruct;
    if !memsuite.is_null() {
        let mut mtemp: *mut crate::expat_h::XML_Memory_Handling_Suite =
            0 as *mut crate::expat_h::XML_Memory_Handling_Suite;
        parser = (*memsuite).malloc_fcn.expect("non-null function pointer")(::std::mem::size_of::<
            XML_ParserStruct,
        >()
            as libc::c_ulong) as crate::expat_h::XML_Parser;
        if !parser.is_null() {
            mtemp = &(*parser).m_mem as *const crate::expat_h::XML_Memory_Handling_Suite
                as *mut crate::expat_h::XML_Memory_Handling_Suite;
            (*mtemp).malloc_fcn = (*memsuite).malloc_fcn;
            (*mtemp).realloc_fcn = (*memsuite).realloc_fcn;
            (*mtemp).free_fcn = (*memsuite).free_fcn
        }
    } else {
        let mut mtemp_0: *mut crate::expat_h::XML_Memory_Handling_Suite =
            0 as *mut crate::expat_h::XML_Memory_Handling_Suite;
        parser = crate::stdlib::malloc(::std::mem::size_of::<XML_ParserStruct>() as libc::c_ulong)
            as crate::expat_h::XML_Parser;
        if !parser.is_null() {
            mtemp_0 = &(*parser).m_mem as *const crate::expat_h::XML_Memory_Handling_Suite
                as *mut crate::expat_h::XML_Memory_Handling_Suite;
            (*mtemp_0).malloc_fcn = Some(
                crate::stdlib::malloc
                    as unsafe extern "C" fn(_: libc::c_ulong) -> *mut libc::c_void,
            );
            (*mtemp_0).realloc_fcn = Some(
                crate::stdlib::realloc
                    as unsafe extern "C" fn(
                        _: *mut libc::c_void,
                        _: libc::c_ulong,
                    ) -> *mut libc::c_void,
            );
            (*mtemp_0).free_fcn =
                Some(::libc::free as unsafe extern "C" fn(_: *mut libc::c_void) -> ())
        }
    }
    if parser.is_null() {
        return parser;
    }
    (*parser).m_buffer = crate::stddef_h::NULL as *mut libc::c_char;
    (*parser).m_bufferLim = crate::stddef_h::NULL as *const libc::c_char;
    (*parser).m_attsSize = INIT_ATTS_SIZE;
    (*parser).m_atts = (*parser)
        .m_mem
        .malloc_fcn
        .expect("non-null function pointer")(
        ((*parser).m_attsSize as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::src::lib::xmltok::ATTRIBUTE,
        >() as libc::c_ulong),
    ) as *mut crate::src::lib::xmltok::ATTRIBUTE;
    if (*parser).m_atts.is_null() {
        (*parser).m_mem.free_fcn.expect("non-null function pointer")(parser as *mut libc::c_void);
        return crate::stddef_h::NULL as crate::expat_h::XML_Parser;
    }
    (*parser).m_dataBuf = (*parser)
        .m_mem
        .malloc_fcn
        .expect("non-null function pointer")(
        (1024 as libc::c_int as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::expat_external_h::XML_Char,
        >() as libc::c_ulong),
    ) as *mut crate::expat_external_h::XML_Char;
    if (*parser).m_dataBuf.is_null() {
        (*parser).m_mem.free_fcn.expect("non-null function pointer")(
            (*parser).m_atts as *mut libc::c_void,
        );
        (*parser).m_mem.free_fcn.expect("non-null function pointer")(parser as *mut libc::c_void);
        return crate::stddef_h::NULL as crate::expat_h::XML_Parser;
    }
    (*parser).m_dataBufEnd = (*parser).m_dataBuf.offset(INIT_DATA_BUF_SIZE as isize);
    if !dtd.is_null() {
        (*parser).m_dtd = dtd
    } else {
        (*parser).m_dtd = dtdCreate(&(*parser).m_mem);
        if (*parser).m_dtd.is_null() {
            (*parser).m_mem.free_fcn.expect("non-null function pointer")(
                (*parser).m_dataBuf as *mut libc::c_void,
            );
            (*parser).m_mem.free_fcn.expect("non-null function pointer")(
                (*parser).m_atts as *mut libc::c_void,
            );
            (*parser).m_mem.free_fcn.expect("non-null function pointer")(
                parser as *mut libc::c_void,
            );
            return crate::stddef_h::NULL as crate::expat_h::XML_Parser;
        }
    }
    (*parser).m_freeBindingList = crate::stddef_h::NULL as *mut BINDING;
    (*parser).m_freeTagList = crate::stddef_h::NULL as *mut TAG;
    (*parser).m_freeInternalEntities = crate::stddef_h::NULL as *mut OPEN_INTERNAL_ENTITY;
    (*parser).m_groupSize = 0 as libc::c_int as libc::c_uint;
    (*parser).m_groupConnector = crate::stddef_h::NULL as *mut libc::c_char;
    (*parser).m_unknownEncodingHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_UnknownEncodingHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_unknownEncodingHandlerData = crate::stddef_h::NULL as *mut libc::c_void;
    (*parser).m_namespaceSeparator =
        crate::ascii_h::ASCII_EXCL as crate::expat_external_h::XML_Char;
    (*parser).m_ns = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    (*parser).m_ns_triplets = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    (*parser).m_nsAtts = crate::stddef_h::NULL as *mut NS_ATT;
    (*parser).m_nsAttsVersion = 0 as libc::c_int as libc::c_ulong;
    (*parser).m_nsAttsPower = 0 as libc::c_int as libc::c_uchar;
    (*parser).m_protocolEncodingName =
        crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    poolInit(&mut (*parser).m_tempPool, &(*parser).m_mem);
    poolInit(&mut (*parser).m_temp2Pool, &(*parser).m_mem);
    parserInit(parser, encodingName);
    if !encodingName.is_null() && (*parser).m_protocolEncodingName.is_null() {
        XML_ParserFree(parser);
        return crate::stddef_h::NULL as crate::expat_h::XML_Parser;
    }
    if !nameSep.is_null() {
        (*parser).m_ns = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
        (*parser).m_internalEncoding =
            crate::src::lib::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncodingNS()
                as *const crate::src::lib::xmltok::encoding;
        (*parser).m_namespaceSeparator = *nameSep
    } else {
        (*parser).m_internalEncoding =
            crate::src::lib::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncoding()
                as *const crate::src::lib::xmltok::encoding
    }
    return parser;
}

unsafe extern "C" fn parserInit(
    mut parser: crate::expat_h::XML_Parser,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) {
    (*parser).m_processor = Some(prologInitProcessor as Processor);
    crate::src::lib::xmlrole::XmlPrologStateInit(
        &mut (*parser).m_prologState as *mut _ as *mut crate::src::lib::xmlrole::prolog_state,
    );
    if !encodingName.is_null() {
        (*parser).m_protocolEncodingName = copyString(encodingName, &(*parser).m_mem)
    }
    (*parser).m_curBase = crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    crate::src::lib::xmltok::xmltok_ns_c::XmlInitEncoding(
        &mut (*parser).m_initEncoding as *mut _ as *mut crate::src::lib::xmltok::INIT_ENCODING,
        &mut (*parser).m_encoding as *mut _ as *mut *const crate::src::lib::xmltok::encoding,
        0 as *const libc::c_char,
    );
    (*parser).m_userData = crate::stddef_h::NULL as *mut libc::c_void;
    (*parser).m_handlerArg = crate::stddef_h::NULL as *mut libc::c_void;
    (*parser).m_startElementHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_StartElementHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_endElementHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_EndElementHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_characterDataHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_CharacterDataHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_processingInstructionHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_ProcessingInstructionHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_commentHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_CommentHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_startCdataSectionHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_StartCdataSectionHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_endCdataSectionHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_EndCdataSectionHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_defaultHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_DefaultHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_startDoctypeDeclHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_StartDoctypeDeclHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_endDoctypeDeclHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_EndDoctypeDeclHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_unparsedEntityDeclHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_UnparsedEntityDeclHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_notationDeclHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_NotationDeclHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_startNamespaceDeclHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_StartNamespaceDeclHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_endNamespaceDeclHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_EndNamespaceDeclHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_notStandaloneHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_NotStandaloneHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_externalEntityRefHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_ExternalEntityRefHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_externalEntityRefHandlerArg = parser;
    (*parser).m_skippedEntityHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_SkippedEntityHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_elementDeclHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_ElementDeclHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_attlistDeclHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_AttlistDeclHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_entityDeclHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_EntityDeclHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_xmlDeclHandler = ::std::mem::transmute::<
        libc::intptr_t,
        crate::expat_h::XML_XmlDeclHandler,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_bufferPtr = (*parser).m_buffer;
    (*parser).m_bufferEnd = (*parser).m_buffer;
    (*parser).m_parseEndByteIndex = 0 as libc::c_int as crate::expat_external_h::XML_Index;
    (*parser).m_parseEndPtr = crate::stddef_h::NULL as *const libc::c_char;
    (*parser).m_declElementType = crate::stddef_h::NULL as *mut ELEMENT_TYPE;
    (*parser).m_declAttributeId = crate::stddef_h::NULL as *mut ATTRIBUTE_ID;
    (*parser).m_declEntity = crate::stddef_h::NULL as *mut ENTITY;
    (*parser).m_doctypeName = crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    (*parser).m_doctypeSysid = crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    (*parser).m_doctypePubid = crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    (*parser).m_declAttributeType =
        crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    (*parser).m_declNotationName =
        crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    (*parser).m_declNotationPublicId =
        crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    (*parser).m_declAttributeIsCdata = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    (*parser).m_declAttributeIsId = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    crate::stdlib::memset(
        &mut (*parser).m_position as *mut crate::src::lib::xmltok::POSITION as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::lib::xmltok::POSITION>() as libc::c_ulong,
    );
    (*parser).m_errorCode = crate::expat_h::XML_ERROR_NONE;
    (*parser).m_eventPtr = crate::stddef_h::NULL as *const libc::c_char;
    (*parser).m_eventEndPtr = crate::stddef_h::NULL as *const libc::c_char;
    (*parser).m_positionPtr = crate::stddef_h::NULL as *const libc::c_char;
    (*parser).m_openInternalEntities = crate::stddef_h::NULL as *mut OPEN_INTERNAL_ENTITY;
    (*parser).m_defaultExpandInternalEntities =
        crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
    (*parser).m_tagLevel = 0 as libc::c_int;
    (*parser).m_tagStack = crate::stddef_h::NULL as *mut TAG;
    (*parser).m_inheritedBindings = crate::stddef_h::NULL as *mut BINDING;
    (*parser).m_nSpecifiedAtts = 0 as libc::c_int;
    (*parser).m_unknownEncodingMem = crate::stddef_h::NULL as *mut libc::c_void;
    (*parser).m_unknownEncodingRelease = ::std::mem::transmute::<
        libc::intptr_t,
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*parser).m_unknownEncodingData = crate::stddef_h::NULL as *mut libc::c_void;
    (*parser).m_parentParser = crate::stddef_h::NULL as crate::expat_h::XML_Parser;
    (*parser).m_parsingStatus.parsing = crate::expat_h::XML_INITIALIZED;
    (*parser).m_isParamEntity = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    (*parser).m_useForeignDTD = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    (*parser).m_paramEntityParsing = crate::expat_h::XML_PARAM_ENTITY_PARSING_NEVER;
    (*parser).m_hash_secret_salt = 0 as libc::c_int as libc::c_ulong;
}
/* moves list of bindings to m_freeBindingList */

unsafe extern "C" fn moveToFreeBindingList(
    mut parser: crate::expat_h::XML_Parser,
    mut bindings: *mut BINDING,
) {
    while !bindings.is_null() {
        let mut b: *mut BINDING = bindings;
        bindings = (*bindings).nextTagBinding;
        (*b).nextTagBinding = (*parser).m_freeBindingList;
        (*parser).m_freeBindingList = b
    }
}
/* Prepare a parser object to be re-used.  This is particularly
   valuable when memory allocation overhead is disproportionately high,
   such as when a large number of small documnents need to be parsed.
   All handlers are cleared from the parser, except for the
   unknownEncodingHandler. The parser's external state is re-initialized
   except for the values of ns and ns_triplets.

   Added in Expat 1.95.3.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_ParserReset(
    mut parser: crate::expat_h::XML_Parser,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Bool {
    let mut tStk: *mut TAG = 0 as *mut TAG;
    let mut openEntityList: *mut OPEN_INTERNAL_ENTITY = 0 as *mut OPEN_INTERNAL_ENTITY;
    if parser.is_null() {
        return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    }
    if !(*parser).m_parentParser.is_null() {
        return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    }
    /* move m_tagStack to m_freeTagList */
    tStk = (*parser).m_tagStack;
    while !tStk.is_null() {
        let mut tag: *mut TAG = tStk;
        tStk = (*tStk).parent;
        (*tag).parent = (*parser).m_freeTagList;
        moveToFreeBindingList(parser, (*tag).bindings);
        (*tag).bindings = crate::stddef_h::NULL as *mut BINDING;
        (*parser).m_freeTagList = tag
    }
    /* move m_openInternalEntities to m_freeInternalEntities */
    openEntityList = (*parser).m_openInternalEntities;
    while !openEntityList.is_null() {
        let mut openEntity: *mut OPEN_INTERNAL_ENTITY = openEntityList;
        openEntityList = (*openEntity).next;
        (*openEntity).next = (*parser).m_freeInternalEntities;
        (*parser).m_freeInternalEntities = openEntity
    }
    moveToFreeBindingList(parser, (*parser).m_inheritedBindings);
    (*parser).m_mem.free_fcn.expect("non-null function pointer")((*parser).m_unknownEncodingMem);
    if (*parser).m_unknownEncodingRelease.is_some() {
        (*parser)
            .m_unknownEncodingRelease
            .expect("non-null function pointer")((*parser).m_unknownEncodingData);
    }
    poolClear(&mut (*parser).m_tempPool);
    poolClear(&mut (*parser).m_temp2Pool);
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(
        (*parser).m_protocolEncodingName as *mut libc::c_void,
    );
    (*parser).m_protocolEncodingName =
        crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    parserInit(parser, encodingName);
    dtdReset((*parser).m_dtd, &(*parser).m_mem);
    return crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
}
/* Returns the last value set by XML_SetUserData or NULL. */
/* This is equivalent to supplying an encoding argument to
   XML_ParserCreate. On success XML_SetEncoding returns non-zero,
   zero otherwise.
   Note: Calling XML_SetEncoding after XML_Parse or XML_ParseBuffer
     has no effect and returns XML_STATUS_ERROR.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_SetEncoding(
    mut parser: crate::expat_h::XML_Parser,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Status {
    if parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
    }
    /* Block after XML_Parse()/XML_ParseBuffer() has been called.
       XXX There's no way for the caller to determine which of the
       XXX possible error cases caused the XML_STATUS_ERROR return.
    */
    if (*parser).m_parsingStatus.parsing as libc::c_uint
        == crate::expat_h::XML_PARSING as libc::c_int as libc::c_uint
        || (*parser).m_parsingStatus.parsing as libc::c_uint
            == crate::expat_h::XML_SUSPENDED as libc::c_int as libc::c_uint
    {
        return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
    }
    /* Get rid of any previous encoding name */
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(
        (*parser).m_protocolEncodingName as *mut libc::c_void,
    );
    if encodingName.is_null() {
        /* No new encoding name */
        (*parser).m_protocolEncodingName =
            crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char
    } else {
        /* Copy the new encoding name into allocated memory */
        (*parser).m_protocolEncodingName = copyString(encodingName, &(*parser).m_mem);
        if (*parser).m_protocolEncodingName.is_null() {
            return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
        }
    }
    return crate::expat_h::XML_STATUS_OK_0 as crate::expat_h::XML_Status;
}
/* Creates an XML_Parser object that can parse an external general
   entity; context is a '\0'-terminated string specifying the parse
   context; encoding is a '\0'-terminated string giving the name of
   the externally specified encoding, or NULL if there is no
   externally specified encoding.  The context string consists of a
   sequence of tokens separated by formfeeds (\f); a token consisting
   of a name specifies that the general entity of the name is open; a
   token of the form prefix=uri specifies the namespace for a
   particular prefix; a token of the form =uri specifies the default
   namespace.  This can be called at any point after the first call to
   an ExternalEntityRefHandler so longer as the parser has not yet
   been freed.  The new parser is completely independent and may
   safely be used in a separate thread.  The handlers and userData are
   initialized from the parser argument.  Returns NULL if out of memory.
   Otherwise returns a new XML_Parser object.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_ExternalEntityParserCreate(
    mut oldParser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    let mut parser: crate::expat_h::XML_Parser = oldParser;
    let mut newDtd: *mut DTD = crate::stddef_h::NULL as *mut DTD;
    let mut oldDtd: *mut DTD = 0 as *mut DTD;
    let mut oldStartElementHandler: crate::expat_h::XML_StartElementHandler = None;
    let mut oldEndElementHandler: crate::expat_h::XML_EndElementHandler = None;
    let mut oldCharacterDataHandler: crate::expat_h::XML_CharacterDataHandler = None;
    let mut oldProcessingInstructionHandler: crate::expat_h::XML_ProcessingInstructionHandler =
        None;
    let mut oldCommentHandler: crate::expat_h::XML_CommentHandler = None;
    let mut oldStartCdataSectionHandler: crate::expat_h::XML_StartCdataSectionHandler = None;
    let mut oldEndCdataSectionHandler: crate::expat_h::XML_EndCdataSectionHandler = None;
    let mut oldDefaultHandler: crate::expat_h::XML_DefaultHandler = None;
    let mut oldUnparsedEntityDeclHandler: crate::expat_h::XML_UnparsedEntityDeclHandler = None;
    let mut oldNotationDeclHandler: crate::expat_h::XML_NotationDeclHandler = None;
    let mut oldStartNamespaceDeclHandler: crate::expat_h::XML_StartNamespaceDeclHandler = None;
    let mut oldEndNamespaceDeclHandler: crate::expat_h::XML_EndNamespaceDeclHandler = None;
    let mut oldNotStandaloneHandler: crate::expat_h::XML_NotStandaloneHandler = None;
    let mut oldExternalEntityRefHandler: crate::expat_h::XML_ExternalEntityRefHandler = None;
    let mut oldSkippedEntityHandler: crate::expat_h::XML_SkippedEntityHandler = None;
    let mut oldUnknownEncodingHandler: crate::expat_h::XML_UnknownEncodingHandler = None;
    let mut oldElementDeclHandler: crate::expat_h::XML_ElementDeclHandler = None;
    let mut oldAttlistDeclHandler: crate::expat_h::XML_AttlistDeclHandler = None;
    let mut oldEntityDeclHandler: crate::expat_h::XML_EntityDeclHandler = None;
    let mut oldXmlDeclHandler: crate::expat_h::XML_XmlDeclHandler = None;
    let mut oldDeclElementType: *mut ELEMENT_TYPE = 0 as *mut ELEMENT_TYPE;
    let mut oldUserData: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut oldHandlerArg: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut oldDefaultExpandInternalEntities: crate::expat_h::XML_Bool = 0;
    let mut oldExternalEntityRefHandlerArg: crate::expat_h::XML_Parser = 0 as *mut XML_ParserStruct;
    let mut oldParamEntityParsing: crate::expat_h::XML_ParamEntityParsing =
        crate::expat_h::XML_PARAM_ENTITY_PARSING_NEVER;
    let mut oldInEntityValue: libc::c_int = 0;
    let mut oldns_triplets: crate::expat_h::XML_Bool = 0;
    /* Note that the new parser shares the same hash secret as the old
       parser, so that dtdCopy and copyEntityTable can lookup values
       from hash tables associated with either parser without us having
       to worry which hash secrets each table has.
    */
    let mut oldhash_secret_salt: libc::c_ulong = 0;
    /* Validate the oldParser parameter before we pull everything out of it */
    if oldParser.is_null() {
        return crate::stddef_h::NULL as crate::expat_h::XML_Parser;
    }
    /* Stash the original parser contents on the stack */
    oldDtd = (*parser).m_dtd;
    oldStartElementHandler = (*parser).m_startElementHandler;
    oldEndElementHandler = (*parser).m_endElementHandler;
    oldCharacterDataHandler = (*parser).m_characterDataHandler;
    oldProcessingInstructionHandler = (*parser).m_processingInstructionHandler;
    oldCommentHandler = (*parser).m_commentHandler;
    oldStartCdataSectionHandler = (*parser).m_startCdataSectionHandler;
    oldEndCdataSectionHandler = (*parser).m_endCdataSectionHandler;
    oldDefaultHandler = (*parser).m_defaultHandler;
    oldUnparsedEntityDeclHandler = (*parser).m_unparsedEntityDeclHandler;
    oldNotationDeclHandler = (*parser).m_notationDeclHandler;
    oldStartNamespaceDeclHandler = (*parser).m_startNamespaceDeclHandler;
    oldEndNamespaceDeclHandler = (*parser).m_endNamespaceDeclHandler;
    oldNotStandaloneHandler = (*parser).m_notStandaloneHandler;
    oldExternalEntityRefHandler = (*parser).m_externalEntityRefHandler;
    oldSkippedEntityHandler = (*parser).m_skippedEntityHandler;
    oldUnknownEncodingHandler = (*parser).m_unknownEncodingHandler;
    oldElementDeclHandler = (*parser).m_elementDeclHandler;
    oldAttlistDeclHandler = (*parser).m_attlistDeclHandler;
    oldEntityDeclHandler = (*parser).m_entityDeclHandler;
    oldXmlDeclHandler = (*parser).m_xmlDeclHandler;
    oldDeclElementType = (*parser).m_declElementType;
    oldUserData = (*parser).m_userData;
    oldHandlerArg = (*parser).m_handlerArg;
    oldDefaultExpandInternalEntities = (*parser).m_defaultExpandInternalEntities;
    oldExternalEntityRefHandlerArg = (*parser).m_externalEntityRefHandlerArg;
    oldParamEntityParsing = (*parser).m_paramEntityParsing;
    oldInEntityValue = (*parser).m_prologState.inEntityValue;
    oldns_triplets = (*parser).m_ns_triplets;
    /* Note that the new parser shares the same hash secret as the old
       parser, so that dtdCopy and copyEntityTable can lookup values
       from hash tables associated with either parser without us having
       to worry which hash secrets each table has.
    */
    oldhash_secret_salt = (*parser).m_hash_secret_salt;
    if context.is_null() {
        newDtd = oldDtd
    }
    /* XML_DTD */
    /* Note that the magical uses of the pre-processor to make field
       access look more like C++ require that `parser' be overwritten
       here.  This makes this function more painful to follow than it
       would be otherwise.
    */
    if (*parser).m_ns != 0 {
        let mut tmp: [crate::expat_external_h::XML_Char; 2] = [0; 2];
        *tmp.as_mut_ptr() = (*parser).m_namespaceSeparator;
        parser = parserCreate(encodingName, &(*parser).m_mem, tmp.as_mut_ptr(), newDtd)
    } else {
        parser = parserCreate(
            encodingName,
            &(*parser).m_mem,
            crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char,
            newDtd,
        )
    }
    if parser.is_null() {
        return crate::stddef_h::NULL as crate::expat_h::XML_Parser;
    }
    (*parser).m_startElementHandler = oldStartElementHandler;
    (*parser).m_endElementHandler = oldEndElementHandler;
    (*parser).m_characterDataHandler = oldCharacterDataHandler;
    (*parser).m_processingInstructionHandler = oldProcessingInstructionHandler;
    (*parser).m_commentHandler = oldCommentHandler;
    (*parser).m_startCdataSectionHandler = oldStartCdataSectionHandler;
    (*parser).m_endCdataSectionHandler = oldEndCdataSectionHandler;
    (*parser).m_defaultHandler = oldDefaultHandler;
    (*parser).m_unparsedEntityDeclHandler = oldUnparsedEntityDeclHandler;
    (*parser).m_notationDeclHandler = oldNotationDeclHandler;
    (*parser).m_startNamespaceDeclHandler = oldStartNamespaceDeclHandler;
    (*parser).m_endNamespaceDeclHandler = oldEndNamespaceDeclHandler;
    (*parser).m_notStandaloneHandler = oldNotStandaloneHandler;
    (*parser).m_externalEntityRefHandler = oldExternalEntityRefHandler;
    (*parser).m_skippedEntityHandler = oldSkippedEntityHandler;
    (*parser).m_unknownEncodingHandler = oldUnknownEncodingHandler;
    (*parser).m_elementDeclHandler = oldElementDeclHandler;
    (*parser).m_attlistDeclHandler = oldAttlistDeclHandler;
    (*parser).m_entityDeclHandler = oldEntityDeclHandler;
    (*parser).m_xmlDeclHandler = oldXmlDeclHandler;
    (*parser).m_declElementType = oldDeclElementType;
    (*parser).m_userData = oldUserData;
    if oldUserData == oldHandlerArg {
        (*parser).m_handlerArg = (*parser).m_userData
    } else {
        (*parser).m_handlerArg = parser as *mut libc::c_void
    }
    if oldExternalEntityRefHandlerArg != oldParser {
        (*parser).m_externalEntityRefHandlerArg = oldExternalEntityRefHandlerArg
    }
    (*parser).m_defaultExpandInternalEntities = oldDefaultExpandInternalEntities;
    (*parser).m_ns_triplets = oldns_triplets;
    (*parser).m_hash_secret_salt = oldhash_secret_salt;
    (*parser).m_parentParser = oldParser;
    (*parser).m_paramEntityParsing = oldParamEntityParsing;
    (*parser).m_prologState.inEntityValue = oldInEntityValue;
    if !context.is_null() {
        /* XML_DTD */
        if dtdCopy(oldParser, (*parser).m_dtd, oldDtd, &(*parser).m_mem) == 0
            || setContext(parser, context) == 0
        {
            XML_ParserFree(parser);
            return crate::stddef_h::NULL as crate::expat_h::XML_Parser;
        }
        (*parser).m_processor = Some(externalEntityInitProcessor as Processor)
    } else {
        /* The DTD instance referenced by parser->m_dtd is shared between the
           document's root parser and external PE parsers, therefore one does not
           need to call setContext. In addition, one also *must* not call
           setContext, because this would overwrite existing prefix->binding
           pointers in parser->m_dtd with ones that get destroyed with the external
           PE parser. This would leave those prefixes with dangling pointers.
        */
        (*parser).m_isParamEntity = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
        crate::src::lib::xmlrole::XmlPrologStateInitExternalEntity(
            &mut (*parser).m_prologState as *mut _ as *mut crate::src::lib::xmlrole::prolog_state,
        );
        (*parser).m_processor = Some(externalParEntInitProcessor as Processor)
    }
    /* XML_DTD */
    return parser;
}

unsafe extern "C" fn destroyBindings(
    mut bindings: *mut BINDING,
    mut parser: crate::expat_h::XML_Parser,
) {
    loop {
        let mut b: *mut BINDING = bindings;
        if b.is_null() {
            break;
        }
        bindings = (*b).nextTagBinding;
        (*parser).m_mem.free_fcn.expect("non-null function pointer")((*b).uri as *mut libc::c_void);
        (*parser).m_mem.free_fcn.expect("non-null function pointer")(b as *mut libc::c_void);
    }
}
/* Frees memory used by the parser. */
#[no_mangle]

pub unsafe extern "C" fn XML_ParserFree(mut parser: crate::expat_h::XML_Parser) {
    let mut tagList: *mut TAG = 0 as *mut TAG;
    let mut entityList: *mut OPEN_INTERNAL_ENTITY = 0 as *mut OPEN_INTERNAL_ENTITY;
    if parser.is_null() {
        return;
    }
    /* free m_tagStack and m_freeTagList */
    tagList = (*parser).m_tagStack;
    loop {
        let mut p: *mut TAG = 0 as *mut TAG;
        if tagList.is_null() {
            if (*parser).m_freeTagList.is_null() {
                break;
            }
            tagList = (*parser).m_freeTagList;
            (*parser).m_freeTagList = crate::stddef_h::NULL as *mut TAG
        }
        p = tagList;
        tagList = (*tagList).parent;
        (*parser).m_mem.free_fcn.expect("non-null function pointer")((*p).buf as *mut libc::c_void);
        destroyBindings((*p).bindings, parser);
        (*parser).m_mem.free_fcn.expect("non-null function pointer")(p as *mut libc::c_void);
    }
    /* free m_openInternalEntities and m_freeInternalEntities */
    entityList = (*parser).m_openInternalEntities;
    loop {
        let mut openEntity: *mut OPEN_INTERNAL_ENTITY = 0 as *mut OPEN_INTERNAL_ENTITY;
        if entityList.is_null() {
            if (*parser).m_freeInternalEntities.is_null() {
                break;
            }
            entityList = (*parser).m_freeInternalEntities;
            (*parser).m_freeInternalEntities = crate::stddef_h::NULL as *mut OPEN_INTERNAL_ENTITY
        }
        openEntity = entityList;
        entityList = (*entityList).next;
        (*parser).m_mem.free_fcn.expect("non-null function pointer")(
            openEntity as *mut libc::c_void,
        );
    }
    destroyBindings((*parser).m_freeBindingList, parser);
    destroyBindings((*parser).m_inheritedBindings, parser);
    poolDestroy(&mut (*parser).m_tempPool);
    poolDestroy(&mut (*parser).m_temp2Pool);
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(
        (*parser).m_protocolEncodingName as *mut libc::c_void,
    );
    /* external parameter entity parsers share the DTD structure
       parser->m_dtd with the root parser, so we must not destroy it
    */
    if (*parser).m_isParamEntity == 0 && !(*parser).m_dtd.is_null() {
        /* XML_DTD */
        dtdDestroy(
            (*parser).m_dtd,
            (*parser).m_parentParser.is_null() as libc::c_int as crate::expat_h::XML_Bool,
            &(*parser).m_mem,
        );
    }
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(
        (*parser).m_atts as *mut libc::c_void,
    );
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(
        (*parser).m_groupConnector as *mut libc::c_void,
    );
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(
        (*parser).m_buffer as *mut libc::c_void,
    );
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(
        (*parser).m_dataBuf as *mut libc::c_void,
    );
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(
        (*parser).m_nsAtts as *mut libc::c_void,
    );
    (*parser).m_mem.free_fcn.expect("non-null function pointer")((*parser).m_unknownEncodingMem);
    if (*parser).m_unknownEncodingRelease.is_some() {
        (*parser)
            .m_unknownEncodingRelease
            .expect("non-null function pointer")((*parser).m_unknownEncodingData);
    }
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(parser as *mut libc::c_void);
}
/* If this function is called, then the parser will be passed as the
   first argument to callbacks instead of userData.  The userData will
   still be accessible using XML_GetUserData.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_UseParserAsHandlerArg(mut parser: crate::expat_h::XML_Parser) {
    if !parser.is_null() {
        (*parser).m_handlerArg = parser as *mut libc::c_void
    };
}
/* If useDTD == XML_TRUE is passed to this function, then the parser
   will assume that there is an external subset, even if none is
   specified in the document. In such a case the parser will call the
   externalEntityRefHandler with a value of NULL for the systemId
   argument (the publicId and context arguments will be NULL as well).
   Note: For the purpose of checking WFC: Entity Declared, passing
     useDTD == XML_TRUE will make the parser behave as if the document
     had a DTD with an external subset.
   Note: If this function is called, then this must be done before
     the first call to XML_Parse or XML_ParseBuffer, since it will
     have no effect after that.  Returns
     XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING.
   Note: If the document does not have a DOCTYPE declaration at all,
     then startDoctypeDeclHandler and endDoctypeDeclHandler will not
     be called, despite an external subset being parsed.
   Note: If XML_DTD is not defined when Expat is compiled, returns
     XML_ERROR_FEATURE_REQUIRES_XML_DTD.
   Note: If parser == NULL, returns XML_ERROR_INVALID_ARGUMENT.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_UseForeignDTD(
    mut parser: crate::expat_h::XML_Parser,
    mut useDTD: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Error {
    if parser.is_null() {
        return crate::expat_h::XML_ERROR_INVALID_ARGUMENT;
    }
    /* block after XML_Parse()/XML_ParseBuffer() has been called */
    if (*parser).m_parsingStatus.parsing as libc::c_uint
        == crate::expat_h::XML_PARSING as libc::c_int as libc::c_uint
        || (*parser).m_parsingStatus.parsing as libc::c_uint
            == crate::expat_h::XML_SUSPENDED as libc::c_int as libc::c_uint
    {
        return crate::expat_h::XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING;
    }
    (*parser).m_useForeignDTD = useDTD;
    return crate::expat_h::XML_ERROR_NONE;
}
/* If do_nst is non-zero, and namespace processing is in effect, and
   a name has a prefix (i.e. an explicit namespace qualifier) then
   that name is returned as a triplet in a single string separated by
   the separator character specified when the parser was created: URI
   + sep + local_name + sep + prefix.

   If do_nst is zero, then namespace information is returned in the
   default manner (URI + sep + local_name) whether or not the name
   has a prefix.

   Note: Calling XML_SetReturnNSTriplet after XML_Parse or
     XML_ParseBuffer has no effect.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_SetReturnNSTriplet(
    mut parser: crate::expat_h::XML_Parser,
    mut do_nst: libc::c_int,
) {
    if parser.is_null() {
        return;
    }
    /* block after XML_Parse()/XML_ParseBuffer() has been called */
    if (*parser).m_parsingStatus.parsing as libc::c_uint
        == crate::expat_h::XML_PARSING as libc::c_int as libc::c_uint
        || (*parser).m_parsingStatus.parsing as libc::c_uint
            == crate::expat_h::XML_SUSPENDED as libc::c_int as libc::c_uint
    {
        return;
    }
    (*parser).m_ns_triplets = if do_nst != 0 {
        crate::expat_h::XML_TRUE
    } else {
        crate::expat_h::XML_FALSE
    } as crate::expat_h::XML_Bool;
}
/* This value is passed as the userData argument to callbacks. */
#[no_mangle]

pub unsafe extern "C" fn XML_SetUserData(
    mut parser: crate::expat_h::XML_Parser,
    mut p: *mut libc::c_void,
) {
    if parser.is_null() {
        return;
    }
    if (*parser).m_handlerArg == (*parser).m_userData {
        (*parser).m_userData = p;
        (*parser).m_handlerArg = (*parser).m_userData
    } else {
        (*parser).m_userData = p
    };
}
/* Sets the base to be used for resolving relative URIs in system
   identifiers in declarations.  Resolving relative identifiers is
   left to the application: this value will be passed through as the
   base argument to the XML_ExternalEntityRefHandler,
   XML_NotationDeclHandler and XML_UnparsedEntityDeclHandler. The base
   argument will be copied.  Returns XML_STATUS_ERROR if out of memory,
   XML_STATUS_OK otherwise.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_SetBase(
    mut parser: crate::expat_h::XML_Parser,
    mut p: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Status {
    if parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
    }
    if !p.is_null() {
        p = poolCopyString(&mut (*(*parser).m_dtd).pool, p);
        if p.is_null() {
            return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
        }
        (*parser).m_curBase = p
    } else {
        (*parser).m_curBase = crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char
    }
    return crate::expat_h::XML_STATUS_OK_0 as crate::expat_h::XML_Status;
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetBase(
    mut parser: crate::expat_h::XML_Parser,
) -> *const crate::expat_external_h::XML_Char {
    if parser.is_null() {
        return crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    }
    return (*parser).m_curBase;
}
/* Returns the number of the attribute/value pairs passed in last call
   to the XML_StartElementHandler that were specified in the start-tag
   rather than defaulted. Each attribute/value pair counts as 2; thus
   this correspondds to an index into the atts array passed to the
   XML_StartElementHandler.  Returns -1 if parser == NULL.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_GetSpecifiedAttributeCount(
    mut parser: crate::expat_h::XML_Parser,
) -> libc::c_int {
    if parser.is_null() {
        return -(1 as libc::c_int);
    }
    return (*parser).m_nSpecifiedAtts;
}
/* Returns the index of the ID attribute passed in the last call to
   XML_StartElementHandler, or -1 if there is no ID attribute or
   parser == NULL.  Each attribute/value pair counts as 2; thus this
   correspondds to an index into the atts array passed to the
   XML_StartElementHandler.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_GetIdAttributeIndex(
    mut parser: crate::expat_h::XML_Parser,
) -> libc::c_int {
    if parser.is_null() {
        return -(1 as libc::c_int);
    }
    return (*parser).m_idAttIndex;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetElementHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartElementHandler,
    mut end: crate::expat_h::XML_EndElementHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startElementHandler = start;
    (*parser).m_endElementHandler = end;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetStartElementHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartElementHandler,
) {
    if !parser.is_null() {
        (*parser).m_startElementHandler = start
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetEndElementHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndElementHandler,
) {
    if !parser.is_null() {
        (*parser).m_endElementHandler = end
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetCharacterDataHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_CharacterDataHandler,
) {
    if !parser.is_null() {
        (*parser).m_characterDataHandler = handler
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetProcessingInstructionHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_ProcessingInstructionHandler,
) {
    if !parser.is_null() {
        (*parser).m_processingInstructionHandler = handler
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetCommentHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_CommentHandler,
) {
    if !parser.is_null() {
        (*parser).m_commentHandler = handler
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetCdataSectionHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartCdataSectionHandler,
    mut end: crate::expat_h::XML_EndCdataSectionHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startCdataSectionHandler = start;
    (*parser).m_endCdataSectionHandler = end;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetStartCdataSectionHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartCdataSectionHandler,
) {
    if !parser.is_null() {
        (*parser).m_startCdataSectionHandler = start
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetEndCdataSectionHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndCdataSectionHandler,
) {
    if !parser.is_null() {
        (*parser).m_endCdataSectionHandler = end
    };
}
/* This sets the default handler and also inhibits expansion of
   internal entities. These entity references will be passed to the
   default handler, or to the skipped entity handler, if one is set.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_SetDefaultHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_DefaultHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_defaultHandler = handler;
    (*parser).m_defaultExpandInternalEntities =
        crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
}
/* This sets the default handler but does not inhibit expansion of
   internal entities.  The entity reference will not be passed to the
   default handler.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_SetDefaultHandlerExpand(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_DefaultHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_defaultHandler = handler;
    (*parser).m_defaultExpandInternalEntities =
        crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetDoctypeDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartDoctypeDeclHandler,
    mut end: crate::expat_h::XML_EndDoctypeDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startDoctypeDeclHandler = start;
    (*parser).m_endDoctypeDeclHandler = end;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetStartDoctypeDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartDoctypeDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_startDoctypeDeclHandler = start
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetEndDoctypeDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndDoctypeDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_endDoctypeDeclHandler = end
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetUnparsedEntityDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_UnparsedEntityDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_unparsedEntityDeclHandler = handler
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetNotationDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_NotationDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_notationDeclHandler = handler
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetNamespaceDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartNamespaceDeclHandler,
    mut end: crate::expat_h::XML_EndNamespaceDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startNamespaceDeclHandler = start;
    (*parser).m_endNamespaceDeclHandler = end;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetStartNamespaceDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartNamespaceDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_startNamespaceDeclHandler = start
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetEndNamespaceDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndNamespaceDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_endNamespaceDeclHandler = end
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetNotStandaloneHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_NotStandaloneHandler,
) {
    if !parser.is_null() {
        (*parser).m_notStandaloneHandler = handler
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetExternalEntityRefHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_ExternalEntityRefHandler,
) {
    if !parser.is_null() {
        (*parser).m_externalEntityRefHandler = handler
    };
}
/* If a non-NULL value for arg is specified here, then it will be
   passed as the first argument to the external entity ref handler
   instead of the parser object.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_SetExternalEntityRefHandlerArg(
    mut parser: crate::expat_h::XML_Parser,
    mut arg: *mut libc::c_void,
) {
    if parser.is_null() {
        return;
    }
    if !arg.is_null() {
        (*parser).m_externalEntityRefHandlerArg = arg as crate::expat_h::XML_Parser
    } else {
        (*parser).m_externalEntityRefHandlerArg = parser
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetSkippedEntityHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_SkippedEntityHandler,
) {
    if !parser.is_null() {
        (*parser).m_skippedEntityHandler = handler
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetUnknownEncodingHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_UnknownEncodingHandler,
    mut data: *mut libc::c_void,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_unknownEncodingHandler = handler;
    (*parser).m_unknownEncodingHandlerData = data;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetElementDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut eldecl: crate::expat_h::XML_ElementDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_elementDeclHandler = eldecl
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetAttlistDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut attdecl: crate::expat_h::XML_AttlistDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_attlistDeclHandler = attdecl
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetEntityDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_EntityDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_entityDeclHandler = handler
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetXmlDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_XmlDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_xmlDeclHandler = handler
    };
}
/* Controls parsing of parameter entities (including the external DTD
   subset). If parsing of parameter entities is enabled, then
   references to external parameter entities (including the external
   DTD subset) will be passed to the handler set with
   XML_SetExternalEntityRefHandler.  The context passed will be 0.

   Unlike external general entities, external parameter entities can
   only be parsed synchronously.  If the external parameter entity is
   to be parsed, it must be parsed during the call to the external
   entity ref handler: the complete sequence of
   XML_ExternalEntityParserCreate, XML_Parse/XML_ParseBuffer and
   XML_ParserFree calls must be made during this call.  After
   XML_ExternalEntityParserCreate has been called to create the parser
   for the external parameter entity (context must be 0 for this
   call), it is illegal to make any calls on the old parser until
   XML_ParserFree has been called on the newly created parser.
   If the library has been compiled without support for parameter
   entity parsing (ie without XML_DTD being defined), then
   XML_SetParamEntityParsing will return 0 if parsing of parameter
   entities is requested; otherwise it will return non-zero.
   Note: If XML_SetParamEntityParsing is called after XML_Parse or
      XML_ParseBuffer, then it has no effect and will always return 0.
   Note: If parser == NULL, the function will do nothing and return 0.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_SetParamEntityParsing(
    mut parser: crate::expat_h::XML_Parser,
    mut peParsing: crate::expat_h::XML_ParamEntityParsing,
) -> libc::c_int {
    if parser.is_null() {
        return 0 as libc::c_int;
    }
    /* block after XML_Parse()/XML_ParseBuffer() has been called */
    if (*parser).m_parsingStatus.parsing as libc::c_uint
        == crate::expat_h::XML_PARSING as libc::c_int as libc::c_uint
        || (*parser).m_parsingStatus.parsing as libc::c_uint
            == crate::expat_h::XML_SUSPENDED as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    (*parser).m_paramEntityParsing = peParsing;
    return 1 as libc::c_int;
}
/* Sets the hash salt to use for internal hash calculations.
   Helps in preventing DoS attacks based on predicting hash
   function behavior. This must be called before parsing is started.
   Returns 1 if successful, 0 when called after parsing has started.
   Note: If parser == NULL, the function will do nothing and return 0.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_SetHashSalt(
    mut parser: crate::expat_h::XML_Parser,
    mut hash_salt: libc::c_ulong,
) -> libc::c_int {
    if parser.is_null() {
        return 0 as libc::c_int;
    }
    if !(*parser).m_parentParser.is_null() {
        return XML_SetHashSalt((*parser).m_parentParser, hash_salt);
    }
    /* block after XML_Parse()/XML_ParseBuffer() has been called */
    if (*parser).m_parsingStatus.parsing as libc::c_uint
        == crate::expat_h::XML_PARSING as libc::c_int as libc::c_uint
        || (*parser).m_parsingStatus.parsing as libc::c_uint
            == crate::expat_h::XML_SUSPENDED as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    (*parser).m_hash_secret_salt = hash_salt;
    return 1 as libc::c_int;
}
/* Parses some input. Returns XML_STATUS_ERROR if a fatal error is
   detected.  The last call to XML_Parse must have isFinal true; len
   may be zero for this call (or any other).

   Though the return values for these functions has always been
   described as a Boolean value, the implementation, at least for the
   1.95.x series, has always returned exactly one of the XML_Status
   values.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_Parse(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const libc::c_char,
    mut len: libc::c_int,
    mut isFinal: libc::c_int,
) -> crate::expat_h::XML_Status {
    if parser.is_null() || len < 0 as libc::c_int || s.is_null() && len != 0 as libc::c_int {
        if !parser.is_null() {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_INVALID_ARGUMENT
        }
        return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
    }
    match (*parser).m_parsingStatus.parsing as libc::c_uint {
        3 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_SUSPENDED;
            return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
        }
        2 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_FINISHED;
            return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
        }
        0 => {
            if (*parser).m_parentParser.is_null() && startParsing(parser) == 0 {
                (*parser).m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
                return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
            }
        }
        _ => {}
    }
    /* fall through */
    (*parser).m_parsingStatus.parsing = crate::expat_h::XML_PARSING;
    if len == 0 as libc::c_int {
        (*parser).m_parsingStatus.finalBuffer = isFinal as crate::expat_h::XML_Bool;
        if isFinal == 0 {
            return crate::expat_h::XML_STATUS_OK_0 as crate::expat_h::XML_Status;
        }
        (*parser).m_positionPtr = (*parser).m_bufferPtr;
        (*parser).m_parseEndPtr = (*parser).m_bufferEnd;
        /* If data are left over from last buffer, and we now know that these
           data are the final chunk of input, then we have to check them again
           to detect errors based on that fact.
        */
        (*parser).m_errorCode = (*parser).m_processor.expect("non-null function pointer")(
            parser,
            (*parser).m_bufferPtr,
            (*parser).m_parseEndPtr,
            &mut (*parser).m_bufferPtr,
        );
        if (*parser).m_errorCode as libc::c_uint
            == crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint
        {
            match (*parser).m_parsingStatus.parsing as libc::c_uint {
                3 => {
                    /* It is hard to be certain, but it seems that this case
                     * cannot occur.  This code is cleaning up a previous parse
                     * with no new data (since len == 0).  Changing the parsing
                     * state requires getting to execute a handler function, and
                     * there doesn't seem to be an opportunity for that while in
                     * this circumstance.
                     *
                     * Given the uncertainty, we retain the code but exclude it
                     * from coverage tests.
                     *
                     * LCOV_EXCL_START
                     */
                    (*(*parser).m_encoding)
                        .updatePosition
                        .expect("non-null function pointer")(
                        (*parser).m_encoding,
                        (*parser).m_positionPtr,
                        (*parser).m_bufferPtr,
                        &mut (*parser).m_position,
                    );
                    (*parser).m_positionPtr = (*parser).m_bufferPtr;
                    return crate::expat_h::XML_STATUS_SUSPENDED_0 as crate::expat_h::XML_Status;
                }
                0 | 1 => {
                    /* LCOV_EXCL_STOP */
                    (*parser).m_parsingStatus.parsing = crate::expat_h::XML_FINISHED
                }
                _ => {}
            }
            /* fall through */
            return crate::expat_h::XML_STATUS_OK_0 as crate::expat_h::XML_Status;
        }
        (*parser).m_eventEndPtr = (*parser).m_eventPtr;
        (*parser).m_processor = Some(errorProcessor as Processor);
        return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
    } else {
        /* not defined XML_CONTEXT_BYTES */
        let mut buff: *mut libc::c_void = XML_GetBuffer(parser, len);
        if buff.is_null() {
            return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
        } else {
            crate::stdlib::memcpy(buff, s as *const libc::c_void, len as libc::c_ulong);
            return XML_ParseBuffer(parser, len, isFinal);
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_ParseBuffer(
    mut parser: crate::expat_h::XML_Parser,
    mut len: libc::c_int,
    mut isFinal: libc::c_int,
) -> crate::expat_h::XML_Status {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut result: crate::expat_h::XML_Status =
        crate::expat_h::XML_STATUS_OK_0 as crate::expat_h::XML_Status;
    if parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
    }
    match (*parser).m_parsingStatus.parsing as libc::c_uint {
        3 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_SUSPENDED;
            return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
        }
        2 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_FINISHED;
            return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
        }
        0 => {
            if (*parser).m_parentParser.is_null() && startParsing(parser) == 0 {
                (*parser).m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
                return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
            }
        }
        _ => {}
    }
    /* fall through */
    (*parser).m_parsingStatus.parsing = crate::expat_h::XML_PARSING;
    start = (*parser).m_bufferPtr;
    (*parser).m_positionPtr = start;
    (*parser).m_bufferEnd = (*parser).m_bufferEnd.offset(len as isize);
    (*parser).m_parseEndPtr = (*parser).m_bufferEnd;
    (*parser).m_parseEndByteIndex += len as libc::c_long;
    (*parser).m_parsingStatus.finalBuffer = isFinal as crate::expat_h::XML_Bool;
    (*parser).m_errorCode = (*parser).m_processor.expect("non-null function pointer")(
        parser,
        start,
        (*parser).m_parseEndPtr,
        &mut (*parser).m_bufferPtr,
    );
    if (*parser).m_errorCode as libc::c_uint
        != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint
    {
        (*parser).m_eventEndPtr = (*parser).m_eventPtr;
        (*parser).m_processor = Some(errorProcessor as Processor);
        return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
    } else {
        match (*parser).m_parsingStatus.parsing as libc::c_uint {
            3 => {
                result = crate::expat_h::XML_STATUS_SUSPENDED_0 as crate::expat_h::XML_Status
                /* should not happen */
            }
            0 | 1 => {
                if isFinal != 0 {
                    (*parser).m_parsingStatus.parsing = crate::expat_h::XML_FINISHED;
                    return result;
                }
            }
            _ => {}
        }
    }
    (*(*parser).m_encoding)
        .updatePosition
        .expect("non-null function pointer")(
        (*parser).m_encoding,
        (*parser).m_positionPtr,
        (*parser).m_bufferPtr,
        &mut (*parser).m_position,
    );
    (*parser).m_positionPtr = (*parser).m_bufferPtr;
    return result;
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetBuffer(
    mut parser: crate::expat_h::XML_Parser,
    mut len: libc::c_int,
) -> *mut libc::c_void {
    if parser.is_null() {
        return crate::stddef_h::NULL as *mut libc::c_void;
    }
    if len < 0 as libc::c_int {
        (*parser).m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
        return crate::stddef_h::NULL as *mut libc::c_void;
    }
    match (*parser).m_parsingStatus.parsing as libc::c_uint {
        3 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_SUSPENDED;
            return crate::stddef_h::NULL as *mut libc::c_void;
        }
        2 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_FINISHED;
            return crate::stddef_h::NULL as *mut libc::c_void;
        }
        _ => {}
    }
    if len as libc::c_long
        > (if !(*parser).m_bufferLim.is_null() && !(*parser).m_bufferEnd.is_null() {
            (*parser)
                .m_bufferLim
                .wrapping_offset_from((*parser).m_bufferEnd) as libc::c_long
        } else {
            0 as libc::c_int as libc::c_long
        })
    {
        let mut keep: libc::c_int = 0;
        /* defined XML_CONTEXT_BYTES */
        /* Do not invoke signed arithmetic overflow: */
        let mut neededSize: libc::c_int = (len as libc::c_uint).wrapping_add(
            (if !(*parser).m_bufferEnd.is_null() && !(*parser).m_bufferPtr.is_null() {
                (*parser)
                    .m_bufferEnd
                    .wrapping_offset_from((*parser).m_bufferPtr) as libc::c_long
            } else {
                0 as libc::c_int as libc::c_long
            }) as libc::c_uint,
        ) as libc::c_int;
        if neededSize < 0 as libc::c_int {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
            return crate::stddef_h::NULL as *mut libc::c_void;
        }
        keep = if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
            (*parser)
                .m_bufferPtr
                .wrapping_offset_from((*parser).m_buffer) as libc::c_long
        } else {
            0 as libc::c_int as libc::c_long
        } as libc::c_int;
        if keep > crate::expat_config_h::XML_CONTEXT_BYTES {
            keep = crate::expat_config_h::XML_CONTEXT_BYTES
        }
        neededSize += keep;
        /* defined XML_CONTEXT_BYTES */
        if neededSize as libc::c_long
            <= (if !(*parser).m_bufferLim.is_null() && !(*parser).m_buffer.is_null() {
                (*parser)
                    .m_bufferLim
                    .wrapping_offset_from((*parser).m_buffer) as libc::c_long
            } else {
                0 as libc::c_int as libc::c_long
            })
        {
            if (keep as libc::c_long)
                < (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
                    (*parser)
                        .m_bufferPtr
                        .wrapping_offset_from((*parser).m_buffer)
                        as libc::c_long
                } else {
                    0 as libc::c_int as libc::c_long
                })
            {
                let mut offset: libc::c_int =
                    (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
                        (*parser)
                            .m_bufferPtr
                            .wrapping_offset_from((*parser).m_buffer)
                            as libc::c_long
                    } else {
                        0 as libc::c_int as libc::c_long
                    }) as libc::c_int
                        - keep;
                /* The buffer pointers cannot be NULL here; we have at least some bytes
                 * in the buffer */
                crate::stdlib::memmove(
                    (*parser).m_buffer as *mut libc::c_void,
                    &mut *(*parser).m_buffer.offset(offset as isize) as *mut libc::c_char
                        as *const libc::c_void,
                    ((*parser)
                        .m_bufferEnd
                        .wrapping_offset_from((*parser).m_bufferPtr)
                        as libc::c_long
                        + keep as libc::c_long) as libc::c_ulong,
                );
                (*parser).m_bufferEnd = (*parser).m_bufferEnd.offset(-(offset as isize));
                (*parser).m_bufferPtr = (*parser).m_bufferPtr.offset(-(offset as isize))
            }
        /* not defined XML_CONTEXT_BYTES */
        } else {
            let mut newBuf: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut bufferSize: libc::c_int = if !(*parser).m_bufferLim.is_null()
                && !(*parser).m_bufferPtr.is_null()
            {
                (*parser)
                    .m_bufferLim
                    .wrapping_offset_from((*parser).m_bufferPtr) as libc::c_long
            } else {
                0 as libc::c_int as libc::c_long
            } as libc::c_int;
            if bufferSize == 0 as libc::c_int {
                bufferSize = INIT_BUFFER_SIZE
            }
            loop {
                /* not defined XML_CONTEXT_BYTES */
                /* Do not invoke signed arithmetic overflow: */
                bufferSize =
                    (2 as libc::c_uint).wrapping_mul(bufferSize as libc::c_uint) as libc::c_int;
                if !(bufferSize < neededSize && bufferSize > 0 as libc::c_int) {
                    break;
                }
            }
            if bufferSize <= 0 as libc::c_int {
                (*parser).m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
                return crate::stddef_h::NULL as *mut libc::c_void;
            }
            newBuf = (*parser)
                .m_mem
                .malloc_fcn
                .expect("non-null function pointer")(
                bufferSize as crate::stddef_h::size_t
            ) as *mut libc::c_char;
            if newBuf.is_null() {
                (*parser).m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
                return crate::stddef_h::NULL as *mut libc::c_void;
            }
            (*parser).m_bufferLim = newBuf.offset(bufferSize as isize);
            if !(*parser).m_bufferPtr.is_null() {
                crate::stdlib::memcpy(
                    newBuf as *mut libc::c_void,
                    &*(*parser).m_bufferPtr.offset(-keep as isize) as *const libc::c_char
                        as *const libc::c_void,
                    ((if !(*parser).m_bufferEnd.is_null() && !(*parser).m_bufferPtr.is_null() {
                        (*parser)
                            .m_bufferEnd
                            .wrapping_offset_from((*parser).m_bufferPtr)
                            as libc::c_long
                    } else {
                        0 as libc::c_int as libc::c_long
                    }) + keep as libc::c_long) as libc::c_ulong,
                );
                (*parser).m_mem.free_fcn.expect("non-null function pointer")(
                    (*parser).m_buffer as *mut libc::c_void,
                );
                (*parser).m_buffer = newBuf;
                (*parser).m_bufferEnd = (*parser)
                    .m_buffer
                    .offset(
                        (if !(*parser).m_bufferEnd.is_null() && !(*parser).m_bufferPtr.is_null() {
                            (*parser)
                                .m_bufferEnd
                                .wrapping_offset_from((*parser).m_bufferPtr)
                                as libc::c_long
                        } else {
                            0 as libc::c_int as libc::c_long
                        }) as isize,
                    )
                    .offset(keep as isize);
                (*parser).m_bufferPtr = (*parser).m_buffer.offset(keep as isize)
            } else {
                /* This must be a brand new buffer with no data in it yet */
                (*parser).m_bufferEnd = newBuf;
                (*parser).m_buffer = newBuf;
                (*parser).m_bufferPtr = (*parser).m_buffer
            }
        }
        (*parser).m_eventEndPtr = crate::stddef_h::NULL as *const libc::c_char;
        (*parser).m_eventPtr = (*parser).m_eventEndPtr;
        (*parser).m_positionPtr = crate::stddef_h::NULL as *const libc::c_char
    }
    return (*parser).m_bufferEnd as *mut libc::c_void;
}
/* Stops parsing, causing XML_Parse() or XML_ParseBuffer() to return.
   Must be called from within a call-back handler, except when aborting
   (resumable = 0) an already suspended parser. Some call-backs may
   still follow because they would otherwise get lost. Examples:
   - endElementHandler() for empty elements when stopped in
     startElementHandler(),
   - endNameSpaceDeclHandler() when stopped in endElementHandler(),
   and possibly others.

   Can be called from most handlers, including DTD related call-backs,
   except when parsing an external parameter entity and resumable != 0.
   Returns XML_STATUS_OK when successful, XML_STATUS_ERROR otherwise.
   Possible error codes:
   - XML_ERROR_SUSPENDED: when suspending an already suspended parser.
   - XML_ERROR_FINISHED: when the parser has already finished.
   - XML_ERROR_SUSPEND_PE: when suspending while parsing an external PE.

   When resumable != 0 (true) then parsing is suspended, that is,
   XML_Parse() and XML_ParseBuffer() return XML_STATUS_SUSPENDED.
   Otherwise, parsing is aborted, that is, XML_Parse() and XML_ParseBuffer()
   return XML_STATUS_ERROR with error code XML_ERROR_ABORTED.

   *Note*:
   This will be applied to the current parser instance only, that is, if
   there is a parent parser then it will continue parsing when the
   externalEntityRefHandler() returns. It is up to the implementation of
   the externalEntityRefHandler() to call XML_StopParser() on the parent
   parser (recursively), if one wants to stop parsing altogether.

   When suspended, parsing can be resumed by calling XML_ResumeParser().
*/
#[no_mangle]

pub unsafe extern "C" fn XML_StopParser(
    mut parser: crate::expat_h::XML_Parser,
    mut resumable: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Status {
    if parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
    }
    match (*parser).m_parsingStatus.parsing as libc::c_uint {
        3 => {
            if resumable != 0 {
                (*parser).m_errorCode = crate::expat_h::XML_ERROR_SUSPENDED;
                return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
            }
            (*parser).m_parsingStatus.parsing = crate::expat_h::XML_FINISHED
        }
        2 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_FINISHED;
            return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
        }
        _ => {
            if resumable != 0 {
                if (*parser).m_isParamEntity != 0 {
                    (*parser).m_errorCode = crate::expat_h::XML_ERROR_SUSPEND_PE;
                    return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
                }
                (*parser).m_parsingStatus.parsing = crate::expat_h::XML_SUSPENDED
            } else {
                (*parser).m_parsingStatus.parsing = crate::expat_h::XML_FINISHED
            }
        }
    }
    return crate::expat_h::XML_STATUS_OK_0 as crate::expat_h::XML_Status;
}
/* Resumes parsing after it has been suspended with XML_StopParser().
   Must not be called from within a handler call-back. Returns same
   status codes as XML_Parse() or XML_ParseBuffer().
   Additional error code XML_ERROR_NOT_SUSPENDED possible.

   *Note*:
   This must be called on the most deeply nested child parser instance
   first, and on its parent parser only after the child parser has finished,
   to be applied recursively until the document entity's parser is restarted.
   That is, the parent parser will not resume by itself and it is up to the
   application to call XML_ResumeParser() on it at the appropriate moment.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_ResumeParser(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Status {
    let mut result: crate::expat_h::XML_Status =
        crate::expat_h::XML_STATUS_OK_0 as crate::expat_h::XML_Status;
    if parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
    }
    if (*parser).m_parsingStatus.parsing as libc::c_uint
        != crate::expat_h::XML_SUSPENDED as libc::c_int as libc::c_uint
    {
        (*parser).m_errorCode = crate::expat_h::XML_ERROR_NOT_SUSPENDED;
        return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
    }
    (*parser).m_parsingStatus.parsing = crate::expat_h::XML_PARSING;
    (*parser).m_errorCode = (*parser).m_processor.expect("non-null function pointer")(
        parser,
        (*parser).m_bufferPtr,
        (*parser).m_parseEndPtr,
        &mut (*parser).m_bufferPtr,
    );
    if (*parser).m_errorCode as libc::c_uint
        != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint
    {
        (*parser).m_eventEndPtr = (*parser).m_eventPtr;
        (*parser).m_processor = Some(errorProcessor as Processor);
        return crate::expat_h::XML_STATUS_ERROR_0 as crate::expat_h::XML_Status;
    } else {
        match (*parser).m_parsingStatus.parsing as libc::c_uint {
            3 => result = crate::expat_h::XML_STATUS_SUSPENDED_0 as crate::expat_h::XML_Status,
            0 | 1 => {
                if (*parser).m_parsingStatus.finalBuffer != 0 {
                    (*parser).m_parsingStatus.parsing = crate::expat_h::XML_FINISHED;
                    return result;
                }
            }
            _ => {}
        }
    }
    (*(*parser).m_encoding)
        .updatePosition
        .expect("non-null function pointer")(
        (*parser).m_encoding,
        (*parser).m_positionPtr,
        (*parser).m_bufferPtr,
        &mut (*parser).m_position,
    );
    (*parser).m_positionPtr = (*parser).m_bufferPtr;
    return result;
}
/* Returns status of parser with respect to being initialized, parsing,
   finished, or suspended and processing the final buffer.
   XXX XML_Parse() and XML_ParseBuffer() should return XML_ParsingStatus,
   XXX with XML_FINISHED_OK or XML_FINISHED_ERROR replacing XML_FINISHED
*/
#[no_mangle]

pub unsafe extern "C" fn XML_GetParsingStatus(
    mut parser: crate::expat_h::XML_Parser,
    mut status: *mut crate::expat_h::XML_ParsingStatus,
) {
    if parser.is_null() {
        return;
    }
    if !status.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"status != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/lib/xmlparse.c\x00" as *const u8
                as *const libc::c_char,
            2113 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                b"void XML_GetParsingStatus(XML_Parser, XML_ParsingStatus *)\x00",
            ))
            .as_ptr(),
        );
    }
    *status = (*parser).m_parsingStatus;
}
/* If XML_Parse or XML_ParseBuffer have returned XML_STATUS_ERROR, then
   XML_GetErrorCode returns information about the error.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_GetErrorCode(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Error {
    if parser.is_null() {
        return crate::expat_h::XML_ERROR_INVALID_ARGUMENT;
    }
    return (*parser).m_errorCode;
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetCurrentByteIndex(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_external_h::XML_Index {
    if parser.is_null() {
        return -(1 as libc::c_int) as crate::expat_external_h::XML_Index;
    }
    if !(*parser).m_eventPtr.is_null() {
        return (*parser).m_parseEndByteIndex
            - (*parser)
                .m_parseEndPtr
                .wrapping_offset_from((*parser).m_eventPtr) as libc::c_long;
    }
    return -(1 as libc::c_int) as crate::expat_external_h::XML_Index;
}
/* Return the number of bytes in the current event.
   Returns 0 if the event is in an internal entity.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_GetCurrentByteCount(
    mut parser: crate::expat_h::XML_Parser,
) -> libc::c_int {
    if parser.is_null() {
        return 0 as libc::c_int;
    }
    if !(*parser).m_eventEndPtr.is_null() && !(*parser).m_eventPtr.is_null() {
        return (*parser)
            .m_eventEndPtr
            .wrapping_offset_from((*parser).m_eventPtr) as libc::c_long
            as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* If XML_CONTEXT_BYTES is defined, returns the input buffer, sets
   the integer pointed to by offset to the offset within this buffer
   of the current parse position, and sets the integer pointed to by size
   to the size of this buffer (the number of input bytes). Otherwise
   returns a NULL pointer. Also returns a NULL pointer if a parse isn't
   active.

   NOTE: The character pointer returned should not be used outside
   the handler that makes the call.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_GetInputContext(
    mut parser: crate::expat_h::XML_Parser,
    mut offset: *mut libc::c_int,
    mut size: *mut libc::c_int,
) -> *const libc::c_char {
    if parser.is_null() {
        return crate::stddef_h::NULL as *const libc::c_char;
    }
    if !(*parser).m_eventPtr.is_null() && !(*parser).m_buffer.is_null() {
        if !offset.is_null() {
            *offset = (*parser)
                .m_eventPtr
                .wrapping_offset_from((*parser).m_buffer) as libc::c_long
                as libc::c_int
        }
        if !size.is_null() {
            *size = (*parser)
                .m_bufferEnd
                .wrapping_offset_from((*parser).m_buffer) as libc::c_long
                as libc::c_int
        }
        return (*parser).m_buffer;
    }
    /* defined XML_CONTEXT_BYTES */
    return 0 as *mut libc::c_char;
}
/* These functions return information about the current parse
   location.  They may be called from any callback called to report
   some parse event; in this case the location is the location of the
   first of the sequence of characters that generated the event.  When
   called from callbacks generated by declarations in the document
   prologue, the location identified isn't as neatly defined, but will
   be within the relevant markup.  When called outside of the callback
   functions, the position indicated will be just past the last parse
   event (regardless of whether there was an associated callback).

   They may also be called after returning from a call to XML_Parse
   or XML_ParseBuffer.  If the return value is XML_STATUS_ERROR then
   the location is the location of the character at which the error
   was detected; otherwise the location is the location of the last
   parse event, as described above.

   Note: XML_GetCurrentLineNumber and XML_GetCurrentColumnNumber
   return 0 to indicate an error.
   Note: XML_GetCurrentByteIndex returns -1 to indicate an error.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_GetCurrentLineNumber(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_external_h::XML_Size {
    if parser.is_null() {
        return 0 as libc::c_int as crate::expat_external_h::XML_Size;
    }
    if !(*parser).m_eventPtr.is_null() && (*parser).m_eventPtr >= (*parser).m_positionPtr {
        (*(*parser).m_encoding)
            .updatePosition
            .expect("non-null function pointer")(
            (*parser).m_encoding,
            (*parser).m_positionPtr,
            (*parser).m_eventPtr,
            &mut (*parser).m_position,
        );
        (*parser).m_positionPtr = (*parser).m_eventPtr
    }
    return (*parser)
        .m_position
        .lineNumber
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetCurrentColumnNumber(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_external_h::XML_Size {
    if parser.is_null() {
        return 0 as libc::c_int as crate::expat_external_h::XML_Size;
    }
    if !(*parser).m_eventPtr.is_null() && (*parser).m_eventPtr >= (*parser).m_positionPtr {
        (*(*parser).m_encoding)
            .updatePosition
            .expect("non-null function pointer")(
            (*parser).m_encoding,
            (*parser).m_positionPtr,
            (*parser).m_eventPtr,
            &mut (*parser).m_position,
        );
        (*parser).m_positionPtr = (*parser).m_eventPtr
    }
    return (*parser).m_position.columnNumber;
}
/* For backwards compatibility with previous versions. */
/* Frees the content model passed to the element declaration handler */
#[no_mangle]

pub unsafe extern "C" fn XML_FreeContentModel(
    mut parser: crate::expat_h::XML_Parser,
    mut model: *mut crate::expat_h::XML_Content,
) {
    if !parser.is_null() {
        (*parser).m_mem.free_fcn.expect("non-null function pointer")(model as *mut libc::c_void);
    };
}
/* Exposing the memory handling functions used in Expat */
#[no_mangle]

pub unsafe extern "C" fn XML_MemMalloc(
    mut parser: crate::expat_h::XML_Parser,
    mut size: crate::stddef_h::size_t,
) -> *mut libc::c_void {
    if parser.is_null() {
        return crate::stddef_h::NULL as *mut libc::c_void;
    }
    return (*parser)
        .m_mem
        .malloc_fcn
        .expect("non-null function pointer")(size);
}
#[no_mangle]

pub unsafe extern "C" fn XML_MemRealloc(
    mut parser: crate::expat_h::XML_Parser,
    mut ptr: *mut libc::c_void,
    mut size: crate::stddef_h::size_t,
) -> *mut libc::c_void {
    if parser.is_null() {
        return crate::stddef_h::NULL as *mut libc::c_void;
    }
    return (*parser)
        .m_mem
        .realloc_fcn
        .expect("non-null function pointer")(ptr, size);
}
#[no_mangle]

pub unsafe extern "C" fn XML_MemFree(
    mut parser: crate::expat_h::XML_Parser,
    mut ptr: *mut libc::c_void,
) {
    if !parser.is_null() {
        (*parser).m_mem.free_fcn.expect("non-null function pointer")(ptr);
    };
}
/* This can be called within a handler for a start element, end
   element, processing instruction or character data.  It causes the
   corresponding markup to be passed to the default handler.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_DefaultCurrent(mut parser: crate::expat_h::XML_Parser) {
    if parser.is_null() {
        return;
    }
    if (*parser).m_defaultHandler.is_some() {
        if !(*parser).m_openInternalEntities.is_null() {
            reportDefault(
                parser,
                (*parser).m_internalEncoding,
                (*(*parser).m_openInternalEntities).internalEventPtr,
                (*(*parser).m_openInternalEntities).internalEventEndPtr,
            );
        } else {
            reportDefault(
                parser,
                (*parser).m_encoding,
                (*parser).m_eventPtr,
                (*parser).m_eventEndPtr,
            );
        }
    };
}
/* Returns a string describing the error. */
#[no_mangle]

pub unsafe extern "C" fn XML_ErrorString(
    mut code: crate::expat_h::XML_Error,
) -> *const crate::expat_external_h::XML_LChar {
    match code as libc::c_uint {
        0 => return crate::stddef_h::NULL as *const crate::expat_external_h::XML_LChar,
        1 => return b"out of memory\x00" as *const u8 as *const libc::c_char,
        2 => return b"syntax error\x00" as *const u8 as *const libc::c_char,
        3 => return b"no element found\x00" as *const u8 as *const libc::c_char,
        4 => return b"not well-formed (invalid token)\x00" as *const u8 as *const libc::c_char,
        5 => return b"unclosed token\x00" as *const u8 as *const libc::c_char,
        6 => return b"partial character\x00" as *const u8 as *const libc::c_char,
        7 => return b"mismatched tag\x00" as *const u8 as *const libc::c_char,
        8 => return b"duplicate attribute\x00" as *const u8 as *const libc::c_char,
        9 => return b"junk after document element\x00" as *const u8 as *const libc::c_char,
        10 => return b"illegal parameter entity reference\x00" as *const u8 as *const libc::c_char,
        11 => return b"undefined entity\x00" as *const u8 as *const libc::c_char,
        12 => return b"recursive entity reference\x00" as *const u8 as *const libc::c_char,
        13 => return b"asynchronous entity\x00" as *const u8 as *const libc::c_char,
        14 => {
            return b"reference to invalid character number\x00" as *const u8 as *const libc::c_char
        }
        15 => return b"reference to binary entity\x00" as *const u8 as *const libc::c_char,
        16 => {
            return b"reference to external entity in attribute\x00" as *const u8
                as *const libc::c_char
        }
        17 => {
            return b"XML or text declaration not at start of entity\x00" as *const u8
                as *const libc::c_char
        }
        18 => return b"unknown encoding\x00" as *const u8 as *const libc::c_char,
        19 => {
            return b"encoding specified in XML declaration is incorrect\x00" as *const u8
                as *const libc::c_char
        }
        20 => return b"unclosed CDATA section\x00" as *const u8 as *const libc::c_char,
        21 => {
            return b"error in processing external entity reference\x00" as *const u8
                as *const libc::c_char
        }
        22 => return b"document is not standalone\x00" as *const u8 as *const libc::c_char,
        23 => {
            return b"unexpected parser state - please send a bug report\x00" as *const u8
                as *const libc::c_char
        }
        24 => {
            return b"entity declared in parameter entity\x00" as *const u8 as *const libc::c_char
        }
        25 => {
            return b"requested feature requires XML_DTD support in Expat\x00" as *const u8
                as *const libc::c_char
        }
        26 => {
            return b"cannot change setting once parsing has begun\x00" as *const u8
                as *const libc::c_char
        }
        27 => {
            /* Added in 1.95.7. */
            return b"unbound prefix\x00" as *const u8 as *const libc::c_char;
        }
        28 => {
            /* Added in 1.95.8. */
            return b"must not undeclare prefix\x00" as *const u8 as *const libc::c_char;
        }
        29 => {
            return b"incomplete markup in parameter entity\x00" as *const u8 as *const libc::c_char
        }
        30 => return b"XML declaration not well-formed\x00" as *const u8 as *const libc::c_char,
        31 => return b"text declaration not well-formed\x00" as *const u8 as *const libc::c_char,
        32 => return b"illegal character(s) in public id\x00" as *const u8 as *const libc::c_char,
        33 => return b"parser suspended\x00" as *const u8 as *const libc::c_char,
        34 => return b"parser not suspended\x00" as *const u8 as *const libc::c_char,
        35 => return b"parsing aborted\x00" as *const u8 as *const libc::c_char,
        36 => return b"parsing finished\x00" as *const u8 as *const libc::c_char,
        37 => {
            return b"cannot suspend in external parameter entity\x00" as *const u8
                as *const libc::c_char
        }
        38 => {
            /* Added in 2.0.0. */
            return b"reserved prefix (xml) must not be undeclared or bound to another namespace name\x00"
                       as *const u8 as *const libc::c_char;
        }
        39 => {
            return b"reserved prefix (xmlns) must not be declared or undeclared\x00" as *const u8
                as *const libc::c_char
        }
        40 => {
            return b"prefix must not be bound to one of the reserved namespace names\x00"
                as *const u8 as *const libc::c_char
        }
        41 => {
            /* Added in 2.2.5. */
            /* Constant added in 2.2.1, already */
            return b"invalid argument\x00" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    return crate::stddef_h::NULL as *const crate::expat_external_h::XML_LChar;
}
/* Return a string containing the version number of this expat */
#[no_mangle]

pub unsafe extern "C" fn XML_ExpatVersion() -> *const crate::expat_external_h::XML_LChar {
    /* V1 is used to string-ize the version number. However, it would
    string-ize the actual version macro *names* unless we get them
    substituted before being passed to V1. CPP is defined to expand
    a macro, then rescan for more expansions. Thus, we use V2 to expand
    the version macros, then CPP will expand the resulting V1() macro
    with the correct numerals. */
    /* ### I'm assuming cpp is portable in this respect... */
    return b"expat_2.2.9\x00" as *const u8 as *const libc::c_char;
}
/* Return an XML_Expat_Version structure containing numeric version
   number information for this version of expat.
*/
#[no_mangle]

pub unsafe extern "C" fn XML_ExpatVersionInfo() -> crate::expat_h::XML_Expat_Version {
    let mut version: crate::expat_h::XML_Expat_Version = crate::expat_h::XML_Expat_Version {
        major: 0,
        minor: 0,
        micro: 0,
    };
    version.major = crate::expat_h::XML_MAJOR_VERSION;
    version.minor = crate::expat_h::XML_MINOR_VERSION;
    version.micro = crate::expat_h::XML_MICRO_VERSION;
    return version;
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetFeatureList() -> *const crate::expat_h::XML_Feature {
    static mut features: [crate::expat_h::XML_Feature; 6] = [
        {
            let mut init = crate::expat_h::XML_Feature {
                feature: crate::expat_h::XML_FEATURE_SIZEOF_XML_CHAR,
                name: b"sizeof(XML_Char)\x00" as *const u8 as *const libc::c_char,
                value: ::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong
                    as libc::c_long,
            };
            init
        },
        {
            let mut init = crate::expat_h::XML_Feature {
                feature: crate::expat_h::XML_FEATURE_SIZEOF_XML_LCHAR,
                name: b"sizeof(XML_LChar)\x00" as *const u8 as *const libc::c_char,
                value: ::std::mem::size_of::<crate::expat_external_h::XML_LChar>() as libc::c_ulong
                    as libc::c_long,
            };
            init
        },
        {
            let mut init = crate::expat_h::XML_Feature {
                feature: crate::expat_h::XML_FEATURE_DTD,
                name: b"XML_DTD\x00" as *const u8 as *const libc::c_char,
                value: 0 as libc::c_int as libc::c_long,
            };
            init
        },
        {
            let mut init = crate::expat_h::XML_Feature {
                feature: crate::expat_h::XML_FEATURE_CONTEXT_BYTES,
                name: b"XML_CONTEXT_BYTES\x00" as *const u8 as *const libc::c_char,
                value: crate::expat_config_h::XML_CONTEXT_BYTES as libc::c_long,
            };
            init
        },
        {
            let mut init = crate::expat_h::XML_Feature {
                feature: crate::expat_h::XML_FEATURE_NS,
                name: b"XML_NS\x00" as *const u8 as *const libc::c_char,
                value: 0 as libc::c_int as libc::c_long,
            };
            init
        },
        {
            let mut init = crate::expat_h::XML_Feature {
                feature: crate::expat_h::XML_FEATURE_END,
                name: crate::stddef_h::NULL as *const crate::expat_external_h::XML_LChar,
                value: 0 as libc::c_int as libc::c_long,
            };
            init
        },
    ];
    return features.as_ptr();
}
/* Initially tag->rawName always points into the parse buffer;
   for those TAG instances opened while the current parse buffer was
   processed, and not yet closed, we need to store tag->rawName in a more
   permanent location, since the parse buffer is about to be discarded.
*/

unsafe extern "C" fn storeRawNames(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Bool {
    let mut tag: *mut TAG = (*parser).m_tagStack;
    while !tag.is_null() {
        let mut bufSize: libc::c_int = 0;
        let mut nameLen: libc::c_int = (::std::mem::size_of::<crate::expat_external_h::XML_Char>()
            as libc::c_ulong)
            .wrapping_mul(((*tag).name.strLen + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int;
        let mut rawNameBuf: *mut libc::c_char = (*tag).buf.offset(nameLen as isize);
        /* Stop if already stored.  Since m_tagStack is a stack, we can stop
           at the first entry that has already been copied; everything
           below it in the stack is already been accounted for in a
           previous call to this function.
        */
        if (*tag).rawName == rawNameBuf as *const libc::c_char {
            break;
        }
        /* For re-use purposes we need to ensure that the
           size of tag->buf is a multiple of sizeof(XML_Char).
        */
        bufSize = (nameLen as libc::c_ulong).wrapping_add(
            ((*tag).rawNameLength as libc::c_ulong).wrapping_add(
                (::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) & !(::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as libc::c_int;
        if bufSize as libc::c_long > (*tag).bufEnd.wrapping_offset_from((*tag).buf) as libc::c_long
        {
            let mut temp: *mut libc::c_char = (*parser)
                .m_mem
                .realloc_fcn
                .expect("non-null function pointer")(
                (*tag).buf as *mut libc::c_void,
                bufSize as crate::stddef_h::size_t,
            ) as *mut libc::c_char;
            if temp.is_null() {
                return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
            }
            /* if tag->name.str points to tag->buf (only when namespace
               processing is off) then we have to update it
            */
            if (*tag).name.str_0
                == (*tag).buf as *mut crate::expat_external_h::XML_Char
                    as *const crate::expat_external_h::XML_Char
            {
                (*tag).name.str_0 = temp as *mut crate::expat_external_h::XML_Char
            }
            /* if tag->name.localPart is set (when namespace processing is on)
               then update it as well, since it will always point into tag->buf
            */
            if !(*tag).name.localPart.is_null() {
                (*tag).name.localPart =
                    (temp as *mut crate::expat_external_h::XML_Char).offset(
                        (*tag).name.localPart.wrapping_offset_from(
                            (*tag).buf as *mut crate::expat_external_h::XML_Char,
                        ) as libc::c_long as isize,
                    )
            } /* XmlContentTok doesn't always set the last arg */
            (*tag).buf = temp;
            (*tag).bufEnd = temp.offset(bufSize as isize);
            rawNameBuf = temp.offset(nameLen as isize)
        }
        crate::stdlib::memcpy(
            rawNameBuf as *mut libc::c_void,
            (*tag).rawName as *const libc::c_void,
            (*tag).rawNameLength as libc::c_ulong,
        );
        (*tag).rawName = rawNameBuf;
        tag = (*tag).parent
    }
    return crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
}

unsafe extern "C" fn contentProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
    mut endPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error = doContent(
        parser,
        0 as libc::c_int,
        (*parser).m_encoding,
        start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as libc::c_int as crate::expat_h::XML_Bool,
    );
    if result as libc::c_uint == crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint {
        if storeRawNames(parser) == 0 {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
    }
    return result;
}

unsafe extern "C" fn externalEntityInitProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
    mut endPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error = initializeEncoding(parser);
    if result as libc::c_uint != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint {
        return result;
    }
    (*parser).m_processor = Some(externalEntityInitProcessor2 as Processor);
    return externalEntityInitProcessor2(parser, start, end, endPtr);
}

unsafe extern "C" fn externalEntityInitProcessor2(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
    mut endPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut next: *const libc::c_char = start;
    let mut tok: libc::c_int = (*(*parser).m_encoding).scanners[1 as libc::c_int as usize]
        .expect("non-null function pointer")(
        (*parser).m_encoding, start, end, &mut next
    );
    match tok {
        crate::src::lib::xmltok::XML_TOK_BOM => {
            /* If we are at the end of the buffer, this would cause the next stage,
               i.e. externalEntityInitProcessor3, to pass control directly to
               doContent (by detecting XML_TOK_NONE) without processing any xml text
               declaration - causing the error XML_ERROR_MISPLACED_XML_PI in doContent.
            */
            if next == end && (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = next; /* XmlContentTok doesn't always set the last arg */
                return crate::expat_h::XML_ERROR_NONE;
            }
            start = next
        }
        crate::src::lib::xmltok::XML_TOK_PARTIAL => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return crate::expat_h::XML_ERROR_NONE;
            }
            (*parser).m_eventPtr = start;
            return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN;
        }
        crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return crate::expat_h::XML_ERROR_NONE;
            }
            (*parser).m_eventPtr = start;
            return crate::expat_h::XML_ERROR_PARTIAL_CHAR;
        }
        _ => {}
    }
    (*parser).m_processor = Some(externalEntityInitProcessor3 as Processor);
    return externalEntityInitProcessor3(parser, start, end, endPtr);
}

unsafe extern "C" fn externalEntityInitProcessor3(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
    mut endPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut tok: libc::c_int = 0;
    let mut next: *const libc::c_char = start;
    (*parser).m_eventPtr = start;
    tok = (*(*parser).m_encoding).scanners[1 as libc::c_int as usize]
        .expect("non-null function pointer")((*parser).m_encoding, start, end, &mut next);
    (*parser).m_eventEndPtr = next;
    match tok {
        crate::src::lib::xmltok::XML_TOK_XML_DECL => {
            let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
            result = processXmlDecl(parser, 1 as libc::c_int, start, next);
            if result as libc::c_uint
                != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint
            {
                return result;
            }
            match (*parser).m_parsingStatus.parsing as libc::c_uint {
                3 => {
                    *endPtr = next;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                2 => return crate::expat_h::XML_ERROR_ABORTED,
                _ => start = next,
            }
        }
        crate::src::lib::xmltok::XML_TOK_PARTIAL => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return crate::expat_h::XML_ERROR_NONE;
            }
            return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN;
        }
        crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return crate::expat_h::XML_ERROR_NONE;
            }
            return crate::expat_h::XML_ERROR_PARTIAL_CHAR;
        }
        _ => {}
    }
    (*parser).m_processor = Some(externalEntityContentProcessor as Processor);
    (*parser).m_tagLevel = 1 as libc::c_int;
    return externalEntityContentProcessor(parser, start, end, endPtr);
}

unsafe extern "C" fn externalEntityContentProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
    mut endPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error = doContent(
        parser,
        1 as libc::c_int,
        (*parser).m_encoding,
        start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as libc::c_int as crate::expat_h::XML_Bool,
    );
    if result as libc::c_uint == crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint {
        if storeRawNames(parser) == 0 {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
    }
    return result;
}

unsafe extern "C" fn doContent(
    mut parser: crate::expat_h::XML_Parser,
    mut startTagLevel: libc::c_int,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut s: *const libc::c_char,
    mut end: *const libc::c_char,
    mut nextPtr: *mut *const libc::c_char,
    mut haveMore: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Error {
    /* save one level of indirection */
    let dtd: *mut DTD = (*parser).m_dtd; /* XmlContentTok doesn't always set the last arg */
    let mut eventPP: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut eventEndPP: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if enc == (*parser).m_encoding {
        eventPP = &mut (*parser).m_eventPtr;
        eventEndPP = &mut (*parser).m_eventEndPtr
    } else {
        eventPP = &mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &mut (*(*parser).m_openInternalEntities).internalEventEndPtr
    }
    *eventPP = s;
    loop {
        let mut next: *const libc::c_char = s;
        let mut tok: libc::c_int = (*enc).scanners[1 as libc::c_int as usize]
            .expect("non-null function pointer")(
            enc, s, end, &mut next
        );
        *eventEndPP = next;
        let mut current_block_275: u64;
        match tok {
            crate::src::lib::xmltok::XML_TOK_TRAILING_CR => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                *eventEndPP = end;
                if (*parser).m_characterDataHandler.is_some() {
                    let mut c: crate::expat_external_h::XML_Char =
                        0xa as libc::c_int as crate::expat_external_h::XML_Char;
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &mut c,
                        1 as libc::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, end);
                }
                /* LCOV_EXCL_STOP */
                /* We are at the end of the final buffer, should we check for
                   XML_SUSPENDED, XML_FINISHED?
                */
                if startTagLevel == 0 as libc::c_int {
                    return crate::expat_h::XML_ERROR_NO_ELEMENTS;
                }
                if (*parser).m_tagLevel != startTagLevel {
                    return crate::expat_h::XML_ERROR_ASYNC_ENTITY;
                }
                *nextPtr = end;
                return crate::expat_h::XML_ERROR_NONE;
            }
            crate::src::lib::xmltok::XML_TOK_NONE => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                if startTagLevel > 0 as libc::c_int {
                    if (*parser).m_tagLevel != startTagLevel {
                        return crate::expat_h::XML_ERROR_ASYNC_ENTITY;
                    }
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                return crate::expat_h::XML_ERROR_NO_ELEMENTS;
            }
            crate::src::lib::xmltok::XML_TOK_INVALID => {
                *eventPP = next;
                return crate::expat_h::XML_ERROR_INVALID_TOKEN;
            }
            crate::src::lib::xmltok::XML_TOK_PARTIAL => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN;
            }
            crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                return crate::expat_h::XML_ERROR_PARTIAL_CHAR;
            }
            crate::src::lib::xmltok::XML_TOK_ENTITY_REF => {
                let mut name: *const crate::expat_external_h::XML_Char =
                    0 as *const crate::expat_external_h::XML_Char;
                let mut entity: *mut ENTITY = 0 as *mut ENTITY;
                let mut ch: crate::expat_external_h::XML_Char = (*enc)
                    .predefinedEntityName
                    .expect("non-null function pointer")(
                    enc,
                    s.offset((*enc).minBytesPerChar as isize),
                    next.offset(-((*enc).minBytesPerChar as isize)),
                )
                    as crate::expat_external_h::XML_Char;
                if ch != 0 {
                    if (*parser).m_characterDataHandler.is_some() {
                        (*parser)
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            &mut ch,
                            1 as libc::c_int,
                        );
                    } else if (*parser).m_defaultHandler.is_some() {
                        reportDefault(parser, enc, s, next);
                    }
                } else {
                    name = poolStoreString(
                        &mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if name.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    entity = lookup(
                        parser,
                        &mut (*dtd).generalEntities,
                        name,
                        0 as libc::c_int as crate::stddef_h::size_t,
                    ) as *mut ENTITY;
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    /* First, determine if a check for an existing declaration is needed;
                       if yes, check that the entity exists, and that it is internal,
                       otherwise call the skipped entity or default handler.
                    */
                    if (*dtd).hasParamEntityRefs == 0 || (*dtd).standalone as libc::c_int != 0 {
                        if entity.is_null() {
                            return crate::expat_h::XML_ERROR_UNDEFINED_ENTITY;
                        } else {
                            if (*entity).is_internal == 0 {
                                return crate::expat_h::XML_ERROR_ENTITY_DECLARED_IN_PE;
                            }
                        }
                        current_block_275 = 10067844863897285902;
                    } else if entity.is_null() {
                        if (*parser).m_skippedEntityHandler.is_some() {
                            (*parser)
                                .m_skippedEntityHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                name,
                                0 as libc::c_int,
                            );
                        } else if (*parser).m_defaultHandler.is_some() {
                            reportDefault(parser, enc, s, next);
                        }
                        current_block_275 = 17939951368883298147;
                    } else {
                        current_block_275 = 10067844863897285902;
                    }
                    match current_block_275 {
                        17939951368883298147 => {}
                        _ => {
                            if (*entity).open != 0 {
                                return crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if !(*entity).notation.is_null() {
                                return crate::expat_h::XML_ERROR_BINARY_ENTITY_REF;
                            }
                            if !(*entity).textPtr.is_null() {
                                let mut result: crate::expat_h::XML_Error =
                                    crate::expat_h::XML_ERROR_NONE;
                                if (*parser).m_defaultExpandInternalEntities == 0 {
                                    if (*parser).m_skippedEntityHandler.is_some() {
                                        (*parser)
                                            .m_skippedEntityHandler
                                            .expect("non-null function pointer")(
                                            (*parser).m_handlerArg,
                                            (*entity).name,
                                            0 as libc::c_int,
                                        );
                                    } else if (*parser).m_defaultHandler.is_some() {
                                        reportDefault(parser, enc, s, next);
                                    }
                                } else {
                                    result = processInternalEntity(
                                        parser,
                                        entity,
                                        crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool,
                                    );
                                    if result as libc::c_uint
                                        != crate::expat_h::XML_ERROR_NONE as libc::c_int
                                            as libc::c_uint
                                    {
                                        return result;
                                    }
                                }
                            } else if (*parser).m_externalEntityRefHandler.is_some() {
                                let mut context: *const crate::expat_external_h::XML_Char =
                                    0 as *const crate::expat_external_h::XML_Char;
                                (*entity).open =
                                    crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                                context = getContext(parser);
                                (*entity).open =
                                    crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                                if context.is_null() {
                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                                }
                                if (*parser)
                                    .m_externalEntityRefHandler
                                    .expect("non-null function pointer")(
                                    (*parser).m_externalEntityRefHandlerArg,
                                    context,
                                    (*entity).base,
                                    (*entity).systemId,
                                    (*entity).publicId,
                                ) == 0
                                {
                                    return crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                }
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.start
                            } else if (*parser).m_defaultHandler.is_some() {
                                reportDefault(parser, enc, s, next);
                            }
                        }
                    }
                }
            }
            crate::src::lib::xmltok::XML_TOK_START_TAG_NO_ATTS
            | crate::src::lib::xmltok::XML_TOK_START_TAG_WITH_ATTS => {
                /* fall through */
                let mut tag: *mut TAG = 0 as *mut TAG;
                let mut result_0: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
                let mut toPtr: *mut crate::expat_external_h::XML_Char =
                    0 as *mut crate::expat_external_h::XML_Char;
                if !(*parser).m_freeTagList.is_null() {
                    tag = (*parser).m_freeTagList;
                    (*parser).m_freeTagList = (*(*parser).m_freeTagList).parent
                } else {
                    tag = (*parser)
                        .m_mem
                        .malloc_fcn
                        .expect("non-null function pointer")(
                        ::std::mem::size_of::<TAG>() as libc::c_ulong,
                    ) as *mut TAG;
                    if tag.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*tag).buf = (*parser)
                        .m_mem
                        .malloc_fcn
                        .expect("non-null function pointer")(
                        32 as libc::c_int as crate::stddef_h::size_t,
                    ) as *mut libc::c_char;
                    if (*tag).buf.is_null() {
                        (*parser).m_mem.free_fcn.expect("non-null function pointer")(
                            tag as *mut libc::c_void,
                        );
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*tag).bufEnd = (*tag).buf.offset(INIT_TAG_BUF_SIZE as isize)
                }
                (*tag).bindings = crate::stddef_h::NULL as *mut BINDING;
                (*tag).parent = (*parser).m_tagStack;
                (*parser).m_tagStack = tag;
                (*tag).name.localPart =
                    crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
                (*tag).name.prefix =
                    crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
                (*tag).rawName = s.offset((*enc).minBytesPerChar as isize);
                (*tag).rawNameLength =
                    (*enc).nameLength.expect("non-null function pointer")(enc, (*tag).rawName);
                (*parser).m_tagLevel += 1;
                let mut rawNameEnd: *const libc::c_char =
                    (*tag).rawName.offset((*tag).rawNameLength as isize);
                let mut fromPtr: *const libc::c_char = (*tag).rawName;
                toPtr = (*tag).buf as *mut crate::expat_external_h::XML_Char;
                loop {
                    let mut bufSize: libc::c_int = 0;
                    let mut convLen: libc::c_int = 0;
                    let convert_res: crate::src::lib::xmltok::XML_Convert_Result =
                        (*enc).utf8Convert.expect("non-null function pointer")(
                            enc,
                            &mut fromPtr,
                            rawNameEnd,
                            &mut toPtr as *mut *mut crate::expat_external_h::XML_Char
                                as *mut *mut ICHAR,
                            ((*tag).bufEnd as *mut ICHAR).offset(-(1 as libc::c_int as isize)),
                        );
                    convLen = toPtr
                        .wrapping_offset_from((*tag).buf as *mut crate::expat_external_h::XML_Char)
                        as libc::c_long as libc::c_int;
                    if fromPtr >= rawNameEnd
                        || convert_res as libc::c_uint
                            == crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE as libc::c_int
                                as libc::c_uint
                    {
                        (*tag).name.strLen = convLen;
                        break;
                    } else {
                        bufSize = ((*tag).bufEnd.wrapping_offset_from((*tag).buf) as libc::c_long
                            as libc::c_int)
                            << 1 as libc::c_int;
                        let mut temp: *mut libc::c_char = (*parser)
                            .m_mem
                            .realloc_fcn
                            .expect("non-null function pointer")(
                            (*tag).buf as *mut libc::c_void,
                            bufSize as crate::stddef_h::size_t,
                        )
                            as *mut libc::c_char;
                        if temp.is_null() {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        (*tag).buf = temp;
                        (*tag).bufEnd = temp.offset(bufSize as isize);
                        toPtr = (temp as *mut crate::expat_external_h::XML_Char)
                            .offset(convLen as isize)
                    }
                }
                (*tag).name.str_0 = (*tag).buf as *mut crate::expat_external_h::XML_Char;
                *toPtr = '\u{0}' as i32 as crate::expat_external_h::XML_Char;
                result_0 = storeAtts(parser, enc, s, &mut (*tag).name, &mut (*tag).bindings);
                if result_0 as u64 != 0 {
                    return result_0;
                }
                if (*parser).m_startElementHandler.is_some() {
                    (*parser)
                        .m_startElementHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*tag).name.str_0,
                        (*parser).m_atts as *mut *const crate::expat_external_h::XML_Char,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                poolClear(&mut (*parser).m_tempPool);
            }
            crate::src::lib::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS
            | crate::src::lib::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS => {
                /* fall through */
                let mut rawName: *const libc::c_char = s.offset((*enc).minBytesPerChar as isize);
                let mut result_1: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
                let mut bindings: *mut BINDING = crate::stddef_h::NULL as *mut BINDING;
                let mut noElmHandlers: crate::expat_h::XML_Bool =
                    crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                let mut name_0: TAG_NAME = TAG_NAME {
                    str_0: 0 as *const crate::expat_external_h::XML_Char,
                    localPart: 0 as *const crate::expat_external_h::XML_Char,
                    prefix: 0 as *const crate::expat_external_h::XML_Char,
                    strLen: 0,
                    uriLen: 0,
                    prefixLen: 0,
                };
                name_0.str_0 = poolStoreString(
                    &mut (*parser).m_tempPool,
                    enc,
                    rawName,
                    rawName.offset((*enc).nameLength.expect("non-null function pointer")(
                        enc, rawName,
                    ) as isize),
                );
                if name_0.str_0.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                result_1 = storeAtts(parser, enc, s, &mut name_0, &mut bindings);
                if result_1 as libc::c_uint
                    != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    freeBindings(parser, bindings);
                    return result_1;
                }
                (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                if (*parser).m_startElementHandler.is_some() {
                    (*parser)
                        .m_startElementHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        name_0.str_0,
                        (*parser).m_atts as *mut *const crate::expat_external_h::XML_Char,
                    );
                    noElmHandlers = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                if (*parser).m_endElementHandler.is_some() {
                    if (*parser).m_startElementHandler.is_some() {
                        *eventPP = *eventEndPP
                    }
                    (*parser)
                        .m_endElementHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg, name_0.str_0
                    );
                    noElmHandlers = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                if noElmHandlers as libc::c_int != 0 && (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                poolClear(&mut (*parser).m_tempPool);
                freeBindings(parser, bindings);
                if (*parser).m_tagLevel == 0 as libc::c_int
                    && (*parser).m_parsingStatus.parsing as libc::c_uint
                        != crate::expat_h::XML_FINISHED as libc::c_int as libc::c_uint
                {
                    if (*parser).m_parsingStatus.parsing as libc::c_uint
                        == crate::expat_h::XML_SUSPENDED as libc::c_int as libc::c_uint
                    {
                        (*parser).m_processor = Some(epilogProcessor as Processor)
                    } else {
                        return epilogProcessor(parser, next, end, nextPtr);
                    }
                }
            }
            crate::src::lib::xmltok::XML_TOK_END_TAG => {
                if (*parser).m_tagLevel == startTagLevel {
                    return crate::expat_h::XML_ERROR_ASYNC_ENTITY;
                } else {
                    let mut len: libc::c_int = 0;
                    let mut rawName_0: *const libc::c_char = 0 as *const libc::c_char;
                    let mut tag_0: *mut TAG = (*parser).m_tagStack;
                    (*parser).m_tagStack = (*tag_0).parent;
                    (*tag_0).parent = (*parser).m_freeTagList;
                    (*parser).m_freeTagList = tag_0;
                    rawName_0 = s.offset(((*enc).minBytesPerChar * 2 as libc::c_int) as isize);
                    len = (*enc).nameLength.expect("non-null function pointer")(enc, rawName_0);
                    if len != (*tag_0).rawNameLength
                        || crate::stdlib::memcmp(
                            (*tag_0).rawName as *const libc::c_void,
                            rawName_0 as *const libc::c_void,
                            len as libc::c_ulong,
                        ) != 0 as libc::c_int
                    {
                        *eventPP = rawName_0;
                        return crate::expat_h::XML_ERROR_TAG_MISMATCH;
                    }
                    (*parser).m_tagLevel -= 1;
                    if (*parser).m_endElementHandler.is_some() {
                        let mut localPart: *const crate::expat_external_h::XML_Char =
                            0 as *const crate::expat_external_h::XML_Char;
                        let mut prefix: *const crate::expat_external_h::XML_Char =
                            0 as *const crate::expat_external_h::XML_Char;
                        let mut uri: *mut crate::expat_external_h::XML_Char =
                            0 as *mut crate::expat_external_h::XML_Char;
                        localPart = (*tag_0).name.localPart;
                        if (*parser).m_ns as libc::c_int != 0 && !localPart.is_null() {
                            /* localPart and prefix may have been overwritten in
                               tag->name.str, since this points to the binding->uri
                               buffer which gets re-used; so we have to add them again
                            */
                            uri = ((*tag_0).name.str_0 as *mut crate::expat_external_h::XML_Char)
                                .offset((*tag_0).name.uriLen as isize);
                            /* don't need to check for space - already done in storeAtts() */
                            while *localPart != 0 {
                                let fresh2 = localPart;
                                localPart = localPart.offset(1);
                                let fresh3 = uri;
                                uri = uri.offset(1);
                                *fresh3 = *fresh2
                            }
                            prefix = (*tag_0).name.prefix as *mut crate::expat_external_h::XML_Char;
                            if (*parser).m_ns_triplets as libc::c_int != 0 && !prefix.is_null() {
                                let fresh4 = uri;
                                uri = uri.offset(1);
                                *fresh4 = (*parser).m_namespaceSeparator;
                                while *prefix != 0 {
                                    let fresh5 = prefix;
                                    prefix = prefix.offset(1);
                                    let fresh6 = uri;
                                    uri = uri.offset(1);
                                    *fresh6 = *fresh5
                                }
                            }
                            *uri = '\u{0}' as i32 as crate::expat_external_h::XML_Char
                        }
                        (*parser)
                            .m_endElementHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*tag_0).name.str_0,
                        );
                    } else if (*parser).m_defaultHandler.is_some() {
                        reportDefault(parser, enc, s, next);
                    }
                    while !(*tag_0).bindings.is_null() {
                        let mut b: *mut BINDING = (*tag_0).bindings;
                        if (*parser).m_endNamespaceDeclHandler.is_some() {
                            (*parser)
                                .m_endNamespaceDeclHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*(*b).prefix).name,
                            );
                        }
                        (*tag_0).bindings = (*(*tag_0).bindings).nextTagBinding;
                        (*b).nextTagBinding = (*parser).m_freeBindingList;
                        (*parser).m_freeBindingList = b;
                        (*(*b).prefix).binding = (*b).prevPrefixBinding
                    }
                    if (*parser).m_tagLevel == 0 as libc::c_int
                        && (*parser).m_parsingStatus.parsing as libc::c_uint
                            != crate::expat_h::XML_FINISHED as libc::c_int as libc::c_uint
                    {
                        if (*parser).m_parsingStatus.parsing as libc::c_uint
                            == crate::expat_h::XML_SUSPENDED as libc::c_int as libc::c_uint
                        {
                            (*parser).m_processor = Some(epilogProcessor as Processor)
                        } else {
                            return epilogProcessor(parser, next, end, nextPtr);
                        }
                    }
                }
            }
            crate::src::lib::xmltok::XML_TOK_CHAR_REF => {
                let mut n: libc::c_int =
                    (*enc).charRefNumber.expect("non-null function pointer")(enc, s);
                if n < 0 as libc::c_int {
                    return crate::expat_h::XML_ERROR_BAD_CHAR_REF;
                }
                if (*parser).m_characterDataHandler.is_some() {
                    let mut buf: [crate::expat_external_h::XML_Char; 4] = [0; 4];
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        buf.as_mut_ptr(),
                        crate::src::lib::xmltok::XmlUtf8Encode(n, buf.as_mut_ptr() as *mut ICHAR),
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            crate::src::lib::xmltok::XML_TOK_XML_DECL => {
                return crate::expat_h::XML_ERROR_MISPLACED_XML_PI
            }
            crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE => {
                if (*parser).m_characterDataHandler.is_some() {
                    let mut c_0: crate::expat_external_h::XML_Char =
                        0xa as libc::c_int as crate::expat_external_h::XML_Char;
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &mut c_0,
                        1 as libc::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            crate::src::lib::xmltok::XML_TOK_CDATA_SECT_OPEN => {
                let mut result_2: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
                if (*parser).m_startCdataSectionHandler.is_some() {
                    (*parser)
                        .m_startCdataSectionHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    );
                } else if 0 as libc::c_int != 0 && (*parser).m_characterDataHandler.is_some() {
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_dataBuf,
                        0 as libc::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                result_2 = doCdataSection(parser, enc, &mut next, end, nextPtr, haveMore);
                if result_2 as libc::c_uint
                    != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return result_2;
                } else {
                    if next.is_null() {
                        (*parser).m_processor = Some(cdataSectionProcessor as Processor);
                        return result_2;
                    }
                }
            }
            crate::src::lib::xmltok::XML_TOK_TRAILING_RSQB => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                if (*parser).m_characterDataHandler.is_some() {
                    if (*enc).isUtf8 == 0 {
                        let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
                        (*enc).utf8Convert.expect("non-null function pointer")(
                            enc,
                            &mut s,
                            end,
                            &mut dataPtr,
                            (*parser).m_dataBufEnd as *mut ICHAR,
                        );
                        (*parser)
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*parser).m_dataBuf,
                            dataPtr.wrapping_offset_from((*parser).m_dataBuf as *mut ICHAR)
                                as libc::c_long as libc::c_int,
                        );
                    } else {
                        (*parser)
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            s as *mut crate::expat_external_h::XML_Char,
                            (end as *mut crate::expat_external_h::XML_Char)
                                .wrapping_offset_from(s as *mut crate::expat_external_h::XML_Char)
                                as libc::c_long as libc::c_int,
                        );
                    }
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, end);
                }
                /* BEGIN disabled code */
                /* Suppose you doing a transformation on a document that involves
                   changing only the character data.  You set up a defaultHandler
                   and a characterDataHandler.  The defaultHandler simply copies
                   characters through.  The characterDataHandler does the
                   transformation and writes the characters out escaping them as
                   necessary.  This case will fail to work if we leave out the
                   following two lines (because & and < inside CDATA sections will
                   be incorrectly escaped).

                   However, now we have a start/endCdataSectionHandler, so it seems
                   easier to let the user deal with this.
                */
                /* END disabled code */
                /* We are at the end of the final buffer, should we check for
                   XML_SUSPENDED, XML_FINISHED?
                */
                if startTagLevel == 0 as libc::c_int {
                    *eventPP = end;
                    return crate::expat_h::XML_ERROR_NO_ELEMENTS;
                }
                if (*parser).m_tagLevel != startTagLevel {
                    *eventPP = end;
                    return crate::expat_h::XML_ERROR_ASYNC_ENTITY;
                }
                *nextPtr = end;
                return crate::expat_h::XML_ERROR_NONE;
            }
            crate::src::lib::xmltok::XML_TOK_DATA_CHARS => {
                let mut charDataHandler: crate::expat_h::XML_CharacterDataHandler =
                    (*parser).m_characterDataHandler;
                if charDataHandler.is_some() {
                    if (*enc).isUtf8 == 0 {
                        loop {
                            let mut dataPtr_0: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
                            let convert_res_0: crate::src::lib::xmltok::XML_Convert_Result =
                                (*enc).utf8Convert.expect("non-null function pointer")(
                                    enc,
                                    &mut s,
                                    next,
                                    &mut dataPtr_0,
                                    (*parser).m_dataBufEnd as *mut ICHAR,
                                );
                            *eventEndPP = s;
                            charDataHandler.expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*parser).m_dataBuf,
                                dataPtr_0.wrapping_offset_from((*parser).m_dataBuf as *mut ICHAR)
                                    as libc::c_long as libc::c_int,
                            );
                            if convert_res_0 as libc::c_uint
                                == crate::src::lib::xmltok::XML_CONVERT_COMPLETED as libc::c_int
                                    as libc::c_uint
                                || convert_res_0 as libc::c_uint
                                    == crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE
                                        as libc::c_int
                                        as libc::c_uint
                            {
                                break;
                            }
                            *eventPP = s
                        }
                    } else {
                        charDataHandler.expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            s as *mut crate::expat_external_h::XML_Char,
                            (next as *mut crate::expat_external_h::XML_Char)
                                .wrapping_offset_from(s as *mut crate::expat_external_h::XML_Char)
                                as libc::c_long as libc::c_int,
                        );
                    }
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            crate::src::lib::xmltok::XML_TOK_PI => {
                if reportProcessingInstruction(parser, enc, s, next) == 0 {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
            }
            crate::src::lib::xmltok::XML_TOK_COMMENT => {
                if reportComment(parser, enc, s, next) == 0 {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
            }
            _ => {
                /* All of the tokens produced by XmlContentTok() have their own
                 * explicit cases, so this default is not strictly necessary.
                 * However it is a useful safety net, so we retain the code and
                 * simply exclude it from the coverage tests.
                 *
                 * LCOV_EXCL_START
                 */
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
        }
        s = next;
        *eventPP = s;
        match (*parser).m_parsingStatus.parsing as libc::c_uint {
            3 => {
                *nextPtr = next;
                return crate::expat_h::XML_ERROR_NONE;
            }
            2 => return crate::expat_h::XML_ERROR_ABORTED,
            _ => {}
        }
    }
    /* not reached */
}
/* XML_DTD */
/* This function does not call free() on the allocated memory, merely
 * moving it to the parser's m_freeBindingList where it can be freed or
 * reused as appropriate.
 */

unsafe extern "C" fn freeBindings(
    mut parser: crate::expat_h::XML_Parser,
    mut bindings: *mut BINDING,
) {
    while !bindings.is_null() {
        let mut b: *mut BINDING = bindings;
        /* m_startNamespaceDeclHandler will have been called for this
         * binding in addBindings(), so call the end handler now.
         */
        if (*parser).m_endNamespaceDeclHandler.is_some() {
            (*parser)
                .m_endNamespaceDeclHandler
                .expect("non-null function pointer")(
                (*parser).m_handlerArg, (*(*b).prefix).name
            );
        }
        bindings = (*bindings).nextTagBinding;
        (*b).nextTagBinding = (*parser).m_freeBindingList;
        (*parser).m_freeBindingList = b;
        (*(*b).prefix).binding = (*b).prevPrefixBinding
    }
}
/* Precondition: all arguments must be non-NULL;
   Purpose:
   - normalize attributes
   - check attributes for well-formedness
   - generate namespace aware attribute names (URI, prefix)
   - build list of attributes for startElementHandler
   - default attributes
   - process namespace declarations (check and report them)
   - generate namespace aware element name (URI, prefix)
*/

unsafe extern "C" fn storeAtts(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut attStr: *const libc::c_char,
    mut tagNamePtr: *mut TAG_NAME,
    mut bindingsPtr: *mut *mut BINDING,
) -> crate::expat_h::XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd; /* save one level of indirection */
    let mut elementType: *mut ELEMENT_TYPE = 0 as *mut ELEMENT_TYPE; /* the attribute list for the application */
    let mut nDefaultAtts: libc::c_int = 0;
    let mut appAtts: *mut *const crate::expat_external_h::XML_Char =
        0 as *mut *const crate::expat_external_h::XML_Char;
    let mut attIndex: libc::c_int = 0 as libc::c_int;
    let mut prefixLen: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut uri: *mut crate::expat_external_h::XML_Char =
        0 as *mut crate::expat_external_h::XML_Char;
    let mut nPrefixes: libc::c_int = 0 as libc::c_int;
    let mut binding: *mut BINDING = 0 as *mut BINDING;
    let mut localPart: *const crate::expat_external_h::XML_Char =
        0 as *const crate::expat_external_h::XML_Char;
    /* lookup the element type name */
    elementType = lookup(
        parser,
        &mut (*dtd).elementTypes,
        (*tagNamePtr).str_0,
        0 as libc::c_int as crate::stddef_h::size_t,
    ) as *mut ELEMENT_TYPE;
    if elementType.is_null() {
        let mut name: *const crate::expat_external_h::XML_Char =
            poolCopyString(&mut (*dtd).pool, (*tagNamePtr).str_0);
        if name.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        elementType = lookup(
            parser,
            &mut (*dtd).elementTypes,
            name,
            ::std::mem::size_of::<ELEMENT_TYPE>() as libc::c_ulong,
        ) as *mut ELEMENT_TYPE;
        if elementType.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        if (*parser).m_ns as libc::c_int != 0 && setElementTypePrefix(parser, elementType) == 0 {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
    }
    nDefaultAtts = (*elementType).nDefaultAtts;
    /* get the attributes from the tokenizer */
    n = (*enc).getAtts.expect("non-null function pointer")(
        enc,
        attStr,
        (*parser).m_attsSize,
        (*parser).m_atts,
    );
    if n + nDefaultAtts > (*parser).m_attsSize {
        let mut oldAttsSize: libc::c_int = (*parser).m_attsSize;
        let mut temp: *mut crate::src::lib::xmltok::ATTRIBUTE =
            0 as *mut crate::src::lib::xmltok::ATTRIBUTE;
        (*parser).m_attsSize = n + nDefaultAtts + INIT_ATTS_SIZE;
        temp = (*parser)
            .m_mem
            .realloc_fcn
            .expect("non-null function pointer")(
            (*parser).m_atts as *mut libc::c_void,
            ((*parser).m_attsSize as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::src::lib::xmltok::ATTRIBUTE,
            >() as libc::c_ulong),
        ) as *mut crate::src::lib::xmltok::ATTRIBUTE;
        if temp.is_null() {
            (*parser).m_attsSize = oldAttsSize;
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        (*parser).m_atts = temp;
        if n > oldAttsSize {
            (*enc).getAtts.expect("non-null function pointer")(enc, attStr, n, (*parser).m_atts);
        }
    }
    appAtts = (*parser).m_atts as *mut *const crate::expat_external_h::XML_Char;
    i = 0 as libc::c_int;
    while i < n {
        let mut currAtt: *mut crate::src::lib::xmltok::ATTRIBUTE =
            &mut *(*parser).m_atts.offset(i as isize) as *mut crate::src::lib::xmltok::ATTRIBUTE;
        /* add the name and value to the attribute list */
        let mut attId: *mut ATTRIBUTE_ID = getAttributeId(
            parser,
            enc,
            (*currAtt).name,
            (*currAtt)
                .name
                .offset(
                    (*enc).nameLength.expect("non-null function pointer")(enc, (*currAtt).name)
                        as isize,
                ),
        );
        if attId.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        /* Detect duplicate attributes by their QNames. This does not work when
           namespace processing is turned on and different prefixes for the same
           namespace are used. For this case we have a check further down.
        */
        if *(*attId).name.offset(-(1 as libc::c_int) as isize) != 0 {
            if enc == (*parser).m_encoding {
                (*parser).m_eventPtr = (*(*parser).m_atts.offset(i as isize)).name
            }
            return crate::expat_h::XML_ERROR_DUPLICATE_ATTRIBUTE;
        }
        *(*attId).name.offset(-(1 as libc::c_int) as isize) =
            1 as libc::c_int as crate::expat_external_h::XML_Char;
        let fresh7 = attIndex;
        attIndex = attIndex + 1;
        let ref mut fresh8 = *appAtts.offset(fresh7 as isize);
        *fresh8 = (*attId).name;
        if (*(*parser).m_atts.offset(i as isize)).normalized == 0 {
            let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
            let mut isCdata: crate::expat_h::XML_Bool =
                crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
            /* figure out whether declared as other than CDATA */
            if (*attId).maybeTokenized != 0 {
                let mut j: libc::c_int = 0;
                j = 0 as libc::c_int;
                while j < nDefaultAtts {
                    if attId
                        == (*(*elementType).defaultAtts.offset(j as isize)).id as *mut ATTRIBUTE_ID
                    {
                        isCdata = (*(*elementType).defaultAtts.offset(j as isize)).isCdata;
                        break;
                    } else {
                        j += 1
                    }
                }
            }
            /* normalize the attribute value */
            result = storeAttributeValue(
                parser,
                enc,
                isCdata,
                (*(*parser).m_atts.offset(i as isize)).valuePtr,
                (*(*parser).m_atts.offset(i as isize)).valueEnd,
                &mut (*parser).m_tempPool,
            );
            if result as u64 != 0 {
                return result;
            }
            let ref mut fresh9 = *appAtts.offset(attIndex as isize);
            *fresh9 = (*parser).m_tempPool.start;
            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr
        } else {
            /* the value did not need normalizing */
            let ref mut fresh10 = *appAtts.offset(attIndex as isize);
            *fresh10 = poolStoreString(
                &mut (*parser).m_tempPool,
                enc,
                (*(*parser).m_atts.offset(i as isize)).valuePtr,
                (*(*parser).m_atts.offset(i as isize)).valueEnd,
            );
            if (*appAtts.offset(attIndex as isize)).is_null() {
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            }
            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr
        }
        /* handle prefixed attribute names */
        if !(*attId).prefix.is_null() {
            if (*attId).xmlns != 0 {
                /* deal with namespace declarations here */
                let mut result_0: crate::expat_h::XML_Error = addBinding(
                    parser,
                    (*attId).prefix,
                    attId,
                    *appAtts.offset(attIndex as isize),
                    bindingsPtr,
                );
                if result_0 as u64 != 0 {
                    return result_0;
                }
                attIndex -= 1
            } else {
                /* deal with other prefixed names later */
                attIndex += 1;
                nPrefixes += 1;
                *(*attId).name.offset(-(1 as libc::c_int) as isize) =
                    2 as libc::c_int as crate::expat_external_h::XML_Char
            }
        } else {
            attIndex += 1
        }
        i += 1
    }
    /* set-up for XML_GetSpecifiedAttributeCount and XML_GetIdAttributeIndex */
    (*parser).m_nSpecifiedAtts = attIndex;
    if !(*elementType).idAtt.is_null()
        && *(*(*elementType).idAtt)
            .name
            .offset(-(1 as libc::c_int) as isize) as libc::c_int
            != 0
    {
        i = 0 as libc::c_int;
        while i < attIndex {
            if *appAtts.offset(i as isize)
                == (*(*elementType).idAtt).name as *const crate::expat_external_h::XML_Char
            {
                (*parser).m_idAttIndex = i;
                break;
            } else {
                i += 2 as libc::c_int
            }
        }
    } else {
        (*parser).m_idAttIndex = -(1 as libc::c_int)
    }
    /* do attribute defaulting */
    i = 0 as libc::c_int;
    while i < nDefaultAtts {
        let mut da: *const DEFAULT_ATTRIBUTE = (*elementType).defaultAtts.offset(i as isize);
        if *(*(*da).id).name.offset(-(1 as libc::c_int) as isize) == 0 && !(*da).value.is_null() {
            if !(*(*da).id).prefix.is_null() {
                if (*(*da).id).xmlns != 0 {
                    let mut result_1: crate::expat_h::XML_Error = addBinding(
                        parser,
                        (*(*da).id).prefix,
                        (*da).id,
                        (*da).value,
                        bindingsPtr,
                    );
                    if result_1 as u64 != 0 {
                        return result_1;
                    }
                } else {
                    *(*(*da).id).name.offset(-(1 as libc::c_int) as isize) =
                        2 as libc::c_int as crate::expat_external_h::XML_Char;
                    nPrefixes += 1;
                    let fresh11 = attIndex;
                    attIndex = attIndex + 1;
                    let ref mut fresh12 = *appAtts.offset(fresh11 as isize);
                    *fresh12 = (*(*da).id).name;
                    let fresh13 = attIndex;
                    attIndex = attIndex + 1;
                    let ref mut fresh14 = *appAtts.offset(fresh13 as isize);
                    *fresh14 = (*da).value
                }
            } else {
                *(*(*da).id).name.offset(-(1 as libc::c_int) as isize) =
                    1 as libc::c_int as crate::expat_external_h::XML_Char;
                let fresh15 = attIndex;
                attIndex = attIndex + 1;
                let ref mut fresh16 = *appAtts.offset(fresh15 as isize);
                *fresh16 = (*(*da).id).name;
                let fresh17 = attIndex;
                attIndex = attIndex + 1;
                let ref mut fresh18 = *appAtts.offset(fresh17 as isize);
                *fresh18 = (*da).value
            }
        }
        i += 1
    }
    let ref mut fresh19 = *appAtts.offset(attIndex as isize);
    *fresh19 = 0 as *const crate::expat_external_h::XML_Char;
    /* expand prefixed attribute names, check for duplicates,
    and clear flags that say whether attributes were specified */
    i = 0 as libc::c_int; /* hash table index */
    if nPrefixes != 0 {
        let mut j_0: libc::c_int = 0;
        let mut version: libc::c_ulong = (*parser).m_nsAttsVersion;
        let mut nsAttsSize: libc::c_int =
            (1 as libc::c_int) << (*parser).m_nsAttsPower as libc::c_int;
        let mut oldNsAttsPower: libc::c_uchar = (*parser).m_nsAttsPower;
        /* size of hash table must be at least 2 * (# of prefixed attributes) */
        if nPrefixes << 1 as libc::c_int >> (*parser).m_nsAttsPower as libc::c_int != 0 {
            /* true for m_nsAttsPower = 0 */
            let mut temp_0: *mut NS_ATT = 0 as *mut NS_ATT;
            loop
            /* hash table size must also be a power of 2 and >= 8 */
            {
                let fresh20 = (*parser).m_nsAttsPower;
                (*parser).m_nsAttsPower = (*parser).m_nsAttsPower.wrapping_add(1);
                if !(nPrefixes >> fresh20 as libc::c_int != 0) {
                    break;
                }
            }
            if ((*parser).m_nsAttsPower as libc::c_int) < 3 as libc::c_int {
                (*parser).m_nsAttsPower = 3 as libc::c_int as libc::c_uchar
            }
            nsAttsSize = (1 as libc::c_int) << (*parser).m_nsAttsPower as libc::c_int;
            temp_0 = (*parser)
                .m_mem
                .realloc_fcn
                .expect("non-null function pointer")(
                (*parser).m_nsAtts as *mut libc::c_void,
                (nsAttsSize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<NS_ATT>() as libc::c_ulong),
            ) as *mut NS_ATT;
            if temp_0.is_null() {
                /* Restore actual size of memory in m_nsAtts */
                (*parser).m_nsAttsPower = oldNsAttsPower;
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            }
            (*parser).m_nsAtts = temp_0;
            version = 0 as libc::c_int as libc::c_ulong
        }
        /* using a version flag saves us from initializing m_nsAtts every time */
        if version == 0 {
            /* initialize version flags when version wraps around */
            version = INIT_ATTS_VERSION as libc::c_ulong;
            j_0 = nsAttsSize;
            while j_0 != 0 as libc::c_int {
                j_0 -= 1;
                (*(*parser).m_nsAtts.offset(j_0 as isize)).version = version
            }
        }
        version = version.wrapping_sub(1);
        (*parser).m_nsAttsVersion = version;
        /* expand prefixed names and check for duplicates */
        while i < attIndex {
            let mut s: *const crate::expat_external_h::XML_Char = *appAtts.offset(i as isize);
            if *s.offset(-(1 as libc::c_int) as isize) as libc::c_int == 2 as libc::c_int {
                let mut id: *mut ATTRIBUTE_ID = 0 as *mut ATTRIBUTE_ID;
                let mut b: *const BINDING = 0 as *const BINDING;
                let mut uriHash: libc::c_ulong = 0;
                let mut sip_state: crate::siphash_h::siphash = crate::siphash_h::siphash {
                    v0: 0,
                    v1: 0,
                    v2: 0,
                    v3: 0,
                    buf: [0; 8],
                    p: 0 as *mut libc::c_uchar,
                    c: 0,
                };
                let mut sip_key: crate::siphash_h::sipkey = crate::siphash_h::sipkey { k: [0; 2] };
                copy_salt_to_sipkey(parser, &mut sip_key);
                sip24_init(&mut sip_state, &mut sip_key);
                /* clear flag */
                /* not prefixed */
                /* prefixed */
                *(s as *mut crate::expat_external_h::XML_Char)
                    .offset(-(1 as libc::c_int) as isize) =
                    0 as libc::c_int as crate::expat_external_h::XML_Char; /* clear flag */
                id = lookup(
                    parser,
                    &mut (*dtd).attributeIds,
                    s,
                    0 as libc::c_int as crate::stddef_h::size_t,
                ) as *mut ATTRIBUTE_ID;
                if id.is_null() || (*id).prefix.is_null() {
                    /* This code is walking through the appAtts array, dealing
                     * with (in this case) a prefixed attribute name.  To be in
                     * the array, the attribute must have already been bound, so
                     * has to have passed through the hash table lookup once
                     * already.  That implies that an entry for it already
                     * exists, so the lookup above will return a pointer to
                     * already allocated memory.  There is no opportunaity for
                     * the allocator to fail, so the condition above cannot be
                     * fulfilled.
                     *
                     * Since it is difficult to be certain that the above
                     * analysis is complete, we retain the test and merely
                     * remove the code from coverage tests.
                     */
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                    /* LCOV_EXCL_LINE */
                }
                b = (*(*id).prefix).binding;
                if b.is_null() {
                    return crate::expat_h::XML_ERROR_UNBOUND_PREFIX;
                }
                j_0 = 0 as libc::c_int;
                while j_0 < (*b).uriLen {
                    let c: crate::expat_external_h::XML_Char = *(*b).uri.offset(j_0 as isize);
                    if if (*parser).m_tempPool.ptr
                        == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                        && poolGrow(&mut (*parser).m_tempPool) == 0
                    {
                        0 as libc::c_int
                    } else {
                        let fresh21 = (*parser).m_tempPool.ptr;
                        (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                        *fresh21 = c;
                        1 as libc::c_int
                    } == 0
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    j_0 += 1
                }
                sip24_update(
                    &mut sip_state,
                    (*b).uri as *const libc::c_void,
                    ((*b).uriLen as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<crate::expat_external_h::XML_Char>()
                            as libc::c_ulong),
                );
                loop {
                    let fresh22 = s;
                    s = s.offset(1);
                    if !(*fresh22 as libc::c_int != 0x3a as libc::c_int) {
                        break;
                    }
                }
                sip24_update(
                    &mut sip_state,
                    s as *const libc::c_void,
                    keylen(s)
                        .wrapping_mul(::std::mem::size_of::<crate::expat_external_h::XML_Char>()
                            as libc::c_ulong),
                );
                loop {
                    /* copies null terminator */
                    if if (*parser).m_tempPool.ptr
                        == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                        && poolGrow(&mut (*parser).m_tempPool) == 0
                    {
                        0 as libc::c_int
                    } else {
                        let fresh23 = (*parser).m_tempPool.ptr;
                        (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                        *fresh23 = *s;
                        1 as libc::c_int
                    } == 0
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    let fresh24 = s;
                    s = s.offset(1);
                    if !(*fresh24 != 0) {
                        break;
                    }
                }
                uriHash = sip24_final(&mut sip_state);
                /* Check hash table for duplicate of expanded name (uriName).
                   Derived from code in lookup(parser, HASH_TABLE *table, ...).
                */
                let mut step: libc::c_uchar = 0 as libc::c_int as libc::c_uchar; /* index into hash table */
                let mut mask: libc::c_ulong = (nsAttsSize - 1 as libc::c_int) as libc::c_ulong;
                j_0 = (uriHash & mask) as libc::c_int;
                while (*(*parser).m_nsAtts.offset(j_0 as isize)).version == version {
                    /* for speed we compare stored hash values first */
                    if uriHash == (*(*parser).m_nsAtts.offset(j_0 as isize)).hash {
                        let mut s1: *const crate::expat_external_h::XML_Char =
                            (*parser).m_tempPool.start;
                        let mut s2: *const crate::expat_external_h::XML_Char =
                            (*(*parser).m_nsAtts.offset(j_0 as isize)).uriName;
                        /* s1 is null terminated, but not s2 */
                        while *s1 as libc::c_int == *s2 as libc::c_int
                            && *s1 as libc::c_int != 0 as libc::c_int
                        {
                            s1 = s1.offset(1);
                            s2 = s2.offset(1)
                        }
                        if *s1 as libc::c_int == 0 as libc::c_int {
                            return crate::expat_h::XML_ERROR_DUPLICATE_ATTRIBUTE;
                        }
                    }
                    if step == 0 {
                        step = ((uriHash & !mask)
                            >> (*parser).m_nsAttsPower as libc::c_int - 1 as libc::c_int
                            & mask >> 2 as libc::c_int
                            | 1 as libc::c_int as libc::c_ulong)
                            as libc::c_uchar
                    }
                    if j_0 < step as libc::c_int {
                        j_0 += nsAttsSize - step as libc::c_int
                    } else {
                        j_0 -= step as libc::c_int
                    };
                }
                if (*parser).m_ns_triplets != 0 {
                    /* append namespace separator and prefix */
                    *(*parser)
                        .m_tempPool
                        .ptr
                        .offset(-(1 as libc::c_int) as isize) = (*parser).m_namespaceSeparator;
                    s = (*(*b).prefix).name;
                    loop {
                        if if (*parser).m_tempPool.ptr
                            == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                            && poolGrow(&mut (*parser).m_tempPool) == 0
                        {
                            0 as libc::c_int
                        } else {
                            let fresh25 = (*parser).m_tempPool.ptr;
                            (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                            *fresh25 = *s;
                            1 as libc::c_int
                        } == 0
                        {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        let fresh26 = s;
                        s = s.offset(1);
                        if !(*fresh26 != 0) {
                            break;
                        }
                    }
                }
                /* store expanded name in attribute list */
                s = (*parser).m_tempPool.start;
                (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                let ref mut fresh27 = *appAtts.offset(i as isize);
                *fresh27 = s;
                /* fill empty slot with new version, uriName and hash value */
                (*(*parser).m_nsAtts.offset(j_0 as isize)).version = version;
                (*(*parser).m_nsAtts.offset(j_0 as isize)).hash = uriHash;
                let ref mut fresh28 = (*(*parser).m_nsAtts.offset(j_0 as isize)).uriName;
                *fresh28 = s;
                nPrefixes -= 1;
                if nPrefixes == 0 {
                    i += 2 as libc::c_int;
                    break;
                }
            } else {
                *(s as *mut crate::expat_external_h::XML_Char)
                    .offset(-(1 as libc::c_int) as isize) =
                    0 as libc::c_int as crate::expat_external_h::XML_Char
            }
            i += 2 as libc::c_int
        }
    }
    /* clear flags for the remaining attributes */
    while i < attIndex {
        *(*appAtts.offset(i as isize) as *mut crate::expat_external_h::XML_Char)
            .offset(-(1 as libc::c_int) as isize) =
            0 as libc::c_int as crate::expat_external_h::XML_Char;
        i += 2 as libc::c_int
    }
    binding = *bindingsPtr;
    while !binding.is_null() {
        *(*(*binding).attId)
            .name
            .offset(-(1 as libc::c_int) as isize) =
            0 as libc::c_int as crate::expat_external_h::XML_Char;
        binding = (*binding).nextTagBinding
    }
    if (*parser).m_ns == 0 {
        return crate::expat_h::XML_ERROR_NONE;
    }
    /* expand the element type name */
    if !(*elementType).prefix.is_null() {
        binding = (*(*elementType).prefix).binding;
        if binding.is_null() {
            return crate::expat_h::XML_ERROR_UNBOUND_PREFIX;
        }
        localPart = (*tagNamePtr).str_0;
        loop {
            let fresh29 = localPart;
            localPart = localPart.offset(1);
            if !(*fresh29 as libc::c_int != 0x3a as libc::c_int) {
                break;
            }
        }
    } else if !(*dtd).defaultPrefix.binding.is_null() {
        binding = (*dtd).defaultPrefix.binding;
        localPart = (*tagNamePtr).str_0
    } else {
        return crate::expat_h::XML_ERROR_NONE;
    }
    prefixLen = 0 as libc::c_int;
    if (*parser).m_ns_triplets as libc::c_int != 0 && !(*(*binding).prefix).name.is_null() {
        loop {
            let fresh30 = prefixLen;
            prefixLen = prefixLen + 1;
            if !(*(*(*binding).prefix).name.offset(fresh30 as isize) != 0) {
                break;
            }
        }
        /* prefixLen includes null terminator */
    } /* i includes null terminator */
    (*tagNamePtr).localPart = localPart;
    (*tagNamePtr).uriLen = (*binding).uriLen;
    (*tagNamePtr).prefix = (*(*binding).prefix).name;
    (*tagNamePtr).prefixLen = prefixLen;
    i = 0 as libc::c_int;
    loop {
        let fresh31 = i;
        i = i + 1;
        if !(*localPart.offset(fresh31 as isize) != 0) {
            break;
        }
    }
    n = i + (*binding).uriLen + prefixLen;
    if n > (*binding).uriAlloc {
        let mut p: *mut TAG = 0 as *mut TAG;
        uri = (*parser)
            .m_mem
            .malloc_fcn
            .expect("non-null function pointer")(
            ((n + 24 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong
                ),
        ) as *mut crate::expat_external_h::XML_Char;
        if uri.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        (*binding).uriAlloc = n + EXPAND_SPARE;
        crate::stdlib::memcpy(
            uri as *mut libc::c_void,
            (*binding).uri as *const libc::c_void,
            ((*binding).uriLen as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::expat_external_h::XML_Char,
            >() as libc::c_ulong),
        );
        p = (*parser).m_tagStack;
        while !p.is_null() {
            if (*p).name.str_0 == (*binding).uri as *const crate::expat_external_h::XML_Char {
                (*p).name.str_0 = uri
            }
            p = (*p).parent
        }
        (*parser).m_mem.free_fcn.expect("non-null function pointer")(
            (*binding).uri as *mut libc::c_void,
        );
        (*binding).uri = uri
    }
    /* if m_namespaceSeparator != '\0' then uri includes it already */
    uri = (*binding).uri.offset((*binding).uriLen as isize);
    crate::stdlib::memcpy(
        uri as *mut libc::c_void,
        localPart as *const libc::c_void,
        (i as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong,
        ),
    );
    /* we always have a namespace separator between localPart and prefix */
    if prefixLen != 0 {
        uri = uri.offset((i - 1 as libc::c_int) as isize); /* replace null terminator */
        *uri = (*parser).m_namespaceSeparator;
        crate::stdlib::memcpy(
            uri.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            (*(*binding).prefix).name as *const libc::c_void,
            (prefixLen as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::expat_external_h::XML_Char,
            >() as libc::c_ulong),
        );
    }
    (*tagNamePtr).str_0 = (*binding).uri;
    return crate::expat_h::XML_ERROR_NONE;
}
// Initialized in run_static_initializers
static mut xmlLen: libc::c_int = 0;
// Initialized in run_static_initializers
static mut xmlnsLen: libc::c_int = 0;
/* addBinding() overwrites the value of prefix->binding without checking.
   Therefore one must keep track of the old value outside of addBinding().
*/

unsafe extern "C" fn addBinding(
    mut parser: crate::expat_h::XML_Parser,
    mut prefix: *mut PREFIX,
    mut attId: *const ATTRIBUTE_ID,
    mut uri: *const crate::expat_external_h::XML_Char,
    mut bindingsPtr: *mut *mut BINDING,
) -> crate::expat_h::XML_Error {
    static mut xmlNamespace: [crate::expat_external_h::XML_Char; 37] = [
        crate::ascii_h::ASCII_h as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_t as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_t as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_p as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_COLON as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_PERIOD as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_3 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_PERIOD as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_o as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_r as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_g as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_X as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_M as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_L as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_1 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_9 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_9 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_8 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_n as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_a as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_m as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_e as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_s as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_p as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_a as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_c as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_e as crate::expat_external_h::XML_Char,
        '\u{0}' as i32 as crate::expat_external_h::XML_Char,
    ];
    static mut xmlnsNamespace: [crate::expat_external_h::XML_Char; 30] = [
        crate::ascii_h::ASCII_h as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_t as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_t as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_p as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_COLON as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_PERIOD as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_3 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_PERIOD as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_o as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_r as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_g as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_2 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_0 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_0 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_0 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_x as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_m as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_l as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_n as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_s as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        '\u{0}' as i32 as crate::expat_external_h::XML_Char,
    ];
    let mut mustBeXML: crate::expat_h::XML_Bool =
        crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    let mut isXML: crate::expat_h::XML_Bool = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
    let mut isXMLNS: crate::expat_h::XML_Bool =
        crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
    let mut b: *mut BINDING = 0 as *mut BINDING;
    let mut len: libc::c_int = 0;
    /* empty URI is only valid for default namespace per XML NS 1.0 (not 1.1) */
    if *uri as libc::c_int == '\u{0}' as i32 && !(*prefix).name.is_null() {
        return crate::expat_h::XML_ERROR_UNDECLARING_PREFIX;
    }
    if !(*prefix).name.is_null()
        && *(*prefix).name.offset(0 as libc::c_int as isize) as libc::c_int == 0x78 as libc::c_int
        && *(*prefix).name.offset(1 as libc::c_int as isize) as libc::c_int == 0x6d as libc::c_int
        && *(*prefix).name.offset(2 as libc::c_int as isize) as libc::c_int == 0x6c as libc::c_int
    {
        /* Not allowed to bind xmlns */
        if *(*prefix).name.offset(3 as libc::c_int as isize) as libc::c_int == 0x6e as libc::c_int
            && *(*prefix).name.offset(4 as libc::c_int as isize) as libc::c_int
                == 0x73 as libc::c_int
            && *(*prefix).name.offset(5 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            return crate::expat_h::XML_ERROR_RESERVED_PREFIX_XMLNS;
        }
        if *(*prefix).name.offset(3 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32 {
            mustBeXML = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool
        }
    }
    len = 0 as libc::c_int;
    while *uri.offset(len as isize) != 0 {
        if isXML as libc::c_int != 0
            && (len > xmlLen
                || *uri.offset(len as isize) as libc::c_int
                    != xmlNamespace[len as usize] as libc::c_int)
        {
            isXML = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
        }
        if mustBeXML == 0
            && isXMLNS as libc::c_int != 0
            && (len > xmlnsLen
                || *uri.offset(len as isize) as libc::c_int
                    != xmlnsNamespace[len as usize] as libc::c_int)
        {
            isXMLNS = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
        }
        len += 1
    }
    isXML = (isXML as libc::c_int != 0 && len == xmlLen) as libc::c_int as crate::expat_h::XML_Bool;
    isXMLNS =
        (isXMLNS as libc::c_int != 0 && len == xmlnsLen) as libc::c_int as crate::expat_h::XML_Bool;
    if mustBeXML as libc::c_int != isXML as libc::c_int {
        return if mustBeXML as libc::c_int != 0 {
            crate::expat_h::XML_ERROR_RESERVED_PREFIX_XML as libc::c_int
        } else {
            crate::expat_h::XML_ERROR_RESERVED_NAMESPACE_URI as libc::c_int
        } as crate::expat_h::XML_Error;
    }
    if isXMLNS != 0 {
        return crate::expat_h::XML_ERROR_RESERVED_NAMESPACE_URI;
    }
    if (*parser).m_namespaceSeparator != 0 {
        len += 1
    }
    if !(*parser).m_freeBindingList.is_null() {
        b = (*parser).m_freeBindingList;
        if len > (*b).uriAlloc {
            let mut temp: *mut crate::expat_external_h::XML_Char = (*parser)
                .m_mem
                .realloc_fcn
                .expect("non-null function pointer")(
                (*b).uri as *mut libc::c_void,
                (::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong)
                    .wrapping_mul((len + 24 as libc::c_int) as libc::c_ulong),
            )
                as *mut crate::expat_external_h::XML_Char;
            if temp.is_null() {
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            }
            (*b).uri = temp;
            (*b).uriAlloc = len + EXPAND_SPARE
        }
        (*parser).m_freeBindingList = (*b).nextTagBinding
    } else {
        b = (*parser)
            .m_mem
            .malloc_fcn
            .expect("non-null function pointer")(
            ::std::mem::size_of::<BINDING>() as libc::c_ulong
        ) as *mut BINDING;
        if b.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        (*b).uri = (*parser)
            .m_mem
            .malloc_fcn
            .expect("non-null function pointer")(
            (::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong)
                .wrapping_mul((len + 24 as libc::c_int) as libc::c_ulong),
        ) as *mut crate::expat_external_h::XML_Char;
        if (*b).uri.is_null() {
            (*parser).m_mem.free_fcn.expect("non-null function pointer")(b as *mut libc::c_void);
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        (*b).uriAlloc = len + EXPAND_SPARE
    }
    (*b).uriLen = len;
    crate::stdlib::memcpy(
        (*b).uri as *mut libc::c_void,
        uri as *const libc::c_void,
        (len as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong,
        ),
    );
    if (*parser).m_namespaceSeparator != 0 {
        *(*b).uri.offset((len - 1 as libc::c_int) as isize) = (*parser).m_namespaceSeparator
    }
    (*b).prefix = prefix;
    (*b).attId = attId;
    (*b).prevPrefixBinding = (*prefix).binding;
    /* NULL binding when default namespace undeclared */
    if *uri as libc::c_int == '\u{0}' as i32
        && prefix == &mut (*(*parser).m_dtd).defaultPrefix as *mut PREFIX
    {
        (*prefix).binding = crate::stddef_h::NULL as *mut BINDING
    } else {
        (*prefix).binding = b
    }
    (*b).nextTagBinding = *bindingsPtr;
    *bindingsPtr = b;
    /* if attId == NULL then we are not starting a namespace scope */
    if !attId.is_null() && (*parser).m_startNamespaceDeclHandler.is_some() {
        (*parser)
            .m_startNamespaceDeclHandler
            .expect("non-null function pointer")(
            (*parser).m_handlerArg,
            (*prefix).name,
            if !(*prefix).binding.is_null() {
                uri
            } else {
                0 as *const crate::expat_external_h::XML_Char
            },
        );
    }
    return crate::expat_h::XML_ERROR_NONE;
}
/* The idea here is to avoid using stack for each CDATA section when
   the whole file is parsed with one call.
*/

unsafe extern "C" fn cdataSectionProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
    mut endPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error = doCdataSection(
        parser,
        (*parser).m_encoding,
        &mut start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as libc::c_int as crate::expat_h::XML_Bool,
    );
    if result as libc::c_uint != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint {
        return result;
    }
    if !start.is_null() {
        if !(*parser).m_parentParser.is_null() {
            /* we are parsing an external entity */
            (*parser).m_processor = Some(externalEntityContentProcessor as Processor);
            return externalEntityContentProcessor(parser, start, end, endPtr);
        } else {
            (*parser).m_processor = Some(contentProcessor as Processor);
            return contentProcessor(parser, start, end, endPtr);
        }
    }
    return result;
}
/* startPtr gets set to non-null if the section is closed, and to null if
   the section is not yet closed.
*/

unsafe extern "C" fn doCdataSection(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut startPtr: *mut *const libc::c_char,
    mut end: *const libc::c_char,
    mut nextPtr: *mut *const libc::c_char,
    mut haveMore: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Error {
    let mut s: *const libc::c_char = *startPtr;
    let mut eventPP: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut eventEndPP: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if enc == (*parser).m_encoding {
        eventPP = &mut (*parser).m_eventPtr;
        *eventPP = s;
        eventEndPP = &mut (*parser).m_eventEndPtr
    } else {
        eventPP = &mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &mut (*(*parser).m_openInternalEntities).internalEventEndPtr
    }
    *eventPP = s;
    *startPtr = crate::stddef_h::NULL as *const libc::c_char;
    loop {
        let mut next: *const libc::c_char = 0 as *const libc::c_char;
        let mut tok: libc::c_int = (*enc).scanners[2 as libc::c_int as usize]
            .expect("non-null function pointer")(
            enc, s, end, &mut next
        );
        *eventEndPP = next;
        match tok {
            crate::src::lib::xmltok::XML_TOK_CDATA_SECT_CLOSE => {
                if (*parser).m_endCdataSectionHandler.is_some() {
                    (*parser)
                        .m_endCdataSectionHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    );
                } else if 0 as libc::c_int != 0 && (*parser).m_characterDataHandler.is_some() {
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_dataBuf,
                        0 as libc::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                *startPtr = next;
                *nextPtr = next;
                if (*parser).m_parsingStatus.parsing as libc::c_uint
                    == crate::expat_h::XML_FINISHED as libc::c_int as libc::c_uint
                {
                    return crate::expat_h::XML_ERROR_ABORTED;
                } else {
                    return crate::expat_h::XML_ERROR_NONE;
                }
                /* BEGIN disabled code */
                /* see comment under XML_TOK_CDATA_SECT_OPEN */
                /* END disabled code */
                /* LCOV_EXCL_STOP */
            }
            crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE => {
                if (*parser).m_characterDataHandler.is_some() {
                    let mut c: crate::expat_external_h::XML_Char =
                        0xa as libc::c_int as crate::expat_external_h::XML_Char;
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &mut c,
                        1 as libc::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            crate::src::lib::xmltok::XML_TOK_DATA_CHARS => {
                let mut charDataHandler: crate::expat_h::XML_CharacterDataHandler =
                    (*parser).m_characterDataHandler;
                if charDataHandler.is_some() {
                    if (*enc).isUtf8 == 0 {
                        loop {
                            let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
                            let convert_res: crate::src::lib::xmltok::XML_Convert_Result =
                                (*enc).utf8Convert.expect("non-null function pointer")(
                                    enc,
                                    &mut s,
                                    next,
                                    &mut dataPtr,
                                    (*parser).m_dataBufEnd as *mut ICHAR,
                                );
                            *eventEndPP = next;
                            charDataHandler.expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*parser).m_dataBuf,
                                dataPtr.wrapping_offset_from((*parser).m_dataBuf as *mut ICHAR)
                                    as libc::c_long as libc::c_int,
                            );
                            if convert_res as libc::c_uint
                                == crate::src::lib::xmltok::XML_CONVERT_COMPLETED as libc::c_int
                                    as libc::c_uint
                                || convert_res as libc::c_uint
                                    == crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE
                                        as libc::c_int
                                        as libc::c_uint
                            {
                                break;
                            }
                            *eventPP = s
                        }
                    } else {
                        charDataHandler.expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            s as *mut crate::expat_external_h::XML_Char,
                            (next as *mut crate::expat_external_h::XML_Char)
                                .wrapping_offset_from(s as *mut crate::expat_external_h::XML_Char)
                                as libc::c_long as libc::c_int,
                        );
                    }
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            crate::src::lib::xmltok::XML_TOK_INVALID => {
                *eventPP = next;
                return crate::expat_h::XML_ERROR_INVALID_TOKEN;
            }
            crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                return crate::expat_h::XML_ERROR_PARTIAL_CHAR;
            }
            crate::src::lib::xmltok::XML_TOK_PARTIAL | crate::src::lib::xmltok::XML_TOK_NONE => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                return crate::expat_h::XML_ERROR_UNCLOSED_CDATA_SECTION;
            }
            _ => {
                /* Every token returned by XmlCdataSectionTok() has its own
                 * explicit case, so this default case will never be executed.
                 * We retain it as a safety net and exclude it from the coverage
                 * statistics.
                 *
                 * LCOV_EXCL_START
                 */
                *eventPP = next;
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            }
        }
        s = next;
        *eventPP = s;
        match (*parser).m_parsingStatus.parsing as libc::c_uint {
            3 => {
                *nextPtr = next;
                return crate::expat_h::XML_ERROR_NONE;
            }
            2 => return crate::expat_h::XML_ERROR_ABORTED,
            _ => {}
        }
    }
    /* not reached */
}
/* The idea here is to avoid using stack for each IGNORE section when
   the whole file is parsed with one call.
*/

unsafe extern "C" fn ignoreSectionProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
    mut endPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error = doIgnoreSection(
        parser,
        (*parser).m_encoding,
        &mut start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as libc::c_int as crate::expat_h::XML_Bool,
    );
    if result as libc::c_uint != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint {
        return result;
    }
    if !start.is_null() {
        (*parser).m_processor = Some(prologProcessor as Processor);
        return prologProcessor(parser, start, end, endPtr);
    }
    return result;
}
/* startPtr gets set to non-null is the section is closed, and to null
   if the section is not yet closed.
*/

unsafe extern "C" fn doIgnoreSection(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut startPtr: *mut *const libc::c_char,
    mut end: *const libc::c_char,
    mut nextPtr: *mut *const libc::c_char,
    mut haveMore: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Error {
    let mut next: *const libc::c_char = 0 as *const libc::c_char;
    let mut tok: libc::c_int = 0;
    let mut s: *const libc::c_char = *startPtr;
    let mut eventPP: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut eventEndPP: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if enc == (*parser).m_encoding {
        eventPP = &mut (*parser).m_eventPtr;
        *eventPP = s;
        eventEndPP = &mut (*parser).m_eventEndPtr
    } else {
        /* It's not entirely clear, but it seems the following two lines
         * of code cannot be executed.  The only occasions on which 'enc'
         * is not 'encoding' are when this function is called
         * from the internal entity processing, and IGNORE sections are an
         * error in internal entities.
         *
         * Since it really isn't clear that this is true, we keep the code
         * and just remove it from our coverage tests.
         *
         * LCOV_EXCL_START
         */
        eventPP = &mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &mut (*(*parser).m_openInternalEntities).internalEventEndPtr
        /* LCOV_EXCL_STOP */
    }
    *eventPP = s;
    *startPtr = crate::stddef_h::NULL as *const libc::c_char;
    tok = (*enc).scanners[3 as libc::c_int as usize].expect("non-null function pointer")(
        enc, s, end, &mut next,
    );
    *eventEndPP = next;
    match tok {
        crate::src::lib::xmltok::XML_TOK_IGNORE_SECT => {
            if (*parser).m_defaultHandler.is_some() {
                reportDefault(parser, enc, s, next);
            }
            *startPtr = next;
            *nextPtr = next;
            if (*parser).m_parsingStatus.parsing as libc::c_uint
                == crate::expat_h::XML_FINISHED as libc::c_int as libc::c_uint
            {
                return crate::expat_h::XML_ERROR_ABORTED;
            } else {
                return crate::expat_h::XML_ERROR_NONE;
            }
            /* LCOV_EXCL_STOP */
        }
        crate::src::lib::xmltok::XML_TOK_INVALID => {
            *eventPP = next; /* XML_ERROR_UNCLOSED_IGNORE_SECTION */
            return crate::expat_h::XML_ERROR_INVALID_TOKEN;
        }
        crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR => {
            if haveMore != 0 {
                *nextPtr = s;
                return crate::expat_h::XML_ERROR_NONE;
            }
            return crate::expat_h::XML_ERROR_PARTIAL_CHAR;
        }
        crate::src::lib::xmltok::XML_TOK_PARTIAL | crate::src::lib::xmltok::XML_TOK_NONE => {
            if haveMore != 0 {
                *nextPtr = s;
                return crate::expat_h::XML_ERROR_NONE;
            }
            return crate::expat_h::XML_ERROR_SYNTAX;
        }
        _ => {
            /* All of the tokens that XmlIgnoreSectionTok() returns have
             * explicit cases to handle them, so this default case is never
             * executed.  We keep it as a safety net anyway, and remove it
             * from our test coverage statistics.
             *
             * LCOV_EXCL_START
             */
            *eventPP = next;
            return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
        }
    };
    /* not reached */
}
/* XML_DTD */

unsafe extern "C" fn initializeEncoding(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Error {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    s = (*parser).m_protocolEncodingName;
    if if (*parser).m_ns as libc::c_int != 0 {
        Some(
            crate::src::lib::xmltok::xmltok_ns_c::XmlInitEncodingNS
                as unsafe extern "C" fn(
                    _: *mut crate::src::lib::xmltok::INIT_ENCODING,
                    _: *mut *const crate::src::lib::xmltok::ENCODING,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        )
    } else {
        Some(
            crate::src::lib::xmltok::xmltok_ns_c::XmlInitEncoding
                as unsafe extern "C" fn(
                    _: *mut crate::src::lib::xmltok::INIT_ENCODING,
                    _: *mut *const crate::src::lib::xmltok::ENCODING,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        )
    }
    .expect("non-null function pointer")(
        &mut (*parser).m_initEncoding,
        &mut (*parser).m_encoding,
        s,
    ) != 0
    {
        return crate::expat_h::XML_ERROR_NONE;
    }
    return handleUnknownEncoding(parser, (*parser).m_protocolEncodingName);
}

unsafe extern "C" fn processXmlDecl(
    mut parser: crate::expat_h::XML_Parser,
    mut isGeneralTextEntity: libc::c_int,
    mut s: *const libc::c_char,
    mut next: *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut encodingName: *const libc::c_char = crate::stddef_h::NULL as *const libc::c_char;
    let mut storedEncName: *const crate::expat_external_h::XML_Char =
        crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    let mut newEncoding: *const crate::src::lib::xmltok::ENCODING =
        crate::stddef_h::NULL as *const crate::src::lib::xmltok::ENCODING;
    let mut version: *const libc::c_char = crate::stddef_h::NULL as *const libc::c_char;
    let mut versionend: *const libc::c_char = 0 as *const libc::c_char;
    let mut storedversion: *const crate::expat_external_h::XML_Char =
        crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    let mut standalone: libc::c_int = -(1 as libc::c_int);
    if if (*parser).m_ns as libc::c_int != 0 {
        Some(
            crate::src::lib::xmltok::xmltok_ns_c::XmlParseXmlDeclNS
                as unsafe extern "C" fn(
                    _: libc::c_int,
                    _: *const crate::src::lib::xmltok::ENCODING,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *mut *const libc::c_char,
                    _: *mut *const libc::c_char,
                    _: *mut *const libc::c_char,
                    _: *mut *const libc::c_char,
                    _: *mut *const crate::src::lib::xmltok::ENCODING,
                    _: *mut libc::c_int,
                ) -> libc::c_int,
        )
    } else {
        Some(
            crate::src::lib::xmltok::xmltok_ns_c::XmlParseXmlDecl
                as unsafe extern "C" fn(
                    _: libc::c_int,
                    _: *const crate::src::lib::xmltok::ENCODING,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *mut *const libc::c_char,
                    _: *mut *const libc::c_char,
                    _: *mut *const libc::c_char,
                    _: *mut *const libc::c_char,
                    _: *mut *const crate::src::lib::xmltok::ENCODING,
                    _: *mut libc::c_int,
                ) -> libc::c_int,
        )
    }
    .expect("non-null function pointer")(
        isGeneralTextEntity,
        (*parser).m_encoding,
        s,
        next,
        &mut (*parser).m_eventPtr,
        &mut version,
        &mut versionend,
        &mut encodingName,
        &mut newEncoding,
        &mut standalone,
    ) == 0
    {
        if isGeneralTextEntity != 0 {
            return crate::expat_h::XML_ERROR_TEXT_DECL;
        } else {
            return crate::expat_h::XML_ERROR_XML_DECL;
        }
    }
    if isGeneralTextEntity == 0 && standalone == 1 as libc::c_int {
        (*(*parser).m_dtd).standalone = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
        if (*parser).m_paramEntityParsing as libc::c_uint
            == crate::expat_h::XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE as libc::c_int
                as libc::c_uint
        {
            (*parser).m_paramEntityParsing = crate::expat_h::XML_PARAM_ENTITY_PARSING_NEVER
        }
        /* XML_DTD */
    }
    if (*parser).m_xmlDeclHandler.is_some() {
        if !encodingName.is_null() {
            storedEncName = poolStoreString(
                &mut (*parser).m_temp2Pool,
                (*parser).m_encoding,
                encodingName,
                encodingName.offset((*(*parser).m_encoding)
                    .nameLength
                    .expect("non-null function pointer")(
                    (*parser).m_encoding, encodingName
                ) as isize),
            );
            if storedEncName.is_null() {
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            }
            (*parser).m_temp2Pool.start = (*parser).m_temp2Pool.ptr
        }
        if !version.is_null() {
            storedversion = poolStoreString(
                &mut (*parser).m_temp2Pool,
                (*parser).m_encoding,
                version,
                versionend.offset(-((*(*parser).m_encoding).minBytesPerChar as isize)),
            );
            if storedversion.is_null() {
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            }
        }
        (*parser)
            .m_xmlDeclHandler
            .expect("non-null function pointer")(
            (*parser).m_handlerArg,
            storedversion,
            storedEncName,
            standalone,
        );
    } else if (*parser).m_defaultHandler.is_some() {
        reportDefault(parser, (*parser).m_encoding, s, next);
    }
    if (*parser).m_protocolEncodingName.is_null() {
        if !newEncoding.is_null() {
            /* Check that the specified encoding does not conflict with what
             * the parser has already deduced.  Do we have the same number
             * of bytes in the smallest representation of a character?  If
             * this is UTF-16, is it the same endianness?
             */
            if (*newEncoding).minBytesPerChar != (*(*parser).m_encoding).minBytesPerChar
                || (*newEncoding).minBytesPerChar == 2 as libc::c_int
                    && newEncoding != (*parser).m_encoding
            {
                (*parser).m_eventPtr = encodingName;
                return crate::expat_h::XML_ERROR_INCORRECT_ENCODING;
            }
            (*parser).m_encoding = newEncoding
        } else if !encodingName.is_null() {
            let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
            if storedEncName.is_null() {
                storedEncName = poolStoreString(
                    &mut (*parser).m_temp2Pool,
                    (*parser).m_encoding,
                    encodingName,
                    encodingName.offset((*(*parser).m_encoding)
                        .nameLength
                        .expect("non-null function pointer")(
                        (*parser).m_encoding, encodingName
                    ) as isize),
                );
                if storedEncName.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
            }
            result = handleUnknownEncoding(parser, storedEncName);
            poolClear(&mut (*parser).m_temp2Pool);
            if result as libc::c_uint
                == crate::expat_h::XML_ERROR_UNKNOWN_ENCODING as libc::c_int as libc::c_uint
            {
                (*parser).m_eventPtr = encodingName
            }
            return result;
        }
    }
    if !storedEncName.is_null() || !storedversion.is_null() {
        poolClear(&mut (*parser).m_temp2Pool);
    }
    return crate::expat_h::XML_ERROR_NONE;
}

unsafe extern "C" fn handleUnknownEncoding(
    mut parser: crate::expat_h::XML_Parser,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Error {
    if (*parser).m_unknownEncodingHandler.is_some() {
        let mut info: crate::expat_h::XML_Encoding = crate::expat_h::XML_Encoding {
            map: [0; 256],
            data: 0 as *mut libc::c_void,
            convert: None,
            release: None,
        };
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            info.map[i as usize] = -(1 as libc::c_int);
            i += 1
        }
        info.convert = ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char) -> libc::c_int,
            >,
        >(crate::stddef_h::NULL as libc::intptr_t);
        info.data = crate::stddef_h::NULL as *mut libc::c_void;
        info.release = ::std::mem::transmute::<
            libc::intptr_t,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(crate::stddef_h::NULL as libc::intptr_t);
        if (*parser)
            .m_unknownEncodingHandler
            .expect("non-null function pointer")(
            (*parser).m_unknownEncodingHandlerData,
            encodingName,
            &mut info,
        ) != 0
        {
            let mut enc: *mut crate::src::lib::xmltok::ENCODING =
                0 as *mut crate::src::lib::xmltok::ENCODING;
            (*parser).m_unknownEncodingMem = (*parser)
                .m_mem
                .malloc_fcn
                .expect("non-null function pointer")(
                crate::src::lib::xmltok::XmlSizeOfUnknownEncoding() as crate::stddef_h::size_t,
            );
            if (*parser).m_unknownEncodingMem.is_null() {
                if info.release.is_some() {
                    info.release.expect("non-null function pointer")(info.data);
                }
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            }
            enc = if (*parser).m_ns as libc::c_int != 0 {
                Some(
                    crate::src::lib::xmltok::XmlInitUnknownEncodingNS
                        as unsafe extern "C" fn(
                            _: *mut libc::c_void,
                            _: *mut libc::c_int,
                            _: crate::src::lib::xmltok::CONVERTER,
                            _: *mut libc::c_void,
                        )
                            -> *mut crate::src::lib::xmltok::ENCODING,
                )
            } else {
                Some(
                    crate::src::lib::xmltok::XmlInitUnknownEncoding
                        as unsafe extern "C" fn(
                            _: *mut libc::c_void,
                            _: *mut libc::c_int,
                            _: crate::src::lib::xmltok::CONVERTER,
                            _: *mut libc::c_void,
                        )
                            -> *mut crate::src::lib::xmltok::ENCODING,
                )
            }
            .expect("non-null function pointer")(
                (*parser).m_unknownEncodingMem,
                info.map.as_mut_ptr(),
                info.convert,
                info.data,
            );
            if !enc.is_null() {
                (*parser).m_unknownEncodingData = info.data;
                (*parser).m_unknownEncodingRelease = info.release;
                (*parser).m_encoding = enc;
                return crate::expat_h::XML_ERROR_NONE;
            }
        }
        if info.release.is_some() {
            info.release.expect("non-null function pointer")(info.data);
        }
    }
    return crate::expat_h::XML_ERROR_UNKNOWN_ENCODING;
}

unsafe extern "C" fn prologInitProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const libc::c_char,
    mut end: *const libc::c_char,
    mut nextPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error = initializeEncoding(parser);
    if result as libc::c_uint != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint {
        return result;
    }
    (*parser).m_processor = Some(prologProcessor as Processor);
    return prologProcessor(parser, s, end, nextPtr);
}

unsafe extern "C" fn externalParEntInitProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const libc::c_char,
    mut end: *const libc::c_char,
    mut nextPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error = initializeEncoding(parser);
    if result as libc::c_uint != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint {
        return result;
    }
    /* we know now that XML_Parse(Buffer) has been called,
    so we consider the external parameter entity read */
    (*(*parser).m_dtd).paramEntityRead = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
    if (*parser).m_prologState.inEntityValue != 0 {
        (*parser).m_processor = Some(entityValueInitProcessor as Processor);
        return entityValueInitProcessor(parser, s, end, nextPtr);
    } else {
        (*parser).m_processor = Some(externalParEntProcessor as Processor);
        return externalParEntProcessor(parser, s, end, nextPtr);
    };
}

unsafe extern "C" fn entityValueInitProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const libc::c_char,
    mut end: *const libc::c_char,
    mut nextPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut tok: libc::c_int = 0;
    let mut start: *const libc::c_char = s;
    let mut next: *const libc::c_char = start;
    (*parser).m_eventPtr = start;
    loop {
        tok = (*(*parser).m_encoding).scanners[0 as libc::c_int as usize]
            .expect("non-null function pointer")(
            (*parser).m_encoding, start, end, &mut next
        );
        (*parser).m_eventEndPtr = next;
        if tok <= 0 as libc::c_int {
            if (*parser).m_parsingStatus.finalBuffer == 0
                && tok != crate::src::lib::xmltok::XML_TOK_INVALID
            {
                *nextPtr = s;
                return crate::expat_h::XML_ERROR_NONE;
            }
            match tok {
                crate::src::lib::xmltok::XML_TOK_INVALID => {
                    return crate::expat_h::XML_ERROR_INVALID_TOKEN
                }
                crate::src::lib::xmltok::XML_TOK_PARTIAL => {
                    return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN
                }
                crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR => {
                    return crate::expat_h::XML_ERROR_PARTIAL_CHAR
                }
                crate::src::lib::xmltok::XML_TOK_NONE | _ => {}
            }
            /* found end of entity value - can store it now */
            return storeEntityValue(parser, (*parser).m_encoding, s, end);
        } else {
            if tok == crate::src::lib::xmltok::XML_TOK_XML_DECL {
                let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
                result = processXmlDecl(parser, 0 as libc::c_int, start, next);
                if result as libc::c_uint
                    != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return result;
                }
                /* At this point, m_parsingStatus.parsing cannot be XML_SUSPENDED.  For
                 * that to happen, a parameter entity parsing handler must have attempted
                 * to suspend the parser, which fails and raises an error.  The parser can
                 * be aborted, but can't be suspended.
                 */
                if (*parser).m_parsingStatus.parsing as libc::c_uint
                    == crate::expat_h::XML_FINISHED as libc::c_int as libc::c_uint
                {
                    return crate::expat_h::XML_ERROR_ABORTED;
                }
                *nextPtr = next;
                /* stop scanning for text declaration - we found one */
                (*parser).m_processor = Some(entityValueProcessor as Processor);
                return entityValueProcessor(parser, next, end, nextPtr);
            } else {
                /* If we are at the end of the buffer, this would cause XmlPrologTok to
                   return XML_TOK_NONE on the next call, which would then cause the
                   function to exit with *nextPtr set to s - that is what we want for other
                   tokens, but not for the BOM - we would rather like to skip it;
                   then, when this routine is entered the next time, XmlPrologTok will
                   return XML_TOK_INVALID, since the BOM is still in the buffer
                */
                if tok == crate::src::lib::xmltok::XML_TOK_BOM
                    && next == end
                    && (*parser).m_parsingStatus.finalBuffer == 0
                {
                    *nextPtr = next;
                    return crate::expat_h::XML_ERROR_NONE;
                } else {
                    /* If we get this token, we have the start of what might be a
                       normal tag, but not a declaration (i.e. it doesn't begin with
                       "<!").  In a DTD context, that isn't legal.
                    */
                    if tok == crate::src::lib::xmltok::XML_TOK_INSTANCE_START {
                        *nextPtr = next;
                        return crate::expat_h::XML_ERROR_SYNTAX;
                    }
                }
            }
        }
        start = next;
        (*parser).m_eventPtr = start
    }
}

unsafe extern "C" fn externalParEntProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const libc::c_char,
    mut end: *const libc::c_char,
    mut nextPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut next: *const libc::c_char = s;
    let mut tok: libc::c_int = 0;
    tok = (*(*parser).m_encoding).scanners[0 as libc::c_int as usize]
        .expect("non-null function pointer")((*parser).m_encoding, s, end, &mut next);
    if tok <= 0 as libc::c_int {
        if (*parser).m_parsingStatus.finalBuffer == 0
            && tok != crate::src::lib::xmltok::XML_TOK_INVALID
        {
            *nextPtr = s;
            return crate::expat_h::XML_ERROR_NONE;
        }
        match tok {
            crate::src::lib::xmltok::XML_TOK_INVALID => {
                return crate::expat_h::XML_ERROR_INVALID_TOKEN
            }
            crate::src::lib::xmltok::XML_TOK_PARTIAL => {
                return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN
            }
            crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR => {
                return crate::expat_h::XML_ERROR_PARTIAL_CHAR
            }
            crate::src::lib::xmltok::XML_TOK_NONE | _ => {}
        }
    } else if tok == crate::src::lib::xmltok::XML_TOK_BOM {
        s = next;
        tok = (*(*parser).m_encoding).scanners[0 as libc::c_int as usize]
            .expect("non-null function pointer")(
            (*parser).m_encoding, s, end, &mut next
        )
    }
    (*parser).m_processor = Some(prologProcessor as Processor);
    return doProlog(
        parser,
        (*parser).m_encoding,
        s,
        end,
        tok,
        next,
        nextPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as libc::c_int as crate::expat_h::XML_Bool,
        crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool,
    );
}

unsafe extern "C" fn entityValueProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const libc::c_char,
    mut end: *const libc::c_char,
    mut nextPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut start: *const libc::c_char = s;
    let mut next: *const libc::c_char = s;
    let mut enc: *const crate::src::lib::xmltok::ENCODING = (*parser).m_encoding;
    let mut tok: libc::c_int = 0;
    loop {
        tok = (*enc).scanners[0 as libc::c_int as usize].expect("non-null function pointer")(
            enc, start, end, &mut next,
        );
        if tok <= 0 as libc::c_int {
            if (*parser).m_parsingStatus.finalBuffer == 0
                && tok != crate::src::lib::xmltok::XML_TOK_INVALID
            {
                *nextPtr = s;
                return crate::expat_h::XML_ERROR_NONE;
            }
            match tok {
                crate::src::lib::xmltok::XML_TOK_INVALID => {
                    return crate::expat_h::XML_ERROR_INVALID_TOKEN
                }
                crate::src::lib::xmltok::XML_TOK_PARTIAL => {
                    return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN
                }
                crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR => {
                    return crate::expat_h::XML_ERROR_PARTIAL_CHAR
                }
                crate::src::lib::xmltok::XML_TOK_NONE | _ => {}
            }
            /* This would cause the next stage, i.e. doProlog to be passed XML_TOK_BOM.
               However, when parsing an external subset, doProlog will not accept a BOM
               as valid, and report a syntax error, so we have to skip the BOM
            */
            /* found end of entity value - can store it now */
            return storeEntityValue(parser, enc, s, end);
        }
        start = next
    }
}
/* XML_DTD */

unsafe extern "C" fn prologProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const libc::c_char,
    mut end: *const libc::c_char,
    mut nextPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut next: *const libc::c_char = s;
    let mut tok: libc::c_int =
        (*(*parser).m_encoding).scanners[0 as libc::c_int as usize]
            .expect("non-null function pointer")((*parser).m_encoding, s, end, &mut next);
    return doProlog(
        parser,
        (*parser).m_encoding,
        s,
        end,
        tok,
        next,
        nextPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as libc::c_int as crate::expat_h::XML_Bool,
        crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool,
    );
}

unsafe extern "C" fn doProlog(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut s: *const libc::c_char,
    mut end: *const libc::c_char,
    mut tok: libc::c_int,
    mut next: *const libc::c_char,
    mut nextPtr: *mut *const libc::c_char,
    mut haveMore: crate::expat_h::XML_Bool,
    mut allowClosingDoctype: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Error {
    let mut current_block: u64;
    static mut externalSubsetName: [crate::expat_external_h::XML_Char; 2] = [
        crate::ascii_h::ASCII_HASH as crate::expat_external_h::XML_Char,
        '\u{0}' as i32 as crate::expat_external_h::XML_Char,
    ];
    /* XML_DTD */
    static mut atypeCDATA: [crate::expat_external_h::XML_Char; 6] = [
        crate::ascii_h::ASCII_C as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_D as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_A as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_A as crate::expat_external_h::XML_Char,
        '\u{0}' as i32 as crate::expat_external_h::XML_Char,
    ];
    static mut atypeID: [crate::expat_external_h::XML_Char; 3] = [
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_D as crate::expat_external_h::XML_Char,
        '\u{0}' as i32 as crate::expat_external_h::XML_Char,
    ];
    static mut atypeIDREF: [crate::expat_external_h::XML_Char; 6] = [
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_D as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_R as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_F as crate::expat_external_h::XML_Char,
        '\u{0}' as i32 as crate::expat_external_h::XML_Char,
    ];
    static mut atypeIDREFS: [crate::expat_external_h::XML_Char; 7] = [
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_D as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_R as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_F as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_S as crate::expat_external_h::XML_Char,
        '\u{0}' as i32 as crate::expat_external_h::XML_Char,
    ];
    static mut atypeENTITY: [crate::expat_external_h::XML_Char; 7] = [
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_Y as crate::expat_external_h::XML_Char,
        '\u{0}' as i32 as crate::expat_external_h::XML_Char,
    ];
    static mut atypeENTITIES: [crate::expat_external_h::XML_Char; 9] = [
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_S as crate::expat_external_h::XML_Char,
        '\u{0}' as i32 as crate::expat_external_h::XML_Char,
    ];
    static mut atypeNMTOKEN: [crate::expat_external_h::XML_Char; 8] = [
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_M as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_O as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_K as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        '\u{0}' as i32 as crate::expat_external_h::XML_Char,
    ];
    static mut atypeNMTOKENS: [crate::expat_external_h::XML_Char; 9] = [
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_M as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_O as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_K as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_S as crate::expat_external_h::XML_Char,
        '\u{0}' as i32 as crate::expat_external_h::XML_Char,
    ];
    static mut notationPrefix: [crate::expat_external_h::XML_Char; 10] = [
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_O as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_A as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_O as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_LPAREN as crate::expat_external_h::XML_Char,
        '\u{0}' as i32 as crate::expat_external_h::XML_Char,
    ];
    static mut enumValueSep: [crate::expat_external_h::XML_Char; 2] = [
        crate::ascii_h::ASCII_PIPE as crate::expat_external_h::XML_Char,
        '\u{0}' as i32 as crate::expat_external_h::XML_Char,
    ];
    static mut enumValueStart: [crate::expat_external_h::XML_Char; 2] = [
        crate::ascii_h::ASCII_LPAREN as crate::expat_external_h::XML_Char,
        '\u{0}' as i32 as crate::expat_external_h::XML_Char,
    ];
    /* save one level of indirection */
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut eventPP: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut eventEndPP: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut quant: crate::expat_h::XML_Content_Quant = crate::expat_h::XML_CQUANT_NONE;
    if enc == (*parser).m_encoding {
        eventPP = &mut (*parser).m_eventPtr;
        eventEndPP = &mut (*parser).m_eventEndPtr
    } else {
        eventPP = &mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &mut (*(*parser).m_openInternalEntities).internalEventEndPtr
    }
    loop {
        let mut role: libc::c_int = 0;
        let mut handleDefault: crate::expat_h::XML_Bool =
            crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
        *eventPP = s;
        *eventEndPP = next;
        if tok <= 0 as libc::c_int {
            if haveMore as libc::c_int != 0 && tok != crate::src::lib::xmltok::XML_TOK_INVALID {
                *nextPtr = s;
                return crate::expat_h::XML_ERROR_NONE;
            }
            match tok {
                crate::src::lib::xmltok::XML_TOK_INVALID => {
                    *eventPP = next;
                    return crate::expat_h::XML_ERROR_INVALID_TOKEN;
                }
                crate::src::lib::xmltok::XML_TOK_PARTIAL => {
                    return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN
                }
                crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR => {
                    return crate::expat_h::XML_ERROR_PARTIAL_CHAR
                }

                -15 => tok = -tok,
                crate::src::lib::xmltok::XML_TOK_NONE => {
                    /* for internal PE NOT referenced between declarations */
                    if enc != (*parser).m_encoding
                        && (*(*parser).m_openInternalEntities).betweenDecl == 0
                    {
                        *nextPtr = s;
                        return crate::expat_h::XML_ERROR_NONE;
                    }
                    /* WFC: PE Between Declarations - must check that PE contains
                       complete markup, not only for external PEs, but also for
                       internal PEs if the reference occurs between declarations.
                    */
                    if (*parser).m_isParamEntity as libc::c_int != 0 || enc != (*parser).m_encoding
                    {
                        if (*parser)
                            .m_prologState
                            .handler
                            .expect("non-null function pointer")(
                            &mut (*parser).m_prologState,
                            -(4 as libc::c_int),
                            end,
                            end,
                            enc,
                        ) == crate::src::lib::xmlrole::XML_ROLE_ERROR as libc::c_int
                        {
                            return crate::expat_h::XML_ERROR_INCOMPLETE_PE;
                        }
                        *nextPtr = s;
                        return crate::expat_h::XML_ERROR_NONE;
                    }
                    /* XML_DTD */
                    return crate::expat_h::XML_ERROR_NO_ELEMENTS;
                }
                _ => {
                    tok = -tok; /* end of big switch */
                    next = end
                }
            }
        } /* always initialize to NULL */
        role = (*parser)
            .m_prologState
            .handler
            .expect("non-null function pointer")(
            &mut (*parser).m_prologState, tok, s, next, enc
        );
        match role {
            1 => {
                let mut result: crate::expat_h::XML_Error =
                    processXmlDecl(parser, 0 as libc::c_int, s, next);
                if result as libc::c_uint
                    != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return result;
                }
                enc = (*parser).m_encoding;
                handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                current_block = 1553878188884632965;
            }
            4 => {
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    (*parser).m_doctypeName =
                        poolStoreString(&mut (*parser).m_tempPool, enc, s, next);
                    if (*parser).m_doctypeName.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    (*parser).m_doctypePubid =
                        crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                (*parser).m_doctypeSysid =
                    crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
                current_block = 1553878188884632965;
            }
            7 => {
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    (*parser)
                        .m_startDoctypeDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_doctypeName,
                        (*parser).m_doctypeSysid,
                        (*parser).m_doctypePubid,
                        1 as libc::c_int,
                    );
                    (*parser).m_doctypeName =
                        crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
                    poolClear(&mut (*parser).m_tempPool);
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            57 => {
                let mut result_0: crate::expat_h::XML_Error =
                    processXmlDecl(parser, 1 as libc::c_int, s, next);
                if result_0 as libc::c_uint
                    != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return result_0;
                }
                enc = (*parser).m_encoding;
                handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                current_block = 1553878188884632965;
            }
            6 => {
                /* XML_DTD */
                (*parser).m_useForeignDTD = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                (*parser).m_declEntity = lookup(
                    parser,
                    &mut (*dtd).paramEntities,
                    externalSubsetName.as_ptr(),
                    ::std::mem::size_of::<ENTITY>() as libc::c_ulong,
                ) as *mut ENTITY;
                if (*parser).m_declEntity.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                /* XML_DTD */
                (*dtd).hasParamEntityRefs = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    let mut pubId: *mut crate::expat_external_h::XML_Char =
                        0 as *mut crate::expat_external_h::XML_Char;
                    if (*enc).isPublicId.expect("non-null function pointer")(enc, s, next, eventPP)
                        == 0
                    {
                        return crate::expat_h::XML_ERROR_PUBLICID;
                    }
                    pubId = poolStoreString(
                        &mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if pubId.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    normalizePublicId(pubId);
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    (*parser).m_doctypePubid = pubId;
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                    current_block = 9007411418488376351;
                } else {
                    current_block = 926243229934402080;
                }
            }
            14 => {
                current_block = 926243229934402080;
            }
            8 => {
                if allowClosingDoctype as libc::c_int != crate::expat_h::XML_TRUE {
                    /* Must not close doctype from within expanded parameter entities */
                    return crate::expat_h::XML_ERROR_INVALID_TOKEN;
                }
                if !(*parser).m_doctypeName.is_null() {
                    (*parser)
                        .m_startDoctypeDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_doctypeName,
                        (*parser).m_doctypeSysid,
                        (*parser).m_doctypePubid,
                        0 as libc::c_int,
                    );
                    poolClear(&mut (*parser).m_tempPool);
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                /* parser->m_doctypeSysid will be non-NULL in the case of a previous
                   XML_ROLE_DOCTYPE_SYSTEM_ID, even if parser->m_startDoctypeDeclHandler
                   was not set, indicating an external subset
                */
                if !(*parser).m_doctypeSysid.is_null()
                    || (*parser).m_useForeignDTD as libc::c_int != 0
                {
                    let mut hadParamEntityRefs: crate::expat_h::XML_Bool =
                        (*dtd).hasParamEntityRefs;
                    (*dtd).hasParamEntityRefs =
                        crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                    if (*parser).m_paramEntityParsing as libc::c_uint != 0
                        && (*parser).m_externalEntityRefHandler.is_some()
                    {
                        let mut entity: *mut ENTITY = lookup(
                            parser,
                            &mut (*dtd).paramEntities,
                            externalSubsetName.as_ptr(),
                            ::std::mem::size_of::<ENTITY>() as libc::c_ulong,
                        ) as *mut ENTITY;
                        if entity.is_null() {
                            /* end of DTD - no need to update dtd->keepProcessing */
                            /* The external subset name "#" will have already been
                             * inserted into the hash table at the start of the
                             * external entity parsing, so no allocation will happen
                             * and lookup() cannot fail.
                             */
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                            /* LCOV_EXCL_LINE */
                        }
                        if (*parser).m_useForeignDTD != 0 {
                            (*entity).base = (*parser).m_curBase
                        }
                        (*dtd).paramEntityRead =
                            crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                        if (*parser)
                            .m_externalEntityRefHandler
                            .expect("non-null function pointer")(
                            (*parser).m_externalEntityRefHandlerArg,
                            0 as *const crate::expat_external_h::XML_Char,
                            (*entity).base,
                            (*entity).systemId,
                            (*entity).publicId,
                        ) == 0
                        {
                            return crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                        }
                        if (*dtd).paramEntityRead != 0 {
                            if (*dtd).standalone == 0
                                && (*parser).m_notStandaloneHandler.is_some()
                                && (*parser)
                                    .m_notStandaloneHandler
                                    .expect("non-null function pointer")(
                                    (*parser).m_handlerArg
                                ) == 0
                            {
                                return crate::expat_h::XML_ERROR_NOT_STANDALONE;
                            }
                        } else if (*parser).m_doctypeSysid.is_null() {
                            (*dtd).hasParamEntityRefs = hadParamEntityRefs
                        }
                    }
                    (*parser).m_useForeignDTD =
                        crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                /* if we didn't read the foreign DTD then this means that there
                   is no external subset and we must reset dtd->hasParamEntityRefs
                */
                /* XML_DTD */
                if (*parser).m_endDoctypeDeclHandler.is_some() {
                    (*parser)
                        .m_endDoctypeDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    );
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            2 => {
                /* if there is no DOCTYPE declaration then now is the
                   last chance to read the foreign DTD
                */
                if (*parser).m_useForeignDTD != 0 {
                    let mut hadParamEntityRefs_0: crate::expat_h::XML_Bool =
                        (*dtd).hasParamEntityRefs;
                    (*dtd).hasParamEntityRefs =
                        crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                    if (*parser).m_paramEntityParsing as libc::c_uint != 0
                        && (*parser).m_externalEntityRefHandler.is_some()
                    {
                        let mut entity_0: *mut ENTITY = lookup(
                            parser,
                            &mut (*dtd).paramEntities,
                            externalSubsetName.as_ptr(),
                            ::std::mem::size_of::<ENTITY>() as libc::c_ulong,
                        ) as *mut ENTITY;
                        if entity_0.is_null() {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        (*entity_0).base = (*parser).m_curBase;
                        (*dtd).paramEntityRead =
                            crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                        if (*parser)
                            .m_externalEntityRefHandler
                            .expect("non-null function pointer")(
                            (*parser).m_externalEntityRefHandlerArg,
                            0 as *const crate::expat_external_h::XML_Char,
                            (*entity_0).base,
                            (*entity_0).systemId,
                            (*entity_0).publicId,
                        ) == 0
                        {
                            return crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                        }
                        if (*dtd).paramEntityRead != 0 {
                            if (*dtd).standalone == 0
                                && (*parser).m_notStandaloneHandler.is_some()
                                && (*parser)
                                    .m_notStandaloneHandler
                                    .expect("non-null function pointer")(
                                    (*parser).m_handlerArg
                                ) == 0
                            {
                                return crate::expat_h::XML_ERROR_NOT_STANDALONE;
                            }
                        } else {
                            /* end of DTD - no need to update dtd->keepProcessing */
                            /* if we didn't read the foreign DTD then this means that there
                               is no external subset and we must reset dtd->hasParamEntityRefs
                            */
                            (*dtd).hasParamEntityRefs = hadParamEntityRefs_0
                        }
                    }
                }
                /* XML_DTD */
                (*parser).m_processor = Some(contentProcessor as Processor);
                return contentProcessor(parser, s, end, nextPtr);
            }
            34 => {
                (*parser).m_declElementType = getElementType(parser, enc, s, next);
                if (*parser).m_declElementType.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                current_block = 6455255476181645667;
            }
            22 => {
                (*parser).m_declAttributeId = getAttributeId(parser, enc, s, next);
                if (*parser).m_declAttributeId.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                (*parser).m_declAttributeIsCdata =
                    crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                (*parser).m_declAttributeType =
                    crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
                (*parser).m_declAttributeIsId =
                    crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                current_block = 6455255476181645667;
            }
            23 => {
                (*parser).m_declAttributeIsCdata =
                    crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                (*parser).m_declAttributeType = atypeCDATA.as_ptr();
                current_block = 6455255476181645667;
            }
            24 => {
                (*parser).m_declAttributeIsId =
                    crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                (*parser).m_declAttributeType = atypeID.as_ptr();
                current_block = 6455255476181645667;
            }
            25 => {
                (*parser).m_declAttributeType = atypeIDREF.as_ptr();
                current_block = 6455255476181645667;
            }
            26 => {
                (*parser).m_declAttributeType = atypeIDREFS.as_ptr();
                current_block = 6455255476181645667;
            }
            27 => {
                (*parser).m_declAttributeType = atypeENTITY.as_ptr();
                current_block = 6455255476181645667;
            }
            28 => {
                (*parser).m_declAttributeType = atypeENTITIES.as_ptr();
                current_block = 6455255476181645667;
            }
            29 => {
                (*parser).m_declAttributeType = atypeNMTOKEN.as_ptr();
                current_block = 6455255476181645667;
            }
            30 => {
                (*parser).m_declAttributeType = atypeNMTOKENS.as_ptr();
                current_block = 6455255476181645667;
            }
            31 | 32 => {
                if (*dtd).keepProcessing as libc::c_int != 0
                    && (*parser).m_attlistDeclHandler.is_some()
                {
                    let mut prefix: *const crate::expat_external_h::XML_Char =
                        0 as *const crate::expat_external_h::XML_Char;
                    if !(*parser).m_declAttributeType.is_null() {
                        prefix = enumValueSep.as_ptr()
                    } else {
                        prefix = if role
                            == crate::src::lib::xmlrole::XML_ROLE_ATTRIBUTE_NOTATION_VALUE
                                as libc::c_int
                        {
                            notationPrefix.as_ptr()
                        } else {
                            enumValueStart.as_ptr()
                        }
                    }
                    if poolAppendString(&mut (*parser).m_tempPool, prefix).is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    if poolAppend(&mut (*parser).m_tempPool, enc, s, next).is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_declAttributeType = (*parser).m_tempPool.start;
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            35 | 36 => {
                if (*dtd).keepProcessing != 0 {
                    if defineAttribute(
                        (*parser).m_declElementType,
                        (*parser).m_declAttributeId,
                        (*parser).m_declAttributeIsCdata,
                        (*parser).m_declAttributeIsId,
                        0 as *const crate::expat_external_h::XML_Char,
                        parser,
                    ) == 0
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    if (*parser).m_attlistDeclHandler.is_some()
                        && !(*parser).m_declAttributeType.is_null()
                    {
                        if *(*parser).m_declAttributeType as libc::c_int == 0x28 as libc::c_int
                            || *(*parser).m_declAttributeType as libc::c_int == 0x4e as libc::c_int
                                && *(*parser)
                                    .m_declAttributeType
                                    .offset(1 as libc::c_int as isize)
                                    as libc::c_int
                                    == 0x4f as libc::c_int
                        {
                            /* Enumerated or Notation type */
                            if (if (*parser).m_tempPool.ptr
                                == (*parser).m_tempPool.end
                                    as *mut crate::expat_external_h::XML_Char
                                && poolGrow(&mut (*parser).m_tempPool) == 0
                            {
                                0 as libc::c_int
                            } else {
                                let fresh32 = (*parser).m_tempPool.ptr;
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                *fresh32 = 0x29 as libc::c_int as crate::expat_external_h::XML_Char;
                                1 as libc::c_int
                            }) == 0
                                || (if (*parser).m_tempPool.ptr
                                    == (*parser).m_tempPool.end
                                        as *mut crate::expat_external_h::XML_Char
                                    && poolGrow(&mut (*parser).m_tempPool) == 0
                                {
                                    0 as libc::c_int
                                } else {
                                    let fresh33 = (*parser).m_tempPool.ptr;
                                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                    *fresh33 = '\u{0}' as i32 as crate::expat_external_h::XML_Char;
                                    1 as libc::c_int
                                }) == 0
                            {
                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                            }
                            (*parser).m_declAttributeType = (*parser).m_tempPool.start;
                            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr
                        }
                        *eventEndPP = s;
                        (*parser)
                            .m_attlistDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declElementType).name,
                            (*(*parser).m_declAttributeId).name,
                            (*parser).m_declAttributeType,
                            0 as *const crate::expat_external_h::XML_Char,
                            (role
                                == crate::src::lib::xmlrole::XML_ROLE_REQUIRED_ATTRIBUTE_VALUE
                                    as libc::c_int) as libc::c_int,
                        );
                        poolClear(&mut (*parser).m_tempPool);
                        handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                    }
                }
                current_block = 1553878188884632965;
            }
            37 | 38 => {
                if (*dtd).keepProcessing != 0 {
                    let mut attVal: *const crate::expat_external_h::XML_Char =
                        0 as *const crate::expat_external_h::XML_Char;
                    let mut result_1: crate::expat_h::XML_Error = storeAttributeValue(
                        parser,
                        enc,
                        (*parser).m_declAttributeIsCdata,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                        &mut (*dtd).pool,
                    );
                    if result_1 as u64 != 0 {
                        return result_1;
                    }
                    attVal = (*dtd).pool.start;
                    (*dtd).pool.start = (*dtd).pool.ptr;
                    /* ID attributes aren't allowed to have a default */
                    if defineAttribute(
                        (*parser).m_declElementType,
                        (*parser).m_declAttributeId,
                        (*parser).m_declAttributeIsCdata,
                        crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool,
                        attVal,
                        parser,
                    ) == 0
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    if (*parser).m_attlistDeclHandler.is_some()
                        && !(*parser).m_declAttributeType.is_null()
                    {
                        if *(*parser).m_declAttributeType as libc::c_int == 0x28 as libc::c_int
                            || *(*parser).m_declAttributeType as libc::c_int == 0x4e as libc::c_int
                                && *(*parser)
                                    .m_declAttributeType
                                    .offset(1 as libc::c_int as isize)
                                    as libc::c_int
                                    == 0x4f as libc::c_int
                        {
                            /* Enumerated or Notation type */
                            if (if (*parser).m_tempPool.ptr
                                == (*parser).m_tempPool.end
                                    as *mut crate::expat_external_h::XML_Char
                                && poolGrow(&mut (*parser).m_tempPool) == 0
                            {
                                0 as libc::c_int
                            } else {
                                let fresh34 = (*parser).m_tempPool.ptr;
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                *fresh34 = 0x29 as libc::c_int as crate::expat_external_h::XML_Char;
                                1 as libc::c_int
                            }) == 0
                                || (if (*parser).m_tempPool.ptr
                                    == (*parser).m_tempPool.end
                                        as *mut crate::expat_external_h::XML_Char
                                    && poolGrow(&mut (*parser).m_tempPool) == 0
                                {
                                    0 as libc::c_int
                                } else {
                                    let fresh35 = (*parser).m_tempPool.ptr;
                                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                    *fresh35 = '\u{0}' as i32 as crate::expat_external_h::XML_Char;
                                    1 as libc::c_int
                                }) == 0
                            {
                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                            }
                            (*parser).m_declAttributeType = (*parser).m_tempPool.start;
                            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr
                        }
                        *eventEndPP = s;
                        (*parser)
                            .m_attlistDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declElementType).name,
                            (*(*parser).m_declAttributeId).name,
                            (*parser).m_declAttributeType,
                            attVal,
                            (role
                                == crate::src::lib::xmlrole::XML_ROLE_FIXED_ATTRIBUTE_VALUE
                                    as libc::c_int) as libc::c_int,
                        );
                        poolClear(&mut (*parser).m_tempPool);
                        handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                    }
                }
                current_block = 1553878188884632965;
            }
            12 => {
                if (*dtd).keepProcessing != 0 {
                    let mut result_2: crate::expat_h::XML_Error = storeEntityValue(
                        parser,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if !(*parser).m_declEntity.is_null() {
                        (*(*parser).m_declEntity).textPtr = (*dtd).entityValuePool.start;
                        (*(*parser).m_declEntity).textLen = (*dtd)
                            .entityValuePool
                            .ptr
                            .wrapping_offset_from((*dtd).entityValuePool.start)
                            as libc::c_long
                            as libc::c_int;
                        (*dtd).entityValuePool.start = (*dtd).entityValuePool.ptr;
                        if (*parser).m_entityDeclHandler.is_some() {
                            *eventEndPP = s;
                            (*parser)
                                .m_entityDeclHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*(*parser).m_declEntity).name,
                                (*(*parser).m_declEntity).is_param as libc::c_int,
                                (*(*parser).m_declEntity).textPtr,
                                (*(*parser).m_declEntity).textLen,
                                (*parser).m_curBase,
                                0 as *const crate::expat_external_h::XML_Char,
                                0 as *const crate::expat_external_h::XML_Char,
                                0 as *const crate::expat_external_h::XML_Char,
                            );
                            handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                        }
                    } else {
                        (*dtd).entityValuePool.ptr = (*dtd).entityValuePool.start
                    }
                    if result_2 as libc::c_uint
                        != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint
                    {
                        return result_2;
                    }
                }
                current_block = 1553878188884632965;
            }
            5 => {
                (*parser).m_useForeignDTD = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                /* XML_DTD */
                (*dtd).hasParamEntityRefs = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    (*parser).m_doctypeSysid = poolStoreString(
                        &mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if (*parser).m_doctypeSysid.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                } else {
                    /* use externalSubsetName to make parser->m_doctypeSysid non-NULL
                    for the case where no parser->m_startDoctypeDeclHandler is set */
                    (*parser).m_doctypeSysid = externalSubsetName.as_ptr()
                }
                /* XML_DTD */
                if (*dtd).standalone == 0
                    && (*parser).m_paramEntityParsing as u64 == 0
                    && (*parser).m_notStandaloneHandler.is_some()
                    && (*parser)
                        .m_notStandaloneHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    ) == 0
                {
                    return crate::expat_h::XML_ERROR_NOT_STANDALONE;
                }
                /* XML_DTD */
                if (*parser).m_declEntity.is_null() {
                    (*parser).m_declEntity = lookup(
                        parser,
                        &mut (*dtd).paramEntities,
                        externalSubsetName.as_ptr(),
                        ::std::mem::size_of::<ENTITY>() as libc::c_ulong,
                    ) as *mut ENTITY;
                    if (*parser).m_declEntity.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*(*parser).m_declEntity).publicId =
                        crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char
                }
                current_block = 15307276507984219638;
            }
            13 => {
                current_block = 15307276507984219638;
            }
            15 => {
                if (*dtd).keepProcessing as libc::c_int != 0
                    && !(*parser).m_declEntity.is_null()
                    && (*parser).m_entityDeclHandler.is_some()
                {
                    *eventEndPP = s;
                    (*parser)
                        .m_entityDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*(*parser).m_declEntity).name,
                        (*(*parser).m_declEntity).is_param as libc::c_int,
                        0 as *const crate::expat_external_h::XML_Char,
                        0 as libc::c_int,
                        (*(*parser).m_declEntity).base,
                        (*(*parser).m_declEntity).systemId,
                        (*(*parser).m_declEntity).publicId,
                        0 as *const crate::expat_external_h::XML_Char,
                    );
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            16 => {
                if (*dtd).keepProcessing as libc::c_int != 0 && !(*parser).m_declEntity.is_null() {
                    (*(*parser).m_declEntity).notation =
                        poolStoreString(&mut (*dtd).pool, enc, s, next);
                    if (*(*parser).m_declEntity).notation.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*dtd).pool.start = (*dtd).pool.ptr;
                    if (*parser).m_unparsedEntityDeclHandler.is_some() {
                        *eventEndPP = s;
                        (*parser)
                            .m_unparsedEntityDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declEntity).name,
                            (*(*parser).m_declEntity).base,
                            (*(*parser).m_declEntity).systemId,
                            (*(*parser).m_declEntity).publicId,
                            (*(*parser).m_declEntity).notation,
                        );
                        handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                    } else if (*parser).m_entityDeclHandler.is_some() {
                        *eventEndPP = s;
                        (*parser)
                            .m_entityDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declEntity).name,
                            0 as libc::c_int,
                            0 as *const crate::expat_external_h::XML_Char,
                            0 as libc::c_int,
                            (*(*parser).m_declEntity).base,
                            (*(*parser).m_declEntity).systemId,
                            (*(*parser).m_declEntity).publicId,
                            (*(*parser).m_declEntity).notation,
                        );
                        handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                    }
                }
                current_block = 1553878188884632965;
            }
            9 => {
                if (*enc)
                    .predefinedEntityName
                    .expect("non-null function pointer")(enc, s, next)
                    != 0
                {
                    (*parser).m_declEntity = crate::stddef_h::NULL as *mut ENTITY
                } else if (*dtd).keepProcessing != 0 {
                    let mut name: *const crate::expat_external_h::XML_Char =
                        poolStoreString(&mut (*dtd).pool, enc, s, next);
                    if name.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_declEntity = lookup(
                        parser,
                        &mut (*dtd).generalEntities,
                        name,
                        ::std::mem::size_of::<ENTITY>() as libc::c_ulong,
                    ) as *mut ENTITY;
                    if (*parser).m_declEntity.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    if (*(*parser).m_declEntity).name != name {
                        (*dtd).pool.ptr = (*dtd).pool.start;
                        (*parser).m_declEntity = crate::stddef_h::NULL as *mut ENTITY
                    } else {
                        (*dtd).pool.start = (*dtd).pool.ptr;
                        (*(*parser).m_declEntity).publicId =
                            crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
                        (*(*parser).m_declEntity).is_param =
                            crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                        /* if we have a parent parser or are reading an internal parameter
                           entity, then the entity declaration is not considered "internal"
                        */
                        (*(*parser).m_declEntity).is_internal =
                            !(!(*parser).m_parentParser.is_null()
                                || !(*parser).m_openInternalEntities.is_null())
                                as libc::c_int
                                as crate::expat_h::XML_Bool;
                        if (*parser).m_entityDeclHandler.is_some() {
                            handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                        }
                    }
                } else {
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    (*parser).m_declEntity = crate::stddef_h::NULL as *mut ENTITY
                }
                current_block = 1553878188884632965;
            }
            10 => {
                if (*dtd).keepProcessing != 0 {
                    let mut name_0: *const crate::expat_external_h::XML_Char =
                        poolStoreString(&mut (*dtd).pool, enc, s, next);
                    if name_0.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_declEntity = lookup(
                        parser,
                        &mut (*dtd).paramEntities,
                        name_0,
                        ::std::mem::size_of::<ENTITY>() as libc::c_ulong,
                    ) as *mut ENTITY;
                    if (*parser).m_declEntity.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    if (*(*parser).m_declEntity).name != name_0 {
                        (*dtd).pool.ptr = (*dtd).pool.start;
                        (*parser).m_declEntity = crate::stddef_h::NULL as *mut ENTITY
                    } else {
                        (*dtd).pool.start = (*dtd).pool.ptr;
                        (*(*parser).m_declEntity).publicId =
                            crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
                        (*(*parser).m_declEntity).is_param =
                            crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                        /* if we have a parent parser or are reading an internal parameter
                           entity, then the entity declaration is not considered "internal"
                        */
                        (*(*parser).m_declEntity).is_internal =
                            !(!(*parser).m_parentParser.is_null()
                                || !(*parser).m_openInternalEntities.is_null())
                                as libc::c_int
                                as crate::expat_h::XML_Bool;
                        if (*parser).m_entityDeclHandler.is_some() {
                            handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                        }
                    }
                } else {
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    (*parser).m_declEntity = crate::stddef_h::NULL as *mut ENTITY
                }
                current_block = 1553878188884632965;
            }
            18 => {
                (*parser).m_declNotationPublicId =
                    crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
                (*parser).m_declNotationName =
                    crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
                if (*parser).m_notationDeclHandler.is_some() {
                    (*parser).m_declNotationName =
                        poolStoreString(&mut (*parser).m_tempPool, enc, s, next);
                    if (*parser).m_declNotationName.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            21 => {
                if (*enc).isPublicId.expect("non-null function pointer")(enc, s, next, eventPP) == 0
                {
                    return crate::expat_h::XML_ERROR_PUBLICID;
                }
                if !(*parser).m_declNotationName.is_null() {
                    /* means m_notationDeclHandler != NULL */
                    let mut tem_0: *mut crate::expat_external_h::XML_Char = poolStoreString(
                        &mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if tem_0.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    normalizePublicId(tem_0);
                    (*parser).m_declNotationPublicId = tem_0;
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            19 => {
                if !(*parser).m_declNotationName.is_null()
                    && (*parser).m_notationDeclHandler.is_some()
                {
                    let mut systemId: *const crate::expat_external_h::XML_Char = poolStoreString(
                        &mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if systemId.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    *eventEndPP = s;
                    (*parser)
                        .m_notationDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_declNotationName,
                        (*parser).m_curBase,
                        systemId,
                        (*parser).m_declNotationPublicId,
                    );
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                poolClear(&mut (*parser).m_tempPool);
                current_block = 1553878188884632965;
            }
            20 => {
                if !(*parser).m_declNotationPublicId.is_null()
                    && (*parser).m_notationDeclHandler.is_some()
                {
                    *eventEndPP = s;
                    (*parser)
                        .m_notationDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_declNotationName,
                        (*parser).m_curBase,
                        0 as *const crate::expat_external_h::XML_Char,
                        (*parser).m_declNotationPublicId,
                    );
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                poolClear(&mut (*parser).m_tempPool);
                current_block = 1553878188884632965;
            }
            -1 => {
                match tok {
                    crate::src::lib::xmltok::XML_TOK_PARAM_ENTITY_REF => {
                        /* PE references in internal subset are
                        not allowed within declarations. */
                        return crate::expat_h::XML_ERROR_PARAM_ENTITY_REF;
                    }
                    crate::src::lib::xmltok::XML_TOK_XML_DECL => {
                        return crate::expat_h::XML_ERROR_MISPLACED_XML_PI
                    }
                    _ => return crate::expat_h::XML_ERROR_SYNTAX,
                }
            }
            58 => {
                let mut result_3: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                result_3 = doIgnoreSection(parser, enc, &mut next, end, nextPtr, haveMore);
                if result_3 as libc::c_uint
                    != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return result_3;
                } else {
                    if next.is_null() {
                        (*parser).m_processor = Some(ignoreSectionProcessor as Processor);
                        return result_3;
                    }
                }
                current_block = 1553878188884632965;
            }
            44 => {
                /* XML_DTD */
                if (*parser).m_prologState.level >= (*parser).m_groupSize {
                    if (*parser).m_groupSize != 0 {
                        (*parser).m_groupSize = (*parser)
                            .m_groupSize
                            .wrapping_mul(2 as libc::c_int as libc::c_uint);
                        let new_connector: *mut libc::c_char = (*parser)
                            .m_mem
                            .realloc_fcn
                            .expect("non-null function pointer")(
                            (*parser).m_groupConnector as *mut libc::c_void,
                            (*parser).m_groupSize as crate::stddef_h::size_t,
                        )
                            as *mut libc::c_char;
                        if new_connector.is_null() {
                            (*parser).m_groupSize = (*parser)
                                .m_groupSize
                                .wrapping_div(2 as libc::c_int as libc::c_uint);
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        (*parser).m_groupConnector = new_connector;
                        if !(*dtd).scaffIndex.is_null() {
                            let new_scaff_index: *mut libc::c_int = (*parser)
                                .m_mem
                                .realloc_fcn
                                .expect("non-null function pointer")(
                                (*dtd).scaffIndex as *mut libc::c_void,
                                ((*parser).m_groupSize as libc::c_ulong).wrapping_mul(
                                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                ),
                            )
                                as *mut libc::c_int;
                            if new_scaff_index.is_null() {
                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                            }
                            (*dtd).scaffIndex = new_scaff_index
                        }
                    } else {
                        (*parser).m_groupSize = 32 as libc::c_int as libc::c_uint;
                        (*parser).m_groupConnector = (*parser)
                            .m_mem
                            .malloc_fcn
                            .expect("non-null function pointer")(
                            (*parser).m_groupSize as crate::stddef_h::size_t,
                        ) as *mut libc::c_char;
                        if (*parser).m_groupConnector.is_null() {
                            (*parser).m_groupSize = 0 as libc::c_int as libc::c_uint;
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                    }
                }
                *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize) =
                    0 as libc::c_int as libc::c_char;
                if (*dtd).in_eldecl != 0 {
                    let mut myindex: libc::c_int = nextScaffoldPart(parser);
                    if myindex < 0 as libc::c_int {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    if !(*dtd).scaffIndex.is_null() {
                    } else {
                        crate::stdlib::__assert_fail(b"dtd->scaffIndex != NULL\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/lib/xmlparse.c\x00"
                                          as *const u8 as *const libc::c_char,
                                      4790 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 136],
                                                                &[libc::c_char; 136]>(b"enum XML_Error doProlog(XML_Parser, const ENCODING *, const char *, const char *, int, const char *, const char **, XML_Bool, XML_Bool)\x00")).as_ptr());
                    }
                    *(*dtd).scaffIndex.offset((*dtd).scaffLevel as isize) = myindex;
                    (*dtd).scaffLevel += 1;
                    (*(*dtd).scaffold.offset(myindex as isize)).type_0 =
                        crate::expat_h::XML_CTYPE_SEQ;
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                    }
                }
                current_block = 1553878188884632965;
            }
            50 => {
                if *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize)
                    as libc::c_int
                    == crate::ascii_h::ASCII_PIPE
                {
                    return crate::expat_h::XML_ERROR_SYNTAX;
                }
                *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize) =
                    crate::ascii_h::ASCII_COMMA as libc::c_char;
                if (*dtd).in_eldecl as libc::c_int != 0 && (*parser).m_elementDeclHandler.is_some()
                {
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            49 => {
                if *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize)
                    as libc::c_int
                    == crate::ascii_h::ASCII_COMMA
                {
                    return crate::expat_h::XML_ERROR_SYNTAX;
                }
                if (*dtd).in_eldecl as libc::c_int != 0
                    && *(*parser)
                        .m_groupConnector
                        .offset((*parser).m_prologState.level as isize)
                        == 0
                    && (*(*dtd).scaffold.offset(
                        *(*dtd)
                            .scaffIndex
                            .offset(((*dtd).scaffLevel - 1 as libc::c_int) as isize)
                            as isize,
                    ))
                    .type_0 as libc::c_uint
                        != crate::expat_h::XML_CTYPE_MIXED as libc::c_int as libc::c_uint
                {
                    (*(*dtd).scaffold.offset(
                        *(*dtd)
                            .scaffIndex
                            .offset(((*dtd).scaffLevel - 1 as libc::c_int) as isize)
                            as isize,
                    ))
                    .type_0 = crate::expat_h::XML_CTYPE_CHOICE;
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                    }
                }
                *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize) =
                    crate::ascii_h::ASCII_PIPE as libc::c_char;
                current_block = 1553878188884632965;
            }
            60 | 59 => {
                (*dtd).hasParamEntityRefs = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                if (*parser).m_paramEntityParsing as u64 == 0 {
                    (*dtd).keepProcessing = (*dtd).standalone;
                    current_block = 10770532911212200937;
                } else {
                    let mut name_1: *const crate::expat_external_h::XML_Char =
                        0 as *const crate::expat_external_h::XML_Char;
                    let mut entity_1: *mut ENTITY = 0 as *mut ENTITY;
                    name_1 = poolStoreString(
                        &mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if name_1.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    entity_1 = lookup(
                        parser,
                        &mut (*dtd).paramEntities,
                        name_1,
                        0 as libc::c_int as crate::stddef_h::size_t,
                    ) as *mut ENTITY;
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    /* first, determine if a check for an existing declaration is needed;
                       if yes, check that the entity exists, and that it is internal,
                       otherwise call the skipped entity handler
                    */
                    if (*parser).m_prologState.documentEntity != 0
                        && (if (*dtd).standalone as libc::c_int != 0 {
                            (*parser).m_openInternalEntities.is_null() as libc::c_int
                        } else {
                            ((*dtd).hasParamEntityRefs == 0) as libc::c_int
                        }) != 0
                    {
                        if entity_1.is_null() {
                            return crate::expat_h::XML_ERROR_UNDEFINED_ENTITY;
                        } else {
                            if (*entity_1).is_internal == 0 {
                                /* It's hard to exhaustively search the code to be sure,
                                 * but there doesn't seem to be a way of executing the
                                 * following line.  There are two cases:
                                 *
                                 * If 'standalone' is false, the DTD must have no
                                 * parameter entities or we wouldn't have passed the outer
                                 * 'if' statement.  That measn the only entity in the hash
                                 * table is the external subset name "#" which cannot be
                                 * given as a parameter entity name in XML syntax, so the
                                 * lookup must have returned NULL and we don't even reach
                                 * the test for an internal entity.
                                 *
                                 * If 'standalone' is true, it does not seem to be
                                 * possible to create entities taking this code path that
                                 * are not internal entities, so fail the test above.
                                 *
                                 * Because this analysis is very uncertain, the code is
                                 * being left in place and merely removed from the
                                 * coverage test statistics.
                                 */
                                return crate::expat_h::XML_ERROR_ENTITY_DECLARED_IN_PE;
                                /* LCOV_EXCL_LINE */
                            }
                        }
                        current_block = 13351260019855268589;
                    } else if entity_1.is_null() {
                        (*dtd).keepProcessing = (*dtd).standalone;
                        /* cannot report skipped entities in declarations */
                        if role
                            == crate::src::lib::xmlrole::XML_ROLE_PARAM_ENTITY_REF as libc::c_int
                            && (*parser).m_skippedEntityHandler.is_some()
                        {
                            (*parser)
                                .m_skippedEntityHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                name_1,
                                1 as libc::c_int,
                            );
                            handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                        }
                        current_block = 1553878188884632965;
                    } else {
                        current_block = 13351260019855268589;
                    }
                    match current_block {
                        1553878188884632965 => {}
                        _ => {
                            if (*entity_1).open != 0 {
                                return crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if !(*entity_1).textPtr.is_null() {
                                let mut result_4: crate::expat_h::XML_Error =
                                    crate::expat_h::XML_ERROR_NONE;
                                let mut betweenDecl: crate::expat_h::XML_Bool = if role
                                    == crate::src::lib::xmlrole::XML_ROLE_PARAM_ENTITY_REF
                                        as libc::c_int
                                {
                                    crate::expat_h::XML_TRUE
                                } else {
                                    crate::expat_h::XML_FALSE
                                }
                                    as crate::expat_h::XML_Bool;
                                result_4 = processInternalEntity(parser, entity_1, betweenDecl);
                                if result_4 as libc::c_uint
                                    != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint
                                {
                                    return result_4;
                                }
                                handleDefault =
                                    crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                                current_block = 1553878188884632965;
                            } else if (*parser).m_externalEntityRefHandler.is_some() {
                                (*dtd).paramEntityRead =
                                    crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                                (*entity_1).open =
                                    crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                                if (*parser)
                                    .m_externalEntityRefHandler
                                    .expect("non-null function pointer")(
                                    (*parser).m_externalEntityRefHandlerArg,
                                    0 as *const crate::expat_external_h::XML_Char,
                                    (*entity_1).base,
                                    (*entity_1).systemId,
                                    (*entity_1).publicId,
                                ) == 0
                                {
                                    (*entity_1).open =
                                        crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                                    return crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                }
                                (*entity_1).open =
                                    crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                                handleDefault =
                                    crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                                if (*dtd).paramEntityRead == 0 {
                                    (*dtd).keepProcessing = (*dtd).standalone;
                                    current_block = 1553878188884632965;
                                } else {
                                    current_block = 10770532911212200937;
                                }
                            } else {
                                (*dtd).keepProcessing = (*dtd).standalone;
                                current_block = 1553878188884632965;
                            }
                        }
                    }
                }
                match current_block {
                    1553878188884632965 => {}
                    _ => {
                        /* XML_DTD */
                        if (*dtd).standalone == 0
                            && (*parser).m_notStandaloneHandler.is_some()
                            && (*parser)
                                .m_notStandaloneHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg
                            ) == 0
                        {
                            return crate::expat_h::XML_ERROR_NOT_STANDALONE;
                        }
                        current_block = 1553878188884632965;
                    }
                }
            }
            40 => {
                /* Element declaration stuff */
                if (*parser).m_elementDeclHandler.is_some() {
                    (*parser).m_declElementType = getElementType(parser, enc, s, next);
                    if (*parser).m_declElementType.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*dtd).scaffLevel = 0 as libc::c_int;
                    (*dtd).scaffCount = 0 as libc::c_int as libc::c_uint;
                    (*dtd).in_eldecl = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            41 | 42 => {
                if (*dtd).in_eldecl != 0 {
                    if (*parser).m_elementDeclHandler.is_some() {
                        let mut content: *mut crate::expat_h::XML_Content =
                            (*parser)
                                .m_mem
                                .malloc_fcn
                                .expect("non-null function pointer")(
                                ::std::mem::size_of::<crate::expat_h::XML_Content>()
                                    as libc::c_ulong,
                            ) as *mut crate::expat_h::XML_Content;
                        if content.is_null() {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        (*content).quant = crate::expat_h::XML_CQUANT_NONE;
                        (*content).name =
                            crate::stddef_h::NULL as *mut crate::expat_external_h::XML_Char;
                        (*content).numchildren = 0 as libc::c_int as libc::c_uint;
                        (*content).children =
                            crate::stddef_h::NULL as *mut crate::expat_h::XML_Content;
                        (*content).type_0 = if role
                            == crate::src::lib::xmlrole::XML_ROLE_CONTENT_ANY as libc::c_int
                        {
                            crate::expat_h::XML_CTYPE_ANY as libc::c_int
                        } else {
                            crate::expat_h::XML_CTYPE_EMPTY as libc::c_int
                        }
                            as crate::expat_h::XML_Content_Type;
                        *eventEndPP = s;
                        (*parser)
                            .m_elementDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declElementType).name,
                            content,
                        );
                        handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                    }
                    (*dtd).in_eldecl = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            43 => {
                if (*dtd).in_eldecl != 0 {
                    (*(*dtd).scaffold.offset(
                        *(*dtd)
                            .scaffIndex
                            .offset(((*dtd).scaffLevel - 1 as libc::c_int) as isize)
                            as isize,
                    ))
                    .type_0 = crate::expat_h::XML_CTYPE_MIXED;
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                    }
                }
                current_block = 1553878188884632965;
            }
            51 => {
                quant = crate::expat_h::XML_CQUANT_NONE;
                current_block = 4542134034984465527;
            }
            53 => {
                quant = crate::expat_h::XML_CQUANT_OPT;
                current_block = 4542134034984465527;
            }
            52 => {
                quant = crate::expat_h::XML_CQUANT_REP;
                current_block = 4542134034984465527;
            }
            54 => {
                quant = crate::expat_h::XML_CQUANT_PLUS;
                current_block = 4542134034984465527;
            }
            45 => {
                quant = crate::expat_h::XML_CQUANT_NONE;
                current_block = 7739131043814808354;
            }
            47 => {
                quant = crate::expat_h::XML_CQUANT_OPT;
                current_block = 7739131043814808354;
            }
            46 => {
                quant = crate::expat_h::XML_CQUANT_REP;
                current_block = 7739131043814808354;
            }
            48 => {
                quant = crate::expat_h::XML_CQUANT_PLUS;
                current_block = 7739131043814808354;
            }
            55 => {
                /* End element declaration stuff */
                if reportProcessingInstruction(parser, enc, s, next) == 0 {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                current_block = 1553878188884632965;
            }
            56 => {
                if reportComment(parser, enc, s, next) == 0 {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                current_block = 1553878188884632965;
            }
            0 => {
                match tok {
                    crate::src::lib::xmltok::XML_TOK_BOM => {
                        handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                    }
                    _ => {}
                }
                current_block = 1553878188884632965;
            }
            3 => {
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            11 => {
                if (*dtd).keepProcessing as libc::c_int != 0
                    && (*parser).m_entityDeclHandler.is_some()
                {
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            17 => {
                if (*parser).m_notationDeclHandler.is_some() {
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            33 => {
                if (*dtd).keepProcessing as libc::c_int != 0
                    && (*parser).m_attlistDeclHandler.is_some()
                {
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            39 => {
                if (*parser).m_elementDeclHandler.is_some() {
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            _ => {
                current_block = 1553878188884632965;
            }
        }
        match current_block {
            926243229934402080 =>
            /* fall through */
            {
                if (*enc).isPublicId.expect("non-null function pointer")(enc, s, next, eventPP) == 0
                {
                    return crate::expat_h::XML_ERROR_PUBLICID;
                }
                current_block = 9007411418488376351;
            }
            15307276507984219638 =>
            /* XML_DTD */
            /* fall through */
            {
                if (*dtd).keepProcessing as libc::c_int != 0 && !(*parser).m_declEntity.is_null() {
                    (*(*parser).m_declEntity).systemId = poolStoreString(
                        &mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if (*(*parser).m_declEntity).systemId.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*(*parser).m_declEntity).base = (*parser).m_curBase;
                    (*dtd).pool.start = (*dtd).pool.ptr;
                    /* Don't suppress the default handler if we fell through from
                     * the XML_ROLE_DOCTYPE_SYSTEM_ID case.
                     */
                    if (*parser).m_entityDeclHandler.is_some()
                        && role
                            == crate::src::lib::xmlrole::XML_ROLE_ENTITY_SYSTEM_ID as libc::c_int
                    {
                        handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                    }
                }
                current_block = 1553878188884632965;
            }
            6455255476181645667 => {
                if (*dtd).keepProcessing as libc::c_int != 0
                    && (*parser).m_attlistDeclHandler.is_some()
                {
                    handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                }
                current_block = 1553878188884632965;
            }
            4542134034984465527 => {
                if (*dtd).in_eldecl != 0 {
                    let mut el: *mut ELEMENT_TYPE = 0 as *mut ELEMENT_TYPE;
                    let mut name_2: *const crate::expat_external_h::XML_Char =
                        0 as *const crate::expat_external_h::XML_Char;
                    let mut nameLen: libc::c_int = 0;
                    let mut nxt: *const libc::c_char = if quant as libc::c_uint
                        == crate::expat_h::XML_CQUANT_NONE as libc::c_int as libc::c_uint
                    {
                        next
                    } else {
                        next.offset(-((*enc).minBytesPerChar as isize))
                    };
                    let mut myindex_0: libc::c_int = nextScaffoldPart(parser);
                    if myindex_0 < 0 as libc::c_int {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*(*dtd).scaffold.offset(myindex_0 as isize)).type_0 =
                        crate::expat_h::XML_CTYPE_NAME;
                    (*(*dtd).scaffold.offset(myindex_0 as isize)).quant = quant;
                    el = getElementType(parser, enc, s, nxt);
                    if el.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    name_2 = (*el).name;
                    let ref mut fresh36 = (*(*dtd).scaffold.offset(myindex_0 as isize)).name;
                    *fresh36 = name_2;
                    nameLen = 0 as libc::c_int;
                    loop {
                        let fresh37 = nameLen;
                        nameLen = nameLen + 1;
                        if !(*name_2.offset(fresh37 as isize) != 0) {
                            break;
                        }
                    }
                    (*dtd).contentStringLen = (*dtd)
                        .contentStringLen
                        .wrapping_add(nameLen as libc::c_uint);
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                    }
                }
                current_block = 1553878188884632965;
            }
            7739131043814808354 => {
                if (*dtd).in_eldecl != 0 {
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                    }
                    (*dtd).scaffLevel -= 1;
                    (*(*dtd)
                        .scaffold
                        .offset(*(*dtd).scaffIndex.offset((*dtd).scaffLevel as isize) as isize))
                    .quant = quant;
                    if (*dtd).scaffLevel == 0 as libc::c_int {
                        if handleDefault == 0 {
                            let mut model: *mut crate::expat_h::XML_Content = build_model(parser);
                            if model.is_null() {
                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                            }
                            *eventEndPP = s;
                            (*parser)
                                .m_elementDeclHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*(*parser).m_declElementType).name,
                                model,
                            );
                        }
                        (*dtd).in_eldecl = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                        (*dtd).contentStringLen = 0 as libc::c_int as libc::c_uint
                    }
                }
                current_block = 1553878188884632965;
            }
            _ => {}
        }
        match current_block {
            9007411418488376351 => {
                if (*dtd).keepProcessing as libc::c_int != 0 && !(*parser).m_declEntity.is_null() {
                    let mut tem: *mut crate::expat_external_h::XML_Char = poolStoreString(
                        &mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if tem.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    normalizePublicId(tem);
                    (*(*parser).m_declEntity).publicId = tem;
                    (*dtd).pool.start = (*dtd).pool.ptr;
                    /* Don't suppress the default handler if we fell through from
                     * the XML_ROLE_DOCTYPE_PUBLIC_ID case.
                     */
                    if (*parser).m_entityDeclHandler.is_some()
                        && role
                            == crate::src::lib::xmlrole::XML_ROLE_ENTITY_PUBLIC_ID as libc::c_int
                    {
                        handleDefault = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool
                    }
                }
            }
            _ => {}
        }
        /* not XML_DTD */
        /* XML_DTD */
        if handleDefault as libc::c_int != 0 && (*parser).m_defaultHandler.is_some() {
            reportDefault(parser, enc, s, next);
        }
        match (*parser).m_parsingStatus.parsing as libc::c_uint {
            3 => {
                *nextPtr = next;
                return crate::expat_h::XML_ERROR_NONE;
            }
            2 => return crate::expat_h::XML_ERROR_ABORTED,
            _ => {
                s = next;
                tok = (*enc).scanners[0 as libc::c_int as usize].expect("non-null function pointer")(
                    enc, s, end, &mut next,
                )
            }
        }
    }
    /* not reached */
}
/* XML_DTD */

unsafe extern "C" fn epilogProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const libc::c_char,
    mut end: *const libc::c_char,
    mut nextPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    (*parser).m_processor = Some(epilogProcessor as Processor);
    (*parser).m_eventPtr = s;
    loop {
        let mut next: *const libc::c_char = crate::stddef_h::NULL as *const libc::c_char;
        let mut tok: libc::c_int = (*(*parser).m_encoding).scanners[0 as libc::c_int as usize]
            .expect("non-null function pointer")(
            (*parser).m_encoding, s, end, &mut next
        );
        (*parser).m_eventEndPtr = next;
        match tok {
            -15 => {
                /* report partial linebreak - it might be the last token */
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, (*parser).m_encoding, s, next);
                    if (*parser).m_parsingStatus.parsing as libc::c_uint
                        == crate::expat_h::XML_FINISHED as libc::c_int as libc::c_uint
                    {
                        return crate::expat_h::XML_ERROR_ABORTED;
                    }
                }
                *nextPtr = next;
                return crate::expat_h::XML_ERROR_NONE;
            }
            crate::src::lib::xmltok::XML_TOK_NONE => {
                *nextPtr = s;
                return crate::expat_h::XML_ERROR_NONE;
            }
            crate::src::lib::xmltok::XML_TOK_PROLOG_S => {
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, (*parser).m_encoding, s, next);
                }
            }
            crate::src::lib::xmltok::XML_TOK_PI => {
                if reportProcessingInstruction(parser, (*parser).m_encoding, s, next) == 0 {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
            }
            crate::src::lib::xmltok::XML_TOK_COMMENT => {
                if reportComment(parser, (*parser).m_encoding, s, next) == 0 {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
            }
            crate::src::lib::xmltok::XML_TOK_INVALID => {
                (*parser).m_eventPtr = next;
                return crate::expat_h::XML_ERROR_INVALID_TOKEN;
            }
            crate::src::lib::xmltok::XML_TOK_PARTIAL => {
                if (*parser).m_parsingStatus.finalBuffer == 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN;
            }
            crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR => {
                if (*parser).m_parsingStatus.finalBuffer == 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                return crate::expat_h::XML_ERROR_PARTIAL_CHAR;
            }
            _ => return crate::expat_h::XML_ERROR_JUNK_AFTER_DOC_ELEMENT,
        }
        s = next;
        (*parser).m_eventPtr = s;
        match (*parser).m_parsingStatus.parsing as libc::c_uint {
            3 => {
                *nextPtr = next;
                return crate::expat_h::XML_ERROR_NONE;
            }
            2 => return crate::expat_h::XML_ERROR_ABORTED,
            _ => {}
        }
    }
}

unsafe extern "C" fn processInternalEntity(
    mut parser: crate::expat_h::XML_Parser,
    mut entity: *mut ENTITY,
    mut betweenDecl: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Error {
    let mut textStart: *const libc::c_char = 0 as *const libc::c_char;
    let mut textEnd: *const libc::c_char = 0 as *const libc::c_char;
    let mut next: *const libc::c_char = 0 as *const libc::c_char;
    let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
    let mut openEntity: *mut OPEN_INTERNAL_ENTITY = 0 as *mut OPEN_INTERNAL_ENTITY;
    if !(*parser).m_freeInternalEntities.is_null() {
        openEntity = (*parser).m_freeInternalEntities;
        (*parser).m_freeInternalEntities = (*openEntity).next
    } else {
        openEntity = (*parser)
            .m_mem
            .malloc_fcn
            .expect("non-null function pointer")(::std::mem::size_of::<
            OPEN_INTERNAL_ENTITY,
        >() as libc::c_ulong) as *mut OPEN_INTERNAL_ENTITY;
        if openEntity.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
    }
    (*entity).open = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
    (*entity).processed = 0 as libc::c_int;
    (*openEntity).next = (*parser).m_openInternalEntities;
    (*parser).m_openInternalEntities = openEntity;
    (*openEntity).entity = entity;
    (*openEntity).startTagLevel = (*parser).m_tagLevel;
    (*openEntity).betweenDecl = betweenDecl;
    (*openEntity).internalEventPtr = crate::stddef_h::NULL as *const libc::c_char;
    (*openEntity).internalEventEndPtr = crate::stddef_h::NULL as *const libc::c_char;
    textStart = (*entity).textPtr as *mut libc::c_char;
    textEnd = (*entity).textPtr.offset((*entity).textLen as isize) as *mut libc::c_char;
    /* Set a safe default value in case 'next' does not get set */
    next = textStart;
    if (*entity).is_param != 0 {
        let mut tok: libc::c_int = (*(*parser).m_internalEncoding).scanners
            [0 as libc::c_int as usize]
            .expect("non-null function pointer")(
            (*parser).m_internalEncoding,
            textStart,
            textEnd,
            &mut next,
        );
        result = doProlog(
            parser,
            (*parser).m_internalEncoding,
            textStart,
            textEnd,
            tok,
            next,
            &mut next,
            crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool,
            crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool,
        )
    } else {
        /* XML_DTD */
        result = doContent(
            parser,
            (*parser).m_tagLevel,
            (*parser).m_internalEncoding,
            textStart,
            textEnd,
            &mut next,
            crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool,
        )
    }
    if result as libc::c_uint == crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint {
        if textEnd != next
            && (*parser).m_parsingStatus.parsing as libc::c_uint
                == crate::expat_h::XML_SUSPENDED as libc::c_int as libc::c_uint
        {
            (*entity).processed =
                next.wrapping_offset_from(textStart) as libc::c_long as libc::c_int;
            (*parser).m_processor = Some(internalEntityProcessor as Processor)
        } else {
            (*entity).open = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
            (*parser).m_openInternalEntities = (*openEntity).next;
            /* put openEntity back in list of free instances */
            (*openEntity).next = (*parser).m_freeInternalEntities;
            (*parser).m_freeInternalEntities = openEntity
        }
    }
    return result;
}

unsafe extern "C" fn internalEntityProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const libc::c_char,
    mut end: *const libc::c_char,
    mut nextPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut entity: *mut ENTITY = 0 as *mut ENTITY;
    let mut textStart: *const libc::c_char = 0 as *const libc::c_char;
    let mut textEnd: *const libc::c_char = 0 as *const libc::c_char;
    let mut next: *const libc::c_char = 0 as *const libc::c_char;
    let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
    let mut openEntity: *mut OPEN_INTERNAL_ENTITY = (*parser).m_openInternalEntities;
    if openEntity.is_null() {
        return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
    }
    entity = (*openEntity).entity;
    textStart = ((*entity).textPtr as *mut libc::c_char).offset((*entity).processed as isize);
    textEnd = (*entity).textPtr.offset((*entity).textLen as isize) as *mut libc::c_char;
    /* Set a safe default value in case 'next' does not get set */
    next = textStart;
    if (*entity).is_param != 0 {
        let mut tok: libc::c_int = (*(*parser).m_internalEncoding).scanners
            [0 as libc::c_int as usize]
            .expect("non-null function pointer")(
            (*parser).m_internalEncoding,
            textStart,
            textEnd,
            &mut next,
        );
        result = doProlog(
            parser,
            (*parser).m_internalEncoding,
            textStart,
            textEnd,
            tok,
            next,
            &mut next,
            crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool,
            crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool,
        )
    } else {
        /* XML_DTD */
        result = doContent(
            parser,
            (*openEntity).startTagLevel,
            (*parser).m_internalEncoding,
            textStart,
            textEnd,
            &mut next,
            crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool,
        )
    }
    if result as libc::c_uint != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint {
        return result;
    } else {
        if textEnd != next
            && (*parser).m_parsingStatus.parsing as libc::c_uint
                == crate::expat_h::XML_SUSPENDED as libc::c_int as libc::c_uint
        {
            (*entity).processed = next.wrapping_offset_from((*entity).textPtr as *mut libc::c_char)
                as libc::c_long as libc::c_int;
            return result;
        } else {
            (*entity).open = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
            (*parser).m_openInternalEntities = (*openEntity).next;
            /* put openEntity back in list of free instances */
            (*openEntity).next = (*parser).m_freeInternalEntities;
            (*parser).m_freeInternalEntities = openEntity
        }
    }
    if (*entity).is_param != 0 {
        let mut tok_0: libc::c_int = 0;
        (*parser).m_processor = Some(prologProcessor as Processor);
        tok_0 = (*(*parser).m_encoding).scanners[0 as libc::c_int as usize]
            .expect("non-null function pointer")(
            (*parser).m_encoding, s, end, &mut next
        );
        return doProlog(
            parser,
            (*parser).m_encoding,
            s,
            end,
            tok_0,
            next,
            nextPtr,
            ((*parser).m_parsingStatus.finalBuffer == 0) as libc::c_int as crate::expat_h::XML_Bool,
            crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool,
        );
    } else {
        /* XML_DTD */
        (*parser).m_processor = Some(contentProcessor as Processor);
        /* see externalEntityContentProcessor vs contentProcessor */
        return doContent(
            parser,
            if !(*parser).m_parentParser.is_null() {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            },
            (*parser).m_encoding,
            s,
            end,
            nextPtr,
            ((*parser).m_parsingStatus.finalBuffer == 0) as libc::c_int as crate::expat_h::XML_Bool,
        );
    }; /* save one level of indirection */
}

unsafe extern "C" fn errorProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const libc::c_char,
    mut end: *const libc::c_char,
    mut nextPtr: *mut *const libc::c_char,
) -> crate::expat_h::XML_Error {
    return (*parser).m_errorCode;
}

unsafe extern "C" fn storeAttributeValue(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut isCdata: crate::expat_h::XML_Bool,
    mut ptr: *const libc::c_char,
    mut end: *const libc::c_char,
    mut pool: *mut STRING_POOL,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error =
        appendAttributeValue(parser, enc, isCdata, ptr, end, pool);
    if result as u64 != 0 {
        return result;
    }
    if isCdata == 0
        && (*pool).ptr.wrapping_offset_from((*pool).start) as libc::c_long != 0
        && *(*pool).ptr.offset(-(1 as libc::c_int) as isize) as libc::c_int == 0x20 as libc::c_int
    {
        (*pool).ptr = (*pool).ptr.offset(-1)
    }
    if if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char
        && poolGrow(pool) == 0
    {
        0 as libc::c_int
    } else {
        let fresh38 = (*pool).ptr;
        (*pool).ptr = (*pool).ptr.offset(1);
        *fresh38 = '\u{0}' as i32 as crate::expat_external_h::XML_Char;
        1 as libc::c_int
    } == 0
    {
        return crate::expat_h::XML_ERROR_NO_MEMORY;
    }
    return crate::expat_h::XML_ERROR_NONE;
}

unsafe extern "C" fn appendAttributeValue(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut isCdata: crate::expat_h::XML_Bool,
    mut ptr: *const libc::c_char,
    mut end: *const libc::c_char,
    mut pool: *mut STRING_POOL,
) -> crate::expat_h::XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd;
    loop {
        let mut next: *const libc::c_char = 0 as *const libc::c_char;
        let mut tok: libc::c_int = (*enc).literalScanners[0 as libc::c_int as usize]
            .expect("non-null function pointer")(
            enc, ptr, end, &mut next
        );
        let mut current_block_62: u64;
        match tok {
            crate::src::lib::xmltok::XML_TOK_NONE => {
                return crate::expat_h::XML_ERROR_NONE;
                /* LCOV_EXCL_STOP */
            }
            crate::src::lib::xmltok::XML_TOK_INVALID => {
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = next
                }
                return crate::expat_h::XML_ERROR_INVALID_TOKEN;
            }
            crate::src::lib::xmltok::XML_TOK_PARTIAL => {
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = ptr
                }
                return crate::expat_h::XML_ERROR_INVALID_TOKEN;
            }
            crate::src::lib::xmltok::XML_TOK_CHAR_REF => {
                let mut buf: [crate::expat_external_h::XML_Char; 4] = [0; 4];
                let mut i: libc::c_int = 0;
                let mut n: libc::c_int =
                    (*enc).charRefNumber.expect("non-null function pointer")(enc, ptr);
                if n < 0 as libc::c_int {
                    if enc == (*parser).m_encoding {
                        (*parser).m_eventPtr = ptr
                    }
                    return crate::expat_h::XML_ERROR_BAD_CHAR_REF;
                }
                if isCdata == 0
                    && n == 0x20 as libc::c_int
                    && ((*pool).ptr.wrapping_offset_from((*pool).start) as libc::c_long
                        == 0 as libc::c_int as libc::c_long
                        || *(*pool).ptr.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == 0x20 as libc::c_int)
                {
                    current_block_62 = 11796148217846552555;
                } else {
                    n = crate::src::lib::xmltok::XmlUtf8Encode(n, buf.as_mut_ptr() as *mut ICHAR);
                    /* The XmlEncode() functions can never return 0 here.  That
                     * error return happens if the code point passed in is either
                     * negative or greater than or equal to 0x110000.  The
                     * XmlCharRefNumber() functions will all return a number
                     * strictly less than 0x110000 or a negative value if an error
                     * occurred.  The negative value is intercepted above, so
                     * XmlEncode() is never passed a value it might return an
                     * error for.
                     */
                    i = 0 as libc::c_int;
                    while i < n {
                        if if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char
                            && poolGrow(pool) == 0
                        {
                            0 as libc::c_int
                        } else {
                            let fresh39 = (*pool).ptr;
                            (*pool).ptr = (*pool).ptr.offset(1);
                            *fresh39 = buf[i as usize];
                            1 as libc::c_int
                        } == 0
                        {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        i += 1
                    }
                    current_block_62 = 11796148217846552555;
                }
            }
            crate::src::lib::xmltok::XML_TOK_DATA_CHARS => {
                if poolAppend(pool, enc, ptr, next).is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                current_block_62 = 11796148217846552555;
            }
            crate::src::lib::xmltok::XML_TOK_TRAILING_CR => {
                next = ptr.offset((*enc).minBytesPerChar as isize);
                current_block_62 = 9696599617798541816;
            }
            crate::src::lib::xmltok::XML_TOK_ATTRIBUTE_VALUE_S
            | crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE => {
                current_block_62 = 9696599617798541816;
            }
            crate::src::lib::xmltok::XML_TOK_ENTITY_REF => {
                let mut name: *const crate::expat_external_h::XML_Char =
                    0 as *const crate::expat_external_h::XML_Char;
                let mut entity: *mut ENTITY = 0 as *mut ENTITY;
                let mut checkEntityDecl: libc::c_char = 0;
                let mut ch: crate::expat_external_h::XML_Char = (*enc)
                    .predefinedEntityName
                    .expect("non-null function pointer")(
                    enc,
                    ptr.offset((*enc).minBytesPerChar as isize),
                    next.offset(-((*enc).minBytesPerChar as isize)),
                )
                    as crate::expat_external_h::XML_Char;
                if ch != 0 {
                    if if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char
                        && poolGrow(pool) == 0
                    {
                        0 as libc::c_int
                    } else {
                        let fresh41 = (*pool).ptr;
                        (*pool).ptr = (*pool).ptr.offset(1);
                        *fresh41 = ch;
                        1 as libc::c_int
                    } == 0
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                } else {
                    name = poolStoreString(
                        &mut (*parser).m_temp2Pool,
                        enc,
                        ptr.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if name.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    entity = lookup(
                        parser,
                        &mut (*dtd).generalEntities,
                        name,
                        0 as libc::c_int as crate::stddef_h::size_t,
                    ) as *mut ENTITY;
                    (*parser).m_temp2Pool.ptr = (*parser).m_temp2Pool.start;
                    /* First, determine if a check for an existing declaration is needed;
                       if yes, check that the entity exists, and that it is internal.
                    */
                    if pool == &mut (*dtd).pool as *mut STRING_POOL {
                        /* are we called from prolog? */
                        checkEntityDecl = ((*parser).m_prologState.documentEntity != 0
                            && (if (*dtd).standalone as libc::c_int != 0 {
                                (*parser).m_openInternalEntities.is_null() as libc::c_int
                            } else {
                                ((*dtd).hasParamEntityRefs == 0) as libc::c_int
                            }) != 0) as libc::c_int
                            as libc::c_char
                    } else {
                        /* if (pool == &parser->m_tempPool): we are called from content */
                        checkEntityDecl = ((*dtd).hasParamEntityRefs == 0
                            || (*dtd).standalone as libc::c_int != 0)
                            as libc::c_int as libc::c_char
                    }
                    if checkEntityDecl != 0 {
                        if entity.is_null() {
                            return crate::expat_h::XML_ERROR_UNDEFINED_ENTITY;
                        } else {
                            if (*entity).is_internal == 0 {
                                return crate::expat_h::XML_ERROR_ENTITY_DECLARED_IN_PE;
                            }
                        }
                        current_block_62 = 11777552016271000781;
                    } else if entity.is_null() {
                        current_block_62 = 11796148217846552555;
                    } else {
                        current_block_62 = 11777552016271000781;
                    }
                    match current_block_62 {
                        11796148217846552555 => {}
                        _ => {
                            if (*entity).open != 0 {
                                if enc == (*parser).m_encoding {
                                    /* It does not appear that this line can be executed.
                                     *
                                     * The "if (entity->open)" check catches recursive entity
                                     * definitions.  In order to be called with an open
                                     * entity, it must have gone through this code before and
                                     * been through the recursive call to
                                     * appendAttributeValue() some lines below.  That call
                                     * sets the local encoding ("enc") to the parser's
                                     * internal encoding (internal_utf8 or internal_utf16),
                                     * which can never be the same as the principle encoding.
                                     * It doesn't appear there is another code path that gets
                                     * here with entity->open being TRUE.
                                     *
                                     * Since it is not certain that this logic is watertight,
                                     * we keep the line and merely exclude it from coverage
                                     * tests.
                                     */
                                    (*parser).m_eventPtr = ptr
                                    /* LCOV_EXCL_LINE */
                                }
                                return crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if !(*entity).notation.is_null() {
                                if enc == (*parser).m_encoding {
                                    (*parser).m_eventPtr = ptr
                                }
                                return crate::expat_h::XML_ERROR_BINARY_ENTITY_REF;
                            }
                            if (*entity).textPtr.is_null() {
                                if enc == (*parser).m_encoding {
                                    (*parser).m_eventPtr = ptr
                                }
                                return crate::expat_h::XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF;
                            } else {
                                let mut result: crate::expat_h::XML_Error =
                                    crate::expat_h::XML_ERROR_NONE;
                                let mut textEnd: *const crate::expat_external_h::XML_Char =
                                    (*entity).textPtr.offset((*entity).textLen as isize);
                                (*entity).open =
                                    crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                                result = appendAttributeValue(
                                    parser,
                                    (*parser).m_internalEncoding,
                                    isCdata,
                                    (*entity).textPtr as *mut libc::c_char,
                                    textEnd as *mut libc::c_char,
                                    pool,
                                );
                                (*entity).open =
                                    crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                                if result as u64 != 0 {
                                    return result;
                                }
                            }
                        }
                    }
                }
                current_block_62 = 11796148217846552555;
            }
            _ => {
                /* The only token returned by XmlAttributeValueTok() that does
                 * not have an explicit case here is XML_TOK_PARTIAL_CHAR.
                 * Getting that would require an entity name to contain an
                 * incomplete XML character (e.g. \xE2\x82); however previous
                 * tokenisers will have already recognised and rejected such
                 * names before XmlAttributeValueTok() gets a look-in.  This
                 * default case should be retained as a safety net, but the code
                 * excluded from coverage tests.
                 *
                 * LCOV_EXCL_START
                 */
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = ptr
                }
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            }
        }
        match current_block_62 {
            9696599617798541816 =>
            /* fall through */
            {
                if !(isCdata == 0
                    && ((*pool).ptr.wrapping_offset_from((*pool).start) as libc::c_long
                        == 0 as libc::c_int as libc::c_long
                        || *(*pool).ptr.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == 0x20 as libc::c_int))
                {
                    if if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char
                        && poolGrow(pool) == 0
                    {
                        0 as libc::c_int
                    } else {
                        let fresh40 = (*pool).ptr;
                        (*pool).ptr = (*pool).ptr.offset(1);
                        *fresh40 = 0x20 as libc::c_int as crate::expat_external_h::XML_Char;
                        1 as libc::c_int
                    } == 0
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                }
            }
            _ => {}
        }
        ptr = next
    }
    /* not reached */
}

unsafe extern "C" fn storeEntityValue(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut entityTextPtr: *const libc::c_char,
    mut entityTextEnd: *const libc::c_char,
) -> crate::expat_h::XML_Error {
    let mut current_block: u64; /* save one level of indirection */
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut pool: *mut STRING_POOL = &mut (*dtd).entityValuePool;
    let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
    let mut oldInEntityValue: libc::c_int = (*parser).m_prologState.inEntityValue;
    (*parser).m_prologState.inEntityValue = 1 as libc::c_int;
    /* XML_DTD */
    /* never return Null for the value argument in EntityDeclHandler,
    since this would indicate an external entity; therefore we
    have to make sure that entityValuePool.start is not null */
    if (*pool).blocks.is_null() {
        if poolGrow(pool) == 0 {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
    }
    's_41: loop {
        let mut next: *const libc::c_char = 0 as *const libc::c_char;
        let mut tok: libc::c_int = (*enc).literalScanners[1 as libc::c_int as usize]
            .expect("non-null function pointer")(
            enc, entityTextPtr, entityTextEnd, &mut next
        );
        match tok {
            crate::src::lib::xmltok::XML_TOK_PARAM_ENTITY_REF => {
                if (*parser).m_isParamEntity as libc::c_int != 0 || enc != (*parser).m_encoding {
                    let mut name: *const crate::expat_external_h::XML_Char =
                        0 as *const crate::expat_external_h::XML_Char;
                    let mut entity: *mut ENTITY = 0 as *mut ENTITY;
                    name = poolStoreString(
                        &mut (*parser).m_tempPool,
                        enc,
                        entityTextPtr.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if name.is_null() {
                        result = crate::expat_h::XML_ERROR_NO_MEMORY;
                        break;
                    } else {
                        entity = lookup(
                            parser,
                            &mut (*dtd).paramEntities,
                            name,
                            0 as libc::c_int as crate::stddef_h::size_t,
                        ) as *mut ENTITY;
                        (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
                        if entity.is_null() {
                            /* not a well-formedness error - see XML 1.0: WFC Entity Declared */
                            /* cannot report skipped entity here - see comments on
                               parser->m_skippedEntityHandler
                            if (parser->m_skippedEntityHandler)
                              parser->m_skippedEntityHandler(parser->m_handlerArg, name, 0);
                            */
                            (*dtd).keepProcessing = (*dtd).standalone;
                            break;
                        } else if (*entity).open != 0 {
                            if enc == (*parser).m_encoding {
                                (*parser).m_eventPtr = entityTextPtr
                            }
                            result = crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
                            break;
                        } else if !(*entity).systemId.is_null() {
                            if (*parser).m_externalEntityRefHandler.is_some() {
                                (*dtd).paramEntityRead =
                                    crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                                (*entity).open =
                                    crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                                if (*parser)
                                    .m_externalEntityRefHandler
                                    .expect("non-null function pointer")(
                                    (*parser).m_externalEntityRefHandlerArg,
                                    0 as *const crate::expat_external_h::XML_Char,
                                    (*entity).base,
                                    (*entity).systemId,
                                    (*entity).publicId,
                                ) == 0
                                {
                                    (*entity).open =
                                        crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                                    result = crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                    break;
                                } else {
                                    (*entity).open =
                                        crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                                    if (*dtd).paramEntityRead == 0 {
                                        (*dtd).keepProcessing = (*dtd).standalone
                                    }
                                }
                            } else {
                                (*dtd).keepProcessing = (*dtd).standalone
                            }
                        } else {
                            (*entity).open = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
                            result = storeEntityValue(
                                parser,
                                (*parser).m_internalEncoding,
                                (*entity).textPtr as *mut libc::c_char,
                                (*entity).textPtr.offset((*entity).textLen as isize)
                                    as *mut libc::c_char,
                            );
                            (*entity).open = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                            if result as u64 != 0 {
                                break;
                            }
                        }
                    }
                } else {
                    /* XML_DTD */
                    /* In the internal subset, PE references are not legal
                    within markup declarations, e.g entity values in this case. */
                    (*parser).m_eventPtr = entityTextPtr;
                    result = crate::expat_h::XML_ERROR_PARAM_ENTITY_REF;
                    break;
                }
                current_block = 10007731352114176167;
                /* LCOV_EXCL_STOP */
            }
            crate::src::lib::xmltok::XML_TOK_NONE => {
                result = crate::expat_h::XML_ERROR_NONE;
                break;
            }
            crate::src::lib::xmltok::XML_TOK_ENTITY_REF
            | crate::src::lib::xmltok::XML_TOK_DATA_CHARS => {
                if poolAppend(pool, enc, entityTextPtr, next).is_null() {
                    result = crate::expat_h::XML_ERROR_NO_MEMORY;
                    break;
                } else {
                    current_block = 10007731352114176167;
                }
            }
            crate::src::lib::xmltok::XML_TOK_TRAILING_CR => {
                next = entityTextPtr.offset((*enc).minBytesPerChar as isize);
                current_block = 13862322071133341448;
            }
            crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE => {
                current_block = 13862322071133341448;
            }
            crate::src::lib::xmltok::XML_TOK_CHAR_REF => {
                let mut buf: [crate::expat_external_h::XML_Char; 4] = [0; 4];
                let mut i: libc::c_int = 0;
                let mut n: libc::c_int =
                    (*enc).charRefNumber.expect("non-null function pointer")(enc, entityTextPtr);
                if n < 0 as libc::c_int {
                    if enc == (*parser).m_encoding {
                        (*parser).m_eventPtr = entityTextPtr
                    }
                    result = crate::expat_h::XML_ERROR_BAD_CHAR_REF;
                    break;
                } else {
                    n = crate::src::lib::xmltok::XmlUtf8Encode(n, buf.as_mut_ptr() as *mut ICHAR);
                    /* The XmlEncode() functions can never return 0 here.  That
                     * error return happens if the code point passed in is either
                     * negative or greater than or equal to 0x110000.  The
                     * XmlCharRefNumber() functions will all return a number
                     * strictly less than 0x110000 or a negative value if an error
                     * occurred.  The negative value is intercepted above, so
                     * XmlEncode() is never passed a value it might return an
                     * error for.
                     */
                    i = 0 as libc::c_int;
                    while i < n {
                        if (*pool).end == (*pool).ptr as *const crate::expat_external_h::XML_Char
                            && poolGrow(pool) == 0
                        {
                            result = crate::expat_h::XML_ERROR_NO_MEMORY;
                            break 's_41;
                        } else {
                            let fresh43 = (*pool).ptr;
                            (*pool).ptr = (*pool).ptr.offset(1);
                            *fresh43 = buf[i as usize];
                            i += 1
                        }
                    }
                }
                current_block = 10007731352114176167;
            }
            crate::src::lib::xmltok::XML_TOK_PARTIAL => {
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = entityTextPtr
                }
                result = crate::expat_h::XML_ERROR_INVALID_TOKEN;
                break;
            }
            crate::src::lib::xmltok::XML_TOK_INVALID => {
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = next
                }
                result = crate::expat_h::XML_ERROR_INVALID_TOKEN;
                break;
            }
            _ => {
                /* This default case should be unnecessary -- all the tokens
                 * that XmlEntityValueTok() can return have their own explicit
                 * cases -- but should be retained for safety.  We do however
                 * exclude it from the coverage statistics.
                 *
                 * LCOV_EXCL_START
                 */
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = entityTextPtr
                }
                result = crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
                break;
            }
        }
        match current_block {
            13862322071133341448 =>
            /* fall through */
            {
                if (*pool).end == (*pool).ptr as *const crate::expat_external_h::XML_Char
                    && poolGrow(pool) == 0
                {
                    result = crate::expat_h::XML_ERROR_NO_MEMORY;
                    break;
                } else {
                    let fresh42 = (*pool).ptr;
                    (*pool).ptr = (*pool).ptr.offset(1);
                    *fresh42 = 0xa as libc::c_int as crate::expat_external_h::XML_Char
                }
            }
            _ => {}
        }
        entityTextPtr = next
    }
    (*parser).m_prologState.inEntityValue = oldInEntityValue;
    /* XML_DTD */
    return result;
}

unsafe extern "C" fn normalizeLines(mut s: *mut crate::expat_external_h::XML_Char) {
    let mut p: *mut crate::expat_external_h::XML_Char = 0 as *mut crate::expat_external_h::XML_Char;
    loop {
        if *s as libc::c_int == '\u{0}' as i32 {
            return;
        }
        if *s as libc::c_int == 0xd as libc::c_int {
            break;
        }
        s = s.offset(1)
    }
    p = s;
    loop {
        if *s as libc::c_int == 0xd as libc::c_int {
            let fresh44 = p;
            p = p.offset(1);
            *fresh44 = 0xa as libc::c_int as crate::expat_external_h::XML_Char;
            s = s.offset(1);
            if *s as libc::c_int == 0xa as libc::c_int {
                s = s.offset(1)
            }
        } else {
            let fresh45 = s;
            s = s.offset(1);
            let fresh46 = p;
            p = p.offset(1);
            *fresh46 = *fresh45
        }
        if !(*s != 0) {
            break;
        }
    }
    *p = '\u{0}' as i32 as crate::expat_external_h::XML_Char;
}

unsafe extern "C" fn reportProcessingInstruction(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
) -> libc::c_int {
    let mut target: *const crate::expat_external_h::XML_Char =
        0 as *const crate::expat_external_h::XML_Char;
    let mut data: *mut crate::expat_external_h::XML_Char =
        0 as *mut crate::expat_external_h::XML_Char;
    let mut tem: *const libc::c_char = 0 as *const libc::c_char;
    if (*parser).m_processingInstructionHandler.is_none() {
        if (*parser).m_defaultHandler.is_some() {
            reportDefault(parser, enc, start, end);
        }
        return 1 as libc::c_int;
    }
    start = start.offset(((*enc).minBytesPerChar * 2 as libc::c_int) as isize);
    tem = start.offset((*enc).nameLength.expect("non-null function pointer")(enc, start) as isize);
    target = poolStoreString(&mut (*parser).m_tempPool, enc, start, tem);
    if target.is_null() {
        return 0 as libc::c_int;
    }
    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
    data = poolStoreString(
        &mut (*parser).m_tempPool,
        enc,
        (*enc).skipS.expect("non-null function pointer")(enc, tem),
        end.offset(-(((*enc).minBytesPerChar * 2 as libc::c_int) as isize)),
    );
    if data.is_null() {
        return 0 as libc::c_int;
    }
    normalizeLines(data);
    (*parser)
        .m_processingInstructionHandler
        .expect("non-null function pointer")((*parser).m_handlerArg, target, data);
    poolClear(&mut (*parser).m_tempPool);
    return 1 as libc::c_int;
}

unsafe extern "C" fn reportComment(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
) -> libc::c_int {
    let mut data: *mut crate::expat_external_h::XML_Char =
        0 as *mut crate::expat_external_h::XML_Char;
    if (*parser).m_commentHandler.is_none() {
        if (*parser).m_defaultHandler.is_some() {
            reportDefault(parser, enc, start, end);
        }
        return 1 as libc::c_int;
    }
    data = poolStoreString(
        &mut (*parser).m_tempPool,
        enc,
        start.offset(((*enc).minBytesPerChar * 4 as libc::c_int) as isize),
        end.offset(-(((*enc).minBytesPerChar * 3 as libc::c_int) as isize)),
    );
    if data.is_null() {
        return 0 as libc::c_int;
    }
    normalizeLines(data);
    (*parser)
        .m_commentHandler
        .expect("non-null function pointer")((*parser).m_handlerArg, data);
    poolClear(&mut (*parser).m_tempPool);
    return 1 as libc::c_int;
}

unsafe extern "C" fn reportDefault(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut s: *const libc::c_char,
    mut end: *const libc::c_char,
) {
    if (*enc).isUtf8 == 0 {
        let mut convert_res: crate::src::lib::xmltok::XML_Convert_Result =
            crate::src::lib::xmltok::XML_CONVERT_COMPLETED;
        let mut eventPP: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
        let mut eventEndPP: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
        if enc == (*parser).m_encoding {
            eventPP = &mut (*parser).m_eventPtr;
            eventEndPP = &mut (*parser).m_eventEndPtr
        } else {
            /* To get here, two things must be true; the parser must be
             * using a character encoding that is not the same as the
             * encoding passed in, and the encoding passed in must need
             * conversion to the internal format (UTF-8 unless XML_UNICODE
             * is defined).  The only occasions on which the encoding passed
             * in is not the same as the parser's encoding are when it is
             * the internal encoding (e.g. a previously defined parameter
             * entity, already converted to internal format).  This by
             * definition doesn't need conversion, so the whole branch never
             * gets executed.
             *
             * For safety's sake we don't delete these lines and merely
             * exclude them from coverage statistics.
             *
             * LCOV_EXCL_START
             */
            eventPP = &mut (*(*parser).m_openInternalEntities).internalEventPtr;
            eventEndPP = &mut (*(*parser).m_openInternalEntities).internalEventEndPtr
            /* LCOV_EXCL_STOP */
        }
        loop {
            let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
            convert_res = (*enc).utf8Convert.expect("non-null function pointer")(
                enc,
                &mut s,
                end,
                &mut dataPtr,
                (*parser).m_dataBufEnd as *mut ICHAR,
            );
            *eventEndPP = s;
            (*parser)
                .m_defaultHandler
                .expect("non-null function pointer")(
                (*parser).m_handlerArg,
                (*parser).m_dataBuf,
                dataPtr.wrapping_offset_from((*parser).m_dataBuf as *mut ICHAR) as libc::c_long
                    as libc::c_int,
            );
            *eventPP = s;
            if !(convert_res as libc::c_uint
                != crate::src::lib::xmltok::XML_CONVERT_COMPLETED as libc::c_int as libc::c_uint
                && convert_res as libc::c_uint
                    != crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE as libc::c_int
                        as libc::c_uint)
            {
                break;
            }
        }
    } else {
        (*parser)
            .m_defaultHandler
            .expect("non-null function pointer")(
            (*parser).m_handlerArg,
            s as *mut crate::expat_external_h::XML_Char,
            (end as *mut crate::expat_external_h::XML_Char)
                .wrapping_offset_from(s as *mut crate::expat_external_h::XML_Char)
                as libc::c_long as libc::c_int,
        );
    };
}

unsafe extern "C" fn defineAttribute(
    mut type_0: *mut ELEMENT_TYPE,
    mut attId: *mut ATTRIBUTE_ID,
    mut isCdata: crate::expat_h::XML_Bool,
    mut isId: crate::expat_h::XML_Bool,
    mut value: *const crate::expat_external_h::XML_Char,
    mut parser: crate::expat_h::XML_Parser,
) -> libc::c_int {
    let mut att: *mut DEFAULT_ATTRIBUTE = 0 as *mut DEFAULT_ATTRIBUTE;
    if !value.is_null() || isId as libc::c_int != 0 {
        /* The handling of default attributes gets messed up if we have
        a default which duplicates a non-default. */
        let mut i: libc::c_int = 0; /* save one level of indirection */
        i = 0 as libc::c_int; /* save one level of indirection */
        while i < (*type_0).nDefaultAtts {
            if attId == (*(*type_0).defaultAtts.offset(i as isize)).id as *mut ATTRIBUTE_ID {
                return 1 as libc::c_int;
            }
            i += 1
        }
        if isId as libc::c_int != 0 && (*type_0).idAtt.is_null() && (*attId).xmlns == 0 {
            (*type_0).idAtt = attId
        }
    }
    if (*type_0).nDefaultAtts == (*type_0).allocDefaultAtts {
        if (*type_0).allocDefaultAtts == 0 as libc::c_int {
            (*type_0).allocDefaultAtts = 8 as libc::c_int;
            (*type_0).defaultAtts = (*parser)
                .m_mem
                .malloc_fcn
                .expect("non-null function pointer")(
                ((*type_0).allocDefaultAtts as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<DEFAULT_ATTRIBUTE>() as libc::c_ulong),
            ) as *mut DEFAULT_ATTRIBUTE;
            if (*type_0).defaultAtts.is_null() {
                (*type_0).allocDefaultAtts = 0 as libc::c_int;
                return 0 as libc::c_int;
            }
        } else {
            let mut temp: *mut DEFAULT_ATTRIBUTE = 0 as *mut DEFAULT_ATTRIBUTE;
            let mut count: libc::c_int = (*type_0).allocDefaultAtts * 2 as libc::c_int;
            temp = (*parser)
                .m_mem
                .realloc_fcn
                .expect("non-null function pointer")(
                (*type_0).defaultAtts as *mut libc::c_void,
                (count as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<DEFAULT_ATTRIBUTE>() as libc::c_ulong),
            ) as *mut DEFAULT_ATTRIBUTE;
            if temp.is_null() {
                return 0 as libc::c_int;
            }
            (*type_0).allocDefaultAtts = count;
            (*type_0).defaultAtts = temp
        }
    }
    att = (*type_0)
        .defaultAtts
        .offset((*type_0).nDefaultAtts as isize);
    (*att).id = attId;
    (*att).value = value;
    (*att).isCdata = isCdata;
    if isCdata == 0 {
        (*attId).maybeTokenized = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool
    }
    (*type_0).nDefaultAtts += 1 as libc::c_int;
    return 1 as libc::c_int;
}

unsafe extern "C" fn setElementTypePrefix(
    mut parser: crate::expat_h::XML_Parser,
    mut elementType: *mut ELEMENT_TYPE,
) -> libc::c_int {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut name: *const crate::expat_external_h::XML_Char =
        0 as *const crate::expat_external_h::XML_Char;
    name = (*elementType).name;
    while *name != 0 {
        if *name as libc::c_int == 0x3a as libc::c_int {
            let mut prefix: *mut PREFIX = 0 as *mut PREFIX;
            let mut s: *const crate::expat_external_h::XML_Char =
                0 as *const crate::expat_external_h::XML_Char;
            s = (*elementType).name;
            while s != name {
                if if (*dtd).pool.ptr == (*dtd).pool.end as *mut crate::expat_external_h::XML_Char
                    && poolGrow(&mut (*dtd).pool) == 0
                {
                    0 as libc::c_int
                } else {
                    let fresh47 = (*dtd).pool.ptr;
                    (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                    *fresh47 = *s;
                    1 as libc::c_int
                } == 0
                {
                    return 0 as libc::c_int;
                }
                s = s.offset(1)
            }
            if if (*dtd).pool.ptr == (*dtd).pool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&mut (*dtd).pool) == 0
            {
                0 as libc::c_int
            } else {
                let fresh48 = (*dtd).pool.ptr;
                (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                *fresh48 = '\u{0}' as i32 as crate::expat_external_h::XML_Char;
                1 as libc::c_int
            } == 0
            {
                return 0 as libc::c_int;
            }
            prefix = lookup(
                parser,
                &mut (*dtd).prefixes,
                (*dtd).pool.start as KEY,
                ::std::mem::size_of::<PREFIX>() as libc::c_ulong,
            ) as *mut PREFIX;
            if prefix.is_null() {
                return 0 as libc::c_int;
            }
            if (*prefix).name == (*dtd).pool.start as *const crate::expat_external_h::XML_Char {
                (*dtd).pool.start = (*dtd).pool.ptr
            } else {
                (*dtd).pool.ptr = (*dtd).pool.start
            }
            (*elementType).prefix = prefix;
            break;
        } else {
            name = name.offset(1)
        }
    }
    return 1 as libc::c_int;
}

unsafe extern "C" fn getAttributeId(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut ATTRIBUTE_ID {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut id: *mut ATTRIBUTE_ID = 0 as *mut ATTRIBUTE_ID;
    let mut name: *const crate::expat_external_h::XML_Char =
        0 as *const crate::expat_external_h::XML_Char;
    if if (*dtd).pool.ptr == (*dtd).pool.end as *mut crate::expat_external_h::XML_Char
        && poolGrow(&mut (*dtd).pool) == 0
    {
        0 as libc::c_int
    } else {
        let fresh49 = (*dtd).pool.ptr;
        (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
        *fresh49 = '\u{0}' as i32 as crate::expat_external_h::XML_Char;
        1 as libc::c_int
    } == 0
    {
        return crate::stddef_h::NULL as *mut ATTRIBUTE_ID;
    }
    name = poolStoreString(&mut (*dtd).pool, enc, start, end);
    if name.is_null() {
        return crate::stddef_h::NULL as *mut ATTRIBUTE_ID;
    }
    /* skip quotation mark - its storage will be re-used (like in name[-1]) */
    name = name.offset(1);
    id = lookup(
        parser,
        &mut (*dtd).attributeIds,
        name,
        ::std::mem::size_of::<ATTRIBUTE_ID>() as libc::c_ulong,
    ) as *mut ATTRIBUTE_ID;
    if id.is_null() {
        return crate::stddef_h::NULL as *mut ATTRIBUTE_ID;
    }
    if (*id).name != name as *mut crate::expat_external_h::XML_Char {
        (*dtd).pool.ptr = (*dtd).pool.start
    } else {
        (*dtd).pool.start = (*dtd).pool.ptr;
        if !((*parser).m_ns == 0) {
            if *name.offset(0 as libc::c_int as isize) as libc::c_int == 0x78 as libc::c_int
                && *name.offset(1 as libc::c_int as isize) as libc::c_int == 0x6d as libc::c_int
                && *name.offset(2 as libc::c_int as isize) as libc::c_int == 0x6c as libc::c_int
                && *name.offset(3 as libc::c_int as isize) as libc::c_int == 0x6e as libc::c_int
                && *name.offset(4 as libc::c_int as isize) as libc::c_int == 0x73 as libc::c_int
                && (*name.offset(5 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                    || *name.offset(5 as libc::c_int as isize) as libc::c_int
                        == 0x3a as libc::c_int)
            {
                if *name.offset(5 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32 {
                    (*id).prefix = &mut (*dtd).defaultPrefix
                } else {
                    (*id).prefix = lookup(
                        parser,
                        &mut (*dtd).prefixes,
                        name.offset(6 as libc::c_int as isize),
                        ::std::mem::size_of::<PREFIX>() as libc::c_ulong,
                    ) as *mut PREFIX
                }
                (*id).xmlns = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool
            } else {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while *name.offset(i as isize) != 0 {
                    /* attributes without prefix are *not* in the default namespace */
                    if *name.offset(i as isize) as libc::c_int == 0x3a as libc::c_int {
                        let mut j: libc::c_int = 0; /* save one level of indirection */
                        j = 0 as libc::c_int;
                        while j < i {
                            if if (*dtd).pool.ptr
                                == (*dtd).pool.end as *mut crate::expat_external_h::XML_Char
                                && poolGrow(&mut (*dtd).pool) == 0
                            {
                                0 as libc::c_int
                            } else {
                                let fresh50 = (*dtd).pool.ptr;
                                (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                                *fresh50 = *name.offset(j as isize);
                                1 as libc::c_int
                            } == 0
                            {
                                return crate::stddef_h::NULL as *mut ATTRIBUTE_ID;
                            }
                            j += 1
                        }
                        if if (*dtd).pool.ptr
                            == (*dtd).pool.end as *mut crate::expat_external_h::XML_Char
                            && poolGrow(&mut (*dtd).pool) == 0
                        {
                            0 as libc::c_int
                        } else {
                            let fresh51 = (*dtd).pool.ptr;
                            (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                            *fresh51 = '\u{0}' as i32 as crate::expat_external_h::XML_Char;
                            1 as libc::c_int
                        } == 0
                        {
                            return crate::stddef_h::NULL as *mut ATTRIBUTE_ID;
                        }
                        (*id).prefix = lookup(
                            parser,
                            &mut (*dtd).prefixes,
                            (*dtd).pool.start as KEY,
                            ::std::mem::size_of::<PREFIX>() as libc::c_ulong,
                        ) as *mut PREFIX;
                        if (*id).prefix.is_null() {
                            return crate::stddef_h::NULL as *mut ATTRIBUTE_ID;
                        }
                        if (*(*id).prefix).name
                            == (*dtd).pool.start as *const crate::expat_external_h::XML_Char
                        {
                            (*dtd).pool.start = (*dtd).pool.ptr
                        } else {
                            (*dtd).pool.ptr = (*dtd).pool.start
                        }
                        break;
                    } else {
                        i += 1
                    }
                }
            }
        }
    }
    return id;
}

unsafe extern "C" fn getContext(
    mut parser: crate::expat_h::XML_Parser,
) -> *const crate::expat_external_h::XML_Char {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: 0 as *mut *mut NAMED,
        end: 0 as *mut *mut NAMED,
    };
    let mut needSep: crate::expat_h::XML_Bool =
        crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    if !(*dtd).defaultPrefix.binding.is_null() {
        let mut i: libc::c_int = 0;
        let mut len: libc::c_int = 0;
        if if (*parser).m_tempPool.ptr
            == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
            && poolGrow(&mut (*parser).m_tempPool) == 0
        {
            0 as libc::c_int
        } else {
            let fresh52 = (*parser).m_tempPool.ptr;
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
            *fresh52 = 0x3d as libc::c_int as crate::expat_external_h::XML_Char;
            1 as libc::c_int
        } == 0
        {
            return crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
        }
        len = (*(*dtd).defaultPrefix.binding).uriLen;
        if (*parser).m_namespaceSeparator != 0 {
            len -= 1
        }
        i = 0 as libc::c_int;
        while i < len {
            if if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&mut (*parser).m_tempPool) == 0
            {
                0 as libc::c_int
            } else {
                let fresh53 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh53 = *(*(*dtd).defaultPrefix.binding).uri.offset(i as isize);
                1 as libc::c_int
            } == 0
            {
                /* Because of memory caching, I don't believe this line can be
                 * executed.
                 *
                 * This is part of a loop copying the default prefix binding
                 * URI into the parser's temporary string pool.  Previously,
                 * that URI was copied into the same string pool, with a
                 * terminating NUL character, as part of setContext().  When
                 * the pool was cleared, that leaves a block definitely big
                 * enough to hold the URI on the free block list of the pool.
                 * The URI copy in getContext() therefore cannot run out of
                 * memory.
                 *
                 * If the pool is used between the setContext() and
                 * getContext() calls, the worst it can do is leave a bigger
                 * block on the front of the free list.  Given that this is
                 * all somewhat inobvious and program logic can be changed, we
                 * don't delete the line but we do exclude it from the test
                 * coverage statistics.
                 */
                return crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
                /* LCOV_EXCL_LINE */
            }
            i += 1
        }
        needSep = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool
    }
    hashTableIterInit(&mut iter, &mut (*dtd).prefixes);
    loop
    /* This test appears to be (justifiable) paranoia.  There does
     * not seem to be a way of injecting a prefix without a binding
     * that doesn't get errored long before this function is called.
     * The test should remain for safety's sake, so we instead
     * exclude the following line from the coverage statistics.
     */
    {
        let mut i_0: libc::c_int = 0; /* save one level of indirection */
        let mut len_0: libc::c_int = 0;
        let mut s: *const crate::expat_external_h::XML_Char =
            0 as *const crate::expat_external_h::XML_Char;
        let mut prefix: *mut PREFIX = hashTableIterNext(&mut iter) as *mut PREFIX;
        if prefix.is_null() {
            break;
        }
        if !(*prefix).binding.is_null() {
            if needSep as libc::c_int != 0
                && (if (*parser).m_tempPool.ptr
                    == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                    && poolGrow(&mut (*parser).m_tempPool) == 0
                {
                    0 as libc::c_int
                } else {
                    let fresh54 = (*parser).m_tempPool.ptr;
                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                    *fresh54 = 0xc as libc::c_int as crate::expat_external_h::XML_Char;
                    1 as libc::c_int
                }) == 0
            {
                return crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
            }
            s = (*prefix).name;
            while *s != 0 {
                if if (*parser).m_tempPool.ptr
                    == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                    && poolGrow(&mut (*parser).m_tempPool) == 0
                {
                    0 as libc::c_int
                } else {
                    let fresh55 = (*parser).m_tempPool.ptr;
                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                    *fresh55 = *s;
                    1 as libc::c_int
                } == 0
                {
                    return crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
                }
                s = s.offset(1)
            }
            if if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&mut (*parser).m_tempPool) == 0
            {
                0 as libc::c_int
            } else {
                let fresh56 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh56 = 0x3d as libc::c_int as crate::expat_external_h::XML_Char;
                1 as libc::c_int
            } == 0
            {
                return crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
            }
            len_0 = (*(*prefix).binding).uriLen;
            if (*parser).m_namespaceSeparator != 0 {
                len_0 -= 1
            }
            i_0 = 0 as libc::c_int;
            while i_0 < len_0 {
                if if (*parser).m_tempPool.ptr
                    == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                    && poolGrow(&mut (*parser).m_tempPool) == 0
                {
                    0 as libc::c_int
                } else {
                    let fresh57 = (*parser).m_tempPool.ptr;
                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                    *fresh57 = *(*(*prefix).binding).uri.offset(i_0 as isize);
                    1 as libc::c_int
                } == 0
                {
                    return crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
                }
                i_0 += 1
            }
            needSep = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool
        }
    }
    hashTableIterInit(&mut iter, &mut (*dtd).generalEntities);
    loop {
        let mut s_0: *const crate::expat_external_h::XML_Char =
            0 as *const crate::expat_external_h::XML_Char;
        let mut e: *mut ENTITY = hashTableIterNext(&mut iter) as *mut ENTITY;
        if e.is_null() {
            break;
        }
        if (*e).open == 0 {
            continue;
        }
        if needSep as libc::c_int != 0
            && (if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&mut (*parser).m_tempPool) == 0
            {
                0 as libc::c_int
            } else {
                let fresh58 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh58 = 0xc as libc::c_int as crate::expat_external_h::XML_Char;
                1 as libc::c_int
            }) == 0
        {
            return crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
        }
        s_0 = (*e).name;
        while *s_0 != 0 {
            if if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&mut (*parser).m_tempPool) == 0
            {
                0 as libc::c_int
            } else {
                let fresh59 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh59 = *s_0;
                1 as libc::c_int
            } == 0
            {
                return 0 as *const crate::expat_external_h::XML_Char;
            }
            s_0 = s_0.offset(1)
        }
        needSep = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool
    }
    if if (*parser).m_tempPool.ptr
        == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
        && poolGrow(&mut (*parser).m_tempPool) == 0
    {
        0 as libc::c_int
    } else {
        let fresh60 = (*parser).m_tempPool.ptr;
        (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
        *fresh60 = '\u{0}' as i32 as crate::expat_external_h::XML_Char;
        1 as libc::c_int
    } == 0
    {
        return crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    }
    return (*parser).m_tempPool.start;
}

unsafe extern "C" fn setContext(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Bool {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut s: *const crate::expat_external_h::XML_Char = context;
    while *context as libc::c_int != '\u{0}' as i32 {
        if *s as libc::c_int == 0xc as libc::c_int || *s as libc::c_int == '\u{0}' as i32 {
            let mut e: *mut ENTITY = 0 as *mut ENTITY;
            if if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&mut (*parser).m_tempPool) == 0
            {
                0 as libc::c_int
            } else {
                let fresh61 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh61 = '\u{0}' as i32 as crate::expat_external_h::XML_Char;
                1 as libc::c_int
            } == 0
            {
                return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
            }
            e = lookup(
                parser,
                &mut (*dtd).generalEntities,
                (*parser).m_tempPool.start as KEY,
                0 as libc::c_int as crate::stddef_h::size_t,
            ) as *mut ENTITY;
            if !e.is_null() {
                (*e).open = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool
            }
            if *s as libc::c_int != '\u{0}' as i32 {
                s = s.offset(1)
            }
            context = s;
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.start
        } else if *s as libc::c_int == 0x3d as libc::c_int {
            let mut prefix: *mut PREFIX = 0 as *mut PREFIX;
            if (*parser)
                .m_tempPool
                .ptr
                .wrapping_offset_from((*parser).m_tempPool.start) as libc::c_long
                == 0 as libc::c_int as libc::c_long
            {
                prefix = &mut (*dtd).defaultPrefix
            } else {
                if if (*parser).m_tempPool.ptr
                    == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                    && poolGrow(&mut (*parser).m_tempPool) == 0
                {
                    0 as libc::c_int
                } else {
                    let fresh62 = (*parser).m_tempPool.ptr;
                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                    *fresh62 = '\u{0}' as i32 as crate::expat_external_h::XML_Char;
                    1 as libc::c_int
                } == 0
                {
                    return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                }
                prefix = lookup(
                    parser,
                    &mut (*dtd).prefixes,
                    (*parser).m_tempPool.start as KEY,
                    ::std::mem::size_of::<PREFIX>() as libc::c_ulong,
                ) as *mut PREFIX;
                if prefix.is_null() {
                    return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                }
                if (*prefix).name
                    == (*parser).m_tempPool.start as *const crate::expat_external_h::XML_Char
                {
                    (*prefix).name = poolCopyString(&mut (*dtd).pool, (*prefix).name);
                    if (*prefix).name.is_null() {
                        return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                    }
                }
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.start
            }
            context = s.offset(1 as libc::c_int as isize);
            while *context as libc::c_int != 0xc as libc::c_int
                && *context as libc::c_int != '\u{0}' as i32
            {
                if if (*parser).m_tempPool.ptr
                    == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                    && poolGrow(&mut (*parser).m_tempPool) == 0
                {
                    0 as libc::c_int
                } else {
                    let fresh63 = (*parser).m_tempPool.ptr;
                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                    *fresh63 = *context;
                    1 as libc::c_int
                } == 0
                {
                    return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
                }
                context = context.offset(1)
            }
            if if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&mut (*parser).m_tempPool) == 0
            {
                0 as libc::c_int
            } else {
                let fresh64 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh64 = '\u{0}' as i32 as crate::expat_external_h::XML_Char;
                1 as libc::c_int
            } == 0
            {
                return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
            }
            if addBinding(
                parser,
                prefix,
                crate::stddef_h::NULL as *const ATTRIBUTE_ID,
                (*parser).m_tempPool.start,
                &mut (*parser).m_inheritedBindings,
            ) as libc::c_uint
                != crate::expat_h::XML_ERROR_NONE as libc::c_int as libc::c_uint
            {
                return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
            }
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
            if *context as libc::c_int != '\u{0}' as i32 {
                context = context.offset(1)
            }
            s = context
        } else {
            if if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&mut (*parser).m_tempPool) == 0
            {
                0 as libc::c_int
            } else {
                let fresh65 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh65 = *s;
                1 as libc::c_int
            } == 0
            {
                return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
            }
            s = s.offset(1)
        }
    }
    return crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
}

unsafe extern "C" fn normalizePublicId(mut publicId: *mut crate::expat_external_h::XML_Char) {
    let mut p: *mut crate::expat_external_h::XML_Char = publicId;
    let mut s: *mut crate::expat_external_h::XML_Char = 0 as *mut crate::expat_external_h::XML_Char;
    s = publicId;
    while *s != 0 {
        match *s as libc::c_int {
            32 | 13 | 10 => {
                if p != publicId
                    && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int != 0x20 as libc::c_int
                {
                    let fresh66 = p;
                    p = p.offset(1);
                    *fresh66 = 0x20 as libc::c_int as crate::expat_external_h::XML_Char
                }
            }
            _ => {
                let fresh67 = p;
                p = p.offset(1);
                *fresh67 = *s
            }
        }
        s = s.offset(1)
    }
    if p != publicId
        && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == 0x20 as libc::c_int
    {
        p = p.offset(-1)
    }
    *p = '\u{0}' as i32 as crate::expat_external_h::XML_Char;
}

unsafe extern "C" fn dtdCreate(
    mut ms: *const crate::expat_h::XML_Memory_Handling_Suite,
) -> *mut DTD {
    let mut p: *mut DTD = (*ms).malloc_fcn.expect("non-null function pointer")(
        ::std::mem::size_of::<DTD>() as libc::c_ulong,
    ) as *mut DTD;
    if p.is_null() {
        return p;
    }
    poolInit(&mut (*p).pool, ms);
    poolInit(&mut (*p).entityValuePool, ms);
    hashTableInit(&mut (*p).generalEntities, ms);
    hashTableInit(&mut (*p).elementTypes, ms);
    hashTableInit(&mut (*p).attributeIds, ms);
    hashTableInit(&mut (*p).prefixes, ms);
    (*p).paramEntityRead = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    hashTableInit(&mut (*p).paramEntities, ms);
    /* XML_DTD */
    (*p).defaultPrefix.name = crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    (*p).defaultPrefix.binding = crate::stddef_h::NULL as *mut BINDING;
    (*p).in_eldecl = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    (*p).scaffIndex = crate::stddef_h::NULL as *mut libc::c_int;
    (*p).scaffold = crate::stddef_h::NULL as *mut CONTENT_SCAFFOLD;
    (*p).scaffLevel = 0 as libc::c_int;
    (*p).scaffSize = 0 as libc::c_int as libc::c_uint;
    (*p).scaffCount = 0 as libc::c_int as libc::c_uint;
    (*p).contentStringLen = 0 as libc::c_int as libc::c_uint;
    (*p).keepProcessing = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
    (*p).hasParamEntityRefs = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    (*p).standalone = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    return p;
}
/* do not call if m_parentParser != NULL */

unsafe extern "C" fn dtdReset(
    mut p: *mut DTD,
    mut ms: *const crate::expat_h::XML_Memory_Handling_Suite,
) {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: 0 as *mut *mut NAMED,
        end: 0 as *mut *mut NAMED,
    };
    hashTableIterInit(&mut iter, &mut (*p).elementTypes);
    loop {
        let mut e: *mut ELEMENT_TYPE = hashTableIterNext(&mut iter) as *mut ELEMENT_TYPE;
        if e.is_null() {
            break;
        }
        if (*e).allocDefaultAtts != 0 as libc::c_int {
            (*ms).free_fcn.expect("non-null function pointer")(
                (*e).defaultAtts as *mut libc::c_void,
            );
        }
    }
    hashTableClear(&mut (*p).generalEntities);
    (*p).paramEntityRead = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    hashTableClear(&mut (*p).paramEntities);
    /* XML_DTD */
    hashTableClear(&mut (*p).elementTypes);
    hashTableClear(&mut (*p).attributeIds);
    hashTableClear(&mut (*p).prefixes);
    poolClear(&mut (*p).pool);
    poolClear(&mut (*p).entityValuePool);
    (*p).defaultPrefix.name = crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    (*p).defaultPrefix.binding = crate::stddef_h::NULL as *mut BINDING;
    (*p).in_eldecl = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    (*ms).free_fcn.expect("non-null function pointer")((*p).scaffIndex as *mut libc::c_void);
    (*p).scaffIndex = crate::stddef_h::NULL as *mut libc::c_int;
    (*ms).free_fcn.expect("non-null function pointer")((*p).scaffold as *mut libc::c_void);
    (*p).scaffold = crate::stddef_h::NULL as *mut CONTENT_SCAFFOLD;
    (*p).scaffLevel = 0 as libc::c_int;
    (*p).scaffSize = 0 as libc::c_int as libc::c_uint;
    (*p).scaffCount = 0 as libc::c_int as libc::c_uint;
    (*p).contentStringLen = 0 as libc::c_int as libc::c_uint;
    (*p).keepProcessing = crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
    (*p).hasParamEntityRefs = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
    (*p).standalone = crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
}

unsafe extern "C" fn dtdDestroy(
    mut p: *mut DTD,
    mut isDocEntity: crate::expat_h::XML_Bool,
    mut ms: *const crate::expat_h::XML_Memory_Handling_Suite,
) {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: 0 as *mut *mut NAMED,
        end: 0 as *mut *mut NAMED,
    };
    hashTableIterInit(&mut iter, &mut (*p).elementTypes);
    loop {
        let mut e: *mut ELEMENT_TYPE = hashTableIterNext(&mut iter) as *mut ELEMENT_TYPE;
        if e.is_null() {
            break;
        }
        if (*e).allocDefaultAtts != 0 as libc::c_int {
            (*ms).free_fcn.expect("non-null function pointer")(
                (*e).defaultAtts as *mut libc::c_void,
            );
        }
    }
    hashTableDestroy(&mut (*p).generalEntities);
    hashTableDestroy(&mut (*p).paramEntities);
    /* XML_DTD */
    hashTableDestroy(&mut (*p).elementTypes);
    hashTableDestroy(&mut (*p).attributeIds);
    hashTableDestroy(&mut (*p).prefixes);
    poolDestroy(&mut (*p).pool);
    poolDestroy(&mut (*p).entityValuePool);
    if isDocEntity != 0 {
        (*ms).free_fcn.expect("non-null function pointer")((*p).scaffIndex as *mut libc::c_void);
        (*ms).free_fcn.expect("non-null function pointer")((*p).scaffold as *mut libc::c_void);
    }
    (*ms).free_fcn.expect("non-null function pointer")(p as *mut libc::c_void);
}
/* Do a deep copy of the DTD. Return 0 for out of memory, non-zero otherwise.
   The new DTD has already been initialized.
*/

unsafe extern "C" fn dtdCopy(
    mut oldParser: crate::expat_h::XML_Parser,
    mut newDtd: *mut DTD,
    mut oldDtd: *const DTD,
    mut ms: *const crate::expat_h::XML_Memory_Handling_Suite,
) -> libc::c_int {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: 0 as *mut *mut NAMED,
        end: 0 as *mut *mut NAMED,
    };
    /* Copy the prefix table. */
    hashTableIterInit(&mut iter, &(*oldDtd).prefixes);
    loop {
        let mut name: *const crate::expat_external_h::XML_Char =
            0 as *const crate::expat_external_h::XML_Char;
        let mut oldP: *const PREFIX = hashTableIterNext(&mut iter) as *mut PREFIX;
        if oldP.is_null() {
            break;
        }
        name = poolCopyString(&mut (*newDtd).pool, (*oldP).name);
        if name.is_null() {
            return 0 as libc::c_int;
        }
        if lookup(
            oldParser,
            &mut (*newDtd).prefixes,
            name,
            ::std::mem::size_of::<PREFIX>() as libc::c_ulong,
        )
        .is_null()
        {
            return 0 as libc::c_int;
        }
    }
    hashTableIterInit(&mut iter, &(*oldDtd).attributeIds);
    loop
    /* Copy the attribute id table. */
    {
        let mut newA: *mut ATTRIBUTE_ID = 0 as *mut ATTRIBUTE_ID;
        let mut name_0: *const crate::expat_external_h::XML_Char =
            0 as *const crate::expat_external_h::XML_Char;
        let mut oldA: *const ATTRIBUTE_ID = hashTableIterNext(&mut iter) as *mut ATTRIBUTE_ID;
        if oldA.is_null() {
            break;
        }
        /* Remember to allocate the scratch byte before the name. */
        if if (*newDtd).pool.ptr == (*newDtd).pool.end as *mut crate::expat_external_h::XML_Char
            && poolGrow(&mut (*newDtd).pool) == 0
        {
            0 as libc::c_int
        } else {
            let fresh68 = (*newDtd).pool.ptr;
            (*newDtd).pool.ptr = (*newDtd).pool.ptr.offset(1);
            *fresh68 = '\u{0}' as i32 as crate::expat_external_h::XML_Char;
            1 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        name_0 = poolCopyString(&mut (*newDtd).pool, (*oldA).name);
        if name_0.is_null() {
            return 0 as libc::c_int;
        }
        name_0 = name_0.offset(1);
        newA = lookup(
            oldParser,
            &mut (*newDtd).attributeIds,
            name_0,
            ::std::mem::size_of::<ATTRIBUTE_ID>() as libc::c_ulong,
        ) as *mut ATTRIBUTE_ID;
        if newA.is_null() {
            return 0 as libc::c_int;
        }
        (*newA).maybeTokenized = (*oldA).maybeTokenized;
        if !(*oldA).prefix.is_null() {
            (*newA).xmlns = (*oldA).xmlns;
            if (*oldA).prefix == &(*oldDtd).defaultPrefix as *const PREFIX as *mut PREFIX {
                (*newA).prefix = &mut (*newDtd).defaultPrefix
            } else {
                (*newA).prefix = lookup(
                    oldParser,
                    &mut (*newDtd).prefixes,
                    (*(*oldA).prefix).name,
                    0 as libc::c_int as crate::stddef_h::size_t,
                ) as *mut PREFIX
            }
        }
    }
    /* Copy the element type table. */
    hashTableIterInit(&mut iter, &(*oldDtd).elementTypes);
    loop {
        let mut i: libc::c_int = 0;
        let mut newE: *mut ELEMENT_TYPE = 0 as *mut ELEMENT_TYPE;
        let mut name_1: *const crate::expat_external_h::XML_Char =
            0 as *const crate::expat_external_h::XML_Char;
        let mut oldE: *const ELEMENT_TYPE = hashTableIterNext(&mut iter) as *mut ELEMENT_TYPE;
        if oldE.is_null() {
            break;
        }
        name_1 = poolCopyString(&mut (*newDtd).pool, (*oldE).name);
        if name_1.is_null() {
            return 0 as libc::c_int;
        }
        newE = lookup(
            oldParser,
            &mut (*newDtd).elementTypes,
            name_1,
            ::std::mem::size_of::<ELEMENT_TYPE>() as libc::c_ulong,
        ) as *mut ELEMENT_TYPE;
        if newE.is_null() {
            return 0 as libc::c_int;
        }
        if (*oldE).nDefaultAtts != 0 {
            (*newE).defaultAtts = (*ms).malloc_fcn.expect("non-null function pointer")(
                ((*oldE).nDefaultAtts as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<DEFAULT_ATTRIBUTE>() as libc::c_ulong),
            ) as *mut DEFAULT_ATTRIBUTE;
            if (*newE).defaultAtts.is_null() {
                return 0 as libc::c_int;
            }
        }
        if !(*oldE).idAtt.is_null() {
            (*newE).idAtt = lookup(
                oldParser,
                &mut (*newDtd).attributeIds,
                (*(*oldE).idAtt).name as KEY,
                0 as libc::c_int as crate::stddef_h::size_t,
            ) as *mut ATTRIBUTE_ID
        }
        (*newE).nDefaultAtts = (*oldE).nDefaultAtts;
        (*newE).allocDefaultAtts = (*newE).nDefaultAtts;
        if !(*oldE).prefix.is_null() {
            (*newE).prefix = lookup(
                oldParser,
                &mut (*newDtd).prefixes,
                (*(*oldE).prefix).name,
                0 as libc::c_int as crate::stddef_h::size_t,
            ) as *mut PREFIX
        }
        i = 0 as libc::c_int;
        while i < (*newE).nDefaultAtts {
            let ref mut fresh69 = (*(*newE).defaultAtts.offset(i as isize)).id;
            *fresh69 = lookup(
                oldParser,
                &mut (*newDtd).attributeIds,
                (*(*(*oldE).defaultAtts.offset(i as isize)).id).name as KEY,
                0 as libc::c_int as crate::stddef_h::size_t,
            ) as *mut ATTRIBUTE_ID;
            (*(*newE).defaultAtts.offset(i as isize)).isCdata =
                (*(*oldE).defaultAtts.offset(i as isize)).isCdata;
            if !(*(*oldE).defaultAtts.offset(i as isize)).value.is_null() {
                let ref mut fresh70 = (*(*newE).defaultAtts.offset(i as isize)).value;
                *fresh70 = poolCopyString(
                    &mut (*newDtd).pool,
                    (*(*oldE).defaultAtts.offset(i as isize)).value,
                );
                if (*(*newE).defaultAtts.offset(i as isize)).value.is_null() {
                    return 0 as libc::c_int;
                }
            } else {
                let ref mut fresh71 = (*(*newE).defaultAtts.offset(i as isize)).value;
                *fresh71 = crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char
            }
            i += 1
        }
    }
    /* Copy the entity tables. */
    if copyEntityTable(
        oldParser,
        &mut (*newDtd).generalEntities,
        &mut (*newDtd).pool,
        &(*oldDtd).generalEntities,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if copyEntityTable(
        oldParser,
        &mut (*newDtd).paramEntities,
        &mut (*newDtd).pool,
        &(*oldDtd).paramEntities,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    (*newDtd).paramEntityRead = (*oldDtd).paramEntityRead;
    /* XML_DTD */
    (*newDtd).keepProcessing = (*oldDtd).keepProcessing;
    (*newDtd).hasParamEntityRefs = (*oldDtd).hasParamEntityRefs;
    (*newDtd).standalone = (*oldDtd).standalone;
    /* Don't want deep copying for scaffolding */
    (*newDtd).in_eldecl = (*oldDtd).in_eldecl;
    (*newDtd).scaffold = (*oldDtd).scaffold;
    (*newDtd).contentStringLen = (*oldDtd).contentStringLen;
    (*newDtd).scaffSize = (*oldDtd).scaffSize;
    (*newDtd).scaffLevel = (*oldDtd).scaffLevel;
    (*newDtd).scaffIndex = (*oldDtd).scaffIndex;
    return 1 as libc::c_int;
}
/* End dtdCopy */

unsafe extern "C" fn copyEntityTable(
    mut oldParser: crate::expat_h::XML_Parser,
    mut newTable: *mut HASH_TABLE,
    mut newPool: *mut STRING_POOL,
    mut oldTable: *const HASH_TABLE,
) -> libc::c_int {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: 0 as *mut *mut NAMED,
        end: 0 as *mut *mut NAMED,
    };
    let mut cachedOldBase: *const crate::expat_external_h::XML_Char =
        crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    let mut cachedNewBase: *const crate::expat_external_h::XML_Char =
        crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    hashTableIterInit(&mut iter, oldTable);
    loop {
        let mut newE: *mut ENTITY = 0 as *mut ENTITY;
        let mut name: *const crate::expat_external_h::XML_Char =
            0 as *const crate::expat_external_h::XML_Char;
        let mut oldE: *const ENTITY = hashTableIterNext(&mut iter) as *mut ENTITY;
        if oldE.is_null() {
            break;
        }
        name = poolCopyString(newPool, (*oldE).name);
        if name.is_null() {
            return 0 as libc::c_int;
        }
        newE = lookup(
            oldParser,
            newTable,
            name,
            ::std::mem::size_of::<ENTITY>() as libc::c_ulong,
        ) as *mut ENTITY;
        if newE.is_null() {
            return 0 as libc::c_int;
        }
        if !(*oldE).systemId.is_null() {
            let mut tem: *const crate::expat_external_h::XML_Char =
                poolCopyString(newPool, (*oldE).systemId);
            if tem.is_null() {
                return 0 as libc::c_int;
            }
            (*newE).systemId = tem;
            if !(*oldE).base.is_null() {
                if (*oldE).base == cachedOldBase {
                    (*newE).base = cachedNewBase
                } else {
                    cachedOldBase = (*oldE).base;
                    tem = poolCopyString(newPool, cachedOldBase);
                    if tem.is_null() {
                        return 0 as libc::c_int;
                    }
                    (*newE).base = tem;
                    cachedNewBase = (*newE).base
                }
            }
            if !(*oldE).publicId.is_null() {
                tem = poolCopyString(newPool, (*oldE).publicId);
                if tem.is_null() {
                    return 0 as libc::c_int;
                }
                (*newE).publicId = tem
            }
        } else {
            let mut tem_0: *const crate::expat_external_h::XML_Char =
                poolCopyStringN(newPool, (*oldE).textPtr, (*oldE).textLen);
            if tem_0.is_null() {
                return 0 as libc::c_int;
            }
            (*newE).textPtr = tem_0;
            (*newE).textLen = (*oldE).textLen
        }
        if !(*oldE).notation.is_null() {
            let mut tem_1: *const crate::expat_external_h::XML_Char =
                poolCopyString(newPool, (*oldE).notation);
            if tem_1.is_null() {
                return 0 as libc::c_int;
            }
            (*newE).notation = tem_1
        }
        (*newE).is_param = (*oldE).is_param;
        (*newE).is_internal = (*oldE).is_internal
    }
    return 1 as libc::c_int;
}

pub const INIT_POWER: libc::c_int = 6 as libc::c_int;

unsafe extern "C" fn keyeq(mut s1: KEY, mut s2: KEY) -> crate::expat_h::XML_Bool {
    while *s1 as libc::c_int == *s2 as libc::c_int {
        if *s1 as libc::c_int == 0 as libc::c_int {
            return crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1)
    }
    return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
}

unsafe extern "C" fn keylen(mut s: KEY) -> crate::stddef_h::size_t {
    let mut len: crate::stddef_h::size_t = 0 as libc::c_int as crate::stddef_h::size_t;
    while *s != 0 {
        s = s.offset(1);
        len = len.wrapping_add(1)
    }
    return len;
}

unsafe extern "C" fn copy_salt_to_sipkey(
    mut parser: crate::expat_h::XML_Parser,
    mut key: *mut crate::siphash_h::sipkey,
) {
    (*key).k[0 as libc::c_int as usize] = 0 as libc::c_int as crate::stdlib::uint64_t;
    (*key).k[1 as libc::c_int as usize] = get_hash_secret_salt(parser);
}

unsafe extern "C" fn hash(mut parser: crate::expat_h::XML_Parser, mut s: KEY) -> libc::c_ulong {
    let mut state: crate::siphash_h::siphash = crate::siphash_h::siphash {
        v0: 0,
        v1: 0,
        v2: 0,
        v3: 0,
        buf: [0; 8],
        p: 0 as *mut libc::c_uchar,
        c: 0,
    };
    let mut key: crate::siphash_h::sipkey = crate::siphash_h::sipkey { k: [0; 2] };
    copy_salt_to_sipkey(parser, &mut key);
    sip24_init(&mut state, &mut key);
    sip24_update(
        &mut state,
        s as *const libc::c_void,
        keylen(s).wrapping_mul(
            ::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong
        ),
    );
    return sip24_final(&mut state);
}

unsafe extern "C" fn lookup(
    mut parser: crate::expat_h::XML_Parser,
    mut table: *mut HASH_TABLE,
    mut name: KEY,
    mut createSize: crate::stddef_h::size_t,
) -> *mut NAMED {
    let mut i: crate::stddef_h::size_t = 0;
    if (*table).size == 0 as libc::c_int as libc::c_ulong {
        let mut tsize: crate::stddef_h::size_t = 0;
        if createSize == 0 {
            return crate::stddef_h::NULL as *mut NAMED;
        }
        (*table).power = INIT_POWER as libc::c_uchar;
        /* table->size is a power of 2 */
        (*table).size = (1 as libc::c_int as crate::stddef_h::size_t) << INIT_POWER;
        tsize = (*table)
            .size
            .wrapping_mul(::std::mem::size_of::<*mut NAMED>() as libc::c_ulong);
        (*table).v = (*(*table).mem)
            .malloc_fcn
            .expect("non-null function pointer")(tsize) as *mut *mut NAMED;
        if (*table).v.is_null() {
            (*table).size = 0 as libc::c_int as crate::stddef_h::size_t;
            return crate::stddef_h::NULL as *mut NAMED;
        }
        crate::stdlib::memset((*table).v as *mut libc::c_void, 0 as libc::c_int, tsize);
        i = hash(parser, name)
            & (*table)
                .size
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    } else {
        let mut h: libc::c_ulong = hash(parser, name);
        let mut mask: libc::c_ulong = (*table)
            .size
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut step: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
        i = h & mask;
        while !(*(*table).v.offset(i as isize)).is_null() {
            if keyeq(name, (**(*table).v.offset(i as isize)).name) != 0 {
                return *(*table).v.offset(i as isize);
            }
            if step == 0 {
                step = ((h & !mask) >> (*table).power as libc::c_int - 1 as libc::c_int
                    & mask >> 2 as libc::c_int
                    | 1 as libc::c_int as libc::c_ulong) as libc::c_uchar
            }
            if i < step as libc::c_ulong {
                i = (i as libc::c_ulong)
                    .wrapping_add((*table).size.wrapping_sub(step as libc::c_ulong))
                    as crate::stddef_h::size_t as crate::stddef_h::size_t
            } else {
                i = (i as libc::c_ulong).wrapping_sub(step as libc::c_ulong)
                    as crate::stddef_h::size_t as crate::stddef_h::size_t
            };
        }
        if createSize == 0 {
            return crate::stddef_h::NULL as *mut NAMED;
        }
        /* check for overflow (table is half full) */
        if (*table).used >> (*table).power as libc::c_int - 1 as libc::c_int != 0 {
            let mut newPower: libc::c_uchar =
                ((*table).power as libc::c_int + 1 as libc::c_int) as libc::c_uchar;
            let mut newSize: crate::stddef_h::size_t =
                (1 as libc::c_int as crate::stddef_h::size_t) << newPower as libc::c_int;
            let mut newMask: libc::c_ulong =
                newSize.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let mut tsize_0: crate::stddef_h::size_t =
                newSize.wrapping_mul(::std::mem::size_of::<*mut NAMED>() as libc::c_ulong);
            let mut newV: *mut *mut NAMED = (*(*table).mem)
                .malloc_fcn
                .expect("non-null function pointer")(
                tsize_0
            ) as *mut *mut NAMED;
            if newV.is_null() {
                return crate::stddef_h::NULL as *mut NAMED;
            }
            crate::stdlib::memset(newV as *mut libc::c_void, 0 as libc::c_int, tsize_0);
            i = 0 as libc::c_int as crate::stddef_h::size_t;
            while i < (*table).size {
                if !(*(*table).v.offset(i as isize)).is_null() {
                    let mut newHash: libc::c_ulong =
                        hash(parser, (**(*table).v.offset(i as isize)).name);
                    let mut j: crate::stddef_h::size_t = newHash & newMask;
                    step = 0 as libc::c_int as libc::c_uchar;
                    while !(*newV.offset(j as isize)).is_null() {
                        if step == 0 {
                            step = ((newHash & !newMask)
                                >> newPower as libc::c_int - 1 as libc::c_int
                                & newMask >> 2 as libc::c_int
                                | 1 as libc::c_int as libc::c_ulong)
                                as libc::c_uchar
                        }
                        if j < step as libc::c_ulong {
                            j = (j as libc::c_ulong)
                                .wrapping_add(newSize.wrapping_sub(step as libc::c_ulong))
                                as crate::stddef_h::size_t
                                as crate::stddef_h::size_t
                        } else {
                            j = (j as libc::c_ulong).wrapping_sub(step as libc::c_ulong)
                                as crate::stddef_h::size_t
                                as crate::stddef_h::size_t
                        };
                    }
                    let ref mut fresh72 = *newV.offset(j as isize);
                    *fresh72 = *(*table).v.offset(i as isize)
                }
                i = i.wrapping_add(1)
            }
            (*(*table).mem).free_fcn.expect("non-null function pointer")(
                (*table).v as *mut libc::c_void,
            );
            (*table).v = newV;
            (*table).power = newPower;
            (*table).size = newSize;
            i = h & newMask;
            step = 0 as libc::c_int as libc::c_uchar;
            while !(*(*table).v.offset(i as isize)).is_null() {
                if step == 0 {
                    step = ((h & !newMask) >> newPower as libc::c_int - 1 as libc::c_int
                        & newMask >> 2 as libc::c_int
                        | 1 as libc::c_int as libc::c_ulong)
                        as libc::c_uchar
                }
                if i < step as libc::c_ulong {
                    i = (i as libc::c_ulong)
                        .wrapping_add(newSize.wrapping_sub(step as libc::c_ulong))
                        as crate::stddef_h::size_t
                        as crate::stddef_h::size_t
                } else {
                    i = (i as libc::c_ulong).wrapping_sub(step as libc::c_ulong)
                        as crate::stddef_h::size_t
                        as crate::stddef_h::size_t
                };
            }
        }
    }
    let ref mut fresh73 = *(*table).v.offset(i as isize);
    *fresh73 = (*(*table).mem)
        .malloc_fcn
        .expect("non-null function pointer")(createSize) as *mut NAMED;
    if (*(*table).v.offset(i as isize)).is_null() {
        return crate::stddef_h::NULL as *mut NAMED;
    }
    crate::stdlib::memset(
        *(*table).v.offset(i as isize) as *mut libc::c_void,
        0 as libc::c_int,
        createSize,
    );
    let ref mut fresh74 = (**(*table).v.offset(i as isize)).name;
    *fresh74 = name;
    (*table).used = (*table).used.wrapping_add(1);
    return *(*table).v.offset(i as isize);
}

unsafe extern "C" fn hashTableClear(mut table: *mut HASH_TABLE) {
    let mut i: crate::stddef_h::size_t = 0;
    i = 0 as libc::c_int as crate::stddef_h::size_t;
    while i < (*table).size {
        (*(*table).mem).free_fcn.expect("non-null function pointer")(
            *(*table).v.offset(i as isize) as *mut libc::c_void,
        );
        let ref mut fresh75 = *(*table).v.offset(i as isize);
        *fresh75 = crate::stddef_h::NULL as *mut NAMED;
        i = i.wrapping_add(1)
    }
    (*table).used = 0 as libc::c_int as crate::stddef_h::size_t;
}

unsafe extern "C" fn hashTableDestroy(mut table: *mut HASH_TABLE) {
    let mut i: crate::stddef_h::size_t = 0;
    i = 0 as libc::c_int as crate::stddef_h::size_t;
    while i < (*table).size {
        (*(*table).mem).free_fcn.expect("non-null function pointer")(
            *(*table).v.offset(i as isize) as *mut libc::c_void,
        );
        i = i.wrapping_add(1)
    }
    (*(*table).mem).free_fcn.expect("non-null function pointer")((*table).v as *mut libc::c_void);
}

unsafe extern "C" fn hashTableInit(
    mut p: *mut HASH_TABLE,
    mut ms: *const crate::expat_h::XML_Memory_Handling_Suite,
) {
    (*p).power = 0 as libc::c_int as libc::c_uchar;
    (*p).size = 0 as libc::c_int as crate::stddef_h::size_t;
    (*p).used = 0 as libc::c_int as crate::stddef_h::size_t;
    (*p).v = crate::stddef_h::NULL as *mut *mut NAMED;
    (*p).mem = ms;
}

unsafe extern "C" fn hashTableIterInit(
    mut iter: *mut HASH_TABLE_ITER,
    mut table: *const HASH_TABLE,
) {
    (*iter).p = (*table).v;
    (*iter).end = (*iter).p.offset((*table).size as isize);
}

unsafe extern "C" fn hashTableIterNext(mut iter: *mut HASH_TABLE_ITER) -> *mut NAMED {
    while (*iter).p != (*iter).end {
        let fresh76 = (*iter).p;
        (*iter).p = (*iter).p.offset(1);
        let mut tem: *mut NAMED = *fresh76;
        if !tem.is_null() {
            return tem;
        }
    }
    return crate::stddef_h::NULL as *mut NAMED;
}

unsafe extern "C" fn poolInit(
    mut pool: *mut STRING_POOL,
    mut ms: *const crate::expat_h::XML_Memory_Handling_Suite,
) {
    (*pool).blocks = crate::stddef_h::NULL as *mut BLOCK;
    (*pool).freeBlocks = crate::stddef_h::NULL as *mut BLOCK;
    (*pool).start = crate::stddef_h::NULL as *mut crate::expat_external_h::XML_Char;
    (*pool).ptr = crate::stddef_h::NULL as *mut crate::expat_external_h::XML_Char;
    (*pool).end = crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    (*pool).mem = ms;
}

unsafe extern "C" fn poolClear(mut pool: *mut STRING_POOL) {
    if (*pool).freeBlocks.is_null() {
        (*pool).freeBlocks = (*pool).blocks
    } else {
        let mut p: *mut BLOCK = (*pool).blocks;
        while !p.is_null() {
            let mut tem: *mut BLOCK = (*p).next;
            (*p).next = (*pool).freeBlocks;
            (*pool).freeBlocks = p;
            p = tem
        }
    }
    (*pool).blocks = crate::stddef_h::NULL as *mut BLOCK;
    (*pool).start = crate::stddef_h::NULL as *mut crate::expat_external_h::XML_Char;
    (*pool).ptr = crate::stddef_h::NULL as *mut crate::expat_external_h::XML_Char;
    (*pool).end = crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
}

unsafe extern "C" fn poolDestroy(mut pool: *mut STRING_POOL) {
    let mut p: *mut BLOCK = (*pool).blocks;
    while !p.is_null() {
        let mut tem: *mut BLOCK = (*p).next;
        (*(*pool).mem).free_fcn.expect("non-null function pointer")(p as *mut libc::c_void);
        p = tem
    }
    p = (*pool).freeBlocks;
    while !p.is_null() {
        let mut tem_0: *mut BLOCK = (*p).next;
        (*(*pool).mem).free_fcn.expect("non-null function pointer")(p as *mut libc::c_void);
        p = tem_0
    }
}

unsafe extern "C" fn poolAppend(
    mut pool: *mut STRING_POOL,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut ptr: *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut crate::expat_external_h::XML_Char {
    if (*pool).ptr.is_null() && poolGrow(pool) == 0 {
        return crate::stddef_h::NULL as *mut crate::expat_external_h::XML_Char;
    }
    loop {
        let convert_res: crate::src::lib::xmltok::XML_Convert_Result =
            (*enc).utf8Convert.expect("non-null function pointer")(
                enc,
                &mut ptr,
                end,
                &mut (*pool).ptr as *mut *mut crate::expat_external_h::XML_Char as *mut *mut ICHAR,
                (*pool).end as *mut ICHAR,
            );
        if convert_res as libc::c_uint
            == crate::src::lib::xmltok::XML_CONVERT_COMPLETED as libc::c_int as libc::c_uint
            || convert_res as libc::c_uint
                == crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE as libc::c_int
                    as libc::c_uint
        {
            break;
        }
        if poolGrow(pool) == 0 {
            return crate::stddef_h::NULL as *mut crate::expat_external_h::XML_Char;
        }
    }
    return (*pool).start;
}

unsafe extern "C" fn poolCopyString(
    mut pool: *mut STRING_POOL,
    mut s: *const crate::expat_external_h::XML_Char,
) -> *const crate::expat_external_h::XML_Char {
    loop {
        if if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char
            && poolGrow(pool) == 0
        {
            0 as libc::c_int
        } else {
            let fresh77 = (*pool).ptr;
            (*pool).ptr = (*pool).ptr.offset(1);
            *fresh77 = *s;
            1 as libc::c_int
        } == 0
        {
            return crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
        }
        let fresh78 = s;
        s = s.offset(1);
        if !(*fresh78 != 0) {
            break;
        }
    }
    s = (*pool).start;
    (*pool).start = (*pool).ptr;
    return s;
}

unsafe extern "C" fn poolCopyStringN(
    mut pool: *mut STRING_POOL,
    mut s: *const crate::expat_external_h::XML_Char,
    mut n: libc::c_int,
) -> *const crate::expat_external_h::XML_Char {
    if (*pool).ptr.is_null() && poolGrow(pool) == 0 {
        /* The following line is unreachable given the current usage of
         * poolCopyStringN().  Currently it is called from exactly one
         * place to copy the text of a simple general entity.  By that
         * point, the name of the entity is already stored in the pool, so
         * pool->ptr cannot be NULL.
         *
         * If poolCopyStringN() is used elsewhere as it well might be,
         * this line may well become executable again.  Regardless, this
         * sort of check shouldn't be removed lightly, so we just exclude
         * it from the coverage statistics.
         */
        return crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
        /* LCOV_EXCL_LINE */
    }
    while n > 0 as libc::c_int {
        if if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char
            && poolGrow(pool) == 0
        {
            0 as libc::c_int
        } else {
            let fresh79 = (*pool).ptr;
            (*pool).ptr = (*pool).ptr.offset(1);
            *fresh79 = *s;
            1 as libc::c_int
        } == 0
        {
            return crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
        }
        n -= 1;
        s = s.offset(1)
    }
    s = (*pool).start;
    (*pool).start = (*pool).ptr;
    return s;
}

unsafe extern "C" fn poolAppendString(
    mut pool: *mut STRING_POOL,
    mut s: *const crate::expat_external_h::XML_Char,
) -> *const crate::expat_external_h::XML_Char {
    while *s != 0 {
        if if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char
            && poolGrow(pool) == 0
        {
            0 as libc::c_int
        } else {
            let fresh80 = (*pool).ptr;
            (*pool).ptr = (*pool).ptr.offset(1);
            *fresh80 = *s;
            1 as libc::c_int
        } == 0
        {
            return crate::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
        }
        s = s.offset(1)
    }
    return (*pool).start;
}

unsafe extern "C" fn poolStoreString(
    mut pool: *mut STRING_POOL,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut ptr: *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut crate::expat_external_h::XML_Char {
    if poolAppend(pool, enc, ptr, end).is_null() {
        return crate::stddef_h::NULL as *mut crate::expat_external_h::XML_Char;
    }
    if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char && poolGrow(pool) == 0 {
        return crate::stddef_h::NULL as *mut crate::expat_external_h::XML_Char;
    }
    let fresh81 = (*pool).ptr;
    (*pool).ptr = (*pool).ptr.offset(1);
    *fresh81 = 0 as libc::c_int as crate::expat_external_h::XML_Char;
    return (*pool).start;
}

unsafe extern "C" fn poolBytesToAllocateFor(mut blockSize: libc::c_int) -> crate::stddef_h::size_t {
    /* Unprotected math would be:
     ** return offsetof(BLOCK, s) + blockSize * sizeof(XML_Char);
     **
     ** Detect overflow, avoiding _signed_ overflow undefined behavior
     ** For a + b * c we check b * c in isolation first, so that addition of a
     ** on top has no chance of making us accept a small non-negative number
     */
    let stretch: crate::stddef_h::size_t =
        ::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong; /* can be 4 bytes */
    if blockSize <= 0 as libc::c_int {
        return 0 as libc::c_int as crate::stddef_h::size_t;
    }
    if blockSize > (::libc::INT_MAX as libc::c_ulong).wrapping_div(stretch) as libc::c_int {
        return 0 as libc::c_int as crate::stddef_h::size_t;
    }
    let stretchedBlockSize: libc::c_int = blockSize * stretch as libc::c_int;
    let bytesToAllocate: libc::c_int = (12 as libc::c_ulong)
        .wrapping_add(stretchedBlockSize as libc::c_uint as libc::c_ulong)
        as libc::c_int;
    if bytesToAllocate < 0 as libc::c_int {
        return 0 as libc::c_int as crate::stddef_h::size_t;
    }
    return bytesToAllocate as crate::stddef_h::size_t;
}

unsafe extern "C" fn poolGrow(mut pool: *mut STRING_POOL) -> crate::expat_h::XML_Bool {
    if !(*pool).freeBlocks.is_null() {
        if (*pool).start.is_null() {
            (*pool).blocks = (*pool).freeBlocks;
            (*pool).freeBlocks = (*(*pool).freeBlocks).next;
            (*(*pool).blocks).next = crate::stddef_h::NULL as *mut block;
            (*pool).start = (*(*pool).blocks).s.as_mut_ptr();
            (*pool).end = (*pool).start.offset((*(*pool).blocks).size as isize);
            (*pool).ptr = (*pool).start;
            return crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
        }
        if ((*pool).end.wrapping_offset_from((*pool).start) as libc::c_long)
            < (*(*pool).freeBlocks).size as libc::c_long
        {
            let mut tem: *mut BLOCK = (*(*pool).freeBlocks).next;
            (*(*pool).freeBlocks).next = (*pool).blocks;
            (*pool).blocks = (*pool).freeBlocks;
            (*pool).freeBlocks = tem;
            crate::stdlib::memcpy(
                (*(*pool).blocks).s.as_mut_ptr() as *mut libc::c_void,
                (*pool).start as *const libc::c_void,
                ((*pool).end.wrapping_offset_from((*pool).start) as libc::c_long as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong
                    ),
            );
            (*pool).ptr = (*(*pool).blocks)
                .s
                .as_mut_ptr()
                .offset((*pool).ptr.wrapping_offset_from((*pool).start) as libc::c_long as isize);
            (*pool).start = (*(*pool).blocks).s.as_mut_ptr();
            (*pool).end = (*pool).start.offset((*(*pool).blocks).size as isize);
            return crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
        }
    }
    if !(*pool).blocks.is_null() && (*pool).start == (*(*pool).blocks).s.as_mut_ptr() {
        let mut temp: *mut BLOCK = 0 as *mut BLOCK;
        let mut blockSize: libc::c_int =
            ((*pool).end.wrapping_offset_from((*pool).start) as libc::c_long as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint) as libc::c_int;
        let mut bytesToAllocate: crate::stddef_h::size_t = 0;
        /* NOTE: Needs to be calculated prior to calling `realloc`
        to avoid dangling pointers: */
        let offsetInsideBlock: crate::stddef_h::ptrdiff_t =
            (*pool).ptr.wrapping_offset_from((*pool).start) as libc::c_long;
        if blockSize < 0 as libc::c_int {
            /* This condition traps a situation where either more than
             * INT_MAX/2 bytes have already been allocated.  This isn't
             * readily testable, since it is unlikely that an average
             * machine will have that much memory, so we exclude it from the
             * coverage statistics.
             */
            return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
            /* LCOV_EXCL_LINE */
        }
        bytesToAllocate = poolBytesToAllocateFor(blockSize);
        if bytesToAllocate == 0 as libc::c_int as libc::c_ulong {
            return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
        }
        temp = (*(*pool).mem)
            .realloc_fcn
            .expect("non-null function pointer")(
            (*pool).blocks as *mut libc::c_void,
            bytesToAllocate as libc::c_uint as crate::stddef_h::size_t,
        ) as *mut BLOCK;
        if temp.is_null() {
            return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
        }
        (*pool).blocks = temp;
        (*(*pool).blocks).size = blockSize;
        (*pool).ptr = (*(*pool).blocks)
            .s
            .as_mut_ptr()
            .offset(offsetInsideBlock as isize);
        (*pool).start = (*(*pool).blocks).s.as_mut_ptr();
        (*pool).end = (*pool).start.offset(blockSize as isize)
    } else {
        let mut tem_0: *mut BLOCK = 0 as *mut BLOCK;
        let mut blockSize_0: libc::c_int =
            (*pool).end.wrapping_offset_from((*pool).start) as libc::c_long as libc::c_int;
        let mut bytesToAllocate_0: crate::stddef_h::size_t = 0;
        if blockSize_0 < 0 as libc::c_int {
            /* This condition traps a situation where either more than
             * INT_MAX bytes have already been allocated (which is prevented
             * by various pieces of program logic, not least this one, never
             * mind the unlikelihood of actually having that much memory) or
             * the pool control fields have been corrupted (which could
             * conceivably happen in an extremely buggy user handler
             * function).  Either way it isn't readily testable, so we
             * exclude it from the coverage statistics.
             */
            return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
            /* LCOV_EXCL_LINE */
        }
        if blockSize_0 < INIT_BLOCK_SIZE {
            blockSize_0 = INIT_BLOCK_SIZE
        } else {
            /* Detect overflow, avoiding _signed_ overflow undefined behavior */
            if ((blockSize_0 as libc::c_uint).wrapping_mul(2 as libc::c_uint) as libc::c_int)
                < 0 as libc::c_int
            {
                return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
            } /* save one level of indirection */
            blockSize_0 *= 2 as libc::c_int
        } /* save one level of indirection */
        bytesToAllocate_0 = poolBytesToAllocateFor(blockSize_0); /* save one level of indirection */
        if bytesToAllocate_0 == 0 as libc::c_int as libc::c_ulong {
            return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
        } /* save one level of indirection */
        tem_0 = (*(*pool).mem)
            .malloc_fcn
            .expect("non-null function pointer")(bytesToAllocate_0) as *mut BLOCK;
        if tem_0.is_null() {
            return crate::expat_h::XML_FALSE as crate::expat_h::XML_Bool;
        }
        (*tem_0).size = blockSize_0;
        (*tem_0).next = (*pool).blocks;
        (*pool).blocks = tem_0;
        if (*pool).ptr != (*pool).start {
            crate::stdlib::memcpy(
                (*tem_0).s.as_mut_ptr() as *mut libc::c_void,
                (*pool).start as *const libc::c_void,
                ((*pool).ptr.wrapping_offset_from((*pool).start) as libc::c_long as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong
                    ),
            );
        }
        (*pool).ptr = (*tem_0)
            .s
            .as_mut_ptr()
            .offset((*pool).ptr.wrapping_offset_from((*pool).start) as libc::c_long as isize);
        (*pool).start = (*tem_0).s.as_mut_ptr();
        (*pool).end = (*tem_0).s.as_mut_ptr().offset(blockSize_0 as isize)
    }
    return crate::expat_h::XML_TRUE as crate::expat_h::XML_Bool;
}

unsafe extern "C" fn nextScaffoldPart(mut parser: crate::expat_h::XML_Parser) -> libc::c_int {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut me: *mut CONTENT_SCAFFOLD = 0 as *mut CONTENT_SCAFFOLD;
    let mut next: libc::c_int = 0;
    if (*dtd).scaffIndex.is_null() {
        (*dtd).scaffIndex = (*parser)
            .m_mem
            .malloc_fcn
            .expect("non-null function pointer")(
            ((*parser).m_groupSize as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if (*dtd).scaffIndex.is_null() {
            return -(1 as libc::c_int);
        }
        *(*dtd).scaffIndex.offset(0 as libc::c_int as isize) = 0 as libc::c_int
    }
    if (*dtd).scaffCount >= (*dtd).scaffSize {
        let mut temp: *mut CONTENT_SCAFFOLD = 0 as *mut CONTENT_SCAFFOLD;
        if !(*dtd).scaffold.is_null() {
            temp = (*parser)
                .m_mem
                .realloc_fcn
                .expect("non-null function pointer")(
                (*dtd).scaffold as *mut libc::c_void,
                ((*dtd)
                    .scaffSize
                    .wrapping_mul(2 as libc::c_int as libc::c_uint)
                    as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<CONTENT_SCAFFOLD>() as libc::c_ulong),
            ) as *mut CONTENT_SCAFFOLD;
            if temp.is_null() {
                return -(1 as libc::c_int);
            }
            (*dtd).scaffSize = (*dtd)
                .scaffSize
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
        } else {
            temp = (*parser)
                .m_mem
                .malloc_fcn
                .expect("non-null function pointer")(
                (32 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<CONTENT_SCAFFOLD>() as libc::c_ulong),
            ) as *mut CONTENT_SCAFFOLD;
            if temp.is_null() {
                return -(1 as libc::c_int);
            }
            (*dtd).scaffSize = INIT_SCAFFOLD_ELEMENTS as libc::c_uint
        }
        (*dtd).scaffold = temp
    }
    let fresh82 = (*dtd).scaffCount;
    (*dtd).scaffCount = (*dtd).scaffCount.wrapping_add(1);
    next = fresh82 as libc::c_int;
    me = &mut *(*dtd).scaffold.offset(next as isize) as *mut CONTENT_SCAFFOLD;
    if (*dtd).scaffLevel != 0 {
        let mut parent: *mut CONTENT_SCAFFOLD = &mut *(*dtd).scaffold.offset(
            *(*dtd)
                .scaffIndex
                .offset(((*dtd).scaffLevel - 1 as libc::c_int) as isize) as isize,
        ) as *mut CONTENT_SCAFFOLD;
        if (*parent).lastchild != 0 {
            (*(*dtd).scaffold.offset((*parent).lastchild as isize)).nextsib = next
        }
        if (*parent).childcnt == 0 {
            (*parent).firstchild = next
        }
        (*parent).lastchild = next;
        (*parent).childcnt += 1
    }
    (*me).nextsib = 0 as libc::c_int;
    (*me).childcnt = (*me).nextsib;
    (*me).lastchild = (*me).childcnt;
    (*me).firstchild = (*me).lastchild;
    return next;
}

unsafe extern "C" fn build_node(
    mut parser: crate::expat_h::XML_Parser,
    mut src_node: libc::c_int,
    mut dest: *mut crate::expat_h::XML_Content,
    mut contpos: *mut *mut crate::expat_h::XML_Content,
    mut strpos: *mut *mut crate::expat_external_h::XML_Char,
) {
    let dtd: *mut DTD = (*parser).m_dtd;
    (*dest).type_0 = (*(*dtd).scaffold.offset(src_node as isize)).type_0;
    (*dest).quant = (*(*dtd).scaffold.offset(src_node as isize)).quant;
    if (*dest).type_0 as libc::c_uint
        == crate::expat_h::XML_CTYPE_NAME as libc::c_int as libc::c_uint
    {
        let mut src: *const crate::expat_external_h::XML_Char =
            0 as *const crate::expat_external_h::XML_Char;
        (*dest).name = *strpos;
        src = (*(*dtd).scaffold.offset(src_node as isize)).name;
        loop {
            let fresh83 = *strpos;
            *strpos = (*strpos).offset(1);
            *fresh83 = *src;
            if *src == 0 {
                break;
            }
            src = src.offset(1)
        }
        (*dest).numchildren = 0 as libc::c_int as libc::c_uint;
        (*dest).children = crate::stddef_h::NULL as *mut crate::expat_h::XML_Content
    } else {
        let mut i: libc::c_uint = 0;
        let mut cn: libc::c_int = 0;
        (*dest).numchildren = (*(*dtd).scaffold.offset(src_node as isize)).childcnt as libc::c_uint;
        (*dest).children = *contpos;
        *contpos = (*contpos).offset((*dest).numchildren as isize);
        i = 0 as libc::c_int as libc::c_uint;
        cn = (*(*dtd).scaffold.offset(src_node as isize)).firstchild;
        while i < (*dest).numchildren {
            build_node(
                parser,
                cn,
                &mut *(*dest).children.offset(i as isize),
                contpos,
                strpos,
            );
            i = i.wrapping_add(1);
            cn = (*(*dtd).scaffold.offset(cn as isize)).nextsib
        }
        (*dest).name = crate::stddef_h::NULL as *mut crate::expat_external_h::XML_Char
    };
}

unsafe extern "C" fn build_model(
    mut parser: crate::expat_h::XML_Parser,
) -> *mut crate::expat_h::XML_Content {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut ret: *mut crate::expat_h::XML_Content = 0 as *mut crate::expat_h::XML_Content;
    let mut cpos: *mut crate::expat_h::XML_Content = 0 as *mut crate::expat_h::XML_Content;
    let mut str: *mut crate::expat_external_h::XML_Char =
        0 as *mut crate::expat_external_h::XML_Char;
    let mut allocsize: libc::c_int = ((*dtd).scaffCount as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<crate::expat_h::XML_Content>() as libc::c_ulong)
        .wrapping_add(((*dtd).contentStringLen as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong,
        )) as libc::c_int;
    ret = (*parser)
        .m_mem
        .malloc_fcn
        .expect("non-null function pointer")(allocsize as crate::stddef_h::size_t)
        as *mut crate::expat_h::XML_Content;
    if ret.is_null() {
        return crate::stddef_h::NULL as *mut crate::expat_h::XML_Content;
    }
    str = &mut *ret.offset((*dtd).scaffCount as isize) as *mut crate::expat_h::XML_Content
        as *mut crate::expat_external_h::XML_Char;
    cpos = &mut *ret.offset(1 as libc::c_int as isize) as *mut crate::expat_h::XML_Content;
    build_node(parser, 0 as libc::c_int, ret, &mut cpos, &mut str);
    return ret;
}

unsafe extern "C" fn getElementType(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut ptr: *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut ELEMENT_TYPE {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut name: *const crate::expat_external_h::XML_Char =
        poolStoreString(&mut (*dtd).pool, enc, ptr, end);
    let mut ret: *mut ELEMENT_TYPE = 0 as *mut ELEMENT_TYPE;
    if name.is_null() {
        return crate::stddef_h::NULL as *mut ELEMENT_TYPE;
    }
    ret = lookup(
        parser,
        &mut (*dtd).elementTypes,
        name,
        ::std::mem::size_of::<ELEMENT_TYPE>() as libc::c_ulong,
    ) as *mut ELEMENT_TYPE;
    if ret.is_null() {
        return crate::stddef_h::NULL as *mut ELEMENT_TYPE;
    }
    if (*ret).name != name {
        (*dtd).pool.ptr = (*dtd).pool.start
    } else {
        (*dtd).pool.start = (*dtd).pool.ptr;
        if setElementTypePrefix(parser, ret) == 0 {
            return crate::stddef_h::NULL as *mut ELEMENT_TYPE;
        }
    }
    return ret;
}

unsafe extern "C" fn copyString(
    mut s: *const crate::expat_external_h::XML_Char,
    mut memsuite: *const crate::expat_h::XML_Memory_Handling_Suite,
) -> *mut crate::expat_external_h::XML_Char {
    let mut charsRequired: libc::c_int = 0 as libc::c_int;
    let mut result: *mut crate::expat_external_h::XML_Char =
        0 as *mut crate::expat_external_h::XML_Char;
    /* First determine how long the string is */
    while *s.offset(charsRequired as isize) as libc::c_int != 0 as libc::c_int {
        charsRequired += 1
    }
    /* Include the terminator */
    charsRequired += 1;
    /* Now allocate space for the copy */
    result = (*memsuite).malloc_fcn.expect("non-null function pointer")(
        (charsRequired as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::expat_external_h::XML_Char,
        >() as libc::c_ulong),
    ) as *mut crate::expat_external_h::XML_Char;
    if result.is_null() {
        return crate::stddef_h::NULL as *mut crate::expat_external_h::XML_Char;
    }
    /* Copy the original into place */
    crate::stdlib::memcpy(
        result as *mut libc::c_void,
        s as *const libc::c_void,
        (charsRequired as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::expat_external_h::XML_Char,
        >() as libc::c_ulong),
    );
    return result;
}
unsafe extern "C" fn run_static_initializers() {
    xmlLen = (::std::mem::size_of::<[crate::expat_external_h::XML_Char; 37]>() as libc::c_ulong
        as libc::c_int as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    xmlnsLen = (::std::mem::size_of::<[crate::expat_external_h::XML_Char; 30]>() as libc::c_ulong
        as libc::c_int as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
