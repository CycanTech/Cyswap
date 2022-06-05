#![allow(non_snake_case)]
use core::ops::Div;

use primitives::{Address, Int24, Int256, Uint24, U160, U256};

use crate::{core::TickMath, swap::FullMath};

use super::{HexStrings, NFTSVG};
use ink_prelude::string::ToString;
use ink_prelude::string::String;
use ink_prelude::vec::Vec;
use ink_prelude::vec;

const sqrt10X128: &'static str = "1076067327063303206878105757264492625226";

#[derive(Clone)]
pub struct ConstructTokenURIParams {
    pub tokenId: U256,
    pub quoteTokenAddress: Address,
    pub baseTokenAddress: Address,
    pub quoteTokenSymbol: String,
    pub baseTokenSymbol: String,
    pub quoteTokenDecimals: u8,
    pub baseTokenDecimals: u8,
    pub flipRatio: bool,
    pub tickLower: Int24,
    pub tickUpper: Int24,
    pub tickCurrent: Int24,
    pub tickSpacing: Int24,
    pub fee: Uint24,
    pub poolAddress: Address,
}

#[derive(Default)]
struct DecimalStringParams {
    // significant figures of decimal
    pub sigfigs: U256,
    // length of decimal string
    pub bufferLength: u8,
    // ending index for significant figures (funtion works backwards when copying sigfigs)
    pub sigfigIndex: u8,
    // index of decimal place (0 if no decimal)
    pub decimalIndex: u8,
    // start index for trailing/leading 0's for very small/large numbers
    pub zerosStartIndex: u8,
    // end index for trailing/leading 0's for very small/large numbers
    pub zerosEndIndex: u8,
    // true if decimal number is less than one
    pub isLessThanOne: bool,
    // true if string should include "%"
    pub isPercent: bool,
}

pub fn constructTokenURI(params: ConstructTokenURIParams) -> String {
    let name: String = generateName(params.clone(), feeToPercentString(params.fee));
    let descriptionPartOne: String = generateDescriptionPartOne(
        escapeQuotes(params.quoteTokenSymbol.clone()),
        escapeQuotes(params.baseTokenSymbol.clone()),
        addressToString(params.poolAddress),
    );
    // string memory descriptionPartTwo =
    //     generateDescriptionPartTwo(
    //         params.tokenId.toString(),
    //         escapeQuotes(params.baseTokenSymbol),
    //         addressToString(params.quoteTokenAddress),
    //         addressToString(params.baseTokenAddress),
    //         feeToPercentString(params.fee)
    //     );
    let descriptionPartTwo: String = generateDescriptionPartTwo(
        params.tokenId.to_string(),
        escapeQuotes(params.baseTokenSymbol.clone()),
        addressToString(params.quoteTokenAddress.clone()),
        addressToString(params.baseTokenAddress),
        feeToPercentString(params.fee.clone()),
    );
    // string memory image = Base64.encode(bytes(generateSVGImage(params)));
    let image: String = base64::encode(generateSVGImage(params.clone()).as_bytes());

    // return
    //     string(
    //         abi.encodePacked(
    //             'data:application/json;base64,',
    //             Base64.encode(
    //                 bytes(
    //                     abi.encodePacked(
    //                         '{"name":"',
    //                         name,
    //                         '", "description":"',
    //                         descriptionPartOne,
    //                         descriptionPartTwo,
    //                         '", "image": "',
    //                         'data:image/svg+xml;base64,',
    //                         image,
    //                         '"}'
    //                     )
    //                 )
    //             )
    //         )
    //     );
    return String::from_utf8_lossy(&scale::Encode::encode(&(
        "data:application/json;base64,",
        base64::encode(scale::Encode::encode(&(
            r#"{"name":""#,
            name,
            r#"", "description":""#,
            descriptionPartOne,
            descriptionPartTwo,
            r#"", "image": ""#,
            r#"data:image/svg+xml;base64,"#,
            image,
            r#""}"#,
        ))),
    )))
    .to_string();
}

