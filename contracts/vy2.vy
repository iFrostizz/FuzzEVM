# @version 0.3.10
# @notice arith maths operations

# runtime
@external
@pure
def isq(a: uint256) -> uint256:
    return isqrt(a)

@external
@pure
def usub1(a: uint256, b: uint256) -> uint256:
    return unsafe_sub(a, b)

@external
@pure
def usub2(a: int256, b: int256) -> int256:
    return unsafe_sub(a, b)

@external
@pure
def umul1(a: uint256, b: uint256) -> uint256:
    return unsafe_mul(a, b)

@external
@pure
def umul2(a: int256, b: int256) -> int256:
    return unsafe_mul(a, b)

@external
@pure
def udiv1(a: uint256, b: uint256) -> uint256:
    return unsafe_div(a, b)

@external
@pure
def udiv2(a: int256, b: int256) -> int256:
    return unsafe_div(a, b)

@external
@pure
def add1(a: uint256, b: uint256) -> uint256:
    return a + b

@external
@pure
def add2(a: int256, b: int256) -> int256:
    return a + b

@external
@pure
def add3(a: int256, b: uint256) -> int256:
    return a + convert(b, int256)

@external
@pure
def add4(a: uint256, b: uint256) -> int256:
    return convert(a, int256) + convert(b, int256)

@external
@pure
def uadd1(a: uint256, b: uint256) -> uint256:
    return unsafe_add(a, b)

@external
@pure
def uadd2(a: int256, b: int256) -> int256:
    return unsafe_add(a, b)

# @external
# @pure
# def ab(_value: int256) -> int256:
#     return abs(_value)
# TODO not working with -0
