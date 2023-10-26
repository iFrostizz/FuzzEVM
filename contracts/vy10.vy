# @version 0.3.10

# tests for abi encode/decode

@external
@pure
def encode1(a: Bytes[100], b: uint256) -> Bytes[228]:
    return _abi_encode(a, b, method_id=method_id("poke(bytes,uint256)"))

@external
@pure
def encode2(a: DynArray[DynArray[Bytes[100], 100], 100], b: DynArray[String[100], 100]) -> Bytes[1945732]:
    return _abi_encode(b, a, method_id=method_id("poke(string[],bytes[][])"))

@external
@pure
def encode3(a: String[100], b: DynArray[String[100], 100]) -> Bytes[1945732]:
    return _abi_encode(a, b)

@external
@pure
def decode1(a: Bytes[200]) -> uint256:
    if len(a) != 32:
        return 0
    return _abi_decode(a, (uint256))

@external
@pure
def decode2(a: Bytes[200]) -> (uint256, uint256):
    return _abi_decode(a, (uint256, uint256))

@external
@pure
def decode3(a: Bytes[200]) -> (uint8, uint64, bytes32):
    return _abi_decode(a, (uint8, uint64, bytes32))
