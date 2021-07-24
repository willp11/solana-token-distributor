import { AccountLayout, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Account, Connection, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, SYSVAR_CLOCK_PUBKEY, Transaction, TransactionInstruction } from "@solana/web3.js";
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

    const connection = new Connection("https://devnet.solana.com", 'singleGossip');
    
    // Accounts expected:
    // 0. [signer] initializer (wallet)

    // 1. [writable] lockup schedule state (empty)
    const lockupScheduleStateAccount = new Account();
    const programId = new PublicKey(programIdString);
    const createLockupScheduleStateAccountIx = SystemProgram.createAccount({
        space: LOCKUP_SCHEDULE_ACCOUNT_DATA_LAYOUT.span,
        lamports: await connection.getMinimumBalanceForRentExemption(LOCKUP_SCHEDULE_ACCOUNT_DATA_LAYOUT.span, 'singleGossip'),
        fromPubkey: wallet.publicKey,
        newAccountPubkey: lockupScheduleStateAccount.publicKey,
        programId: programId
    });

    // 2. [] token mint
    const tokenMint = new PublicKey(tokenMintString);
    // 3. [] clock sysvar = SYSVAR_CLOCK_PUBKEY
    // 4. [] rent sysvar = SYSVAR_RENT_PUBKEY

    const startTimestampBytes = new BN(startTimestamp).toArray("le", 8);
    const unlockPeriodsBytes = new BN(unlockPeriods).toArray("le", 8);
    const periodDurationBytes = new BN(periodDuration).toArray("le", 8);
    const lockupQuantityBytes = new BN(lockupQuantity).toArray("le", 8);

    const data = Buffer.from(Uint8Array.of(0, ...startTimestampBytes, ...unlockPeriodsBytes, ...periodDurationBytes, ...lockupQuantityBytes));

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
    let signed = await wallet.signTransaction(tx);
    let txid = await connection.sendRawTransaction(signed.serialize());

    return txid;
}