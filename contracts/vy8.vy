# @version 0.3.10

# test for data manip

@external
@pure
def cat1(a: bytes7, b: Bytes[100], c: bytes32) -> Bytes[139]:
    return concat(a, b, c)

@external
@pure
def cat2(a: bytes32, b: bytes32, c: bytes32) -> Bytes[96]:
    return concat(a, b, c)

@external
@pure
def cat3(a: Bytes[100], b: Bytes[100], c: Bytes[100]) -> Bytes[300]:
    return concat(a, b, c)

@external
@pure
def conv1(_value: bytes32) -> uint256:
    return convert(_value, uint256)

@external
@pure
def conv2(_value: Bytes[100]) -> String[100]:
    return convert(_value, String[100])

@external
@pure
def conv3(_value: bytes32) -> bool:
    return convert(_value, bool)

@external
@pure
def cut1(_value: bytes32) -> Bytes[10]:
    return slice(_value, 0, 10)

@external
@pure
def cut2(_value: bytes32) -> Bytes[10]:
    return slice(_value, 20, 10)

@external
@pure
def cut3(_value: Bytes[150]) -> Bytes[42]:
    return slice(_value, 69, 42)
