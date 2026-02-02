# Emulatore in Rust di CHIP-8

## 1. Introduzione

### 1.1 Descrizione del Progetto

Il progetto consiste nell'implementazione di un emulatore per CHIP-8, un linguaggio interpretato sviluppato originariamente a metà degli anni '70 per microcomputer come il COSMAC VIP e l'ETI 660. CHIP-8 permetteva di scrivere videogiochi in modo portabile grazie a una macchina virtuale semplice ma efficace.

L'obiettivo di questo progetto è emulare l'architettura hardware sottostante utilizzando il linguaggio Rust, sfruttando le sue garanzie di sicurezza della memoria e il suo sistema di tipi per creare un interprete robusto e performante.

### 1.2 Obiettivi

- Implementare il ciclo Fetch-Decode-Execute della CPU CHIP-8.
- Gestire la memoria, i registri e lo stack secondo le specifiche originali.
- Realizzare un sistema di rendering grafico utilizzando il crate minifb.
- Gestire l'input utente mappando il tastierino esadecimale originale su una tastiera moderna.

## 2. Architettura del Sistema

L'emulatore è stato progettato seguendo un approccio modulare orientato agli oggetti, dove ogni componente hardware è rappresentato da una struttura dati distinta.

### 2.1 Struttura Principale: Il Bus

Per simulare la comunicazione tra le parti, è stata creata una struttura Bus che funge da contenitore e amministratore degli altri componenti:

- CPU: Fetch-Decode-Execute.
- RAM: La memoria di sistema.
- Display: Il buffer video.
- Keypad: Input dall'utente.

Questa scelta architetturale permette di centralizzare la logica di ownership, risolvendo le complessità tipiche del Borrow Checker di Rust quando componenti diversi devono interagire tra loro.

### 2.2 Memoria (RAM)

La memoria è implementata come un array statico di 4096 byte ([u8; 4096]), coprendo l'indirizzamento da 0x000 a 0xFFF.

- Zona Interprete (0x000 - 0x1FF): Riservata per il caricamento del Fontset predefinito (sprite esadecimali 0-F).
- Zona Programma (0x200 - 0xFFF): Area dove vengono caricate le ROM ed eseguite le istruzioni.

### 2.3 CPU e Registri

La CPU implementa i seguenti registri:

- General Purpose (V0-VF): 16 registri a 8-bit
- Indirizzi (I): 16-bit di cui solo i 12-bit meno significativi indirizzano la memoria
- Program Counter (PC): 16-bit inizializzato a 0x200, punta alla prossima istruzione da eseguire.
- Stack: Un array per gestire le chiamate a subroutine (fino a 16 livelli di nidificazione) tramite le istruzioni CALL (2nnn) e RET (00EE).
- Timer: Implementazione dei registri DT (Delay Timer) e ST (Sound Timer), che vengono decrementati a una frequenza di 60Hz.

### 2.4 Grafica

Il display originale CHIP-8 ha una risoluzione di 64x32 pixel monocromatici. Nel progetto, il display è rappresentato da un buffer di booleani ([bool; 64 * 32]). Durante il rendering, questo buffer viene convertito in un array di u32 (pixel ARGB) per essere visualizzato nella finestra gestita dalla libreria minifb. L'istruzione di disegno Dxyn gestisce il collision detection tramite operazione XOR, impostando il registro VF (flag) se un pixel viene cancellato, come da specifica.

## 3. Scelte Implementative in Rust

### 3.1 Pattern Matching per Opcode

La decodifica delle istruzioni (Opcode a 2 byte) è gestita tramite un blocco match esaustivo. Questo approccio è superiore allo switch-case di altri linguaggi, permettendo di destrutturare l'opcode nelle sue componenti (nnn, x, y, kk) direttamente nella firma del match, migliorando leggibilità e sicurezza.

Esempio di logica implementata:

```
match (op_1, x, y, n) {
    (0, 0, 0xE, 0) => display.clear(),    // 00E0: CLS
    (1, _, _, _) => self.pc = nnn,        // 1NNN: JP addr
    (6, _, _, _) => self.v_reg[x] = nn,   // 6XNN: LD Vx, byte [cite: 178]
    // ...
}
```

### 3.2 Gestione delle Dipendenze

Sono state selezionate librerie minime e performanti, definite nel file Cargo.toml:

- minifb (v0.28.0): Scelta per la sua leggerezza e facilità nel creare un framebuffer grafico senza l'overhead di motori di gioco completi.
- rand (v0.9.2): Necessaria per l'implementazione dell'istruzione Cxkk (Random Number Generation).
- log / env_logger: Utilizzati per tracciare l'esecuzione e il debug senza sporcare il codice con println!.

### 3.3 Sicurezza e Gestione Errori

L'utilizzo di Rust garantisce l'assenza di buffer overflow (comuni negli emulatori scritti in C/C++), specialmente nella gestione dell'accesso alla memoria e allo stack. Le operazioni aritmetiche che potrebbero causare overflow (come ADD Vx, byte) sono gestite usando metodi espliciti come wrapping_add o overflowing_add per emulare fedelmente il comportamento dei registri a 8-bit.

## 4. Manuale Utente

### 4.1 Requisiti

- Sistema Operativo: Windows, Linux o macOS.
- Rust Toolchain (cargo) installata.

### 4.2 Compilazione ed Esecuzione

Per ottenere le massime prestazioni, è possibile compilare in modalità release.

Per scegliere quale ROM utilizzare, si deve modificare la seguente riga del file src/main.rs:

    const ROM: &str = "roms/space-invaders.ch8";

Una volta scelta la ROM, bisogna aprire il terminale nella cartella del progetto ed eseguire:

    cargo run --release--

### 4.3 Controlli

L'emulatore mappa il tastierino esadecimale CHIP-8 (0-F) sulla tastiera moderna secondo la seguente configurazione standard:

| CHIP-8 Keypad | Tastiera PC (QUERTY) |
| :-----------: | :------------------: |
| 1 2 3 C       | 1 2 3 4              |
| 4 5 6 D       | Q W E R              |
| 7 8 9 E       | A S D F              |
| A 0 B F       | Z X C V              |

Premere ESC per chiudere l'emulatore.

## 5. Conclusioni

Il progetto ha permesso di approfondire la comprensione dell'architettura degli elaboratori e delle sfide legate all'emulazione software. L'uso di Rust si è rivelato vincente per gestire la complessità dello stato mutabile della CPU mantenendo il codice sicuro e manutenibile.

L'emulatore supera i test standard (es. IBM Logo, Pong, Space Invaders).

## 6. Riferimenti

- Documentazione Tecnica: Greene, T. P. (1997). Cowgod's Chip-8 Technical Reference v1.0. [File PDF allegato al progetto].
- Specifiche Librerie: Documentazione ufficiale Crates.io per minifb e rand.