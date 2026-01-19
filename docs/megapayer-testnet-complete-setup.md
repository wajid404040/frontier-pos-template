# MegapayerIstanbul Test Network - Complete Network Documentation

**Network Name:** MegapayerIstanbul test network  
**Token Symbol:** MPC  
**Chain ID:** 20240  
**Total Supply:** 1,000,000,000 MPC (1 billion tokens)  
**Validators:** 3 (Minimum: 3)  
**Consensus:** BABE + GRANDPA (NPOS)  
**EVM Compatible:** Yes

---

## 📋 Network Overview

The MegapayerIstanbul test network is a 3-validator testnet running on 2 VPS servers:
- **VPS 1:** Runs Node 1 (Alice) and Node 2 (Bob)
- **VPS 2:** Runs Node 3 (Charlie)

### Supply Distribution

**Total Supply:** 1,000,000,000 MPC

**Distribution:**
1. `0x524aAdF5104493fb3112CCD64F20725b9D0D61bc` → 250,000,000 MPC
2. `0x54B0C17AaE30BE8c56Fb8D1DC84213F192035E55` → 250,000,000 MPC
3. `0x182090B7Df5B848b76F7e019A0Ac7300569982B0` → 500,000,000 MPC (receives both allocations)

---

## 🖥️ VPS 1 Details

### Server Information

- **VPS IPv4:** `195.35.0.211`
- **VPS IPv6:** `2a02:4780:28:75f2::1`
- **Hostname:** srv860054
- **SSH:** `ssh root@195.35.0.211`

---

## 🖥️ Node 1 Details (Alice)

### Server Information

- **VPS IP:** `195.35.0.211`
- **Hostname:** srv860054
- **SSH:** `ssh root@195.35.0.211`
- **Network Interface:** eth0

### Node Configuration

- **Node Name:** Megapayer-Node-1
- **Peer ID:** `12D3KooWDd53TUC6aS5naNNHsZaBfJvfovUDesZCFC7XzZuDjAh3`
- **Base Path:** `/data/node1`
- **Port:** 30333 (P2P)
- **RPC Port:** 9944
- **Validator Index:** 0 (Alice)
- **Session Key Seed:** `//Alice`

### Bootnode Address

```
/ip4/195.35.0.211/tcp/30333/p2p/12D3KooWDd53TUC6aS5naNNHsZaBfJvfovUDesZCFC7XzZuDjAh3
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
  --chain megapayer-testnet-spec.json \
  --validator \
  --name "Megapayer-Node-1" \
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
screen -r megapayer-node1

# Detach: Ctrl+A then D
# List sessions: screen -ls
```

### RPC Endpoints

- **HTTP:** `http://195.35.0.211:9944`
- **WebSocket:** `ws://195.35.0.211:9944`

---

## 🖥️ Node 2 Details (Bob)

### Server Information

- **VPS IP:** `195.35.0.211`
- **Hostname:** srv860054
- **SSH:** `ssh root@195.35.0.211`
- **Network Interface:** eth0

### Node Configuration

- **Node Name:** Megapayer-Node-2
- **Peer ID:** `12D3KooWSMkVkayNDmUKAagkAw3ShduRDHNhp6Nj8164uSshvkKf`
- **Base Path:** `/data/node2`
- **Port:** 30334 (P2P)
- **RPC Port:** 9945
- **Validator Index:** 1 (Bob)
- **Session Key Seed:** `//Bob`

### Peer Address

```
/ip4/195.35.0.211/tcp/30334/p2p/12D3KooWSMkVkayNDmUKAagkAw3ShduRDHNhp6Nj8164uSshvkKf
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
  --chain megapayer-testnet-spec.json \
  --validator \
  --name "Megapayer-Node-2" \
  --port 30334 \
  --rpc-port 9945 \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --unsafe-rpc-external \
  --pruning archive \
  --bootnodes /ip4/195.35.0.211/tcp/30333/p2p/12D3KooWDd53TUC6aS5naNNHsZaBfJvfovUDesZCFC7XzZuDjAh3
```

### Screen Management

```bash
# Attach to screen
screen -r megapayer-node2

# Detach: Ctrl+A then D
# List sessions: screen -ls
```

### RPC Endpoints

- **HTTP:** `http://195.35.0.211:9945`
- **WebSocket:** `ws://195.35.0.211:9945`

---

## 🖥️ Node 3 Details (Charlie)

### Server Information

- **VPS IP:** (To be configured)
- **Hostname:** (To be configured)
- **SSH:** (To be configured)

### Node Configuration

- **Node Name:** Megapayer-Node-3
- **Peer ID:** (To be generated)
- **Base Path:** `/data/node3`
- **Port:** 30333 (P2P)
- **RPC Port:** 9944
- **Validator Index:** 2 (Charlie)
- **Session Key Seed:** `//Charlie`

### Session Keys

All session keys to be inserted using seed: `//Charlie`

- gran (Grandpa) - ed25519
- babe (BABE) - sr25519
- imon (ImOnline) - sr25519
- auth (AuthorityDiscovery) - sr25519
- mixn (Mixnet) - sr25519
- beef (Beefy) - ecdsa

### Start Command

```bash
cd ~/frontier-pos-template
./target/release/substrate \
  --base-path /data/node3 \
  --chain megapayer-testnet-spec.json \
  --validator \
  --name "Megapayer-Node-3" \
  --port 30333 \
  --rpc-port 9944 \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --unsafe-rpc-external \
  --pruning archive \
  --bootnodes /ip4/195.35.0.211/tcp/30333/p2p/12D3KooWDd53TUC6aS5naNNHsZaBfJvfovUDesZCFC7XzZuDjAh3
```

---

## 🔌 Network Connection Information

