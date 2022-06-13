// import { expect } from './expect'
// import { BigNumber, ContractAbi, ContractFunction, PopulatedTransaction, TransactionResponse } from '@redspot/patract/types';

// export default async function snapshotGasCost(
//   x:
//     | TransactionResponse
//     | Promise<TransactionResponse>
// ): Promise<void> {
//   const resolved = await x
//   if ('deployTransaction' in resolved) {
//     const receipt = await resolved.deployTransaction.wait()
//     expect(receipt.gasUsed.toNumber()).toMatchSnapshot()
//   } else if ('wait' in resolved) {
//     const waited = await resolved.wait()
//     expect(waited.gasUsed.toNumber()).toMatchSnapshot()
//   } else if (BigNumber.isBigNumber(resolved)) {
//     expect(resolved.toNumber()).toMatchSnapshot()
//   }
// }