fn generateSVGImage(params: ConstructTokenURIParams) -> String {
    let svgParams: NFTSVG::SVGParams = NFTSVG::SVGParams {
        quoteToken: addressToString(params.quoteTokenAddress),
        baseToken: addressToString(params.baseTokenAddress),
        poolAddress: params.poolAddress,
        quoteTokenSymbol: params.quoteTokenSymbol,
        baseTokenSymbol: params.baseTokenSymbol,
        feeTier: feeToPercentString(params.fee),
        tickLower: params.tickLower,
        tickUpper: params.tickUpper,
        tickSpacing: params.tickSpacing,
        overRange: overRange(params.tickLower, params.tickUpper, params.tickCurrent),
        tokenId: params.tokenId,
        color0: tokenToColorHex(
            U256::from_little_endian(params.quoteTokenAddress.as_ref()),
            136,
        ),
        color1: tokenToColorHex(
            U256::from_little_endian(params.baseTokenAddress.as_ref()),
            136,
        ),
        color2: tokenToColorHex(
            U256::from_little_endian(params.quoteTokenAddress.as_ref()),
            0,
        ),
        color3: tokenToColorHex(
            U256::from_little_endian(params.baseTokenAddress.as_ref()),
            0,
        ),
        x1: scale(
            getCircleCoord(
                U256::from_little_endian(params.quoteTokenAddress.as_ref()),
                U256::from(16),
                params.tokenId,
            ),
            U256::from(0),
            U256::from(255),
            U256::from(16),
            U256::from(274),
        ),
        y1: scale(
            getCircleCoord(
                U256::from_little_endian(params.baseTokenAddress.as_ref()),
                U256::from(16),
                params.tokenId,
            ),
            U256::from(0),
            U256::from(255),
            U256::from(100),
            U256::from(484),
        ),
        x2: scale(
            getCircleCoord(
                U256::from_little_endian(params.quoteTokenAddress.as_ref()),
                U256::from(32),
                params.tokenId,
            ),
            U256::from(0),
            U256::from(255),
            U256::from(16),
            U256::from(274),
        ),
        y2: scale(
            getCircleCoord(
                U256::from_little_endian(params.baseTokenAddress.as_ref()),
                U256::from(32),
                params.tokenId,
            ),
            U256::from(0),
            U256::from(255),
            U256::from(100),
            U256::from(484),
        ),
        x3: scale(
            getCircleCoord(
                U256::from_little_endian(params.quoteTokenAddress.as_ref()),
                U256::from(48),
                params.tokenId,
            ),
            U256::from(0),
            U256::from(255),
            U256::from(16),
            U256::from(274),
        ),
        y3: scale(
            getCircleCoord(
                U256::from_little_endian(params.baseTokenAddress.as_ref()),
                U256::from(48),
                params.tokenId,
            ),
            U256::from(0),
            U256::from(255),
            U256::from(100),
            U256::from(484),
        ),
    };

    return NFTSVG::generateSVG(svgParams);
}

fn scale(n: U256, inMn: U256, inMx: U256, outMn: U256, outMx: U256) -> String {
    // return (n.sub(inMn).mul(outMx.sub(outMn)).div(inMx.sub(inMn)).add(outMn)).toString();
    return (n
        .saturating_sub(inMn)
        .saturating_mul(outMx.saturating_sub(outMn))
        .div(inMx.saturating_sub(inMn))
        .saturating_add(outMn))
    .to_string();
}

fn getCircleCoord(tokenAddress: U256, offset: U256, tokenId: U256) -> U256 {
    return (sliceTokenHex(tokenAddress, offset) * tokenId) % 255;
}

fn sliceTokenHex(token: U256, offset: U256) -> U256 {
    // return uint256(uint8(token >> offset));
    return token >> offset;
}

fn tokenToColorHex(token: U256, offset: u32) -> String {
    return HexStrings::toHexStringNoPrefix(token >> offset, 3);
}

fn overRange(tickLower: Int24, tickUpper: Int24, tickCurrent: Int24) -> i8 {
    if tickCurrent < tickLower {
        return -1;
    } else if tickCurrent > tickUpper {
        return 1;
    } else {
        return 0;
    }
}

