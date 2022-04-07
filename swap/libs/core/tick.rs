use primitives::{U256, Uint160, I56, U160, Uint24, Int24};

use super::tick_math;

// info stored for each initialized individual tick
struct Info {
    // the total position liquidity that references this tick
    liquidityGross:u128,
    // amount of net liquidity added (subtracted) when tick is crossed from left to right (right to left),
    liquidityNet:u128,
    // fee growth per unit of liquidity on the _other_ side of this tick (relative to the current tick)
    // only has relative meaning, not absolute — the value depends on when the tick is initialized
    feeGrowthOutside0X128:U256,
    feeGrowthOutside1X128:U256,
    // the cumulative tick value on the other side of the tick
    tickCumulativeOutside:I56,
    // the seconds per unit of liquidity on the _other_ side of this tick (relative to the current tick)
    // only has relative meaning, not absolute — the value depends on when the tick is initialized
    secondsPerLiquidityOutsideX128:U160,
    // the seconds spent on the other side of the tick (relative to the current tick)
    // only has relative meaning, not absolute — the value depends on when the tick is initialized
    secondsOutside:u32,
    // true iff the tick is initialized, i.e. the value is exactly equivalent to the expression liquidityGross != 0
    // these 8 bits are set to prevent fresh sstores when crossing newly initialized ticks
    initialized:bool,
}

/// @notice Derives max liquidity per tick from given tick spacing
/// @dev Executed within the pool constructor
/// @param tickSpacing The amount of required tick separation, realized in multiples of `tickSpacing`
///     e.g., a tickSpacing of 3 requires ticks to be initialized every 3rd tick i.e., ..., -6, -3, 0, 3, 6, ...
/// @return The max liquidity per tick
pub fn tick_spacing_to_max_liquidity_per_tick(tick_spacing:Int24)  ->u128 {
    // int24 minTick = (TickMath.MIN_TICK / tickSpacing) * tickSpacing;
    //     int24 maxTick = (TickMath.MAX_TICK / tickSpacing) * tickSpacing;
    //     uint24 numTicks = uint24((maxTick - minTick) / tickSpacing) + 1;
    //     return type(uint128).max / numTicks;
    let min_tick:Int24 = (tick_math::MIN_TICK / tick_spacing) * tick_spacing;
    let max_tick:Int24 = (tick_math::MAX_TICK / tick_spacing) * tick_spacing;
    let num_ticks:Uint24 = (max_tick.saturating_sub(min_tick)).saturating_div(tick_spacing).saturating_add(1).try_into().unwrap();
    u128::MAX.saturating_div(num_ticks.into())
}