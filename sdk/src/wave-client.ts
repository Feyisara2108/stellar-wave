import {
  BASE_FEE,
  Contract,
  Networks,
  Transaction,
  TransactionBuilder,
  nativeToScVal,
  rpc
} from "@stellar/stellar-sdk";

export interface WalletAdapter {
  signTransaction(xdr: string, options: { networkPassphrase: string }): Promise<{ signedTxXdr: string }>;
}

export interface InvokeOptions {
  sourceAccount: string;
  networkPassphrase?: string;
  wallet?: WalletAdapter;
}

/**
 * Lightweight Soroban client for interacting with the wave contract.
 */
export class WaveClient {
  private readonly server: rpc.Server;
  private readonly contract: Contract;
  private readonly networkPassphrase: string;

  constructor(rpcUrl: string, contractId: string, networkPassphrase = Networks.TESTNET) {
    this.server = new rpc.Server(rpcUrl);
    this.contract = new Contract(contractId);
    this.networkPassphrase = networkPassphrase;
  }

  /**
   * Build an unsigned transaction for claim(stream_id, recipient).
   */
  async buildClaimTx(streamId: bigint, recipient: string, options: InvokeOptions): Promise<string> {
    const account = await this.server.getAccount(options.sourceAccount);
    const operation = this.contract.call("claim", nativeToScVal(streamId, { type: "u64" }), nativeToScVal(recipient, { type: "address" }));
    const tx = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: options.networkPassphrase ?? this.networkPassphrase
    })
      .addOperation(operation)
      .setTimeout(30)
      .build();

    return tx.toXDR();
  }

  /**
   * Sign and submit any prepared XDR transaction.
   */
  async signAndSend(preparedXdr: string, options: InvokeOptions): Promise<rpc.Api.SendTransactionResponse> {
    if (!options.wallet) {
      throw new Error("wallet adapter is required for signing");
    }

    const signed = await options.wallet.signTransaction(preparedXdr, {
      networkPassphrase: options.networkPassphrase ?? this.networkPassphrase
    });

    const tx = TransactionBuilder.fromXDR(
      signed.signedTxXdr,
      options.networkPassphrase ?? this.networkPassphrase
    ) as Transaction;
    return this.server.sendTransaction(tx);
  }
}