fn addressToString(addr: Address) -> String {
    // return (uint256(addr)).toHexString(20);
    let addr_array: &[u8] = addr.as_ref();
    hex::encode(addr_array)
    // return &addr.toHexString(20);
}

fn generateName(params: ConstructTokenURIParams, feeTier: String) -> String {
    // return
    //     string(
    //         abi.encodePacked(
    //             'Uniswap - ',
    //             feeTier,
    //             ' - ',
    //             escapeQuotes(params.quoteTokenSymbol),
    //             '/',
    //             escapeQuotes(params.baseTokenSymbol),
    //             ' - ',
    //             tickToDecimalString(
    //                 !params.flipRatio ? params.tickLower : params.tickUpper,
    //                 params.tickSpacing,
    //                 params.baseTokenDecimals,
    //                 params.quoteTokenDecimals,
    //                 params.flipRatio
    //             ),
    //             '<>',
    //             tickToDecimalString(
    //                 !params.flipRatio ? params.tickUpper : params.tickLower,
    //                 params.tickSpacing,
    //                 params.baseTokenDecimals,
    //                 params.quoteTokenDecimals,
    //                 params.flipRatio
    //             )
    //         )
    //     );
    return String::from_utf8_lossy(&scale::Encode::encode(&(
        "Uniswap - ",
        feeTier,
        " - ",
        escapeQuotes(params.quoteTokenSymbol),
        "/",
        escapeQuotes(params.baseTokenSymbol),
        " - ",
        tickToDecimalString(
            if !params.flipRatio {
                params.tickLower
            } else {
                params.tickUpper
            },
            params.tickSpacing,
            params.baseTokenDecimals,
            params.quoteTokenDecimals,
            params.flipRatio,
        ),
        "<>",
        tickToDecimalString(
            if !params.flipRatio {
                params.tickUpper
            } else {
                params.tickLower
            },
            params.tickSpacing,
            params.baseTokenDecimals,
            params.quoteTokenDecimals,
            params.flipRatio,
        ),
    )))
    .to_string();
}

fn escapeQuotes(symbol: String) -> String {
    // bytes memory symbolBytes = bytes(symbol);
    let symbolBytes = symbol.bytes();
    // uint8 quotesCount = 0;
    let mut quotesCount: u8 = 0;
    // for (uint8 i = 0; i < symbolBytes.length; i++) {
    //     if (symbolBytes[i] == '"') {
    //         quotesCount++;
    //     }
    // }
    for symbol_byte in symbolBytes.clone() {
        if symbol_byte == b'"' {
            quotesCount += 1;
        }
    }
    // if (quotesCount > 0) {
    //     bytes memory escapedBytes = new bytes(symbolBytes.length + (quotesCount));
    //     uint256 index;
    //     for (uint8 i = 0; i < symbolBytes.length; i++) {
    //         if (symbolBytes[i] == '"') {
    //             escapedBytes[index++] = '\\';
    //         }
    //         escapedBytes[index++] = symbolBytes[i];
    //     }
    //     return string(escapedBytes);
    // }
    // return symbol;
    if quotesCount > 0 {
        let mut escapedBytes: Vec<u8> =vec!(0;symbolBytes.len() + usize::from(quotesCount));
        let mut index: U256 = U256::zero();
        for symbol_byte in symbolBytes {
            if symbol_byte == b'"' {
                index += U256::one();
                escapedBytes[index.as_usize()] = b'\\';
            }
            index += U256::one();
            escapedBytes[index.as_usize()] = symbol_byte;
        }
        return String::from_utf8_lossy(&escapedBytes).to_string();
    }
    // return symbol;
    return symbol;
}

