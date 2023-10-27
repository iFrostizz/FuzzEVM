# @version 0.3.10

# tests for signatures

@external
@pure
def sig1(hash: bytes32, v: uint8, r: bytes32, s: bytes32) -> address:
    return ecrecover(hash, v, r, s)

@external
@view
def sig2(hash: bytes32, v: uint8, r: bytes32, s: bytes32) -> address:
    return ecrecover(hash, v, r, s)
