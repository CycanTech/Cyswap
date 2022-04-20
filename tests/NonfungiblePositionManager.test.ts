import { expect } from 'chai';
import { artifacts, network, patract } from 'redspot';

const { getContractFactory, getRandomSigner } = patract;

const { api, getAddresses, getSigners } = network;

import { setupContract, fromSigner, setupProxy } from './helpers'

describe('WETH9', () => {
  after(() => {
    return api.disconnect();
  });

  async function setup() {
    // await api.isReady;
    // const signerAddresses = await getAddresses();
    // const Alice = signerAddresses[0];
    // const sender = await getRandomSigner(Alice, '20000 UNIT');
    // const contractFactory = await getContractFactory('weth9_contract', sender.address);
    // const contract = await contractFactory.deploy('new', 'None','None');
    // // const abi = artifacts.readArtifact('metadata');
    // const receiver = await getRandomSigner();
    // return { sender, contractFactory, contract, receiver, Alice };
  }

  it('test deposit and withdraw',async () =>{
    await api.isReady;
    const { query:factoryQuery,tx:factoryTx,contract:factoryContract} = await setupContract("factory","new");
    const { contract:weth9Contract} = await setupContract('weth9_contract','new','weth9','weth9');
    // pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
    const { contract:positionDescriptor} = await setupContract('NonfungibleTokenPositionDescriptor','new',weth9Contract.address,"_nativeCurrencyLabelBytes");
    // pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
    const { query,tx,alice,defaultSigner,contract:positionMangerContract } = await setupContract('NonfungiblePositionManager','new',factoryContract.address,weth9Contract.address,positionDescriptor.address);
    console.log("positionMangerContract address is:",positionMangerContract.address.toHuman());
    //TODO 开始测试初始化一个交易池.

    // const result = await query.balanceOf(defaultSigner.address);
    // console.log("result is:",result.output);
    // expect(result.output).to.equal(0);
    // var balance = await api.query.system.account(defaultSigner.address);
    // console.log("native balance is:",balance.toHuman());
    // //为用户收钱.
    // await tx.deposit({value:1000});
    // var balance = await api.query.system.account(defaultSigner.address);
    // console.log("native balance is:",balance.toHuman());
    // const resultAfterDefault = await query.balanceOf(defaultSigner.address);
    // console.log("resultAfterDefault is:",resultAfterDefault.output);
    // expect(resultAfterDefault.output).to.equal(1000);
    // await tx.withdraw(800);
    // const resultAfterWithdraw = await query.balanceOf(defaultSigner.address);
    // console.log("resultAfterDefault is:",resultAfterWithdraw.output?.toHuman());
    // expect(resultAfterWithdraw.output).to.equal(200);
  });

});