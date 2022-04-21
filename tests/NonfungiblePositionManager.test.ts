import { expect } from 'chai';
import { artifacts, network, patract } from 'redspot';

const { getContractFactory, getRandomSigner } = patract;

const { api, getAddresses, getSigners } = network;

import { setupContract, fromSigner, setupProxy } from './helpers'

describe('positionManager initialize', () => {
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

  it('test initialize pool',async () =>{
    await api.isReady;
    const { query:factoryQuery,tx:factoryTx,contract:factoryContract,abi:factoryAbi} = await setupContract("factory","new");
    const { contract:weth9Contract} = await setupContract('weth9_contract','new','weth9','weth9');
    await setupContract('pool','new1');
    // pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
    const { contract:positionDescriptor} = await setupContract('NonfungibleTokenPositionDescriptor','new',weth9Contract.address,"_nativeCurrencyLabelBytes");
    // pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
    const { query:positionManagerQuery,tx:positionManagerTx,alice,defaultSigner,contract:positionMangerContract } = await setupContract('NonfungiblePositionManager','new',factoryContract.address,weth9Contract.address,positionDescriptor.address);
    console.log("positionMangerContract address is:",positionMangerContract.address.toHuman());
    const { contract:CHECoinContract} = await setupContract('stable_coin_contract','new',"CHE","CHE");
    const { contract:AAACoinContract} = await setupContract('stable_coin_contract','new',"AAA","AAA");
      // &mut self,token0: AccountId,token1: AccountId,fee: u32,sqrt_price_x96: Uint160,) -> Address 
      //fee 500,3000,10000
    console.log("positionManagerTx is:",positionManagerTx.createAndInitializePoolIfNecessary);
    console.log("CHECoinContract.address is:",CHECoinContract.address.toHuman());
    console.log("AAACoinContract.address is:",AAACoinContract.address.toHuman());
    let token0;
    let token1;
    if(CHECoinContract.address.toHuman()<AAACoinContract.address.toHuman()) {
      token0 = CHECoinContract.address;
      token1 = AAACoinContract.address;
    }else{
      token1 = CHECoinContract.address;
      token0 = AAACoinContract.address;
    }
    console.log("factoryContract.address is:",factoryContract.address.toHuman());
    // factory:Address,token0: Address, token1: Address, fee: Uint24, tick_spacing: Int24
    // const { abi:poolAbi} = await setupContract('pool','new',factoryContract.address,token0,token1,500,0);
    
    // console.log("poolAbi is:",poolAbi);
    // const pool_code_hash = (await poolAbi).source.hash;
    // await positionManagerTx.initial(pool_code_hash);
    // const poolCodeHash = await positionManagerQuery.getPoolCodeHash();
    // console.log("poolCodeHash is:",poolCodeHash);
    // const poolAddress = await positionManagerTx.createAndInitializePoolIfNecessary(token0,token1,500,1000000000000);
  });

});