fn tickToDecimalString(
    tick: Int24,
    tickSpacing: Int24,
    baseTokenDecimals: u8,
    quoteTokenDecimals: u8,
    flipRatio: bool,
) -> String {
    if tick == (TickMath::MIN_TICK / tickSpacing) * tickSpacing {
        return if !flipRatio {
            String::from("MIN")
        } else {
            String::from("MAX")
        };
    } else if tick == (TickMath::MAX_TICK / tickSpacing) * tickSpacing {
        return if !flipRatio {
            String::from("MAX")
        } else {
            String::from("MIN")
        };
    } else {
        let mut sqrtRatioX96: U160 = TickMath::getSqrtRatioAtTick(tick);
        if flipRatio {
            sqrtRatioX96 = (U256::from(1) << 192) / (sqrtRatioX96);
        }
        return fixedPointToDecimalString(sqrtRatioX96, baseTokenDecimals, quoteTokenDecimals);
    }
}

// @notice Returns string that includes first 5 significant figures of a decimal number
// @param sqrtRatioX96 a sqrt price
fn fixedPointToDecimalString(
    sqrtRatioX96: U160,
    baseTokenDecimals: u8,
    quoteTokenDecimals: u8,
) -> String {
    // uint256 adjustedSqrtRatioX96 = adjustForDecimalPrecision(sqrtRatioX96, baseTokenDecimals, quoteTokenDecimals);
    // uint256 value = FullMath.mulDiv(adjustedSqrtRatioX96, adjustedSqrtRatioX96, 1 << 64);
    let adjustedSqrtRatioX96: U256 =
        adjustForDecimalPrecision(sqrtRatioX96, baseTokenDecimals, quoteTokenDecimals);
    let mut value: U256 = FullMath::mulDiv(
        adjustedSqrtRatioX96,
        adjustedSqrtRatioX96,
        U256::one() << 64,
    );
    // bool priceBelow1 = adjustedSqrtRatioX96 < 2**96;
    let priceBelow1: bool = adjustedSqrtRatioX96 < (U256::one() << 96);
    // if (priceBelow1) {
    //     // 10 ** 43 is precision needed to retreive 5 sigfigs of smallest possible price + 1 for rounding
    //     value = FullMath.mulDiv(value, 10**44, 1 << 128);
    // } else {
    //     // leave precision for 4 decimal places + 1 place for rounding
    //     value = FullMath.mulDiv(value, 10**5, 1 << 128);
    // }
    if priceBelow1 {
        // 10 ** 43 is precision needed to retreive 5 sigfigs of smallest possible price + 1 for rounding
        value = FullMath::mulDiv(
            value,
            U256::from(10).pow(U256::from(44)),
            U256::one() << 128,
        );
    } else {
        // leave precision for 4 decimal places + 1 place for rounding
        value = FullMath::mulDiv(value, U256::from(10).pow(U256::from(5)), U256::one() << 128);
    }

    // // get digit count
    // uint256 temp = value;
    // uint8 digits;
    let mut temp: U256 = value;
    let mut digits: u8 = 0;
    // while (temp != 0) {
    //     digits++;
    //     temp /= 10;
    // }
    while !temp.is_zero() {
        digits += 1;
        temp /= 10;
    }
    // // don't count extra digit kept for rounding
    // digits = digits - 1;
    digits = digits - 1;

    // // address rounding
    // (uint256 sigfigs, bool extraDigit) = sigfigsRounded(value, digits);
    // if (extraDigit) {
    //     digits++;
    // }
    let (sigfigs, extraDigit) = sigfigsRounded(value, digits);
    if extraDigit {
        digits += 1;
    }

    // DecimalStringParams memory params;
    // if (priceBelow1) {
    //     // 7 bytes ( "0." and 5 sigfigs) + leading 0's bytes
    //     params.bufferLength = uint8(uint8(7).add(uint8(43).sub(digits)));
    //     params.zerosStartIndex = 2;
    //     params.zerosEndIndex = uint8(uint256(43).sub(digits).add(1));
    //     params.sigfigIndex = uint8(params.bufferLength.sub(1));
    // } else if (digits >= 9) {
    //     // no decimal in price string
    //     params.bufferLength = uint8(digits.sub(4));
    //     params.zerosStartIndex = 5;
    //     params.zerosEndIndex = uint8(params.bufferLength.sub(1));
    //     params.sigfigIndex = 4;
    // } else {
    //     // 5 sigfigs surround decimal
    //     params.bufferLength = 6;
    //     params.sigfigIndex = 5;
    //     params.decimalIndex = uint8(digits.sub(5).add(1));
    // }
    let mut params: DecimalStringParams = Default::default();
    if priceBelow1 {
        // 7 bytes ( "0." and 5 sigfigs) + leading 0's bytes
        params.bufferLength = 7 + 43 - digits;
        params.zerosStartIndex = 2;
        params.zerosEndIndex = 43 - digits + 1;
        params.sigfigIndex = params.bufferLength - 1;
    } else if digits >= 9 {
        // no decimal in price string
        params.bufferLength = digits - 4;
        params.zerosStartIndex = 5;
        params.zerosEndIndex = params.bufferLength - 1;
        params.sigfigIndex = 4;
    } else {
        // 5 sigfigs surround decimal
        params.bufferLength = 6;
        params.sigfigIndex = 5;
        params.decimalIndex = digits - 5 + 1;
    }
    // params.sigfigs = sigfigs;
    // params.isLessThanOne = priceBelow1;
    // params.isPercent = false;
    params.sigfigs = sigfigs;
    params.isLessThanOne = priceBelow1;
    params.isPercent = false;

    // return generateDecimalString(params);
    return generateDecimalString(params);
}

