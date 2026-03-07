use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod chain_logistics {
    use super::*;

    pub fn registrar_paquete(
        context: Context<RegistrarPaquete>,
        id_paquete: String,
        nombre: String,
        destino: String,
    ) -> Result<()> {
        let paquete = &mut context.accounts.paquete;
        let gerente = &context.accounts.gerente;

        paquete.gerente = gerente.key();
        paquete.id_paquete = id_paquete;
        paquete.nombre = nombre;
        paquete.estado = String::from("En Almacén");
        paquete.destino = destino;

        msg!(
            "📦 Paquete {} registrado correctamente. Destino: {}",
            paquete.nombre,
            paquete.destino
        );
        Ok(())
    }

    pub fn actualizar_estado(
        context: Context<ModificarPaquete>,
        id_paquete: String,
        nuevo_estado: String,
    ) -> Result<()> {
        let paquete = &mut context.accounts.paquete;

        require!(
            paquete.gerente == context.accounts.gerente.key(),
            Errores::NoEresElGerente
        );

        paquete.estado = nuevo_estado.clone();

        msg!(
            "Estado del paquete {} actualizado a: {}",
            paquete.id_paquete,
            nuevo_estado
        );
        Ok(())
    }

    pub fn eliminar_paquete(
        context: Context<ModificarPaquete>,
        id_paquete: String,
    ) -> Result<()> {
        let paquete = &context.accounts.paquete;

        require!(
            paquete.gerente == context.accounts.gerente.key(),
            Errores::NoEresElGerente
        );

        msg!(
            "🗑️ El registro del paquete {} ha sido eliminado.",
            paquete.id_paquete
        );
        Ok(())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error: Solo el gerente que registró este paquete puede modificarlo.")]
    NoEresElGerente,
}

#[derive(InitSpace)]
pub struct PaqueteInventario {
    pub gerente: Pubkey,

    #[max_len(20)]
    pub id_paquete: String,

    #[max_len(50)]
    pub nombre: String,

    #[max_len(20)]
    pub estado: String,

    #[max_len(100)]
    pub destino: String,
}

#[derive(Accounts)]
#[instruction(id_paquete: String)]
pub struct RegistrarPaquete<'info> {
    #[account(mut)]
    pub gerente: Signer<'info>,

    #[account(
        init, 
        payer = gerente, 
        space = PaqueteInventario::INIT_SPACE + 8,
        seeds = [b"paquete", gerente.key().as_ref(), id_paquete.as_bytes()], 
        bump 
    )]
    pub paquete: Account<'info, PaqueteInventario>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id_paquete: String)]
pub struct ModificarPaquete<'info> {
    #[account(mut)]
    pub gerente: Signer<'info>,

    #[account(
        mut,
        seeds = [b"paquete", gerente.key().as_ref(), id_paquete.as_bytes()], 
        bump,
        close = gerente
    )]
    pub paquete: Account<'info, PaqueteInventario>,
}
