local assert = require("luassert")

describe("p4lua_ext.random SMGen", function()

    local random = require("p4lua_ext").random

    it("mkSMGen 1 produces seed and gamma matching Haskell SplitMix", function()
        local sm = random.mkSMGen(1)
        local seed, gamma = random.unseedSMGen(sm)

        -- Haskell-verified expected values
        assert.equals(seed, 12994781566227106604)
        assert.equals(gamma, 10451216379200822465)
    end)

    it("should produce the expected next u64 value for a given seed", function()
        local sm = random.mkSMGen(1)

        local val1, sm1 = random.nextU64(sm)
        local seed, gamma = random.unseedSMGen(sm1)

        -- Haskell-verified expected values
        assert.equals(val1, 16204969531660614133)
        assert.equals(seed, 4999253871718377453)
        assert.equals(gamma, 10451216379200822465)
    end)

    it("should create SMGen with given seed and odd gamma", function()
        local seed_input = 123
        local gamma_input = 456  -- even gamma for test

        local sm = random.seedSMGen(seed_input, gamma_input)
        local seed, gamma = random.unseedSMGen(sm)

        -- seed should be same as input
        assert.equals(seed, seed_input)
        -- gamma should be odd regardless of input
        assert.equals(gamma % 2, 1)
    end)

    it("odd gamma input remains odd", function()
        local seed_input = 789
        local gamma_input = 101  -- already odd

        local sm = random.seedSMGen(seed_input, gamma_input)
        local seed, gamma = random.unseedSMGen(sm)

        assert.equals(seed, seed_input)
        assert.equals(gamma, gamma_input)  -- stays the same
    end)

    it("calling nextU64 multiple times on same seed produces same values", function()
        local sm1 = random.mkSMGen(123)
        local sm2 = random.mkSMGen(123)

        local val1_a, sm1_a = random.nextU64(sm1)
        local val1_b, sm1_b = random.nextU64(sm1_a)

        local val2_a, sm2_a = random.nextU64(sm2)
        local val2_b, sm2_b = random.nextU64(sm2_a)

        assert.equals(val1_a, val2_a)
        assert.equals(val1_b, val2_b)

        assert.not_equals(val1_a, val1_b)
        assert.not_equals(val2_a, val2_b)
    end)

end)
