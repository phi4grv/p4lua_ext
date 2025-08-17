use mlua::prelude::*;
use mlua::AnyUserData;

// Haskell SplitMix constants
const GOLDEN_GAMMA: u64 = 0x9E3779B97F4A7C15;

#[derive(Clone, Copy)]
struct SMGen {
    seed: u64,
    gamma: u64,
}

// shiftXor function from Haskell
#[inline]
fn shift_xor(x: u64, shift: u32) -> u64 {
    x ^ (x >> shift)
}

// shiftXorMultiply function from Haskell
#[inline]
fn shift_xor_multiply(x: u64, shift: u32, mul: u64) -> u64 {
    shift_xor(x, shift).wrapping_mul(mul) // wrapping_mul ensures 64-bit overflow behaves like Haskell
}

// mix64 function translated from Haskell
#[inline]
fn mix64(z: u64) -> u64 {
    let z1 = shift_xor_multiply(z, 33, 0xff51afd7ed558ccd);
    let z2 = shift_xor_multiply(z1, 33, 0xc4ceb9fe1a85ec53);

    shift_xor(z2, 33)
}

// mix64variant13 used in mixGamma, translated from Haskell
#[inline]
fn mix64variant13(z: u64) -> u64 {
    let z1 = shift_xor_multiply(z, 30, 0xbf58476d1ce4e5b9);
    let z2 = shift_xor_multiply(z1, 27, 0x94d049bb133111eb);

    shift_xor(z2, 31)
}

// mixGamma function translated from Haskell
#[inline]
fn mix_gamma(z: u64) -> u64 {
    let z1 = mix64variant13(z) | 1; // force to be odd
    let n = (z1 ^ (z1 >> 1)).count_ones();

    if n >= 24 {
        z1
    } else {
        z1 ^ 0xaaaaaaaaaaaaaaaa
    }
}

fn mk_smgen(lua: &Lua, s: u64) -> LuaResult<AnyUserData> {
    let smgen = SMGen {
        seed: mix64(s),
        gamma: mix_gamma(s.wrapping_add(GOLDEN_GAMMA)),
    };

    lua.create_any_userdata(smgen)
}

fn next_u64(lua: &Lua, lua_smgen: AnyUserData) -> LuaResult<(u64, AnyUserData)> {
    let smgen = lua_smgen.borrow::<SMGen>()?;
    let new_seed = smgen.seed.wrapping_add(smgen.gamma);
    let val = mix64(new_seed);
    let new_smgen = SMGen {
        seed: new_seed,
        gamma: smgen.gamma,
    };

    Ok((val, lua.create_any_userdata(new_smgen)?))
}

fn seed_smgen(lua: &Lua, (seed, gamma): (u64, u64)) -> LuaResult<AnyUserData> {
    // gamma is forced to be odd
    let smgen = SMGen {
        seed,
        gamma: gamma | 1,
    };
    lua.create_any_userdata(smgen)
}

fn unseed_smgen(_lua: &Lua, lua_smgen: AnyUserData) -> LuaResult<(u64, u64)> {
    let smgen = lua_smgen.borrow::<SMGen>()?;
    Ok((smgen.seed, smgen.gamma))
}

// random module table for Lua
fn smgen_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("mkSMGen", lua.create_function(mk_smgen)?)?;
    exports.set("nextU64", lua.create_function(next_u64)?)?;
    exports.set("seedSMGen", lua.create_function(seed_smgen)?)?;
    exports.set("unseedSMGen", lua.create_function(unseed_smgen)?)?;

    Ok(exports)
}

// Lua module export
#[mlua::lua_module()]
fn p4lua_ext(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("random", smgen_module(lua)?)?;
    Ok(exports)
}
