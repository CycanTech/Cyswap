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
    const { query:factoryQuery,tx:factoryTx,contract:factoryContract,abi:factoryAbi} = await setupContract("factory","new");
    const { contract:weth9Contract} = await setupContract('weth9_contract','new','weth9','weth9');
    // pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
    const { contract:positionDescriptor} = await setupContract('NonfungibleTokenPositionDescriptor','new',weth9Contract.address,"_nativeCurrencyLabelBytes");
    // pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
    const { query:positionManagerQuery,tx:positionManagerTx,alice,defaultSigner,contract:positionMangerContract } = await setupContract('NonfungiblePositionManager','new',factoryContract.address,weth9Contract.address,positionDescriptor.address);
    console.log("positionMangerContract address is:",positionMangerContract.address.toHuman());
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
    var poolAddress = await positionManagerTx.createAndInitializePoolIfNecessary(token0,token1,500,1000000000000);

    // console.log("poolAddress is:",poolAddress);
  });

});