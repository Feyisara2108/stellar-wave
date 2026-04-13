import { WaveSDK } from 'stellar-wave-sdk';

// Example: Continuous Payroll Setup
async function runPayroll() {
    const wave = new WaveSDK('CABC1234');
    
    // Creating a 1 year long payroll stream for employee
    await wave.createStream({
        streamId: "payroll_001",
        sender: "G_EMPLOYER_COMPANY",
        recipient: "G_EMPLOYEE_WALLET",
        token: "C_USDC_TOKEN_ID",
        totalAmount: 120_000n, // $120k / year
        startLedger: 1000,
        cliffLedger: 1000, // No cliff
        endLedger: 1000 + 5256000 // Roughly 1 year in 6-second ledgers
    });

    console.log("Payroll initialized successfully.");
}

runPayroll();
