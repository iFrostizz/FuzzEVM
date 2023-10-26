# @version 0.3.10

# test for data length

@external
@pure
def len1(a: Bytes[100]) -> uint256:
    return len(a)

@external
@pure
def len2(a: String[100]) -> uint256:
    return len(a)

@external
@pure
def len3(a: DynArray[uint256, 100]) -> uint256:
    return len(a)

@external
@pure
def len4(a: DynArray[DynArray[uint256, 100], 100]) -> uint256:
    return len(a)

@external
@pure
def len5(a: DynArray[DynArray[Bytes[100], 100], 100]) -> uint256:
    return len(a)
