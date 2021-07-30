import * as BufferLayout from "buffer-layout";

/**
 * Layout for a public key
 */
const publicKey = (property = "publicKey") => {
  return BufferLayout.blob(32, property);
};

/**
 * Layout for a 64bit unsigned value
 */
const uint64 = (property = "uint64") => {
  return BufferLayout.blob(8, property);
};

export const LOCKUP_SCHEDULE_ACCOUNT_DATA_LAYOUT = BufferLayout.struct([
    BufferLayout.u8("isInitialized"),
    publicKey("initializer"),
    publicKey("tokenMint"),
    uint64("startTimestamp"),
    uint64("numberPeriods"),
    uint64("periodDuration"),
    uint64("totalTokenQuantity"),
    uint64("tokenQuantityLocked")
  ]);

  export const LOCKUP_ACCOUNT_DATA_LAYOUT = BufferLayout.struct([
    BufferLayout.u8("isInitialized"),
    publicKey("lockupScheduleState"),
    publicKey("receivingAccount"),
    publicKey("lockupTokenAccount"),
    uint64("tokenQuantity"),
    uint64("periodsRedeemed")
  ]);