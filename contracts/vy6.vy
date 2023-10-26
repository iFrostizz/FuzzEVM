# @version 0.3.10

# test for units

@external
@pure
def as_wei1(_value: uint256) -> uint256:
    return as_wei_value(_value, "ether")

@external
@pure
def as_wei2(_value: uint8) -> uint256:
    return as_wei_value(_value, "ether")

@external
@pure
def as_wei3(_value: uint256) -> uint256:
    return as_wei_value(_value, "wei")

@external
@pure
def as_wei4(_value: uint8) -> uint256:
    return as_wei_value(_value, "wei")

@external
@pure
def as_wei5(_value: uint256) -> uint256:
    return as_wei_value(_value, "femtoether")

@external
@pure
def as_wei6(_value: uint8) -> uint256:
    return as_wei_value(_value, "kwei")

@external
@pure
def as_wei7(_value: uint256) -> uint256:
    return as_wei_value(_value, "babbage")

@external
@pure
def as_wei8(_value: uint256) -> uint256:
    return as_wei_value(_value, "picoether")

@external
@pure
def as_wei9(_value: uint8) -> uint256:
    return as_wei_value(_value, "mwei")

@external
@pure
def as_wei10(_value: uint256) -> uint256:
    return as_wei_value(_value, "lovelace")

@external
@pure
def as_wei11(_value: uint8) -> uint256:
    return as_wei_value(_value, "nanoether")
