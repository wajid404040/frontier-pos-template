# Ettios Mainnet - Node 2 Setup Information

## Node 2 (Bob Validator) - Server: srv1244719

### Step 1: Node Key Generation

**Command:**
```bash
./target/release/substrate key generate-node-key --base-path /data/node2
```

**Result:**
- **Peer ID:** `12D3KooWC3nbzcb6cygeaFQZU5JmVi4wUvSfQ5jZJgDJVsCTjNsk` ✅
- **Key Location:** `/data/node2/chains/ettios-mainnet/network/secret_ed25519`

---

### Step 2: Session Keys Insertion

**Status:** ✅ Completed

**Session Keys Password:** `@Wajid824158!` (KEEP SECURE!)

**Validator Index:** 1 (Bob)

**Commands Executed:**
```bash
export SESSION_KEYS_PASSWORD="@Wajid824158!"
export INDEX=1

./target/release/substrate key insert --key-type gran --scheme ed25519 --base-path /data/node2 --suri //$SESSION_KEYS_PASSWORD//fir//ed//$INDEX
./target/release/substrate key insert --key-type babe --scheme sr25519 --base-path /data/node2 --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX
./target/release/substrate key insert --key-type imon --scheme sr25519 --base-path /data/node2 --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX
./target/release/substrate key insert --key-type auth --scheme sr25519 --base-path /data/node2 --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX
./target/release/substrate key insert --key-type mixn --scheme sr25519 --base-path /data/node2 --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX
./target/release/substrate key insert --key-type beef --scheme ecdsa --base-path /data/node2 --suri //$SESSION_KEYS_PASSWORD//fir//ecdsa//$INDEX
```

**Session Keys Inserted:**
- ✅ gran (Grandpa) - ed25519
- ✅ babe (BABE) - sr25519
- ✅ imon (ImOnline) - sr25519
- ✅ auth (AuthorityDiscovery) - sr25519
- ✅ mixn (Mixnet) - sr25519
- ✅ beef (Beefy) - ecdsa

---

### Step 3: Network Information

**VPS IP Address:** `72.62.107.223` ✅
**VPS Hostname:** srv1244719
**Network Interface:** eth0 (72.62.107.223/24)

**Bootnode Address (Connected to Node 1):**
```
/ip4/72.62.107.218/tcp/30333/p2p/12D3KooWLhdbXziHhvXWhaL4cq3JqoPqsFmHmwWB2srPoaPDkU9b
```

**Node 2 Peer Address (for other nodes to connect):**
```
/ip4/72.62.107.223/tcp/30333/p2p/12D3KooWC3nbzcb6cygeaFQZU5JmVi4wUvSfQ5jZJgDJVsCTjNsk
```

---

### Step 4: Node Configuration

**Node Name:** Ettios-Validator-2
**Base Path:** /data/node2
**Port:** 30333
**RPC Port:** 9944
**Chain:** mainnet (identifier)
**Validator Index:** 1 (Bob)

---

### Step 5: Node Startup Status

**Status:** ✅ Node 2 is Running and Connected Successfully

**Startup Information:**
- **Local Node Identity:** `12D3KooWC3nbzcb6cygeaFQZU5JmVi4wUvSfQ5jZJgDJVsCTjNsk` ✅
- **External Address Discovered:** `/ip4/72.62.107.223/tcp/30333/p2p/12D3KooWC3nbzcb6cygeaFQZU5JmVi4wUvSfQ5jZJgDJVsCTjNsk`
- **RPC Server:** Running on `0.0.0.0:9944`
- **Current State:** Idle (1 peers) ✅ - Connected to Node 1
- **Best Block:** #0 (Genesis block)
- **Finalized Block:** #0

**System Specifications:**
- **OS:** Ubuntu 24.04.3 LTS
- **Kernel:** 6.8.0-90-generic
- **CPU:** AMD EPYC 9354P 32-Core Processor (1 core allocated)
- **Memory:** 3915MB
- **Virtual Machine:** Yes

**Hardware Performance Scores:**
- **CPU Score:** 1.13 GiBs ✅
- **Memory Score:** 10.77 GiBs ⚠️ (Below recommended 11.49 GiBs)
- **Disk Score (seq. writes):** 1.02 GiBs ✅
- **Disk Score (rand. writes):** 415.33 MiBs ⚠️ (Slightly below recommended 420 MiBs)

**⚠️ Performance Warning:**
The memory and random write performance are slightly below recommended levels, but the node is running successfully.

**Services Started:**
- ✅ BABE Authorship worker started
- ✅ Prometheus exporter at 127.0.0.1:9615
- ✅ BEEFY gadget waiting for pallet availability
- ✅ JSON-RPC server active
- ✅ Connected to Node 1 (1 peer) ✅

**Current Status:** Node 2 is running and successfully connected to Node 1. Both nodes are synced and ready. Once Nodes 3 and 4 connect, block production will begin.

---

### Important Notes

- Keep the session keys password secure
- Node 2 connects to Node 1 as bootnode
- Once all 4 nodes are connected, block production will begin

