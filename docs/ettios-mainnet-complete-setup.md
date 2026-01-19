# Ettios Mainnet - Complete Network Documentation

**⚠️ CONFIDENTIAL - KEEP SECURE ⚠️**

This document contains all sensitive information about the Ettios Mainnet blockchain network including credentials, peer IDs, and connection details.

---

## 📋 Table of Contents

1. [Network Overview](#network-overview)
2. [Blockchain Configuration](#blockchain-configuration)
3. [Node 1 Details (Alice)](#node-1-details-alice)
4. [Node 2 Details (Bob)](#node-2-details-bob)
5. [Node 3 Details (Charlie)](#node-3-details-charlie)
6. [Node 4 Details (Dave)](#node-4-details-dave)
7. [Network Connection Information](#network-connection-information)
8. [Supply Distribution](#supply-distribution)
9. [Management Commands](#management-commands)
10. [Troubleshooting](#troubleshooting)

---

## 🌐 Network Overview

**Blockchain Name:** ettios mainnet  
**Token Symbol:** ettia  
**Chain ID:** 2237  
**Decimals:** 18  
**Total Supply:** 100,000,000,000 ETTIA (100 billion tokens)  
**Validators:** 4 nodes  
**Status:** ✅ Operational - Blocks Producing

---

## ⚙️ Blockchain Configuration

### Genesis Configuration

- **Validators:** 4 (Alice, Bob, Charlie, Dave)
- **Minimum Validator Count:** 4
- **Consensus:** BABE + Grandpa
- **EVM Compatible:** Yes
- **Chain Type:** Live (Production)

### Session Keys Seeds

The genesis expects session keys from these seeds:
- **Node 1 (Alice):** `//Alice`
- **Node 2 (Bob):** `//Bob`
- **Node 3 (Charlie):** `//Charlie`
- **Node 4 (Dave):** `//Dave`

---

## 🖥️ Node 1 Details (Alice)

### Server Information

- **VPS IP:** `72.62.107.218`
- **Hostname:** srv1244691
- **SSH:** `ssh root@72.62.107.218`
- **Network Interface:** eth0 (72.62.107.218/24)

### Node Configuration

- **Node Name:** Ettios-Validator-1
- **Peer ID:** `12D3KooWFtDuhbhgfkVSHtiEaJ99ix6cdJAADXPGj4hV2hwP9TnX`
- **Base Path:** `/data/node1`
- **Port:** 30333 (P2P)
- **RPC Port:** 9944
- **Validator Index:** 0 (Alice)
- **Session Key Seed:** `//Alice`

### Bootnode Address

```
/ip4/72.62.107.218/tcp/30333/p2p/12D3KooWFtDuhbhgfkVSHtiEaJ99ix6cdJAADXPGj4hV2hwP9TnX
```

### Session Keys

All session keys inserted using seed: `//Alice`

- ✅ gran (Grandpa) - ed25519
- ✅ babe (BABE) - sr25519
- ✅ imon (ImOnline) - sr25519
- ✅ auth (AuthorityDiscovery) - sr25519
- ✅ mixn (Mixnet) - sr25519
- ✅ beef (Beefy) - ecdsa

### Start Command

```bash
cd ~/frontier-pos-template
./target/release/substrate \
  --base-path /data/node1 \
  --chain mainnet \
  --validator \
  --name "Ettios-Validator-1" \
  --port 30333 \
  --rpc-port 9944 \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --unsafe-rpc-external \
  --pruning archive
```

### Screen Management

```bash
# Attach to screen
screen -r ettios-node1

# Detach: Ctrl+A then D
# List sessions: screen -ls
```

### RPC Endpoints

- **HTTP:** `http://72.62.107.218:9944`
- **WebSocket:** `ws://72.62.107.218:9944`

---

## 🖥️ Node 2 Details (Bob)

### Server Information

- **VPS IP:** `72.62.107.223`
- **Hostname:** srv1244719
- **SSH:** `ssh root@72.62.107.223`
- **Network Interface:** eth0 (72.62.107.223/24)

### Node Configuration

- **Node Name:** Ettios-Validator-2
- **Peer ID:** `12D3KooWC3nbzcb6cygeaFQZU5JmVi4wUvSfQ5jZJgDJVsCTjNsk`
- **Base Path:** `/data/node2`
- **Port:** 30333 (P2P)
- **RPC Port:** 9944
- **Validator Index:** 1 (Bob)
- **Session Key Seed:** `//Bob`

### Peer Address

```
/ip4/72.62.107.223/tcp/30333/p2p/12D3KooWC3nbzcb6cygeaFQZU5JmVi4wUvSfQ5jZJgDJVsCTjNsk
```

### Session Keys

All session keys inserted using seed: `//Bob`

- ✅ gran (Grandpa) - ed25519
- ✅ babe (BABE) - sr25519
- ✅ imon (ImOnline) - sr25519
- ✅ auth (AuthorityDiscovery) - sr25519
- ✅ mixn (Mixnet) - sr25519
- ✅ beef (Beefy) - ecdsa

### Start Command

```bash
cd ~/frontier-pos-template
./target/release/substrate \
  --base-path /data/node2 \
  --chain mainnet \
  --validator \
  --name "Ettios-Validator-2" \
  --port 30333 \
  --rpc-port 9944 \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --unsafe-rpc-external \
  --pruning archive \
  --bootnodes /ip4/72.62.107.218/tcp/30333/p2p/12D3KooWFtDuhbhgfkVSHtiEaJ99ix6cdJAADXPGj4hV2hwP9TnX
```

### Screen Management

```bash
# Attach to screen
screen -r ettios-node2

# Detach: Ctrl+A then D
# List sessions: screen -ls
```

### RPC Endpoints

- **HTTP:** `http://72.62.107.223:9944`
- **WebSocket:** `ws://72.62.107.223:9944`

---

## 🖥️ Node 3 Details (Charlie)

### Server Information

- **VPS IP:** `72.62.107.229`
- **Hostname:** srv1244739
- **SSH:** `ssh root@72.62.107.229`
- **Network Interface:** eth0 (72.62.107.229/24)

### Node Configuration

- **Node Name:** Ettios-Validator-3
- **Peer ID:** `12D3KooWMGqyJtzPGZjCWK5q3NVCjofvGFbtDU99C5CivzLS3oUe`
- **Base Path:** `/data/node3`
- **Port:** 30333 (P2P)
- **RPC Port:** 9944
- **Validator Index:** 2 (Charlie)
- **Session Key Seed:** `//Charlie`

### Peer Address

```
/ip4/72.62.107.229/tcp/30333/p2p/12D3KooWMGqyJtzPGZjCWK5q3NVCjofvGFbtDU99C5CivzLS3oUe
```

### Session Keys

All session keys inserted using seed: `//Charlie`

- ✅ gran (Grandpa) - ed25519
- ✅ babe (BABE) - sr25519
- ✅ imon (ImOnline) - sr25519
- ✅ auth (AuthorityDiscovery) - sr25519
- ✅ mixn (Mixnet) - sr25519
- ✅ beef (Beefy) - ecdsa

### Start Command

```bash
cd ~/frontier-pos-template
./target/release/substrate \
  --base-path /data/node3 \
  --chain mainnet \
  --validator \
  --name "Ettios-Validator-3" \
  --port 30333 \
  --rpc-port 9944 \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --unsafe-rpc-external \
  --pruning archive \
  --bootnodes /ip4/72.62.107.218/tcp/30333/p2p/12D3KooWFtDuhbhgfkVSHtiEaJ99ix6cdJAADXPGj4hV2hwP9TnX
```

### Screen Management

```bash
# Attach to screen
screen -r ettios-node3

# Detach: Ctrl+A then D
# List sessions: screen -ls
```

### RPC Endpoints

- **HTTP:** `http://72.62.107.229:9944`
- **WebSocket:** `ws://72.62.107.229:9944`

---

## 🖥️ Node 4 Details (Dave)

### Server Information

- **VPS IP:** `72.62.107.226`
- **Hostname:** srv1244734
- **SSH:** `ssh root@72.62.107.226`
- **Network Interface:** eth0 (72.62.107.226/24)

### Node Configuration

- **Node Name:** Ettios-Validator-4
- **Peer ID:** `12D3KooWNd2jGwrhGCUmT1zyW4mSnysxLSXhoQrohNs4fm9sV7Xo`
- **Base Path:** `/data/node4`
- **Port:** 30333 (P2P)
- **RPC Port:** 9944
- **Validator Index:** 3 (Dave)
- **Session Key Seed:** `//Dave`

### Peer Address

```
/ip4/72.62.107.226/tcp/30333/p2p/12D3KooWNd2jGwrhGCUmT1zyW4mSnysxLSXhoQrohNs4fm9sV7Xo
```

### Session Keys

All session keys inserted using seed: `//Dave`

- ✅ gran (Grandpa) - ed25519
- ✅ babe (BABE) - sr25519
- ✅ imon (ImOnline) - sr25519
- ✅ auth (AuthorityDiscovery) - sr25519
- ✅ mixn (Mixnet) - sr25519
- ✅ beef (Beefy) - ecdsa

### Start Command

```bash
cd ~/frontier-pos-template
./target/release/substrate \
  --base-path /data/node4 \
  --chain mainnet \
  --validator \
  --name "Ettios-Validator-4" \
  --port 30333 \
  --rpc-port 9944 \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --unsafe-rpc-external \
  --pruning archive \
  --bootnodes /ip4/72.62.107.218/tcp/30333/p2p/12D3KooWFtDuhbhgfkVSHtiEaJ99ix6cdJAADXPGj4hV2hwP9TnX
```

### Screen Management

```bash
# Attach to screen
screen -r ettios-node4

# Detach: Ctrl+A then D
# List sessions: screen -ls
```

### RPC Endpoints

- **HTTP:** `http://72.62.107.226:9944`
- **WebSocket:** `ws://72.62.107.226:9944`

---

## 🔌 Network Connection Information

### Chain ID

```
2237
```

### RPC URLs (HTTP)

**Primary (Node 1):**
```
http://72.62.107.218:9944
```

**Secondary (Node 2):**
```
http://72.62.107.223:9944
```

**Secondary (Node 3):**
```
http://72.62.107.229:9944
```

**Secondary (Node 4):**
```
http://72.62.107.226:9944
```

### WebSocket URLs (WS)

**Primary (Node 1):**
```
ws://72.62.107.218:9944
```

**Secondary (Node 2):**
```
ws://72.62.107.223:9944
```

**Secondary (Node 3):**
```
ws://72.62.107.229:9944
```

**Secondary (Node 4):**
```
ws://72.62.107.226:9944
```

### MetaMask Configuration

**Network Name:** ettios mainnet  
**RPC URL:** `http://72.62.107.218:9944` (or any node)  
**Chain ID:** `2237`  
**Currency Symbol:** `ettia`  
**Block Explorer URL:** (To be configured)

### Test Connection

```bash
# Check chain ID
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "eth_chainId"}' http://72.62.107.218:9944

# Expected response: {"jsonrpc":"2.0","result":"0x8bd","id":1}
# 0x8bd in hex = 2237 in decimal
```

---

## 💰 Supply Distribution

**Total Supply:** 100,000,000,000 ETTIA (100 billion tokens)

**Distribution:** 25 billion ETTIA to each of 4 addresses

1. **Address 1:** `0x06e7e5d101e69cfd62e628b05c53785aa4aaf912`
   - **Balance:** 25,000,000,000 ETTIA

2. **Address 2:** `0x27f69008e0675cbd1fb730657010beb668e1222a`
   - **Balance:** 25,000,000,000 ETTIA

3. **Address 3:** `0x34fd827b6160b550d1ce93f8db27e763c78a8494`
   - **Balance:** 25,000,000,000 ETTIA

4. **Address 4:** `0x2bd700dc26d5ca3f179e3fbb286b361960ef8bb3`
   - **Balance:** 25,000,000,000 ETTIA

---

## 🛠️ Management Commands

### Check Node Status

**On any node:**

```bash
# Check node health
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' http://localhost:9944

# Check peers
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' http://localhost:9944

# Check current block
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock"}' http://localhost:9944
```

### Screen Commands

**List all screen sessions:**
```bash
screen -ls
```

**Attach to node screens:**
```bash
screen -r ettios-node1  # Node 1
screen -r ettios-node2  # Node 2
screen -r ettios-node3  # Node 3
screen -r ettios-node4  # Node 4
```

**Detach from screen:** `Ctrl+A` then `D`

**Kill screen session:**
```bash
screen -X -S ettios-node1 quit
```

### Restart Node

**To restart a node:**

1. Attach to screen: `screen -r ettios-node1`
2. Stop node: `Ctrl+C`
3. Run start command again
4. Detach: `Ctrl+A` then `D`

---

## 🔧 Troubleshooting

### Node Not Producing Blocks

**Check:**
1. All 4 nodes are running
2. All nodes see 3 peers each
3. Session keys are correctly inserted (using seeds: Alice, Bob, Charlie, Dave)
4. All nodes are at the same block number

### Node Not Connecting

**Check:**
1. Firewall ports are open (30333, 9944)
2. Bootnode address is correct
3. Node 1 is running (for other nodes to connect)

### Verify Session Keys

**On each node, check keys exist:**

```bash
# Node 1
ls -la /data/node1/chains/ettios-mainnet/keystore/

# Node 2
ls -la /data/node2/chains/ettios-mainnet/keystore/

# Node 3
ls -la /data/node3/chains/ettios-mainnet/keystore/

# Node 4
ls -la /data/node4/chains/ettios-mainnet/keystore/
```

Each should have 6 key files (gran, babe, imon, auth, mixn, beef).

### Re-insert Session Keys

If keys are incorrect, re-insert using the correct seeds:

**Node 1:**
```bash
./target/release/substrate key insert --key-type gran --scheme ed25519 --base-path /data/node1 --suri //Alice
./target/release/substrate key insert --key-type babe --scheme sr25519 --base-path /data/node1 --suri //Alice
./target/release/substrate key insert --key-type imon --scheme sr25519 --base-path /data/node1 --suri //Alice
./target/release/substrate key insert --key-type auth --scheme sr25519 --base-path /data/node1 --suri //Alice
./target/release/substrate key insert --key-type mixn --scheme sr25519 --base-path /data/node1 --suri //Alice
./target/release/substrate key insert --key-type beef --scheme ecdsa --base-path /data/node1 --suri //Alice
```

**Node 2:** Use `//Bob`  
**Node 3:** Use `//Charlie`  
**Node 4:** Use `//Dave`

---

## 📝 Important Notes

### Security

- ⚠️ **KEEP THIS DOCUMENT SECURE** - Contains sensitive information
- ⚠️ **Session keys are critical** - Never share or expose
- ⚠️ **Peer IDs are public** - Safe to share for network connections
- ⚠️ **RPC endpoints are public** - Can be shared for wallet connections

### Backup

- Keep backups of all node keys
- Store session keys securely
- Document any changes to node configuration

### Maintenance

- Monitor all 4 nodes regularly
- Check block production status
- Monitor disk space and performance
- Keep nodes updated if needed

---

## 📞 Quick Reference

### All Node IPs

- Node 1: `72.62.107.218`
- Node 2: `72.62.107.223`
- Node 3: `72.62.107.229`
- Node 4: `72.62.107.226`

### All Peer IDs

- Node 1: `12D3KooWFtDuhbhgfkVSHtiEaJ99ix6cdJAADXPGj4hV2hwP9TnX`
- Node 2: `12D3KooWC3nbzcb6cygeaFQZU5JmVi4wUvSfQ5jZJgDJVsCTjNsk`
- Node 3: `12D3KooWMGqyJtzPGZjCWK5q3NVCjofvGFbtDU99C5CivzLS3oUe`
- Node 4: `12D3KooWNd2jGwrhGCUmT1zyW4mSnysxLSXhoQrohNs4fm9sV7Xo`

### Primary RPC

```
http://72.62.107.218:9944
```

### Chain ID

```
2237
```

---

**Document Version:** 1.0  
**Last Updated:** 2026-01-18  
**Status:** ✅ Network Operational - All 4 Validators Running