fn adjustForDecimalPrecision(
    sqrtRatioX96: U160,
    baseTokenDecimals: u8,
    quoteTokenDecimals: u8,
) -> U256 {
    // uint256 difference = abs(int256(baseTokenDecimals).sub(int256(quoteTokenDecimals)));
    let difference: U256 =
        U256::from((Int256::from(baseTokenDecimals) - (Int256::from(quoteTokenDecimals))).abs());
    // if (difference > 0 && difference <= 18) {
    //     if (baseTokenDecimals > quoteTokenDecimals) {
    //         adjustedSqrtRatioX96 = sqrtRatioX96.mul(10**(difference.div(2)));
    //         if (difference % 2 == 1) {
    //             adjustedSqrtRatioX96 = FullMath.mulDiv(adjustedSqrtRatioX96, sqrt10X128, 1 << 128);
    //         }
    //     } else {
    //         adjustedSqrtRatioX96 = sqrtRatioX96.div(10**(difference.div(2)));
    //         if (difference % 2 == 1) {
    //             adjustedSqrtRatioX96 = FullMath.mulDiv(adjustedSqrtRatioX96, 1 << 128, sqrt10X128);
    //         }
    //     }
    // } else {
    //     adjustedSqrtRatioX96 = uint256(sqrtRatioX96);
    // }
    let mut adjustedSqrtRatioX96: U256;
    let sqrt10X128_U256 = U256::from_dec_str(sqrt10X128).expect("sqrt10X128 from dec str error!");
    if difference > U256::zero() && difference <= U256::from(18) {
        if baseTokenDecimals > quoteTokenDecimals {
            adjustedSqrtRatioX96 = sqrtRatioX96 * (U256::from(10).pow(difference / 2));
            if difference % 2 == U256::one() {
                adjustedSqrtRatioX96 =
                    FullMath::mulDiv(adjustedSqrtRatioX96, sqrt10X128_U256, U256::zero() << 128);
            }
        } else {
            adjustedSqrtRatioX96 = sqrtRatioX96 / (U256::from(10).pow(difference / (2)));
            if difference % 2 == U256::one() {
                adjustedSqrtRatioX96 =
                    FullMath::mulDiv(adjustedSqrtRatioX96, U256::zero() << 128, sqrt10X128_U256);
            }
        }
    } else {
        adjustedSqrtRatioX96 = sqrtRatioX96;
    }
    return adjustedSqrtRatioX96;
}

fn sigfigsRounded(mut value: U256, digits: u8) -> (U256, bool) {
    let mut extraDigit: bool = false;
    if digits > 5 {
        value = value / (U256::from(10).pow(U256::from(digits - 5)));
    }
    let roundUp = value % 10 > U256::from(4);
    value = value / 10;
    if roundUp {
        value = value + 1;
    }
    // 99999 -> 100000 gives an extra sigfig
    if value == U256::from(100000) {
        value /= 10;
        extraDigit = true;
    }
    return (value, extraDigit);
}

