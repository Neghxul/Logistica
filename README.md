# Sistema de Inventario y Logística de Refacciones (Solana Smart Contract)

Este proyecto es un contrato inteligente descentralizado construido en la blockchain de Solana utilizando el framework **Anchor**. Actúa como el backend inmutable para un sistema de gestión de almacenes de refacciones, permitiendo la trazabilidad absoluta de piezas mecánicas, desde su registro aduanal hasta su despacho final.

La arquitectura está diseñada para integrarse como una API descentralizada con aplicaciones móviles, permitiendo que el escaneo de códigos QR en el mundo físico actualice el inventario en la blockchain en tiempo real.



## Funcionalidades Principales

El Smart Contract maneja cuatro pilares logísticos mediante Program Derived Addresses (PDAs) para máxima seguridad:

1. **Gestión de Almacenes (`crear_almacen`):**
   * Registra sucursales o bodegas físicas en la blockchain.
   * Almacena datos clave: ID único, Nombre y Ubicación.
2. **Catálogo de Refacciones (`registrar_producto` y `eliminar_producto`):**
   * Vincula cada pieza (SKU) a un almacén específico.
   * Registra si la pieza es de importación mediante el campo `pedimento`.
   * Permite eliminar el registro de piezas descontinuadas (solo si el stock físico es 0).
3. **Control de Inventario (`registrar_entrada` y `registrar_salida`):**
   * Incrementa o reduce el stock disponible para ajustes de inventario o mermas.
   * Validaciones estrictas: El contrato impide matemáticamente las salidas si el stock es insuficiente, bloqueando transacciones inválidas.
4. **Despacho de Pedidos (`despachar_pedido`):**
   * Emite un "recibo inmutable" en la blockchain.
   * Descuenta automáticamente el stock del almacén.
   * Genera una cuenta permanente que vincula la Orden de Trabajo, el Cliente, la cantidad despachada y un timestamp (fecha/hora exacta de la transacción).

## Estructura de Cuentas (State)

El programa utiliza tres estructuras principales de almacenamiento:
* `Almacen`: Guarda la información de la bodega y el `Pubkey` del gerente responsable.
* `Producto`: Representa la refacción física. Está vinculada criptográficamente a su `Almacen` padre.
* `Pedido`: El registro histórico de una entrega. Vinculado al `Producto` y firmado por el gerente.

## Tecnologías Utilizadas

* **Blockchain:** Solana
* **Lenguaje:** Rust
* **Framework:** Anchor (v0.30+)
* **Pruebas (Testing):** TypeScript

## Cómo ejecutar este proyecto (Solana Playground)

Para probar este Smart Contract sin necesidad de configurar un entorno local complejo, puedes utilizar [Solana Playground](https://beta.solpg.io/).

1. **Importar el proyecto:** Crea un nuevo proyecto en Solana Playground seleccionando el framework "Anchor".
2. **Pegar el código:** Copia el contenido del archivo principal en `src/lib.rs`.
3. **Configurar las pruebas:** Copia el flujo de testing en `tests/anchor.test.ts`.
4. **Construir y Desplegar:**
   * Abre la terminal integrada del Playground.
   * Ejecuta `build` para compilar el código Rust.
   * Ejecuta `deploy` para lanzar el contrato a la red Devnet.
5. **Ejecutar Pruebas:**
   * Utiliza la "Playground Wallet" (billetera interna) para evitar bloqueos del navegador.
   * Ejecuta `test` en la terminal para simular el flujo logístico completo (Creación de almacén -> Registro de pieza -> Entradas -> Despacho a cliente -> Eliminación).

---
*Construido para revolucionar la trazabilidad en la cadena de suministro.*
