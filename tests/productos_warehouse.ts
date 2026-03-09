import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AduanaTracker } from "../target/types/aduana_tracker";

describe("aduana_tracker", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AduanaTracker as Program<AduanaTracker>;
  const wallet = provider.wallet as anchor.Wallet;

  const idAlmacen = "ALM-QRO-01";
  const skuProducto = "SKU-TEST-99";
  const ordenTrabajo = "OT-2026-03";

  const [almacenPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("almacen"), wallet.publicKey.toBuffer(), Buffer.from(idAlmacen)],
    program.programId
  );

  const [productoPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("producto"), wallet.publicKey.toBuffer(), Buffer.from(skuProducto)],
    program.programId
  );

  const [pedidoPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("pedido"), wallet.publicKey.toBuffer(), Buffer.from(ordenTrabajo)],
    program.programId
  );

  it("1. Construye el Almacén", async () => {
    const tx = await program.methods
      .crearAlmacen(idAlmacen, "Bodega Central", "Querétaro, MX")
      .accounts({
        almacen: almacenPda,
        gerente: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    
    console.log("\nAlmacén creado exitosamente. Firma:", tx);
  });

  it("2. Registra el Producto", async () => {
    const tx = await program.methods
      .registrarProducto(skuProducto, "Filtros de Aceite", "PED-IMPORT-2026")
      .accounts({
        producto: productoPda,
        almacen: almacenPda,
        gerente: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    
    console.log("Producto vinculado al almacén. Firma:", tx);
  });

  it("3. Registra una Entrada de 150 unidades", async () => {
    await program.methods
      .registrarEntrada(skuProducto, new anchor.BN(150))
      .accounts({
        producto: productoPda,
        gerente: wallet.publicKey,
      })
      .rpc();
    
    const productoData = await program.account.producto.fetch(productoPda);
    console.log("Stock actual on-chain:", productoData.cantidad.toString());
  });

  it("4. Despacha un Pedido a un Cliente", async () => {
    const tx = await program.methods
      .despacharPedido(ordenTrabajo, "Taller Mecánico Ruiz", new anchor.BN(50))
      .accounts({
        producto: productoPda,
        pedido: pedidoPda,
        gerente: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    
    const productoData = await program.account.producto.fetch(productoPda);
    console.log(`Pedido despachado. Recibo en blockchain: ${tx}`);
    console.log(`Stock restante después del pedido: ${productoData.cantidad.toString()}`);
  });

  it("5. Registra una Salida de Ajuste (100 unidades)", async () => {
    await program.methods
      .registrarSalida(skuProducto, new anchor.BN(100))
      .accounts({
        producto: productoPda,
        gerente: wallet.publicKey,
      })
      .rpc();
    
    const productoData = await program.account.producto.fetch(productoPda);
    console.log("Stock después de salida de ajuste:", productoData.cantidad.toString());
  });

  it("6. Elimina el Producto de la blockchain", async () => {
    const tx = await program.methods
      .eliminarProducto(skuProducto)
      .accounts({
        producto: productoPda,
        gerente: wallet.publicKey,
      })
      .rpc();
    
    console.log("Producto eliminado permanentemente de Solana. Firma:", tx);
  });
});
