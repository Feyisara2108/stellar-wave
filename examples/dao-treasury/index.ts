import { WaveSDK } from 'stellar-wave-sdk';

async function distributeTreasury() {
   const wave = new WaveSDK('CABC1234');
   await wave.createStream({
        streamId: "treasury_allocation",
        sender: "G_DAO_TREASURY",
        recipient: "G_MARKETING_POD",
        token: "C_USDC_TOKEN_ID",
        totalAmount: 250_000n,
        startLedger: 5000,
        cliffLedger: 5000, // Immediate vesting start
        endLedger: 5000 + 1000000 
    });
   console.log("Treasury stream active.");
}
distributeTreasury();
