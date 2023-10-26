# @version 0.3.10

# test for misc

@external
@pure
def to_str(val: uint256) -> String[100]:
    return uint2str(val)