fn generateDescriptionPartOne(
    quoteTokenSymbol: String,
    baseTokenSymbol: String,
    poolAddress: String,
) -> String {
    return
        // string(
        //     abi.encodePacked(
        //         "This NFT represents a liquidity position in a Uniswap V3 ",
        //         quoteTokenSymbol,
        //         "-",
        //         baseTokenSymbol,
        //         " pool. ",
        //         "The owner of this NFT can modify or redeem the position.\\n",
        //         "\\nPool Address: ",
        //         poolAddress,
        //         "\\n",
        //         quoteTokenSymbol
        //     )
        // );
        String::from_utf8_lossy(
            &scale::Encode::encode(&(
                "This NFT represents a liquidity position in a Uniswap V3 ",
                quoteTokenSymbol.clone(),
                "-",
                baseTokenSymbol,
                " pool. ",
                "The owner of this NFT can modify or redeem the position.\\n",
                "\\nPool Address: ",
                poolAddress,
                "\\n",
                quoteTokenSymbol)
            )
        ).to_string()
}

fn generateDescriptionPartTwo(
    tokenId: String,
    baseTokenSymbol: String,
    quoteTokenAddress: String,
    baseTokenAddress: String,
    feeTier: String,
) -> String {
    return
        String::from_utf8_lossy(
            &scale::Encode::encode(&(
                " Address: ",
                quoteTokenAddress,
                "\\n",
                baseTokenSymbol,
                " Address: ",
                baseTokenAddress,
                "\\nFee Tier: ",
                feeTier,
                "\\nToken ID: ",
                tokenId,
                "\\n\\n",
                "⚠️ DISCLAIMER: Due diligence is imperative when assessing this NFT. Make sure token addresses match the expected tokens, as token symbols may be imitated."
            ))
        ).to_string();
}

// @notice Returns string as decimal percentage of fee amount.
// @param fee fee amount
fn feeToPercentString(fee: Uint24) -> String {
    // if (fee == 0) {
    //     return '0%';
    // }
    if fee == 0 {
        return String::from("0%");
    }
    // uint24 temp = fee;
    // uint256 digits;
    // uint8 numSigfigs;
    let mut temp: Uint24 = fee;
    let mut digits: u8 = 0;
    let mut numSigfigs: u8 = 0;
    // while (temp != 0) {
    //     if (numSigfigs > 0) {
    //         // count all digits preceding least significant figure
    //         numSigfigs++;
    //     } else if (temp % 10 != 0) {
    //         numSigfigs++;
    //     }
    //     digits++;
    //     temp /= 10;
    // }
    while temp != 0 {
        if numSigfigs > 0 {
            // count all digits preceding least significant figure
            numSigfigs += 1;
        } else if temp % 10 != 0 {
            numSigfigs += 1;
        }
        digits += 1;
        temp /= 10;
    }
    // DecimalStringParams memory params;
    // uint256 nZeros;
    let mut params: DecimalStringParams = Default::default();
    let nZeros: u8;
    // if (digits >= 5) {
    //     // if decimal > 1 (5th digit is the ones place)
    //     uint256 decimalPlace = digits.sub(numSigfigs) >= 4 ? 0 : 1;
    //     nZeros = digits.sub(5) < (numSigfigs.sub(1)) ? 0 : digits.sub(5).sub(numSigfigs.sub(1));
    if digits >= 5 {
        // if decimal > 1 (5th digit is the ones place)
        let decimalPlace = if digits - numSigfigs >= 4 { 0 } else { 1 };
        nZeros = if digits - 5 < numSigfigs - 1 {
            0
        } else {
            digits - 5 - (numSigfigs - 1)
        };
        //     params.zerosStartIndex = numSigfigs;
        //     params.zerosEndIndex = uint8(params.zerosStartIndex.add(nZeros).sub(1));
        //     params.sigfigIndex = uint8(params.zerosStartIndex.sub(1).add(decimalPlace));
        //     params.bufferLength = uint8(nZeros.add(numSigfigs.add(1)).add(decimalPlace));
        params.zerosStartIndex = numSigfigs;
        params.zerosEndIndex = params.zerosStartIndex + nZeros - 1;
        params.sigfigIndex = params.zerosStartIndex - (1) + (decimalPlace);
        params.bufferLength = nZeros + numSigfigs + 1 + decimalPlace;

    // } else {
    //     // else if decimal < 1
    //     nZeros = uint256(5).sub(digits);
    //     params.zerosStartIndex = 2;
    //     params.zerosEndIndex = uint8(nZeros.add(params.zerosStartIndex).sub(1));
    //     params.bufferLength = uint8(nZeros.add(numSigfigs.add(2)));
    //     params.sigfigIndex = uint8((params.bufferLength).sub(2));
    //     params.isLessThanOne = true;
    // }
    } else {
        // else if decimal < 1
        nZeros = 5 - digits;
        params.zerosStartIndex = 2;
        params.zerosEndIndex = nZeros + params.zerosStartIndex - 1;
        params.bufferLength = nZeros + numSigfigs + 2;
        params.sigfigIndex = params.bufferLength - 2;
        params.isLessThanOne = true;
    }
    // params.sigfigs = uint256(fee).div(10**(digits.sub(numSigfigs)));
    // params.isPercent = true;
    // params.decimalIndex = digits > 4 ? uint8(digits.sub(4)) : 0;

    // return generateDecimalString(params);
    params.sigfigs = U256::from(fee)
        .checked_div(U256::from(10u32.pow(u32::from(digits - numSigfigs))))
        .expect("check dive error!");
    params.isPercent = true;
    params.decimalIndex = if digits > 4 { digits - 4 } else { 0 };

    return generateDecimalString(params);
}

