// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract sol2 {
    function isq(uint256 x) external pure returns (uint256) {
        if (x == 0) return 0;
        uint256 xx = x;
        uint256 r = 1;
        if (xx >= 0x100000000000000000000000000000000) {
            xx >>= 128;
            r <<= 64;
        }
        if (xx >= 0x10000000000000000) {
            xx >>= 64;
            r <<= 32;
        }
        if (xx >= 0x100000000) {
            xx >>= 32;
            r <<= 16;
        }
        if (xx >= 0x10000) {
            xx >>= 16;
            r <<= 8;
        }
        if (xx >= 0x100) {
            xx >>= 8;
            r <<= 4;
        }
        if (xx >= 0x10) {
            xx >>= 4;
            r <<= 2;
        }
        if (xx >= 0x8) {
            r <<= 1;
        }
        r = (r + x / r) >> 1;
        r = (r + x / r) >> 1;
        r = (r + x / r) >> 1;
        r = (r + x / r) >> 1;
        r = (r + x / r) >> 1;
        r = (r + x / r) >> 1;
        r = (r + x / r) >> 1; // Seven iterations should be enough
        uint256 r1 = x / r;
        return (r < r1 ? r : r1);
    }

    function usub1(uint256 a, uint256 b) external pure returns (uint256) {
        unchecked {
            return a - b;
        }
    }

    function usub2(int256 a, int256 b) external pure returns (int256) {
        unchecked {
            return a - b;
        }
    }

    function umul1(uint256 a, uint256 b) external pure returns (uint256) {
        unchecked {
            return a * b;
        }
    }

    function umul2(int256 a, int256 b) external pure returns (int256) {
        unchecked {
            return a * b;
        }
    }

    function udiv1(uint256 a, uint256 b) external pure returns (uint256) {
        if (b == 0) return 0;
        unchecked {
            return a / b;
        }
    }

    function udiv2(int256 a, int256 b) external pure returns (int256) {
        if (b == 0) return 0;
        unchecked {
            return a / b;
        }
    }

    function add1(uint256 a, uint256 b) external pure returns (uint256) {
        unchecked {
            uint256 res = a + b;
            require(res >= a);
            return res;
        }
    }

    function add2(int256 a, int256 b) external pure returns (int256) {
        unchecked {
            int256 res = a + b;
            bool al = b < 0;
            bool rl = res < a;
            if (!((al && rl) || (!al && !rl))) {
                // <=> ^
                revert();
            }
            return res;
        }
    }

    function add3(int256 a, uint256 b) external pure returns (int256) {
        if (b >> 255 != 0) {
            revert();
        }
        int256 _b = int256(b);
        unchecked {
            int256 res = a + _b;
            if (res < a) {
                revert();
            }
            return res;
        }
    }

    function add4(uint256 a, uint256 b) external pure returns (int256) {
        if (a >> 255 != 0 || b >> 255 != 0) {
            revert();
        }
        int256 _a = int256(a);
        int256 _b = int256(b);
        unchecked {
            int256 res = _a + _b;
            bool al = _b < 0;
            bool rl = res < _a;
            if (!((al && rl) || (!al && !rl))) {
                // <=> ^
                revert();
            }
            return res;
        }
    }

    function uadd1(uint256 a, uint256 b) external pure returns (uint256) {
        unchecked {
            return a + b;
        }
    }

    function uadd2(int256 a, int256 b) external pure returns (int256) {
        unchecked {
            return a + b;
        }
    }

    // function ab(int256 _value) external pure returns (int256) {
    //     assembly {
    //         if eq(
    //             _value,
    //             0x8000000000000000000000000000000000000000000000000000000000000000
    //         ) {
    //             return(0, 32)
    //         }
    //     }

    //     return _value >= 0 ? _value : -_value;
    // }
}
