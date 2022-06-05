#![allow(non_snake_case)]
use ink_env::hash::{HashOutput, Sha2x256};
use ink_prelude::string::String;
use ink_prelude::string::ToString;
use primitives::{Address, Int24, U256};

use crate::core::BitMath;

const curve1: &str = "M1 1C41 41 105 105 145 145";
const curve2: &str = "M1 1C33 49 97 113 145 145";
const curve3: &str = "M1 1C33 57 89 113 145 145";
const curve4: &str = "M1 1C25 65 81 121 145 145";
const curve5: &str = "M1 1C17 73 73 129 145 145";
const curve6: &str = "M1 1C9 81 65 137 145 145";
const curve7: &str = "M1 1C1 89 57.5 145 145 145";
const curve8: &str = "M1 1C1 97 49 145 145 145";

#[derive(Clone, Default)]
pub struct SVGParams {
    pub quoteToken: String,
    pub baseToken: String,
    pub poolAddress: Address,
    pub quoteTokenSymbol: String,
    pub baseTokenSymbol: String,
    pub feeTier: String,
    pub tickLower: Int24,
    pub tickUpper: Int24,
    pub tickSpacing: Int24,
    pub overRange: i8,
    pub tokenId: U256,
    pub color0: String,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub x1: String,
    pub y1: String,
    pub x2: String,
    pub y2: String,
    pub x3: String,
    pub y3: String,
}

pub fn generateSVG(params: SVGParams) -> String {
    /*
    address: "0xe8ab59d3bcde16a29912de83a90eb39628cfc163",
    msg: "Forged in SVG for Uniswap in 2021 by 0xe8ab59d3bcde16a29912de83a90eb39628cfc163",
    sig: "0x2df0e99d9cbfec33a705d83f75666d98b22dea7c1af412c584f7d626d83f02875993df740dc87563b9c73378f8462426da572d7989de88079a382ad96c57b68d1b",
    version: "2"
    */
    return
        // string(
        //     abi.encodePacked(
        //         generateSVGDefs(params),
        //         generateSVGBorderText(
        //             params.quoteToken,
        //             params.baseToken,
        //             params.quoteTokenSymbol,
        //             params.baseTokenSymbol
        //         ),
        //         generateSVGCardMantle(params.quoteTokenSymbol, params.baseTokenSymbol, params.feeTier),
        //         generageSvgCurve(params.tickLower, params.tickUpper, params.tickSpacing, params.overRange),
        //         generateSVGPositionDataAndLocationCurve(
        //             params.tokenId.toString(),
        //             params.tickLower,
        //             params.tickUpper
        //         ),
        //         generateSVGRareSparkle(params.tokenId, params.poolAddress),
        //         </svg>'
        //     )
        // );
        String::from_utf8_lossy(
            &scale::Encode::encode(&(
                generateSVGDefs(params.clone()),
                generateSVGBorderText(
                    params.quoteToken,
                    params.baseToken,
                    params.quoteTokenSymbol.clone(),
                    params.baseTokenSymbol.clone()
                ),
                generateSVGCardMantle(params.quoteTokenSymbol.clone(), params.baseTokenSymbol.clone(), params.feeTier),
                generageSvgCurve(params.tickLower, params.tickUpper, params.tickSpacing, params.overRange),
                generateSVGPositionDataAndLocationCurve(
                    params.tokenId.to_string(),
                    params.tickLower,
                    params.tickUpper
                ),
                generateSVGRareSparkle(params.tokenId, params.poolAddress),
                "</svg>"
            ))
        ).to_string();
}

fn generateSVGRareSparkle(tokenId: U256, poolAddress: Address) -> String {
    let svg;
    if isRare(tokenId, poolAddress) {
        svg = String::from_utf8_lossy(
            &scale::Encode::encode(&
                r#"<g style="transform:translate(226px, 392px)"><rect width="36px" height="36px" rx="8px" ry="8px" fill="none" stroke="rgba(255,255,255,0.2)" />,
                "<g><path style="transform:translate(6px,6px)" d="M12 0L12.6522 9.56587L18 1.6077L13.7819 10.2181L22.3923 6L14.4341 ,
                "11.3478L24 12L14.4341 12.6522L22.3923 18L13.7819 13.7819L18 22.3923L12.6522 14.4341L12 24L11.3478 14.4341L6 22.39,
                "23L10.2181 13.7819L1.6077 18L9.56587 12.6522L0 12L9.56587 11.3478L1.6077 6L10.2181 10.2181L6 1.6077L11.3478 9.56587L12 0Z" fill="white" />,
                "<animateTransform attributeName="transform" type="rotate" from="0 18 18" to="360 18 18" dur="10s" repeatCount="indefinite"/></g></g>"#
            )
        ).to_string();
    } else {
        svg = "".to_string();
    }
    svg
}

