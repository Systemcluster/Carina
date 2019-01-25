# Carina Type Examples

```carina

var : Number = 10
ref := &foo
con : Ordered : 20

StructWithParam :: st
    InnerType: Type
    SomeValue: Number


agg := StructWithParam
    InnerType: Number
    SomeValue: 34


Range :: tr
    inclusive :: fn (from to)


RangeAble :: tr
    inclusive : fn from to: &Self -> Range
    exclusive : fn from to: &Self -> Range

RangeAble :+ i64
    inclusive : fn from to =>
    exclusive : fn from to =>


# range with extra parameters and restrictions
Regular :: tr
    InnerType  : Number' + Ordered'
    innerValue : Number + Ordered
    action : fn &self b: InnerType -> InnerType




# mutable reassignment

(a b) =: (b a)

```
