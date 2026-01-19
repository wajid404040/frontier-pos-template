# Ettios Mainnet - Node 4 Setup Information

## Node 4 (Dave Validator)

### Step 1: Transfer Binary from Node 2 or Node 3

**Status:** ✅ Completed

**Binary transferred from:** Node 2 or Node 3
**Binary version:** substrate 3.0.0-dev-48e3c8b39c8

---

### Step 2: Node Key Generation

**Command:**
```bash
./target/release/substrate key generate-node-key --base-path /data/node4
```

**Result:**
- **Peer ID:** `12D3KooWNd2jGwrhGCUmT1zyW4mSnysxLSXhoQrohNs4fm9sV7Xo` ✅
- **Key Location:** `/data/node4/chains/ettios-mainnet/network/secret_ed25519`

---

### Step 3: Session Keys Insertion

**Status:** Pending

**Session Keys Password:** `@Wajid824158!` (KEEP SECURE!)

**Validator Index:** 3 (Dave)

---

### Step 4: Network Information

**VPS IP Address:** `72.62.107.226` ✅
**VPS Hostname:** srv1244734

**Bootnode Address (Connect to Node 1):**
```
/ip4/72.62.107.218/tcp/30333/p2p/12D3KooWLhdbXziHhvXWhaL4cq3JqoPqsFmHmwWB2srPoaPDkU9b
```

**Node 4 Peer Address (for reference):**
```
/ip4/72.62.107.226/tcp/30333/p2p/12D3KooWNd2jGwrhGCUmT1zyW4mSnysxLSXhoQrohNs4fm9sV7Xo
```

**Bootnode Address (Connect to Node 1):**
```
/ip4/72.62.107.218/tcp/30333/p2p/12D3KooWLhdbXziHhvXWhaL4cq3JqoPqsFmHmwWB2srPoaPDkU9b
```

---

### Step 5: Node Configuration

**Node Name:** Ettios-Validator-4
**Base Path:** /data/node4
**Port:** 30333
**RPC Port:** 9944
**Chain:** mainnet (identifier)
**Validator Index:** 3 (Dave)

---

### Step 6: Node Startup Status

**Status:** Pending

---

### Important Notes

- Keep the session keys password secure
- Node 4 connects to Node 1 as bootnode
- Once Node 4 connects, all 4 validators will be online and block production will begin!

