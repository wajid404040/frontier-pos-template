# Ettios Mainnet - Node 3 Setup Information

## Node 3 (Charlie Validator) - Server: srv1244739

### Step 1: Transfer Binary from Node 2

**Status:** ✅ Completed

**Binary transferred from:** Node 2 (72.62.107.223)
**Binary version:** substrate 3.0.0-dev-48e3c8b39c8

---

### Step 2: Node Key Generation

**Command:**
```bash
./target/release/substrate key generate-node-key --base-path /data/node3
```

**Result:**
- **Peer ID:** `12D3KooWMGqyJtzPGZjCWK5q3NVCjofvGFbtDU99C5CivzLS3oUe` ✅
- **Key Location:** `/data/node3/chains/ettios-mainnet/network/secret_ed25519`

---

### Step 3: Session Keys Insertion

**Status:** Pending

**Session Keys Password:** `@Wajid824158!` (KEEP SECURE!)

**Validator Index:** 2 (Charlie)

---

### Step 4: Network Information

**VPS IP Address:** `72.62.107.229` ✅
**VPS Hostname:** srv1244739

**Bootnode Address (Connect to Node 1):**
```
/ip4/72.62.107.218/tcp/30333/p2p/12D3KooWLhdbXziHhvXWhaL4cq3JqoPqsFmHmwWB2srPoaPDkU9b
```

**Node 3 Peer Address (for other nodes to connect):**
```
/ip4/72.62.107.229/tcp/30333/p2p/12D3KooWMGqyJtzPGZjCWK5q3NVCjofvGFbtDU99C5CivzLS3oUe
```

**Bootnode Address (Connect to Node 1):**
```
/ip4/72.62.107.218/tcp/30333/p2p/12D3KooWLhdbXziHhvXWhaL4cq3JqoPqsFmHmwWB2srPoaPDkU9b
```

---

### Step 5: Node Configuration

**Node Name:** Ettios-Validator-3
**Base Path:** /data/node3
**Port:** 30333
**RPC Port:** 9944
**Chain:** mainnet (identifier)
**Validator Index:** 2 (Charlie)

---

### Step 6: Node Startup Status

**Status:** Pending

---

### Important Notes

- Keep the session keys password secure
- Node 3 connects to Node 1 as bootnode
- Once all 4 nodes are connected, block production will begin