### Chain ID

```
20240
```

### RPC URLs (HTTP)

**Node 1:**
```
http://195.35.0.211:9944
```

**Node 2:**
```
http://195.35.0.211:9945
```

**Node 3:**
```
http://NODE3_IP:9944
```

### WebSocket URLs (WS)

**Node 1:**
```
ws://195.35.0.211:9944
```

**Node 2:**
```
ws://195.35.0.211:9945
```

**Node 3:**
```
ws://NODE3_IP:9944
```

### MetaMask Configuration

**Network Name:** MegapayerIstanbul test network  
**RPC URL:** `http://195.35.0.211:9944` (or any node)  
**Chain ID:** `20240`  
**Currency Symbol:** `MPC`  
**Block Explorer URL:** (To be configured)

### Test Connection

```bash
# Check chain ID
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "eth_chainId"}' http://195.35.0.211:9944

# Expected response: {"jsonrpc":"2.0","result":"0x4f10","id":1}
# 0x4f10 in hex = 20240 in decimal
```

---

## 💰 Supply Distribution

**Total Supply:** 1,000,000,000 MPC (1 billion tokens)

**Distribution:**

1. **Address 1:** `0x524aAdF5104493fb3112CCD64F20725b9D0D61bc`
   - **Balance:** 250,000,000 MPC

2. **Address 2:** `0x54B0C17AaE30BE8c56Fb8D1DC84213F192035E55`
   - **Balance:** 250,000,000 MPC

3. **Address 3:** `0x182090B7Df5B848b76F7e019A0Ac7300569982B0`
   - **Balance:** 500,000,000 MPC (receives both allocations due to duplicate in genesis)

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
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "eth_blockNumber"}' http://localhost:9944

# Check chain ID
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "eth_chainId"}' http://localhost:9944
```

### Stop a Node

```bash
# Inside screen session: Press Ctrl+C
# Then type: exit
```

### Restart a Node

```bash
# Stop the screen session first
screen -S megapayer-node1 -X quit

# Then restart
screen -S megapayer-node1
cd ~/frontier-pos-template
./target/release/substrate --base-path /data/node1 --chain megapayer-testnet-spec.json --validator --name "Megapayer-Node-1" --port 30333 --rpc-port 9944 --rpc-cors all --rpc-methods Unsafe --unsafe-rpc-external --pruning archive
```

---

## 🔑 Session Key Insertion Commands

### Node 1 (Alice)

```bash
./target/release/substrate key insert --key-type gran --scheme ed25519 --base-path /data/node1 --suri //Alice
./target/release/substrate key insert --key-type babe --scheme sr25519 --base-path /data/node1 --suri //Alice
./target/release/substrate key insert --key-type imon --scheme sr25519 --base-path /data/node1 --suri //Alice
./target/release/substrate key insert --key-type auth --scheme sr25519 --base-path /data/node1 --suri //Alice
./target/release/substrate key insert --key-type mixn --scheme sr25519 --base-path /data/node1 --suri //Alice
./target/release/substrate key insert --key-type beef --scheme ecdsa --base-path /data/node1 --suri //Alice
```

### Node 2 (Bob)

```bash
./target/release/substrate key insert --key-type gran --scheme ed25519 --base-path /data/node2 --suri //Bob
./target/release/substrate key insert --key-type babe --scheme sr25519 --base-path /data/node2 --suri //Bob
./target/release/substrate key insert --key-type imon --scheme sr25519 --base-path /data/node2 --suri //Bob
./target/release/substrate key insert --key-type auth --scheme sr25519 --base-path /data/node2 --suri //Bob
./target/release/substrate key insert --key-type mixn --scheme sr25519 --base-path /data/node2 --suri //Bob
./target/release/substrate key insert --key-type beef --scheme ecdsa --base-path /data/node2 --suri //Bob
```

### Node 3 (Charlie)

```bash
./target/release/substrate key insert --key-type gran --scheme ed25519 --base-path /data/node3 --suri //Charlie
./target/release/substrate key insert --key-type babe --scheme sr25519 --base-path /data/node3 --suri //Charlie
./target/release/substrate key insert --key-type imon --scheme sr25519 --base-path /data/node3 --suri //Charlie
./target/release/substrate key insert --key-type auth --scheme sr25519 --base-path /data/node3 --suri //Charlie
./target/release/substrate key insert --key-type mixn --scheme sr25519 --base-path /data/node3 --suri //Charlie
./target/release/substrate key insert --key-type beef --scheme ecdsa --base-path /data/node3 --suri //Charlie
```

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

- Monitor all 3 nodes regularly
- Check block production status
- Monitor disk space and performance
- Keep nodes updated if needed

---

## 📞 Quick Reference

### All Node IPs

- Node 1: `195.35.0.211` (VPS 1)
- Node 2: `195.35.0.211` (VPS 1)
- Node 3: (VPS 2 - To be configured)

### All Peer IDs

- Node 1: `12D3KooWDd53TUC6aS5naNNHsZaBfJvfovUDesZCFC7XzZuDjAh3`
- Node 2: `12D3KooWSMkVkayNDmUKAagkAw3ShduRDHNhp6Nj8164uSshvkKf`
- Node 3: (To be generated)

### Primary RPC

```
http://195.35.0.211:9944
```

### Chain ID

```
20240
```

### Bootnode for New Nodes

```
/ip4/195.35.0.211/tcp/30333/p2p/12D3KooWDd53TUC6aS5naNNHsZaBfJvfovUDesZCFC7XzZuDjAh3
```

---

**Document Version:** 1.0  
**Last Updated:** 2026-01-19  
**Status:** ✅ Network Setup in Progress - Node 1 & 2 Configured, Node 3 Pending

