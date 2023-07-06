use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;
use solana_program::sysvar::instructions::{ID as IX_ID, load_instruction_at_checked};
use solana_program::ed25519_program::{ID as ED25519_ID};
use std::convert::TryInto;
use anchor_lang::solana_program::ed25519_program::ID as ED25519_PROGRAM_ID;

// use anchor_lang::solana_program::error::ErrorCode;
// use anchor_lang::solana_program::error;
declare_id!("GTUsCbZ9e3doYtwcGpgZfSTpWtk2drV34kZNTb7MApcw");

#[program]
pub mod ed25519_poc {
    use super::*;
    const EXPECTED_PUBLIC_KEY_OFFSET: usize = 16;
    const EXPECTED_PUBLIC_KEY_RANGE: std::ops::Range<usize> =
        EXPECTED_PUBLIC_KEY_OFFSET..(EXPECTED_PUBLIC_KEY_OFFSET + 32);
    const EXPTECED_IX_SYSVAR_INDEX: usize = 0;
    pub fn verify_ed25519(
        ctx: Context<Verify>, 
        pubkey: [u8; 32], 
        msg: Vec<u8>, 
        sig: [u8; 64]) -> Result<()> {
        // Get what should be the Ed25519Program instruction
        // msg!("{}" , pubkey);
        // msg!("{}" , msg);
        // msg!("{}" , sig);
     
        let ix: Instruction = load_instruction_at_checked(
            EXPTECED_IX_SYSVAR_INDEX,
            &ctx.accounts.ix_sysvar,
        )?;
        
        // let pubkey:&[u8] = pubkey.as_bytes();
        // let msg:&[u8] = msg.as_bytes();
        // let sig: &[u8] = sig.as_bytes();
        // Check that ix is what we expect to have been sent
       //let is_true =  utils::verify_ed25519_ix(&ix, &pubkey, &msg, &sig);
       let is_true =  utils::verify_ed25519_ix(&ix);
       msg!("is true {}" , is_true);
       if !is_true { 
        msg!("line number 38 executed");
        return Err(MyError::SigVerificationFailed.into());
           }
        msg!(" you can see msg macro when message varify{}");
        // Do other stuff
        let pub_key = Pubkey::new(&ix.data[EXPECTED_PUBLIC_KEY_RANGE]);
        let order = &ix.data[112..];
       // return Ok((pub_key, order.to_vec()));
        Ok(())
    }
   
}
pub mod utils { 
    use super::*;
    const EXPECTED_PUBLIC_KEY_OFFSET: usize = 16;
    const EXPECTED_PUBLIC_KEY_RANGE: std::ops::Range<usize> =
        EXPECTED_PUBLIC_KEY_OFFSET..(EXPECTED_PUBLIC_KEY_OFFSET + 32);
    const EXPTECED_IX_SYSVAR_INDEX: usize = 0;
    /// Verify Ed25519Program instruction fields
    //ix: &Instruction, pubkey: &[u8], msg: &[u8], sig: &[u8]
    pub fn verify_ed25519_ix(ix: &anchor_lang::solana_program::instruction::Instruction) -> bool {
      msg!("program id {}" , ix.program_id);
      msg!("program id {}" , ED25519_ID);
      msg!("accounts len{}" , ix.accounts.len());
      if ix.program_id != ED25519_PROGRAM_ID || ix.accounts.len() != 0 {
        msg!("line number 57 executed");
        return false;
    }
    let ix_data = &ix.data;
    let public_key_offset = &ix_data[6..=7];
    let exp_public_key_offset = u64::try_from(EXPECTED_PUBLIC_KEY_OFFSET)
        .unwrap()
        .to_le_bytes();
    let expected_num_signatures: u8 = 1;
    return public_key_offset       == &exp_public_key_offset                        && // pulic_key in expected offset (16)
        &[ix_data[0]]           == &expected_num_signatures.to_le_bytes()        && // num_signatures is 1
        &[ix_data[1]]           == &[0]                                          && // padding is 0
        &ix_data[4..=5]         == &u16::MAX.to_le_bytes()                       && // signature_instruction_index is not defined by user (default value)
        &ix_data[8..=9]         == &u16::MAX.to_le_bytes()                       && // public_key_instruction_index is not defined by user (default value)
        &ix_data[14..=15]       == &u16::MAX.to_le_bytes(); // message_instruction_index is not defined by user (default value)
    }
 /// Verify serialized Ed25519Program instruction data
       pub fn check_ed25519_data(data: &[u8], pubkey: &[u8], msg: &[u8], sig: &[u8]) -> bool {
        msg!("line number 65");
        
        let public_key_offset = &data[6..=7];

        let exp_public_key_offset = u64::try_from(EXPECTED_PUBLIC_KEY_OFFSET)
            .unwrap()
            .to_le_bytes();
            let expected_num_signatures: u8 = 1;

            // if  public_key_offset != &exp_public_key_offset { 
            //     msg!("line number 74 ");
            //     return Err(MyError::SigVerificationFailed.into());
            // }
         //let num_signatures             = &[data[0]];        // Byte  0
     

         return public_key_offset       == &exp_public_key_offset
    
    }
}
#[derive(Accounts)]
pub struct Initialize {}

 #[derive(Accounts)]
 pub struct Verify<'info> { 
    pub sender: Signer<'info>,
    #[account(address = IX_ID)]
    ///CHECK: `doc comment explaining why no checks through types are necessary 
    pub ix_sysvar: AccountInfo<'info>,
 }

  #[error_code]
  pub enum MyError { 
    #[msg("Signature varification failed")]
    SigVerificationFailed
  }