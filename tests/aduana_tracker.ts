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

  const [almacenPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("almacen"), wallet.publicKey.toBuffer(), Buffer.from(idAlmacen)],
    program.programId
  );

  const [productoPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("producto"), wallet.publicKey.toBuffer(), Buffer.from(skuProducto)],
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
      .registrarProducto(skuProducto, "Laptops Dell XPS", "PED-2026-MX")
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

  it("4. Registra una Salida Total (150 unidades)", async () => {
    await program.methods
      .registrarSalida(skuProducto, new anchor.BN(150))
      .accounts({
        producto: productoPda,
        gerente: wallet.publicKey,
      })
      .rpc();
    
    const productoData = await program.account.producto.fetch(productoPda);
    console.log("Stock después de salida total:", productoData.cantidad.toString());
  });

  it("5. Elimina el Producto de la blockchain", async () => {
    const tx = await program.methods
      .eliminarProducto(skuProducto)
      .accounts({
        producto: productoPda,
        gerente: wallet.publicKey,
      })
      .rpc();
    
    console.log("🗑️ Producto eliminado permanentemente de Solana. Firma:", tx);
  });
});