fn isRare(tokenId: U256, poolAddress: Address) -> bool {
    // bytes32 h = keccak256(abi.encodePacked(tokenId, poolAddress));
    // return uint256(h) < type(uint256).max / (1 + BitMath.mostSignificantBit(tokenId) * 2);
    let mut buffer = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
    ink_env::hash_encoded::<Sha2x256, _>(
        &scale::Encode::encode(&(tokenId, poolAddress)),
        &mut buffer,
    );
    let h: U256 = U256::from_little_endian(&buffer);
    return h < U256::MAX / (1 + BitMath::mostSignificantBit(tokenId) * 2);
}

fn generateSVGPositionDataAndLocationCurve(
    tokenId: String,
    tickLower: Int24,
    tickUpper: Int24,
) -> String {
    let tickLowerStr: String = tickToString(tickLower);
    let tickUpperStr: String = tickToString(tickUpper);
    let str1length = tokenId.len() + 4;
    let str2length = tickLowerStr.len() + 10;
    let str3length = tickUpperStr.len() + 10;
    let (xCoord, yCoord) = rangeLocation(tickLower, tickUpper);
    let svg = String::from_utf8_lossy(&
        scale::Encode::encode(&(
            scale::Encode::encode(&(r#" <g style="transform:translate(29px, 384px)">"#,
            r#"<rect width=""#,
            U256::from(7 * (str1length + 4)).to_string(),
            r#"px" height="26px" rx="8px" ry="8px" fill="rgba(0,0,0,0.6)" />"#,
            r#"<text x="12px" y="17px" font-family="\"Courier New\", monospace" font-size="12px" fill="white"><tspan fill="rgba(255,255,255,0.6)">ID: </tspan>"#,
            tokenId,
            r#"</text></g>"#,
            r#" <g style="transform:translate(29px, 414px)">"#,
            r#"<rect width=""#,
            U256::from(7 * (str2length + 4)).to_string(),
            r#"px" height="26px" rx="8px" ry="8px" fill="rgba(0,0,0,0.6)" />"#,
            r#"<text x="12px" y="17px" font-family="\"Courier New\", monospace" font-size="12px" fill="white"><tspan fill="rgba(255,255,255,0.6)">Min Tick: </tspan>"#,
            tickLowerStr,
            r#"</text></g>"#,
            r#" <g style="transform:translate(29px, 444px)">"#,
            r#"<rect width=""#,
        )),

            U256::from(7 * (str3length + 4)).to_string(),
            r#"px" height="26px" rx="8px" ry="8px" fill="rgba(0,0,0,0.6)" />"#,
            r#"<text x="12px" y="17px" font-family="\"Courier New\", monospace" font-size="12px" fill="white"><tspan fill="rgba(255,255,255,0.6)">Max Tick: </tspan>"#,
            tickUpperStr,
            r#"</text></g>"#,
            r#"<g style="transform:translate(226px, 433px)">"#,
            r#"<rect width="36px" height="36px" rx="8px" ry="8px" fill="none" stroke="rgba(255,255,255,0.2)" />"#,
            r#"<path stroke-linecap="round" d="M8 9C8.00004 22.9494 16.2099 28 27 28" fill="none" stroke="white" />"#,
            r#"<circle style="transform:translate3d("#,
            xCoord,
            r#"px, "#,
            yCoord,
            r#"px, 0px)" cx="0px" cy="0px" r="4px" fill="white"/></g>"#
        ))
    ).to_string();
    svg
}

