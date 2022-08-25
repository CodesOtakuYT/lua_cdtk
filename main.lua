local a = CDTK.vec_i128()
local b = CDTK.vec_i64()
local c = CDTK.vec_i32()
local d = CDTK.vec_i16()
local e = CDTK.vec_i8()
local f = CDTK.vec_f64()
local e = CDTK.vec_f32()

a:range(1, 5, 2)
a:range(5, 1, 1)
a:fill(1, 5)
a:fill(2, 3)

local examples = {#a, a, a+a, a-a, a*a, a/a, a%a, a^a, a..a, a:min(), a:max(), a:product(), a:sum(), a == a, a > a, a < a, a >= a, a <= a}
for i, v in ipairs(examples) do
    print(i, v)
end

a:negate()
a:push(10)
a:fill(a:pop(), a:pop())
a:range(a:pop(), a:pop(), a:pop())
print(a)
