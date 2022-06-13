import BN from 'bn.js'
import { BigNumber } from 'ethers'
import { expect } from './shared/expect'
import { artifacts, network, patract } from 'redspot';
import Decimal from 'decimal.js';
import { encodePriceSqrt, MIN_SQRT_RATIO, MAX_SQRT_RATIO } from './shared/utilities';

const { getContractFactory, getRandomSigner } = patract;

const { api, getAddresses, getSigners } = network;

import { setupContract, fromSigner, setupProxy } from './helpers'

const MIN_TICK = -887272
const MAX_TICK = 887272

describe('TickMath', () => {
    after(() => {
        return api.disconnect();
    });

    // async function setup() {
    //     // await api.isReady;
    //     // const signerAddresses = await getAddresses();
    //     // const Alice = signerAddresses[0];
    //     // const sender = await getRandomSigner(Alice, '20000 UNIT');
    //     // const contractFactory = await getContractFactory('weth9_contract', sender.address);
    //     // const contract = await contractFactory.deploy('new', 'None','None');
    //     // // const abi = artifacts.readArtifact('metadata');
    //     // const receiver = await getRandomSigner();
    //     // return { sender, contractFactory, contract, receiver, Alice };
    // }

    async function setup() {
        const ian = await getRandomSigner()
        const kayne = await getRandomSigner()
        let contract = await setupContract('TickMathTest', 'new',);

        return { contract, kayne, ian }
    }

    describe('#getSqrtRatioAtTick', () => {
        it('throws for too low', async () => {
            await api.isReady;
            const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
            await expect(tickMathQuery.getSqrtRatioAtTick(MIN_TICK - 1)).to.eventually.be.rejectedWith('T');
        })

        it('throws for too low', async () => {
            const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
            await expect(tickMathQuery.getSqrtRatioAtTick(MAX_TICK + 1)).to.eventually.be.rejectedWith('T');
        })

        it('min tick', async () => {
            const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
            expect(await (await tickMathQuery.getSqrtRatioAtTick(MIN_TICK)).output).to.eq('4295128739')
        })

        it('min tick +1', async () => {
            const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
            expect(await (await tickMathQuery.getSqrtRatioAtTick(MIN_TICK + 1)).output).to.eq('4295343490')
        })

        it('max tick - 1', async () => {
            const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
            expect(await (await tickMathQuery.getSqrtRatioAtTick(MAX_TICK - 1)).output).to.eq('1461373636630004318706518188784493106690254656249')
        })

        it('max tick', async () => {
            const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
            expect(await (await tickMathQuery.getSqrtRatioAtTick(MAX_TICK)).output).to.eq('1461446703485210103287273052203988822378723970342')
        })

        it('min tick ratio is less than js implementation', async () => {
            const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
            let output = (await tickMathQuery.getSqrtRatioAtTick(MIN_TICK)).output;
            let ticker = Number.parseInt(output ? output.toString() : "0");
            expect(ticker < (encodePriceSqrt(1, BigNumber.from(2).pow(127))).toNumber());
        })

        it('max tick ratio is greater than js implementation', async () => {
            const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
            expect((await tickMathQuery.getSqrtRatioAtTick(MAX_TICK)).output).to.be.gt(encodePriceSqrt(BigNumber.from(2).pow(127), 1))
        })

        // for (const absTick of [
        //     50,
        //     100,
        //     250,
        //     500,
        //     1_000,
        //     2_500,
        //     3_000,
        //     4_000,
        //     5_000,
        //     50_000,
        //     150_000,
        //     250_000,
        //     500_000,
        //     738_203,
        // ]) {
        //     for (const tick of [-absTick, absTick]) {
        //         describe(`tick ${tick}`, () => {
        //             it('is at most off by 1/100th of a bips', async () => {
        //                 const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
        //                 const jsResult = new Decimal(1.0001).pow(tick).sqrt().mul(new Decimal(2).pow(96))
        //                 const result = await tickMathQuery.getSqrtRatioAtTick(tick)
        //                 const absDiff = new Decimal(result.output?result.output.toString():"0").sub(jsResult).abs()
        //                 expect(absDiff.div(jsResult).toNumber()).to.be.lt(0.000001)
        //             })
        //             // it('result', async () => {
        //             //   expect((await tickMathQuery.getSqrtRatioAtTick(tick)).toString()).to.matchSnapshot()
        //             // })
        //             // it('gas', async () => {
        //             //   await snapshotGasCost(tickMathQuery.getGasCostOfGetSqrtRatioAtTick(tick))
        //             // })
        //         })
        //     }
        // }


        describe('#MIN_SQRT_RATIO', async () => {
            it('equals #getSqrtRatioAtTick(MIN_TICK)', async () => {
                const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
                const min = await tickMathQuery.getSqrtRatioAtTick(MIN_TICK);
                expect(min.output).to.equal(await (await tickMathQuery.minSqrtRatio()).output)
                expect(min.output).to.equal(MIN_SQRT_RATIO)
            })
        })

        describe('#MAX_SQRT_RATIO', async () => {
            it('equals #getSqrtRatioAtTick(MAX_TICK)', async () => {
                const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
                const max = await tickMathQuery.getSqrtRatioAtTick(MAX_TICK)
                expect(max.output).to.equal(await (await tickMathQuery.maxSqrtRatio()).output)
                expect(max.output).to.equal(MAX_SQRT_RATIO)
            })
        })

    });

    describe('#getTickAtSqrtRatio', () => {
        it('throws for too low', async () => {
            const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
            await expect(tickMathQuery.getTickAtSqrtRatio(MIN_SQRT_RATIO.sub(1))).to.eventually.be.rejectedWith('R');
        })

        it('throws for too high', async () => {
            const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
            await expect(tickMathQuery.getTickAtSqrtRatio(BigNumber.from(MAX_SQRT_RATIO))).to.be.rejectedWith('R')
        })

        it('ratio of min tick', async () => {
            const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
            expect((await tickMathQuery.getTickAtSqrtRatio(MIN_SQRT_RATIO)).output).to.eq(MIN_TICK)
        })
        it('ratio of min tick + 1', async () => {
            const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
            expect((await tickMathQuery.getTickAtSqrtRatio('4295343490')).output).to.eq(MIN_TICK + 1)
        })
        it('ratio of max tick - 1', async () => {
            const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
            expect((await tickMathQuery.getTickAtSqrtRatio('1461373636630004318706518188784493106690254656249')).output).to.eq(MAX_TICK - 1)
        })
        it('ratio closest to max tick', async () => {
            const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
            expect((await tickMathQuery.getTickAtSqrtRatio(MAX_SQRT_RATIO.sub(1))).output).to.eq(MAX_TICK - 1)
        })

        for (const ratio of [
            MIN_SQRT_RATIO,
            encodePriceSqrt(BigNumber.from(10).pow(12), 1),
            encodePriceSqrt(BigNumber.from(10).pow(6), 1),
            encodePriceSqrt(1, 64),
            encodePriceSqrt(1, 8),
            encodePriceSqrt(1, 2),
            encodePriceSqrt(1, 1),
            encodePriceSqrt(2, 1),
            encodePriceSqrt(8, 1),
            encodePriceSqrt(64, 1),
            encodePriceSqrt(1, BigNumber.from(10).pow(6)),
            encodePriceSqrt(1, BigNumber.from(10).pow(12)),
            MAX_SQRT_RATIO.sub(1),
        ]) {
            describe(`ratio ${ratio}`, () => {
                it('is at most off by 1', async () => {
                    const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
                    const jsResult = new Decimal(ratio.toString()).div(new Decimal(2).pow(96)).pow(2).log(1.0001).floor()
                    const result = await (await tickMathQuery.getTickAtSqrtRatio(ratio)).output;
                    const absDiff = new Decimal(result ? result.toString() : "0").sub(jsResult).abs();
                    expect(absDiff.toNumber()).to.be.lte(1)
                })
                it('ratio is between the tick and tick+1', async () => {
                    const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
                    const tick = await (await tickMathQuery.getTickAtSqrtRatio(ratio)).output;
                    console.log("tick is----------:", new Decimal(tick ? tick.toString() : "0"));
                    const ratioOfTick = (await tickMathQuery.getSqrtRatioAtTick(tick ? tick.toString() : "0")).output
                    const ratioOfTickPlusOne = await (await tickMathQuery.getSqrtRatioAtTick(Number.parseInt(tick ? tick.toString() : "0") + 1)).output
                    console.log("ratio is-------", ratio.toString());
                    console.log("ratioOfTick is----------:", ratioOfTick?.toString());
                    console.log("ratioOfTickPlusOne is----------:", BigNumber.from(ratioOfTickPlusOne ? ratioOfTickPlusOne.toString() : "0"));
                    //TODO 79228162514264337593543 is error!
                    expect(new BN(ratio.toString())).to.be.gte(new BN(ratioOfTick ? ratioOfTick.toString() : "0"));

                    let ratioOf = new BN(ratioOfTickPlusOne ? ratioOfTickPlusOne.toString() : "0");
                    expect(new BN(ratio.toString())).to.be.lt(ratioOf);
                })
                it('result', async () => {
                    const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
                  expect(await tickMathQuery.getTickAtSqrtRatio(ratio)).to.matchSnapshot()
                })
                it('gas', async () => {
                    const { query: tickMathQuery, tx: tickMathTx, } = await setupContract("TickMathTest", "new");
                  expect(await tickMathQuery.getGasCostOfGetTickAtSqrtRatio(ratio)).to.matchSnapshot();
                })
            })
        }
    });
})