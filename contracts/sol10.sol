// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.14;

contract sol10 {
    function encode1(
        bytes memory a,
        uint256 b
    ) external payable returns (bytes memory) {
        return abi.encodeWithSignature("poke(bytes,uint256)", a, b);
    }

    function encode2(
        bytes[][] memory a,
        string[] memory b
    ) external pure returns (bytes memory) {
        return abi.encodeWithSignature("poke(string[],bytes[][])", b, a);
    }

    function encode3(
        string memory a,
        string[] memory b
    ) external pure returns (bytes memory) {
        return abi.encode(a, b);
    }

    function decode1(bytes memory data) external pure returns (uint256) {
        if (data.length != 32) {
            return 0;
        }
        return abi.decode(data, (uint256));
    }

    function decode2(
        bytes memory data
    ) external pure returns (uint256, uint256) {
        return abi.decode(data, (uint256, uint256));
    }

    function decode3(
        bytes memory data
    ) external pure returns (uint8, uint64, bytes32) {
        return abi.decode(data, (uint8, uint64, bytes32));
    }
}
