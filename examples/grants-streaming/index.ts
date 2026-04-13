import { WaveSDK } from 'stellar-wave-sdk';

// Example: Releasing Grantees capital with a cliff
async function grantRelease() {
    const wave = new WaveSDK('CABC1234');
    
    await wave.createStream({
        streamId: "grant_x2",
        sender: "G_DAO_TREASURY",
        recipient: "G_BUILDER",
        token: "C_USDC_TOKEN_ID",
        totalAmount: 50_000n,
        startLedger: 2000,
        cliffLedger: 2000 + 43200, // 30 day cliff!
        endLedger: 2000 + 5256000 
    });

    console.log("Grant deployed with cliff!");
}

grantRelease();
