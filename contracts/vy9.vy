# @version 0.3.10

# test for hashes
# https://docs.vyperlang.org/en/stable/built-in-functions.html?highlight=send#built-in-functions

@external
@pure
def cak1(_value: Bytes[100]) -> bytes32:
    return keccak256(_value)

@external
@pure
def cak2(_value: String[100]) -> bytes32:
    return keccak256(_value)

@external
@pure
def cak3(_value: bytes32) -> bytes32:
    return keccak256(_value)

@external
@view
def cha1(_value: Bytes[100]) -> bytes32:
    return sha256(_value)

@external
@view
def cha2(_value: String[100]) -> bytes32:
    return sha256(_value)

@external
@view
def cha3(_value: bytes32) -> bytes32:
    return sha256(_value)
