# Ettios Mainnet - Node 1 Setup Information

## Node 1 (Alice Validator) - Server: srv1244691

**⚠️ Note:** Node 1 was reinstalled and re-setup. New peer ID generated.

### Step 1: Node Key Generation

**Command:**
```bash
./target/release/substrate key generate-node-key --base-path /data/node1
```

**Result (NEW after reinstall):**
- **Peer ID:** `12D3KooWFtDuhbhgfkVSHtiEaJ99ix6cdJAADXPGj4hV2hwP9TnX` ✅
- **Key Location:** `/data/node1/chains/ettios-mainnet/network/secret_ed25519`

**Bootnode Address (NEW - for Nodes 2, 3, 4 to connect):**
```
/ip4/72.62.107.218/tcp/30333/p2p/12D3KooWFtDuhbhgfkVSHtiEaJ99ix6cdJAADXPGj4hV2hwP9TnX
```

**Old Peer ID (before reinstall):** `12D3KooWLhdbXziHhvXWhaL4cq3JqoPqsFmHmwWB2srPoaPDkU9b`

---

### Step 2: Session Keys Insertion

**Status:** ✅ Completed

**Session Keys Password:** `@Wajid824158!` (KEEP SECURE!)

**Validator Index:** 0 (Alice)

**Commands Executed:**
```bash
export SESSION_KEYS_PASSWORD="@Wajid824158!"
export INDEX=0

./target/release/substrate key insert --key-type gran --scheme ed25519 --base-path /data/node1 --suri //$SESSION_KEYS_PASSWORD//fir//ed//$INDEX
./target/release/substrate key insert --key-type babe --scheme sr25519 --base-path /data/node1 --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX
./target/release/substrate key insert --key-type imon --scheme sr25519 --base-path /data/node1 --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX
./target/release/substrate key insert --key-type auth --scheme sr25519 --base-path /data/node1 --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX
./target/release/substrate key insert --key-type mixn --scheme sr25519 --base-path /data/node1 --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX
./target/release/substrate key insert --key-type beef --scheme ecdsa --base-path /data/node1 --suri //$SESSION_KEYS_PASSWORD//fir//ecdsa//$INDEX
```

**Session Keys Inserted:**
- ✅ gran (Grandpa) - ed25519
- ✅ babe (BABE) - sr25519
- ✅ imon (ImOnline) - sr25519
- ✅ auth (AuthorityDiscovery) - sr25519
- ✅ mixn (Mixnet) - sr25519
- ✅ beef (Beefy) - ecdsa

---

### Step 3: Chain Spec Generation

**Status:** ✅ Completed

**Command:**
```bash
./target/release/substrate build-spec --chain mainnet --raw --disable-default-bootnode > ettios-mainnet-spec.json
```

**Result:**
- **File:** `ettios-mainnet-spec.json`
- **Size:** 3.3 MB
- **Generated:** 4 NPoS voters (4 validators, 0 nominators)
- **Generated:** 4 NPoS targets
- **Status:** ✅ Successfully generated without errors

**Output:**
```
2026-01-17 23:01:14 Building chain spec
2026-01-17 23:01:27 [0] generated 4 npos voters, 4 from validators and 0 nominators
2026-01-17 23:01:27 [0] generated 4 npos targets
```

**File Location:** `/root/frontier-pos-template/ettios-mainnet-spec.json`

---

### Step 4: Peer ID Verification

**Command:**
```bash
./target/release/substrate key inspect-node-key --file /data/node1/chains/ettios-mainnet/network/secret_ed25519
```

**Result:**
- **Peer ID:** `12D3KooWLhdbXziHhvXWhaL4cq3JqoPqsFmHmwWB2srPoaPDkU9b` ✅ Verified1

---

### Step 5: Network Information

**VPS IP Address:** `72.62.107.218` ✅
**VPS Hostname:** srv1244691
**Network Interface:** eth0 (72.62.107.218/24)

**Bootnode Address (for Nodes 2, 3, 4 to connect):**
```
/ip4/72.62.107.218/tcp/30333/p2p/12D3KooWLhdbXziHhvXWhaL4cq3JqoPqsFmHmwWB2srPoaPDkU9b
```

**Save this bootnode address - you'll need it for setting up Nodes 2, 3, and 4!**

---

### Step 6: Node Configuration

**Node Name:** Ettios-Validator-1
**Base Path:** /data/node1
**Port:** 30333
**RPC Port:** 9944
**Chain:** mainnet (identifier)
**Validator Index:** 0 (Alice)

---

### Step 7: Node Startup Status

**Status:** ✅ Node 1 is Running Successfully

**Startup Information:**
- **Local Node Identity:** `12D3KooWLhdbXziHhvXWhaL4cq3JqoPqsFmHmwWB2srPoaPDkU9b` ✅
- **RPC Server:** Running on `0.0.0.0:9944`
- **Protocol ID:** `sup` (default)
- **Current State:** Idle (0 peers) - This is normal for the first node
- **Best Block:** #0 (Genesis block)
- **Finalized Block:** #0

**System Specifications:**
- **OS:** Ubuntu 24.04.3 LTS
- **Kernel:** 6.8.0-90-generic
- **CPU:** AMD EPYC 7543P 32-Core Processor (1 core allocated)
- **Memory:** 3916MB
- **Virtual Machine:** Yes

**Hardware Performance Scores:**
- **CPU Score:** 973.28 MiBs ✅
- **Memory Score:** 14.06 GiBs ✅
- **Disk Score (seq. writes):** 793.40 MiBs ⚠️ (Below recommended 950 MiBs)
- **Disk Score (rand. writes):** 276.38 MiBs ⚠️ (Below recommended 420 MiBs)

**⚠️ Performance Warning:**
The disk performance is below recommended levels for a validator role. This may impact performance but won't prevent the node from running. Consider upgrading disk I/O for production use.

**Services Started:**
- ✅ BABE Authorship worker started
- ✅ Prometheus exporter at 127.0.0.1:9615
- ✅ BEEFY gadget waiting for pallet availability
- ✅ JSON-RPC server active

**Current Status:** Node is running and waiting for peers. Once Nodes 2, 3, and 4 connect, the network will begin producing blocks.

---

### Important Notes

- Keep the session keys password secure
- Save the peer ID for connecting other nodes
- The bootnode address will be needed for Nodes 2, 3, and 4
- Node is currently idle with 0 peers - this is expected until other nodes connect

