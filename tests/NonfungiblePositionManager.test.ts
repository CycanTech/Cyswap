import BN from 'bn.js'
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
    const {defaultSigner:defaultSigner1,alice:alice1, query:factoryQuery,tx:factoryTx,contract:factoryContract,abi:factoryAbi} = await setupContract("factory","new");
    console.log("factory is:",1);
    console.log("defaultSigner is:",defaultSigner1.address.toString());
    console.log("alice1 is:",alice1.address.toString());
    const { contract:weth9Contract} = await setupContract('weth9_contract','new','weth9','weth9');
    console.log("factory is:",2);
    // pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
    const { contract:positionDescriptor} = await setupContract('NonfungibleTokenPositionDescriptor','new',weth9Contract.address,"_nativeCurrencyLabelBytes");
    console.log("factory is:",3);
    // pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
    const { query:positionManagerQuery,tx:positionManagerTx,alice,defaultSigner,contract:positionMangerContract } = await setupContract('NonfungiblePositionManager','new',factoryContract.address,weth9Contract.address,positionDescriptor.address,{value:1000000000});
    console.log("factory is:",4);
    const { contract:CHECoinContract} = await setupContract('stable_coin_contract','new',"CHE","CHE");
    const { contract:AAACoinContract} = await setupContract('stable_coin_contract','new',"AAA","AAA");
      // &mut self,token0: AccountId,token1: AccountId,fee: u32,sqrt_price_x96: Uint160,) -> Address 
      //fee 500,3000,10000
    console.log("positionMangerContract.address is:",positionMangerContract.address.toHuman());
    
    let token0;
    let token1;
    if(CHECoinContract.address.toHuman()<AAACoinContract.address.toHuman()) {
      token0 = CHECoinContract.address;
      token1 = AAACoinContract.address;
    }else{
      token1 = CHECoinContract.address;
      token0 = AAACoinContract.address;
    }
    console.log("token0.address is:",token0.toHuman());
    console.log("token1.address is:",token1.toHuman());
    console.log("factoryContract.address is:",factoryContract.address.toHuman());
    // factory:Address,token0: Address, token1: Address, fee: Uint24, tick_spacing: Int24
    const { abi:poolAbi} = await setupContract('pool','new',factoryContract.address,token0,token1,500,0);
    
    
    var pool_code_hash = (await poolAbi).source.hash;
    console.log("pool_code_hash is:",pool_code_hash);
    // pool_code_hash = pool_code_hash.substring(2);
    console.log("pool_code_hash is:",pool_code_hash);
    
    await factoryTx.initial(pool_code_hash);
    const poolCodeHash = await factoryQuery.getPoolCodeHash();
    console.log("poolCodeHash is:",poolCodeHash.output?.toHuman());
    // &mut self,fee:u32,token_a:Address,token_b:Address
    // var poolAddress = await factoryTx.createPool(500,token0,token1);
    await positionMangerContract.connect(alice);
    await expect(positionManagerTx.testEvent()).to.emit(positionMangerContract,"TestEvent")
    .withArgs(1);
    await positionManagerTx.createAndInitializePoolIfNecessary(token0,token1,500,1000000000000);
    // await expect(positionManagerTx.createAndInitializePoolIfNecessary(token0,token1,500,1000000000000))
    // .to.emit(factoryContract,"PoolCreated")
    // .withArgs(token0,token1,500,10,"0x111");

    // console.log("poolAddress is:",poolAddress.txHash?.toString());
  });

  it('test position manager mint',async () =>{
    await api.isReady;
    const {defaultSigner:defaultSigner1,alice:alice1, query:factoryQuery,tx:factoryTx,contract:factoryContract,abi:factoryAbi} = await setupContract("factory","new");
    console.log("factory is:",1);
    console.log("defaultSigner is:",defaultSigner1.address.toString());
    console.log("alice1 is:",alice1.address.toString());
    const { contract:weth9Contract} = await setupContract('weth9_contract','new','weth9','weth9');
    console.log("factory is:",2);
    // pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
    const { contract:positionDescriptor} = await setupContract('NonfungibleTokenPositionDescriptor','new',weth9Contract.address,"_nativeCurrencyLabelBytes");
    console.log("factory is:",3);
    // pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
    const { query:positionManagerQuery,tx:positionManagerTx,alice,defaultSigner,contract:positionMangerContract } = await setupContract('NonfungiblePositionManager','new',factoryContract.address,weth9Contract.address,positionDescriptor.address,{value:1000000000});
    console.log("factory is:",4);
    const { contract:CHECoinContract} = await setupContract('stable_coin_contract','new',"CHE","CHE");
    const { contract:AAACoinContract} = await setupContract('stable_coin_contract','new',"AAA","AAA");
      // &mut self,token0: AccountId,token1: AccountId,fee: u32,sqrt_price_x96: Uint160,) -> Address 
      //fee 500,3000,10000
    console.log("positionMangerContract.address is:",positionMangerContract.address.toHuman());
    
    let token0;
    let token1;
    if(CHECoinContract.address.toHuman()<AAACoinContract.address.toHuman()) {
      token0 = CHECoinContract.address;
      token1 = AAACoinContract.address;
    }else{
      token1 = CHECoinContract.address;
      token0 = AAACoinContract.address;
    }
    console.log("token0.address is:",token0.toHuman());
    console.log("token1.address is:",token1.toHuman());
    console.log("factoryContract.address is:",factoryContract.address.toHuman());
    console.log("positionMangerContract.address is:",positionMangerContract.address);
    // factory:Address,token0: Address, token1: Address, fee: Uint24, tick_spacing: Int24
    const { abi:poolAbi} = await setupContract('pool','new',factoryContract.address,token0,token1,500,0);
    
    
    var pool_code_hash = (await poolAbi).source.hash;
    console.log("pool_code_hash is:",pool_code_hash);
    // pool_code_hash = pool_code_hash.substring(2);
    
    await factoryTx.initial(pool_code_hash);
    const poolCodeHash = await factoryQuery.getPoolCodeHash();
    console.log("poolCodeHash is:",poolCodeHash.output?.toHuman());
    // &mut self,fee:u32,token_a:Address,token_b:Address
    // var poolAddress = await factoryTx.createPool(500,token0,token1);
    await positionMangerContract.connect(alice);
    console.log("@@@@@@@@@@@@@@@@@@@@@0");
    await expect(positionManagerTx.testEvent()).to.emit(positionMangerContract,"TestEvent")
    .withArgs(1);
    console.log("@@@@@@@@@@@@@@@@@@@@@1");
    await positionManagerTx.createAndInitializePoolIfNecessary(token0,token1,500,1000000000000,{value:1000000000});
    console.log("@@@@@@@@@@@@@@@@@@@@@2");
  //   pub struct MintParams {
  //     pub token0: Address,
  //     pub token1: Address,
  //     pub fee: Uint24,
  //     pub tickLower: Int24,
  //     pub tickUpper: Int24,
  //     pub amount0Desired: Uint256,
  //     pub amount1Desired: Uint256,
  //     pub amount0Min: Uint256,
  //     pub amount1Min: Uint256,
  //     pub recipient: Address,
  //     pub deadline: Uint256,
  // }
    // var mintParams = {
    //   token0:token0,
    //   token1:token1,
    //   fee:500,
    //   tickLower:100,
    //   tickUpper:100000,
    //   amount0Desired:1000,
    //   amount1Desired:1000,
    //   amount0Min:100,
    //   amount1Min:100,
    //   recipient:alice,
    //   deadline:10,
    // };
    // console.log("mintParams:",mintParams);
    await positionManagerTx.mint(token0,token1,500,100,10000,1000,1000,100,100,alice,10);
    console.log("@@@@@@@@@@@@@@@@@@@@@3");
    // await expect(positionManagerTx.createAndInitializePoolIfNecessary(token0,token1,500,1000000000000))
    // .to.emit(factoryContract,"PoolCreated")
    // .withArgs(token0,token1,500,10,"0x111");

    // console.log("poolAddress is:",poolAddress.txHash?.toString());
  });

});