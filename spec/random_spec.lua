local assert = require("luassert")

describe("p4lua_ext.random SMGen", function()

    local random = require("p4lua_ext").random

    local mk_vectors = {
        { 0, 0, 16294208416658607535.0 },
        { -1, 0, 16294208416658607535 },
        { 1, 12994781566227106604.0, 10451216379200822465.0 },
        { 123, 9208534749291869864.0, 13032462758197477675.0 },
        { 456, 18374074050542612762.0, 4319379447247319917.0 },
        { 789, 6731787026808254075.0, 12641749288371070621.0 },
        { 101112, 16849254472015705062.0, 4102504896626294589.0 },
        { 999999999999999999, 7386266618110926366.0, 11134796211643504527.0 },
        { 1000000000000000000, 7386266618110926366.0, 11134796211643504527.0 },
        { 10000000000000000000, 17066386762545895036.0, 9750652057807481691.0 },
    }

    it("mkSMGen produces expected initial seed and gamma", function()
        for _, v in ipairs(mk_vectors) do
            local input_seed, expected_seed, expected_gamma = table.unpack(v)
            local seed, gamma = random.mkSMGen(input_seed)

            assert.equals(expected_seed, seed)
            assert.equals(expected_gamma, gamma)
        end
    end)

    -- nextF64 test vectors
    local next_vectors = {
        { "_", 12994781566227106604.0,  10451216379200822465.0 },
        { 5377770843362382902.0, 4999253871718377473.0, 10451216379200823297.0 },
        { 6692969586998531959.0, 15450470250919200769.0, 10451216379200823297.0 },
        { 3058447273524533477.0, 7454942556410472449.0, 10451216379200823297.0 },
        { 16131882002684958427.0, 17906158935611295745.0, 10451216379200823297.0 },
        { 9288969435623161458.0, 9910631241102567425.0, 10451216379200823297.0 },
        { 13373700455485258283.0, 1915103546593839105.0, 10451216379200823297.0 },
        { 11829664833672711699.0, 12366319925794662401.0, 10451216379200823297.0 },
        { 1572841555155296541.0, 4370792231285934081.0, 10451216379200823297.0 },
        { 4902354981831719285.0, 14822008610486757377.0, 10451216379200823297.0 },
    }

    it("nextF64 produces expected values and maintains odd gamma", function()
        local seed, gamma = next_vectors[1][2], next_vectors[1][3]

        for i = 2, #next_vectors do
            local expected_val, expected_seed, expected_gamma = table.unpack(next_vectors[i])
            local val, new_seed, new_gamma = random.nextF64(seed, gamma)

            assert.equals(expected_val, val)
            assert.equals(expected_seed, new_seed)
            assert.equals(expected_gamma, new_gamma)

            seed, gamma = new_seed, new_gamma
        end
    end)

    it("calling nextF64 multiple times produces deterministic sequence", function()
        local seed1, gamma1 = random.mkSMGen(123456)
        local seed2, gamma2 = random.mkSMGen(123456)

        local vals1 = {}
        local vals2 = {}

        for i = 1, 5 do
            local val1, new_seed1, new_gamma1 = random.nextF64(seed1, gamma1)
            local val2, new_seed2, new_gamma2 = random.nextF64(seed2, gamma2)

            table.insert(vals1, val1)
            table.insert(vals2, val2)

            seed1, gamma1 = new_seed1, new_gamma1
            seed2, gamma2 = new_seed2, new_gamma2
        end

        for i = 1, #vals1 do
            assert.equals(vals1[i], vals2[i])
        end
    end)

end)
