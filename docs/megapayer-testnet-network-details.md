# MegapayerIstanbul Test Network - Network Details

## 🌐 Network Information

### Basic Network Details

- **Network Name:** MegapayerIstanbul test network
- **Chain ID:** `20240` (0x4f10 in hex)
- **Token Symbol:** MPC
- **Token Name:** Megapayer Coin
- **Decimals:** 18
- **Total Supply:** 1,000,000,000 MPC (1 billion tokens)
- **Consensus:** BABE + GRANDPA (NPOS - Nominated Proof-of-Stake)
- **EVM Compatible:** Yes
- **Block Time:** ~6 seconds (BABE)
- **Finality:** GRANDPA (deterministic)

---

## 🔌 RPC Endpoints

### HTTP RPC URLs

**Node 1 (Primary):**
```
http://195.35.0.211:9944
```

**Node 2:**
```
http://195.35.0.211:9945
```

**Node 3:**
```
http://VPS2_IPV4:9944
```
(Get IPv4 with: `ip addr show eth0 | grep "inet " | awk '{print $2}' | cut -d/ -f1`)

**Domain RPC (Recommended):**
```
https://istanbul.megapayer.net
```

### WebSocket RPC URLs

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
ws://VPS2_IPV4:9944
```

**Domain WebSocket:**
```
wss://istanbul.megapayer.net
```

---

## 👛 MetaMask Configuration

### Network Settings

**Network Name:** `MegapayerIstanbul test network`

**RPC URL:** 
- `https://istanbul.megapayer.net` (Recommended - with SSL)
- `http://195.35.0.211:9944` (Direct IP)

**Chain ID:** `20240`

**Currency Symbol:** `MPC`

**Block Explorer URL:** (To be configured)

### Quick Add to MetaMask

1. Open MetaMask
2. Click network dropdown → "Add Network" → "Add a network manually"
3. Enter the following:
   - **Network Name:** MegapayerIstanbul test network
   - **RPC URL:** https://istanbul.megapayer.net
   - **Chain ID:** 20240
   - **Currency Symbol:** MPC
4. Click "Save"

---

## 🔗 Network Connection Details

### Chain ID Verification

```bash
# Test chain ID via RPC
curl -X POST https://istanbul.megapayer.net \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "eth_chainId", "params": []}'

# Expected response:
# {"jsonrpc":"2.0","result":"0x4f10","id":1}
# 0x4f10 in hex = 20240 in decimal
```

### Network Status Check

```bash
# Check if network is online
curl -X POST https://istanbul.megapayer.net \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "eth_blockNumber", "params": []}'

# Get latest block
curl -X POST https://istanbul.megapayer.net \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "eth_getBlockByNumber", "params": ["latest", false]}'
```

---

## 💰 Token Information

### MPC Token Details

- **Token Name:** Megapayer Coin
- **Token Symbol:** MPC
- **Decimals:** 18
- **Total Supply:** 1,000,000,000 MPC
- **Chain ID:** 20240
- **Standard:** Native token (not ERC-20)

### Supply Distribution

**Total:** 1,000,000,000 MPC

1. `0x524aAdF5104493fb3112CCD64F20725b9D0D61bc` → **250,000,000 MPC**
2. `0x54B0C17AaE30BE8c56Fb8D1DC84213F192035E55` → **250,000,000 MPC**
3. `0x182090B7Df5B848b76F7e019A0Ac7300569982B0` → **500,000,000 MPC**

---

## 🛡️ Validator Information

### Validator Count

- **Total Validators:** 4 (configured in genesis)
- **Active Validators:** 3 (minimum required)
- **Minimum Validator Count:** 3

### Validator Nodes

1. **Node 1 (Alice)** - VPS 1 - Port 30333, RPC 9944
2. **Node 2 (Bob)** - VPS 1 - Port 30334, RPC 9945
3. **Node 3 (Charlie)** - VPS 2 - Port 30333, RPC 9944
4. **Node 4 (Dave)** - Not running (optional)

---

## 🔧 Development Tools

### Hardhat Configuration

```javascript
// hardhat.config.js
module.exports = {
  networks: {
    megapayerTestnet: {
      url: "https://istanbul.megapayer.net",
      chainId: 20240,
      accounts: [process.env.PRIVATE_KEY]
    }
  }
};
```

### Web3.js / Ethers.js Connection

```javascript
// Web3.js
const Web3 = require('web3');
const web3 = new Web3('https://istanbul.megapayer.net');

// Ethers.js
const { ethers } = require('ethers');
const provider = new ethers.providers.JsonRpcProvider('https://istanbul.megapayer.net');
```

### Truffle Configuration

```javascript
// truffle-config.js
module.exports = {
  networks: {
    megapayerTestnet: {
      host: "istanbul.megapayer.net",
      port: 443,
      network_id: 20240,
      gasPrice: 20000000000
    }
  }
};
```

---

## 📊 Network Statistics

### Block Information

- **Block Time:** ~6 seconds (BABE)
- **Finality:** GRANDPA (deterministic, usually within 1-2 blocks)
- **Gas Price:** Dynamic (EVM compatible)

### Consensus Details

- **Block Production:** BABE (Blind Assignment for Blockchain Extension)
- **Finality:** GRANDPA (GHOST-based Recursive ANcestor Deriving Prefix Agreement)
- **Staking:** NPOS (Nominated Proof-of-Stake)
- **Validator Selection:** Based on stake

---

## 🔍 Explorer & Tools

### Block Explorer

- **URL:** (To be configured)
- **Alternative:** Use RPC calls or Polkadot.js Apps

### Polkadot.js Apps

Connect to custom endpoint:
```
wss://istanbul.megapayer.net
```
or
```
ws://195.35.0.211:9944
```

---

## 📝 Quick Reference

### Network Summary

| Property | Value |
|----------|-------|
| Network Name | MegapayerIstanbul test network |
| Chain ID | 20240 (0x4f10) |
| Token Symbol | MPC |
| Token Decimals | 18 |
| Total Supply | 1,000,000,000 MPC |
| RPC URL | https://istanbul.megapayer.net |
| WebSocket URL | wss://istanbul.megapayer.net |
| Validators | 3 active (minimum: 3) |
| Consensus | BABE + GRANDPA (NPOS) |
| EVM Compatible | Yes |

### RPC Methods Supported

All standard Ethereum JSON-RPC methods are supported:
- `eth_blockNumber`
- `eth_getBalance`
- `eth_sendTransaction`
- `eth_sendRawTransaction`
- `eth_getTransactionReceipt`
- `eth_call`
- `eth_estimateGas`
- `eth_getCode`
- And more...

---

**Last Updated:** 2026-01-19  
**Network Status:** ✅ Operational

