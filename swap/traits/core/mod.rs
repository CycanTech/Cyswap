pub mod factory;
pub mod pool;
//由于ink!合约的初始化合约的方式修改了.此处不需要使用pool_deployer的方式来部署,故pool_deployer废弃不用.
// pub mod pool_deployer;