import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Connection, PublicKey,SYSVAR_CLOCK_PUBKEY, Transaction, TransactionInstruction } from "@solana/web3.js";
import BN from "bn.js";
import {LOCKUP_ACCOUNT_DATA_LAYOUT} from '../util/layout';

export const redeemTokens = async (
    programIdString,
    wallet,
    receivingTokenString,
    lockupScheduleStateString,
    lockupStateString,
    lockupTokenAccountString
) => {
    const connection = new Connection("http://localhost:8899", 'confirmed');
    const programId = new PublicKey(programIdString);

    // Accounts expected:
    // 0. [signer] token receiver's main Solana account - wallet
    // 1. [] lockup schedule state
    const lockupScheduleStatePubkey = new PublicKey(lockupScheduleStateString);
    // 2. [writable] lockup state
    const lockupStatePubkey = new PublicKey(lockupStateString);
    // 3. [writable] lockup token account
    const lockupTokenPubkey = new PublicKey(lockupTokenAccountString);
    // 4. [writable] receiving token account
    const receivingTokenPubkey = new PublicKey(receivingTokenString);
    // 5. [] program-derived-address (owns lockup token account)
    const PDA = await PublicKey.findProgramAddress([Buffer.from("tokenDistributor")], programId);
    // 6. [] token program - TOKEN_PROGRAM_ID
    // 7. [] clock sysvar - SYSVAR_CLOCK_PUBKEY

    const redeemTokensIx = new TransactionInstruction({
        programId,
        keys: [
            { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
            { pubkey: lockupScheduleStatePubkey, isSigner: false, isWritable: false },
            { pubkey: lockupStatePubkey, isSigner: false, isWritable: true },
            { pubkey: lockupTokenPubkey, isSigner: false, isWritable: true},
            { pubkey: receivingTokenPubkey, isSigner: false, isWritable: true},
            { pubkey: PDA[0], isSigner: false, isWritable: false},
            { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            { pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false}
        ],
        data: Buffer.from(Uint8Array.of(2))
    });

    const tx = new Transaction().add(
        redeemTokensIx
    );

    let { blockhash } = await connection.getRecentBlockhash();
    tx.recentBlockhash = blockhash;
    tx.feePayer = wallet.publicKey;
    let signed = await wallet.signTransaction(tx);
    let txid = await connection.sendRawTransaction(signed.serialize());

    await connection.confirmTransaction(txid);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    const encodedLockupState = (await connection.getAccountInfo(lockupStatePubkey, 'confirmed')).data;
    const decodedLockupState = LOCKUP_ACCOUNT_DATA_LAYOUT.decode(encodedLockupState);

    const lockupStateObj = {
        stateAccount: lockupStatePubkey.toBase58(),
        isInitialized: new BN(decodedLockupState.isInitialized, 10, "le").toNumber(),
        lockupScheduleState: new PublicKey(decodedLockupState.lockupScheduleState).toBase58(),
        receivingAccount: new PublicKey(decodedLockupState.receivingAccount).toBase58(),
        lockupTokenAccount: new PublicKey(decodedLockupState.lockupTokenAccount).toBase58(),
        tokenQuantity: new BN(decodedLockupState.tokenQuantity, 10, "le").toNumber(),
        periodsRedeemed: new BN(decodedLockupState.periodsRedeemed, 10, "le").toNumber()
    }

    return lockupStateObj;
}