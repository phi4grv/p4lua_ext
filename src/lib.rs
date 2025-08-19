// Ported from Haskell SplitMix 0.1.3.1 source
// Reference: https://hackage.haskell.org/package/splitmix-0.1.3.1
//
// -- Note: in JDK implementations the mix64 and mix64variant13
// -- (which is inlined into mixGamma) are swapped.
// mix64 :: Word64 -> Word64
// mix64 z0 =
//    -- MurmurHash3Mixer
//     let z1 = shiftXorMultiply 33 0xff51afd7ed558ccd z0
//         z2 = shiftXorMultiply 33 0xc4ceb9fe1a85ec53 z1
//         z3 = shiftXor 33 z2
//     in z3
//
// -- used only in mixGamma
// mix64variant13 :: Word64 -> Word64
// mix64variant13 z0 =
//    -- Better Bit Mixing - Improving on MurmurHash3's 64-bit Finalizer
//    -- http://zimbry.blogspot.fi/2011/09/better-bit-mixing-improving-on.html
//    --
//    -- Stafford's Mix13
//     let z1 = shiftXorMultiply 30 0xbf58476d1ce4e5b9 z0 -- MurmurHash3 mix constants
//         z2 = shiftXorMultiply 27 0x94d049bb133111eb z1
//         z3 = shiftXor 31 z2
//     in z3
//
// mixGamma :: Word64 -> Word64
// mixGamma z0 =
//     let z1 = mix64variant13 z0 .|. 1             -- force to be odd
//         n  = popCount (z1 `xor` (z1 `shiftR` 1))
//     -- see: http://www.pcg-random.org/posts/bugs-in-splitmix.html
//     -- let's trust the text of the paper, not the code.
//     in if n >= 24
//         then z1
//         else z1 `xor` 0xaaaaaaaaaaaaaaaa
//
// shiftXor :: Int -> Word64 -> Word64
// shiftXor n w = w `xor` (w `shiftR` n)
//
// shiftXorMultiply :: Int -> Word64 -> Word64 -> Word64
// shiftXorMultiply n k w = shiftXor n w `mult` k
//
// nextWord64 :: SMGen -> (Word64, SMGen)
// nextWord64 (SMGen seed gamma) = (mix64 seed', SMGen seed' gamma)
//   where
//     seed' = seed `plus` gamma
//
// mkSMGen :: Word64 -> SMGen
// mkSMGen s = SMGen (mix64 s) (mixGamma (s `plus` goldenGamma))

use mlua::prelude::*;

const GOLDEN_GAMMA: u64 = 0x9E3779B97F4A7C15;

#[inline]
fn shift_xor(n: u32, w: u64) -> u64 {
    w ^ (w >> n)
}

#[inline]
fn shift_xor_multiply(n: u32, k: u64, w: u64) -> u64 {
    shift_xor(n, w).wrapping_mul(k)
}

#[inline]
fn mix64(z0: u64) -> u64 {
    let z1 = shift_xor_multiply(33, 0xff51afd7ed558ccd, z0);
    let z2 = shift_xor_multiply(33, 0xc4ceb9fe1a85ec53, z1);
    shift_xor(33, z2)
}

#[inline]
fn mix64variant13(z0: u64) -> u64 {
    let z1 = shift_xor_multiply(30, 0xbf58476d1ce4e5b9, z0);
    let z2 = shift_xor_multiply(27, 0x94d049bb133111eb, z1);
    shift_xor(31, z2)
}

#[inline]
fn mix_gamma(z0: u64) -> u64 {
    let z1 = mix64variant13(z0) | 1; // force odd
    let n = (z1 ^ (z1 >> 1)).count_ones();
    if n < 24 {
        z1 ^ 0xaaaaaaaaaaaaaaaa
    } else {
        z1
    }
}

fn mk_smgen(_lua: &Lua, s: f64) -> LuaResult<(f64, f64)> {
    let seed = mix64(s as u64);
    let gamma = mix_gamma((s as u64).wrapping_add(GOLDEN_GAMMA));

    Ok((seed as f64, gamma as f64))
}

fn next_f64(_lua: &Lua, (seed, gamma): (f64, f64)) -> LuaResult<(f64, f64, f64)> {
    let gamma_prime = (gamma as u64) | 1;
    let seed_prime = (seed as u64).wrapping_add(gamma_prime as u64);

    let value = mix64(seed_prime);

    Ok((value as f64, seed_prime as f64, gamma_prime as f64))
}

/// Lua module
fn smgen_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("mkSMGen", lua.create_function(mk_smgen)?)?;
    exports.set("nextF64", lua.create_function(next_f64)?)?;
    Ok(exports)
}

#[mlua::lua_module]
fn p4lua_ext(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("random", smgen_module(lua)?)?;
    Ok(exports)
}