fn generateDecimalString(mut params: DecimalStringParams) -> String {
    // bytes memory buffer = new bytes(params.bufferLength);
    let len = usize::from(params.bufferLength);
    ink_env::debug_println!("len is:{}",len);
    let mut buffer: Vec<u8> = vec!(0;len);
    ink_env::debug_println!("buffer.len() is:{}",buffer.len());
    // if params.isPercent {
    //     buffer[buffer.length - 1] = '%';
    // }
    if params.isPercent {
        buffer[len - 1] = b'%';
    }
    // if (params.isLessThanOne) {
    //     buffer[0] = '0';
    //     buffer[1] = '.';
    // }
    if params.isLessThanOne {
        buffer[0] = b'0';
        buffer[1] = b'.';
    }

    // // add leading/trailing 0's
    // for (uint256 zerosCursor = params.zerosStartIndex; zerosCursor < params.zerosEndIndex.add(1); zerosCursor++) {
    //     buffer[zerosCursor] = bytes1(uint8(48));
    // }

    for i in params.zerosStartIndex..params.zerosEndIndex {
        buffer[usize::from(i)] = 48u8;
    }
    // // add sigfigs
    // while (params.sigfigs > 0) {
    //     if (params.decimalIndex > 0 && params.sigfigIndex == params.decimalIndex) {
    //         buffer[params.sigfigIndex--] = '.';
    //     }
    //     buffer[params.sigfigIndex--] = bytes1(uint8(uint256(48).add(params.sigfigs % 10)));
    //     params.sigfigs /= 10;
    // }
    while params.sigfigs > U256::zero() {
        if params.decimalIndex > 0 && params.sigfigIndex == params.decimalIndex {
            buffer[usize::from(params.sigfigIndex)] = b'.';
            params.sigfigIndex -= 1;
        }
        ink_env::debug_println!("params.sigfigIndex is:{:?}",params.sigfigIndex);
        buffer[usize::from(params.sigfigIndex)] =
            u8::try_from((U256::from(48) + (params.sigfigs % U256::from(10))).as_u32())
                .expect("u8 try_from error!");
        params.sigfigIndex -= 1;
        params.sigfigs /= 10;
    }
    // return string(buffer);
    String::from_utf8_lossy(&buffer).to_string()
}
