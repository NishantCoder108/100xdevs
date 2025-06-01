import * as anchor from "@coral-xyz/anchor";
import { Program, BN, web3 } from "@coral-xyz/anchor";
import { Staking } from "../target/types/staking";
import { assert } from "chai";

describe("staking", () => {
  // 1) Configure client + program
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.staking as Program<Staking>;
  const user = provider.wallet; // signer / fee payer

  // 2) Derive PDA (pdaAccount) once before all tests
  let pdaAccount: web3.PublicKey;
  let bump: number;
  before(async () => {
    [pdaAccount, bump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("client1"), user.publicKey.toBuffer()],
      program.programId
    );
  });

  it("1) Create PDA account", async () => {
    // call createPdaAccount({ payer, pdaAccount, systemProgram })
    await program.methods
      .createPdaAccount()
      .accounts({
        payer: user.publicKey,
        pdaAccount: pdaAccount,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    // fetch on‐chain StakeAccount data (account type is "stakeAccount")
    const acct = await program.account.stakeAccount.fetch(pdaAccount);

    assert.ok(acct.owner.equals(user.publicKey));
    assert.equal(acct.stakedAmount.toNumber(), 0);
    assert.equal(acct.totalPoints.toNumber(), 0);
    assert.equal(acct.bump, bump);
    // last_update_time is set to Clock::get().unix_timestamp but we won't assert exact value here
  });

  it("2) Stake 1 SOL (1_000_000_000 lamports)", async () => {
    const stakeAmount = new BN(1_000_000_000);

    // record PDA lamports before
    const beforeBal = await provider.connection.getBalance(pdaAccount);

    await program.methods
      .stake(stakeAmount)
      .accounts({
        user: user.publicKey,
        pdaAccount: pdaAccount,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    // fetch updated on‐chain data
    const acct = await program.account.stakeAccount.fetch(pdaAccount);
    assert.equal(acct.stakedAmount.toNumber(), stakeAmount.toNumber());

    const afterBal = await provider.connection.getBalance(pdaAccount);
    assert.isAbove(afterBal, beforeBal);
  });

  it("3) Unstake 0.5 SOL (500_000_000 lamports)", async () => {
    const unstakeAmount = new BN(500_000_000);

    // ensure there is at least 1 SOL staked
    const beforeAcct = await program.account.stakeAccount.fetch(pdaAccount);
    assert.equal(beforeAcct.stakedAmount.toNumber(), 1_000_000_000);

    await program.methods
      .unstake(unstakeAmount)
      .accounts({
        user: user.publicKey,
        pdaAccount: pdaAccount,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    const afterAcct = await program.account.stakeAccount.fetch(pdaAccount);
    assert.equal(
      afterAcct.stakedAmount.toNumber(),
      beforeAcct.stakedAmount.toNumber() - unstakeAmount.toNumber()
    );
  });

  it("4) Claim points (should be zero immediately)", async () => {
    // totalPoints is still 0 because no time has elapsed
    const beforeAcct = await program.account.stakeAccount.fetch(pdaAccount);
    assert.equal(beforeAcct.totalPoints.toNumber(), 0);

    // call claimPoints({ user, pdaAccount })
    await program.methods
      .claimPoints()
      .accounts({
        user: user.publicKey,
        pdaAccount: pdaAccount,
      })
      .rpc();

    // after calling, totalPoints resets to 0 again
    const afterAcct = await program.account.stakeAccount.fetch(pdaAccount);
    assert.equal(afterAcct.totalPoints.toNumber(), 0);
  });

  it("5) Get points (logs current points)", async () => {
    // just ensure getPoints() runs without error and logs zero
    const txSig = await program.methods
      .getPoints()
      .accounts({
        user: user.publicKey,
        pdaAccount: pdaAccount,
      })
      .rpc();

    assert.ok(txSig);
  });
});
