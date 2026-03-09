use anchor_lang::prelude::*;

declare_id!("GEZJ9bwD1WFG3f9wjkj2x5xZ2rmGT5AsGYP67wV4dePb"); 

#[program]
pub mod aduana_tracker {
    use super::*;

    pub fn crear_almacen(ctx: Context<CrearAlmacen>, id_almacen: String, nombre: String, ubicacion: String) -> Result<()> {
        let almacen = &mut ctx.accounts.almacen;
        almacen.gerente = ctx.accounts.gerente.key();
        almacen.id_almacen = id_almacen;
        almacen.nombre = nombre;
        almacen.ubicacion = ubicacion;
        msg!("Almacén registrado: {}", almacen.nombre);
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
        msg!("Producto {} vinculado al almacén", producto.sku);
        Ok(())
    }

    pub fn registrar_entrada(ctx: Context<ManejarInventario>, _sku: String, cantidad: u64) -> Result<()> {
        let producto = &mut ctx.accounts.producto;
        producto.cantidad = producto.cantidad.checked_add(cantidad).unwrap();
        msg!("Entrada exitosa. Stock actual: {}", producto.cantidad);
        Ok(())
    }

    pub fn despachar_pedido(ctx: Context<DespacharPedido>, orden_trabajo: String, cliente: String, cantidad: u64) -> Result<()> {
        let producto = &mut ctx.accounts.producto;
        let pedido = &mut ctx.accounts.pedido;

        require!(producto.cantidad >= cantidad, Errores::StockInsuficiente);
        
        producto.cantidad = producto.cantidad.checked_sub(cantidad).unwrap();

        pedido.gerente = ctx.accounts.gerente.key();
        pedido.producto_vinculado = producto.key();
        pedido.orden_trabajo = orden_trabajo;
        pedido.cliente = cliente;
        pedido.cantidad_despachada = cantidad;
        pedido.fecha_timestamp = Clock::get()?.unix_timestamp;

        msg!("Pedido {} despachado a {}. Unidades: {}", pedido.orden_trabajo, pedido.cliente, cantidad);
        Ok(())
    }

    pub fn registrar_salida(ctx: Context<ManejarInventario>, _sku: String, cantidad: u64) -> Result<()> {
        let producto = &mut ctx.accounts.producto;
        require!(producto.cantidad >= cantidad, Errores::StockInsuficiente);
        producto.cantidad = producto.cantidad.checked_sub(cantidad).unwrap();
        msg!("Ajuste de salida exitoso. Stock restante: {}", producto.cantidad);
        Ok(())
    }

    pub fn eliminar_producto(ctx: Context<EliminarProducto>, _sku: String) -> Result<()> {
        let producto = &ctx.accounts.producto;
        require!(producto.cantidad == 0, Errores::NoSePuedeBorrarConStock);
        msg!("El registro del SKU {} ha sido eliminado del sistema.", producto.sku);
        Ok(())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error: No hay stock suficiente para esta salida o pedido.")]
    StockInsuficiente,
    #[msg("Error: No puedes eliminar un registro que aún tiene inventario físico.")]
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

#[account]
#[derive(InitSpace)]
pub struct Pedido {
    pub gerente: Pubkey,
    pub producto_vinculado: Pubkey,
    #[max_len(30)]
    pub orden_trabajo: String,
    #[max_len(50)]
    pub cliente: String,
    pub cantidad_despachada: u64,
    pub fecha_timestamp: i64,
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
#[instruction(orden_trabajo: String)]
pub struct DespacharPedido<'info> {
    #[account(mut)]
    pub gerente: Signer<'info>,
    #[account(mut, seeds = [b"producto", gerente.key().as_ref(), producto.sku.as_bytes()], bump, has_one = gerente)]
    pub producto: Account<'info, Producto>,
    #[account(init, payer = gerente, space = 8 + Pedido::INIT_SPACE, seeds = [b"pedido", gerente.key().as_ref(), orden_trabajo.as_bytes()], bump)]
    pub pedido: Account<'info, Pedido>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(sku: String)]
pub struct EliminarProducto<'info> {
    #[account(mut)]
    pub gerente: Signer<'info>,
    #[account(mut, seeds = [b"producto", gerente.key().as_ref(), sku.as_bytes()], bump, has_one = gerente, close = gerente)]
    pub producto: Account<'info, Producto>,
}
