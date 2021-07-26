import { AccountLayout, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, Connection, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, SYSVAR_CLOCK_PUBKEY, Transaction, TransactionInstruction } from "@solana/web3.js";
import BN from "bn.js";
import {LOCKUP_ACCOUNT_DATA_LAYOUT} from '../util/layout';

export const lockTokens = async (
    programIdString,
    wallet,
    lockupScheduleStatePubkeyString,
    tokenMint,
    initializerTokenAccount,
    receiverPubkeyString,
    quantity
) => {
    const connection = new Connection("http://localhost:8899", 'confirmed');

    const programId = new PublicKey(programIdString);

    // Accounts expected:
    // 0. [signer] initializer - wallet
    // 1. [writable] lockup schedule state
    const lockupScheduleStatePubkey = new PublicKey(lockupScheduleStatePubkeyString);
    // 2. [writable] lockup state account (create new)
    const lockupStateAccount = new Keypair();
    const createLockupStateAccountIx = SystemProgram.createAccount({
        space: LOCKUP_ACCOUNT_DATA_LAYOUT.span,
        lamports: await connection.getMinimumBalanceForRentExemption(LOCKUP_ACCOUNT_DATA_LAYOUT.span, 'confirmed'),
        fromPubkey: wallet.publicKey,
        newAccountPubkey: lockupStateAccount.publicKey,
        programId: programId
    });
    // 3. [] token receiver main Solana account
    const receiverPubkey = new PublicKey(receiverPubkeyString);
    // 4. [writable] temporary lockup token account (create new)
    const tempTokenAccount = new Keypair();
    const createTempTokenAccountIx = SystemProgram.createAccount({
        programId: TOKEN_PROGRAM_ID,
        space: AccountLayout.span,
        lamports: await connection.getMinimumBalanceForRentExemption(AccountLayout.span, 'singleGossip'),
        fromPubkey: wallet.publicKey,
        newAccountPubkey: tempTokenAccount.publicKey
    });
    // initialize temp token account
    const tokenMintAccountPubkey = new PublicKey(tokenMint);
    const initTempAccountIx = Token.createInitAccountInstruction(TOKEN_PROGRAM_ID, tokenMintAccountPubkey, tempTokenAccount.publicKey, wallet.publicKey);
    // send tokens to temp account
    const initializerTokenPubkey = new PublicKey(initializerTokenAccount);
    const transferTokensToTempAccIx = Token.createTransferInstruction(TOKEN_PROGRAM_ID, initializerTokenPubkey, tempTokenAccount.publicKey, wallet.publicKey, [], quantity);

    // 5. [] token program (transfer ownership of temp token account to PDA) - TOKEN_PROGRAM_ID
    // 6. [] clock sysvar - SYSVAR_CLOCK_PUBKEY
    // 7. [] rent sysvar - SYSVAR_RENT_PUBKEY

    const quantityBytes = new BN(quantity).toArray("le", 8);

    const data = Buffer.from(Uint8Array.of(1, ...quantityBytes));

    const createLockupIx = new TransactionInstruction({
        programId: programId,
        keys: [
            { pubkey: wallet.publicKey, isSigner: true, isWritable: false },
            { pubkey: lockupScheduleStatePubkey, isSigner: false, isWritable: true },
            { pubkey: lockupStateAccount.publicKey, isSigner: false, isWritable: true },
            { pubkey: receiverPubkey, isSigner: false, isWritable: false },
            { pubkey: tempTokenAccount.publicKey, isSigner: false, isWritable: true },
            { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            { pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false},
            { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false }
        ],
        data: data
    });

    const tx = new Transaction().add(
        createLockupStateAccountIx, 
        createTempTokenAccountIx,
        initTempAccountIx,
        transferTokensToTempAccIx,
        createLockupIx
    );

    let { blockhash } = await connection.getRecentBlockhash();
    tx.recentBlockhash = blockhash;
    tx.feePayer = wallet.publicKey;
    let partialSigners = [lockupStateAccount, tempTokenAccount];
    tx.partialSign(...partialSigners);
    let signed = await wallet.signTransaction(tx);
    let txid = await connection.sendRawTransaction(signed.serialize());

    await connection.confirmTransaction(txid);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    const encodedLockupState = (await connection.getAccountInfo(lockupStateAccount.publicKey, 'confirmed')).data;
    const decodedLockupState = LOCKUP_ACCOUNT_DATA_LAYOUT.decode(encodedLockupState);

    const lockupScheduleStateObj = {
        stateAccount: lockupStateAccount.publicKey.toBase58(),
        isInitialized: new BN(decodedLockupState.isInitialized, 10, "le").toNumber(),
        lockupScheduleState: new PublicKey(decodedLockupState.lockupScheduleState).toBase58(),
        receivingAccount: new PublicKey(decodedLockupState.receivingAccount).toBase58(),
        lockupTokenAccount: new PublicKey(decodedLockupState.lockupTokenAccount).toBase58(),
        tokenQuantity: new BN(decodedLockupState.tokenQuantity, 10, "le").toNumber(),
        periodsRedeemed: new BN(decodedLockupState.periodsRedeemed, 10, "le").toNumber()
    }

    return lockupScheduleStateObj;
}