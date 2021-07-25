import { AccountLayout, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, Connection, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, SYSVAR_CLOCK_PUBKEY, Transaction, TransactionInstruction } from "@solana/web3.js";
import BN from "bn.js";
import {LOCKUP_SCHEDULE_ACCOUNT_DATA_LAYOUT} from '../util/layout';

export const createLockupSchedule = async (
    programIdString,
    wallet,
    startTimestamp,
    unlockPeriods,
    periodDuration,
    lockupQuantity,
    tokenMintString
) => {

    const connection = new Connection("https://api.devnet.solana.com", 'confirmed');
    
    // Accounts expected:
    // 0. [signer] initializer (wallet)
    // 1. [writable] lockup schedule state (empty)
    // 2. [] token mint
    // 3. [] clock sysvar = SYSVAR_CLOCK_PUBKEY
    // 4. [] rent sysvar = SYSVAR_RENT_PUBKEY

    const lockupScheduleStateAccount = new Keypair();
    const programId = new PublicKey(programIdString);
    const createLockupScheduleStateAccountIx = SystemProgram.createAccount({
        space: LOCKUP_SCHEDULE_ACCOUNT_DATA_LAYOUT.span,
        lamports: await connection.getMinimumBalanceForRentExemption(LOCKUP_SCHEDULE_ACCOUNT_DATA_LAYOUT.span, 'confirmed'),
        fromPubkey: wallet.publicKey,
        newAccountPubkey: lockupScheduleStateAccount.publicKey,
        programId: programId
    });
    const tokenMint = new PublicKey(tokenMintString);

    console.log(programIdString, programId);
    console.log(tokenMintString, tokenMint);

    const startTimestampBytes = new BN(startTimestamp).toArray("le", 8);
    const unlockPeriodsBytes = new BN(unlockPeriods).toArray("le", 8);
    const periodDurationBytes = new BN(periodDuration).toArray("le", 8);
    const lockupQuantityBytes = new BN(lockupQuantity).toArray("le", 8);

    const data = Buffer.from(Uint8Array.of(0, ...startTimestampBytes, ...unlockPeriodsBytes, ...periodDurationBytes, ...lockupQuantityBytes));

    console.log(data);

    const createLockupScheduleIx = new TransactionInstruction({
        programId: programId,
        keys: [
            { pubkey: wallet.publicKey, isSigner: true, isWritable: false },
            { pubkey: lockupScheduleStateAccount.publicKey, isSigner: false, isWritable: true },
            { pubkey: tokenMint, isSigner: false, isWritable: false },
            { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            { pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false }
        ],
        data: data
    });

    const tx = new Transaction().add(
        createLockupScheduleStateAccountIx, 
        createLockupScheduleIx
    );

    let { blockhash } = await connection.getRecentBlockhash();
    tx.recentBlockhash = blockhash;
    tx.feePayer = wallet.publicKey;
    tx.partialSign(lockupScheduleStateAccount);
    let signed = await wallet.signTransaction(tx);
    console.log(signed.verifySignatures());
    let txid = await connection.sendRawTransaction(signed.serialize());

    await connection.confirmTransaction(txid);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    const encodedLockupScheduleState = (await connection.getAccountInfo(lockupScheduleStateAccount.publicKey, 'confirmed')).data;
    const decodedLockupScheduleState = LOCKUP_SCHEDULE_ACCOUNT_DATA_LAYOUT.decode(encodedLockupScheduleState);

    const lockupScheduleStateObj = {
        isInitialized: new BN(decodedLockupScheduleState.isInitialized, 10, "le").toNumber(),
        tokenMint: new PublicKey(decodedLockupScheduleState.tokenMint).toBase58(),
        startTimestamp: new BN(decodedLockupScheduleState.startTimestamp, 10, "le").toNumber(),
        numberPeriods: new BN(decodedLockupScheduleState.numberPeriods, 10, "le").toNumber(),
        periodDuration: new BN(decodedLockupScheduleState.periodDuration, 10, "le").toNumber(),
        totalTokenQuantity: new BN(decodedLockupScheduleState.totalTokenQuantity, 10, "le").toNumber(),
        tokenQuantityLocked: new BN(decodedLockupScheduleState.tokenQuantityLocked, 10, "le").toNumber()
    }

    return lockupScheduleStateObj;
}