fn rangeLocation(tickLower: Int24, tickUpper: Int24) -> (String, String) {
    let midPoint: Int24 = (tickLower + tickUpper) / 2;
    if midPoint < -125_000 {
        return ("8".to_string(), "7".to_string());
    } else if midPoint < -75_000 {
        return ("8".to_string(), "10.5".to_string());
    } else if midPoint < -25_000 {
        return ("8".to_string(), "14.25".to_string());
    } else if midPoint < -5_000 {
        return ("10".to_string(), "18".to_string());
    } else if midPoint < 0 {
        return ("11".to_string(), "21".to_string());
    } else if midPoint < 5_000 {
        return ("13".to_string(), "23".to_string());
    } else if midPoint < 25_000 {
        return ("15".to_string(), "25".to_string());
    } else if midPoint < 75_000 {
        return ("18".to_string(), "26".to_string());
    } else if midPoint < 125_000 {
        return ("21".to_string(), "27".to_string());
    } else {
        return ("24".to_string(), "27".to_string());
    }
}

fn tickToString(mut tick: Int24) -> String {
    let mut sign: String = "".to_string();
    if tick < 0 {
        tick = tick * -1;
        sign = "-".to_string();
    }
    return String::from_utf8_lossy(&scale::Encode::encode(&(
        sign,
        U256::from(tick).to_string(),
    )))
    .to_string();
}

fn generageSvgCurve(
    tickLower: Int24,
    tickUpper: Int24,
    tickSpacing: Int24,
    overRange: i8,
) -> String {
    let fade: String = if overRange == 1 {
        "#fade-up".to_string()
    } else if overRange == -1 {
        "#fade-down".to_string()
    } else {
        "#none".to_string()
    };
    let curve: String = getCurve(tickLower, tickUpper, tickSpacing);
    let svg = String::from_utf8_lossy(&scale::Encode::encode(&(
        r#"<g mask="url("#,
        fade.clone(),
        r#")""#,
        r#" style="transform:translate(72px,189px)">"#,
        r#"<rect x="-16px" y="-16px" width="180px" height="180px" fill="none" />"#,
        r#"<path d=""#,
        curve.clone(),
        r#"" stroke="rgba(0,0,0,0.3)" stroke-width="32px" fill="none" stroke-linecap="round" />"#,
        r#"</g><g mask="url("#,
        fade,
        r#")""#,
        r#" style="transform:translate(72px,189px)">"#,
        r#"<rect x="-16px" y="-16px" width="180px" height="180px" fill="none" />"#,
        r#"<path d=""#,
        curve,
        r#"" stroke="rgba(255,255,255,1)" fill="none" stroke-linecap="round" /></g>"#,
        generateSVGCurveCircle(overRange),
    )))
    .to_string();
    svg
}

fn generateSVGCurveCircle(overRange: i8) -> String {
    let curvex1: String = "73".to_string();
    let curvey1: String = "190".to_string();
    let curvex2: String = "217".to_string();
    let curvey2: String = "334".to_string();
    let svg;
    if overRange == 1 || overRange == -1 {
        svg = String::from_utf8_lossy(&scale::Encode::encode(&(
            r#"<circle cx=""#,
            if overRange == -1 {
                curvex1.clone()
            } else {
                curvex2.clone()
            },
            r#"px" cy=""#,
            if overRange == -1 {
                curvey1.clone()
            } else {
                curvey2.clone()
            },
            r#"px" r="4px" fill="white" /><circle cx=""#,
            if overRange == -1 { curvex1 } else { curvex2 },
            r#"px" cy=""#,
            if overRange == -1 { curvey1 } else { curvey2 },
            r#"px" r="24px" fill="none" stroke="white" />"#,
        )))
        .to_string();
    } else {
        svg = String::from_utf8_lossy(&scale::Encode::encode(&(
            r#"<circle cx=""#,
            curvex1,
            r#"px" cy=""#,
            curvey1,
            r#"px" r="4px" fill="white" />"#,
            r#"<circle cx=""#,
            curvex2,
            r#"px" cy=""#,
            curvey2,
            r#"px" r="4px" fill="white" />"#,
        )))
        .to_string();
    }
    svg
}

fn getCurve(tickLower: Int24, tickUpper: Int24, tickSpacing: Int24) -> String {
    let curve;
    let tickRange: Int24 = (tickUpper - tickLower) / tickSpacing;
    if tickRange <= 4 {
        curve = curve1;
    } else if tickRange <= 8 {
        curve = curve2;
    } else if tickRange <= 16 {
        curve = curve3;
    } else if tickRange <= 32 {
        curve = curve4;
    } else if tickRange <= 64 {
        curve = curve5;
    } else if tickRange <= 128 {
        curve = curve6;
    } else if tickRange <= 256 {
        curve = curve7;
    } else {
        curve = curve8;
    }
    curve.to_string()
}

