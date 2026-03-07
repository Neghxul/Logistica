use anchor_lang::prelude::*;

declare_id!("222h85kgey8WC7nbot7w8m44Y1ou6E7WZe68TsQChkX6"); 

#[program]
pub mod aduana_tracker {
    use super::*;

    pub fn crear_almacen(ctx: Context<CrearAlmacen>, id_almacen: String, nombre: String, ubicacion: String) -> Result<()> {
        let almacen = &mut ctx.accounts.almacen;
        almacen.gerente = ctx.accounts.gerente.key();
        almacen.id_almacen = id_almacen;
        almacen.nombre = nombre;
        almacen.ubicacion = ubicacion;
        msg!("🏢 Almacén registrado: {}", almacen.nombre);
        Ok(())
    }

    pub fn registrar_producto(ctx: Context<RegistrarProducto>, sku: String, nombre: String, pedimento: String) -> Result<()> {
        let producto = &mut ctx.accounts.producto;
        let almacen = &ctx.accounts.almacen;

        producto.gerente = ctx.accounts.gerente.key();
        producto.almacen_vinculado = almacen.key(); 
        producto.sku = sku;
        producto.nombre = nombre;
        producto.cantidad = 0; 
        producto.pedimento = pedimento;
        msg!("📦 Producto {} vinculado al almacén", producto.sku);
        Ok(())
    }

    pub fn registrar_entrada(ctx: Context<ManejarInventario>, _sku: String, cantidad: u64) -> Result<()> {
        let producto = &mut ctx.accounts.producto;
        producto.cantidad = producto.cantidad.checked_add(cantidad).unwrap();
        msg!("📥 Entrada exitosa. Stock actual: {}", producto.cantidad);
        Ok(())
    }

    pub fn registrar_salida(ctx: Context<ManejarInventario>, _sku: String, cantidad: u64) -> Result<()> {
        let producto = &mut ctx.accounts.producto;
        require!(producto.cantidad >= cantidad, Errores::StockInsuficiente);
        producto.cantidad = producto.cantidad.checked_sub(cantidad).unwrap();
        msg!("📤 Salida exitosa. Stock restante: {}", producto.cantidad);
        Ok(())
    }

    pub fn eliminar_producto(ctx: Context<EliminarProducto>, _sku: String) -> Result<()> {
        let producto = &ctx.accounts.producto;
        require!(producto.cantidad == 0, Errores::NoSePuedeBorrarConStock);
        msg!("🗑️ El registro del SKU {} ha sido eliminado del sistema.", producto.sku);
        Ok(())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error: No hay stock suficiente para esta salida.")]
    StockInsuficiente,
    #[msg("Error: No puedes eliminar un registro que aún tiene inventario.")]
    NoSePuedeBorrarConStock,
}

#[account]
#[derive(InitSpace)]
pub struct Almacen {
    pub gerente: Pubkey,
    #[max_len(20)]
    pub id_almacen: String,
    #[max_len(50)]
    pub nombre: String,
    #[max_len(100)]
    pub ubicacion: String,
}

#[account]
#[derive(InitSpace)]
pub struct Producto {
    pub gerente: Pubkey,
    pub almacen_vinculado: Pubkey, 
    #[max_len(20)]
    pub sku: String,
    #[max_len(50)]
    pub nombre: String,
    pub cantidad: u64,
    #[max_len(50)]
    pub pedimento: String,
}

#[derive(Accounts)]
#[instruction(id_almacen: String)]
pub struct CrearAlmacen<'info> {
    #[account(mut)]
    pub gerente: Signer<'info>,
    #[account(init, payer = gerente, space = 8 + Almacen::INIT_SPACE, seeds = [b"almacen", gerente.key().as_ref(), id_almacen.as_bytes()], bump)]
    pub almacen: Account<'info, Almacen>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(sku: String)]
pub struct RegistrarProducto<'info> {
    #[account(mut)]
    pub gerente: Signer<'info>,
    #[account(seeds = [b"almacen", gerente.key().as_ref(), almacen.id_almacen.as_bytes()], bump, has_one = gerente)]
    pub almacen: Account<'info, Almacen>,
    #[account(init, payer = gerente, space = 8 + Producto::INIT_SPACE, seeds = [b"producto", gerente.key().as_ref(), sku.as_bytes()], bump)]
    pub producto: Account<'info, Producto>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(sku: String)]
pub struct ManejarInventario<'info> {
    #[account(mut)]
    pub gerente: Signer<'info>,
    #[account(mut, seeds = [b"producto", gerente.key().as_ref(), sku.as_bytes()], bump, has_one = gerente)]
    pub producto: Account<'info, Producto>,
}

#[derive(Accounts)]
#[instruction(sku: String)]
pub struct EliminarProducto<'info> {
    #[account(mut)]
    pub gerente: Signer<'info>,
    #[account(mut, seeds = [b"producto", gerente.key().as_ref(), sku.as_bytes()], bump, has_one = gerente, close = gerente)]
    pub producto: Account<'info, Producto>,
}
