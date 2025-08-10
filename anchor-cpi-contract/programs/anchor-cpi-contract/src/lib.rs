use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("8Ft9e7UjZ2ctvi3CKiVievfPNinvtdHFDKCCgdp52sts");

#[program]
pub mod anchor_cpi_contract {
    use anchor_lang::solana_program::{instruction::Instruction, program::invoke};

    use super::*;

    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.sender.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey,
            },
        );

        transfer(cpi_context, amount)?;
        Ok(())
    }

    pub fn transfer_sol_generic(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.sender.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        // account metadatas
        let account_metas = vec![
            AccountMeta::new(from_pubkey.key(), true),
            AccountMeta::new(to_pubkey.key(), false),
        ];

        // NOTE: in this example, we're constructing instruction_data manually, but this can also be done using borsh!

        // in the system program instructions struct, transfer is the 2nd indexed instruction
        let instruction_discriminator: u32 = 2;

        // 4 for storing instruction_discriminator, 8 for storing actual instruction
        let mut instruction_data = Vec::with_capacity(4 + 8);
        // to left aligned bytes
        instruction_data.extend_from_slice(&instruction_discriminator.to_le_bytes());
        instruction_data.extend_from_slice(&amount.to_le_bytes());

        let instruction = Instruction {
            program_id: program_id.key(),
            accounts: account_metas,
            data: instruction_data,
        };

        invoke(&instruction, &[from_pubkey, to_pubkey, program_id])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>, // SystemAccount is an account owned by System program
    system_program: Program<'info, System>,
}