fn generateSVGCardMantle(
    quoteTokenSymbol: String,
    baseTokenSymbol: String,
    feeTier: String,
) -> String {
    let svg = String::from_utf8_lossy(&
        scale::Encode::encode(&(
            r#"<g mask="url(#fade-symbol)"><rect fill="none" x="0px" y="0px" width="290px" height="200px" /> <text y="70px" x="32px" fill="white" font-family="Courier New, monospace" font-weight="200" font-size="36px">"#,
            quoteTokenSymbol,
            "/",
            baseTokenSymbol,
            r#"</text><text y="115px" x="32px" fill="white" font-family="Courier New, monospace" font-weight="200" font-size="36px">"#,
            feeTier,
            "</text></g>",
            r#"<rect x="16" y="16" width="258" height="468" rx="26" ry="26" fill="rgba(0,0,0,0)" stroke="rgba(255,255,255,0.2)" />"#
        ))
    ).to_string();
    svg
}

fn generateSVGBorderText(
    quoteToken: String,
    baseToken: String,
    quoteTokenSymbol: String,
    baseTokenSymbol: String,
) -> String {
    let svg = String::from_utf8_lossy(&
        scale::Encode::encode(&(
            scale::Encode::encode(&(r##"<text text-rendering="optimizeSpeed">"##,
            r##"<textPath startOffset="-100%" fill="white" font-family="Courier New, monospace" font-size="10px" xlink:href="#text-path-a">"##,
            baseToken.clone(),
            ".",
            baseTokenSymbol.clone(),
            r##" <animate additive="sum" attributeName="startOffset" from="0%" to="100%" begin="0s" dur="30s" repeatCount="indefinite" />"##,
            r##"</textPath> <textPath startOffset="0%" fill="white" font-family="Courier New, monospace" font-size="10px" xlink:href="#text-path-a">"##,
            baseToken,
            ".",
            baseTokenSymbol,
            r##" <animate additive="sum" attributeName="startOffset" from="0%" to="100%" begin="0s" dur="30s" repeatCount="indefinite" /> </textPath>"##,
            r##"<textPath startOffset="50%" fill="white" font-family="Courier New, monospace" font-size="10px" xlink:href="#text-path-a">"##,
            quoteToken.clone(),)),


            ".",
            quoteTokenSymbol.clone(),
            r##" <animate additive="sum" attributeName="startOffset" from="0%" to="100%" begin="0s" dur="30s""##,
            r##" repeatCount="indefinite" /></textPath><textPath startOffset="-50%" fill="white" font-family="Courier New, monospace" font-size="10px" xlink:href="#text-path-a">"##,
            quoteToken,
            ".",
            quoteTokenSymbol,
            r##" <animate additive="sum" attributeName="startOffset" from="0%" to="100%" begin="0s" dur="30s" repeatCount="indefinite" /></textPath></text>"##,
        ))
    ).to_string();
    svg
}

