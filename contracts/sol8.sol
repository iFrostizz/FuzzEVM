// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract sol8 {
    function cat1(
        bytes7 a,
        bytes memory b,
        bytes32 c
    ) external pure returns (bytes memory) {
        return abi.encodePacked(a, b, c);
    }

    function cat2(
        bytes32 a,
        bytes32 b,
        bytes32 c
    ) external pure returns (bytes memory) {
        return abi.encodePacked(a, b, c);
    }

    function cat3(
        bytes memory a,
        bytes memory b,
        bytes memory c
    ) external pure returns (bytes memory) {
        return abi.encodePacked(a, b, c);
    }

    function conv1(bytes32 _value) external pure returns (uint256) {
        return uint256(_value);
    }

    function conv2(bytes memory _value) external pure returns (string memory) {
        return string(_value);
    }

    function conv3(bytes32 _value) external pure returns (bool) {
        return _value != bytes32(0);
    }

    function cut1(bytes32 _value) external pure returns (bytes memory) {
        bytes32 shifted = (_value >> 176) << 176;
        assembly {
            let free := mload(0x40)
            mstore(free, 0x20)
            mstore(add(free, 0x20), 0x0a)
            mstore(add(free, 0x40), shifted)
            mstore(0x40, add(free, 0x60))
            return(free, 0x60)
        }
    }

    function cut2(bytes32 _value) external pure returns (bytes memory) {
        bytes32 shifted = (_value >> 16) << 176;
        assembly {
            let free := mload(0x40)
            mstore(free, 0x20)
            mstore(add(free, 0x20), 0x0a)
            mstore(add(free, 0x40), shifted)
            mstore(0x40, add(free, 0x60))
            return(free, 0x60)
        }
    }

    function cut3(bytes calldata _value) external pure returns (bytes memory) {
        require(_value.length >= 69 + 42);
        assembly {
            let free := mload(0x40)
            mstore(free, add(free, 0x20))
            mstore(add(free, 0x20), 42)
            calldatacopy(
                add(free, 0x40),
                add(calldataload(_value.offset), 0x20),
                42
            )
            mstore(add(free, 0x20), 0x40)
            return(free, add(0x40, 42))
        }
    }
}
