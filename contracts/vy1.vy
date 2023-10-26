# @version 0.3.10
# @notice min/max maths operations

# runtime
@external
@pure
def max1(a: uint256, b: uint256) -> uint256:
    return max(a, b)

@external
@pure
def max2(a: int256, b: int256) -> int256:
    return max(a, b)

@external
@pure
def min1(a: uint256, b: uint256) -> uint256:
    return min(a, b)

@external
@pure
def min2(a: int256, b: int256) -> int256:
    return min(a, b)

# compile
@external
@pure
def comp_max1() -> uint256:
    return max(0, 1)

@external
@pure
def comp_max2() -> int256:
    return max(123456, 1984791749812)

@external
@pure
def comp_max3() -> int256:
    return max(-1, 1984791749812)

@external
@pure
def comp_max4() -> int256:
    return max(123456, 123456)

@external
@pure
def comp_max5() -> int256:
    return max(-123456, -123456)

@external
@pure
def comp_max6() -> int256:
    return max(-123457, -123456)

@external
@pure
def comp_min1() -> uint256:
    return min(0, 1)

@external
@pure
def comp_min2() -> int256:
    return min(123456, 1984791749812)

@external
@pure
def comp_min3() -> int256:
    return min(-1, 1984791749812)

@external
@pure
def comp_min4() -> int256:
    return min(123456, 123456)

@external
@pure
def comp_min5() -> int256:
    return min(-123456, -123456)

@external
@pure
def comp_min6() -> int256:
    return min(-123457, -123456)