fn generateSVGDefs(params: SVGParams) -> String {
    let mut svg = String::from("");
    svg.push_str("<svg width=\"290\" height=\"500\" viewBox=\"0 0 290 500\" xmlns=\"http://www.w3.org/2000/svg\"");
    svg.push_str(" xmlns:xlink='http://www.w3.org/1999/xlink'>");
    svg.push_str("<defs>");
    svg.push_str(r#"<filter id="f1"><feImage result="p0" xlink:href="data:image/svg+xml;base64,"#);
    svg.push_str(r#"<svg width='290' height='500' viewBox='0 0 290 500' xmlns='http://www.w3.org/2000/svg'><rect width='290px' height='500px' fill='"#);
    svg.push_str(&params.color0);
    svg.push_str("'/></svg>");
    svg.push_str(r#""/><feImage result="p1" xlink:href="data:image/svg+xml;base64,"#);
    svg.push_str("<svg width='290' height='500' viewBox='0 0 290 500' xmlns='http://www.w3.org/2000/svg'><circle cx='");
    svg.push_str(&params.x1);
    svg.push_str("' cy='");
    svg.push_str(&params.y1);
    svg.push_str("' r='120px' fill='#");
    svg.push_str(&params.color1);
    svg.push_str("'/></svg>");
    svg.push_str(r#""/><feImage result="p\" xlink:href="data:image/svg+xml;base64,"#);
    svg.push_str(r#"<svg width='290' height='500' viewBox='0 0 290 500' xmlns='http://www.w3.org/2000/svg'><circle cx='"#);
    svg.push_str(&params.x2);
    svg.push_str("' cy='");
    svg.push_str(&params.y2);
    svg.push_str("' r='120px' fill='#");
    svg.push_str(&params.color2);
    svg.push_str("'/></svg>");
    svg.push_str(r#"'" />""#);
    svg.push_str(r#"<feImage result="p3" xlink:href="data:image/svg+xml;base64,"#);
    svg.push_str(&base64::encode(
                    {
                    let mut base = String::from("");
                    base.push_str("<svg width='290' height='500' viewBox='0 0 290 500' xmlns='http://www.w3.org/2000/svg'><circle cx='");
                    base.push_str(&params.x3);
                    base.push_str("' cy='");
                    base.push_str(&params.y3);
                    base.push_str("' r='100px' fill='#");
                    base.push_str(&params.color3);
                    base.push_str("'/></svg>");
                    base
                }
        ));
    svg.push_str(r#"" /><feBlend mode="overlay" in="p0" in2="p1" /><feBlend mode="exclusion" in2="p2" /><feBlend mode="overlay" in2="p3" result="blendOut" /><feGaussianBlur '
            in="blendOut" stdDeviation="42" /></filter> <clipPath id="corners"><rect width="290" height="500" rx="42" ry="42" /></clipPath>',
            <path id="text-path-a" d="M40 12 H250 A28 28 0 0 1 278 40 V460 A28 28 0 0 1 250 488 H40 A28 28 0 0 1 12 460 V40 A28 28 0 0 1 40 12 z" />',
            <path id="minimap" d="M234 444C234 457.949 242.21 463 253 463" />',
            <filter id="top-region-blur"><feGaussianBlur in="SourceGraphic" stdDeviation="24" /></filter>',
            <linearGradient id="grad-up" x1="1" x2="0" y1="1" y2="0"><stop offset="0.0" stop-color="white" stop-opacity="1" />',
            <stop offset=".9" stop-color="white" stop-opacity="0" /></linearGradient>',
            <linearGradient id="grad-down" x1="0" x2="1" y1="0" y2="1"><stop offset="0.0" stop-color="white" stop-opacity="1" /><stop offset="0.9" stop-color="white" stop-opacity="0" /></linearGradient>',
            <mask id="fade-up" maskContentUnits="objectBoundingBox"><rect width="1" height="1" fill="url(#grad-up)" /></mask>',
            <mask id="fade-down" maskContentUnits="objectBoundingBox"><rect width="1" height="1" fill="url(#grad-down)" /></mask>',
            <mask id="none" maskContentUnits="objectBoundingBox"><rect width="1" height="1" fill="white" /></mask>',
            <linearGradient id="grad-symbol"><stop offset="0.7" stop-color="white" stop-opacity="1" /><stop offset=".95" stop-color="white" stop-opacity="0" /></linearGradient>',
            <mask id="fade-symbol" maskContentUnits="userSpaceOnUse"><rect width="290px" height="200px" fill="url(#grad-symbol)" /></mask></defs>',
            <g clip-path="url(#corners)">',
            <rect fill=""#);
    svg.push_str(&params.color0);
    svg.push_str(r##"" x="0px" y="0px" width="290px" height="500px" />
            <rect style="filter: url(#f1)" x="0px" y="0px" width="290px" height="500px" />
             <g style="filter:url(#top-region-blur); transform:scale(1.5); transform-origin:center top;">
            <rect fill="none" x="0px" y="0px" width="290px" height="500px" />
            <ellipse cx="50%" cy="0px" rx="180px" ry="120px" fill="#000" opacity="0.85" /></g>
            <rect x="0" y="0" width="290" height="500" rx="42" ry="42" fill="rgba(0,0,0,0)" stroke="rgba(255,255,255,0.2)" /></g>"##);
    svg
}

#[cfg(test)]
pub mod test {
    use super::{generateSVGDefs, SVGParams};

    #[test]
    pub fn it_works() {
        let svf_dfg = generateSVGDefs(SVGParams {
            color0: String::from("color0"),
            ..SVGParams::default()
        });
        println!("svf_dfg is:{:?}", svf_dfg);
    }
}
