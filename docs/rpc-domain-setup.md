# Setting Up Domain RPC URL for Megapayer Testnet

This guide shows how to set up a domain name (e.g., `rpc.megapayer-testnet.net`) to access the blockchain RPC endpoint.

## Prerequisites

- Domain name pointing to VPS 1 IP (195.35.0.211)
- SSH access to VPS 1
- Node 1 running on port 9944

## Step 1: Install Nginx

```bash
sudo apt update
sudo apt install -y nginx
```

## Step 2: Configure Nginx Reverse Proxy

Create Nginx configuration file:

```bash
sudo nano /etc/nginx/sites-available/megapayer-rpc
```

Add the following configuration (replace `your-domain.com` with your actual domain):

```nginx
server {
    listen 80;
    server_name rpc.megapayer-testnet.net;  # Replace with your domain

    # Increase timeouts for RPC calls
    proxy_connect_timeout 300s;
    proxy_send_timeout 300s;
    proxy_read_timeout 300s;
    send_timeout 300s;

    # Increase body size for large requests
    client_max_body_size 10M;

    location / {
        proxy_pass http://127.0.0.1:9944;
        proxy_http_version 1.1;
        
        # WebSocket support
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        
        # Standard proxy headers
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # CORS headers (if needed)
        add_header 'Access-Control-Allow-Origin' '*' always;
        add_header 'Access-Control-Allow-Methods' 'GET, POST, OPTIONS' always;
        add_header 'Access-Control-Allow-Headers' 'DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range' always;
        
        # Handle preflight requests
        if ($request_method = 'OPTIONS') {
            add_header 'Access-Control-Allow-Origin' '*';
            add_header 'Access-Control-Allow-Methods' 'GET, POST, OPTIONS';
            add_header 'Access-Control-Allow-Headers' 'DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range';
            add_header 'Access-Control-Max-Age' 1728000;
            add_header 'Content-Type' 'text/plain; charset=utf-8';
            add_header 'Content-Length' 0;
            return 204;
        }
    }
}
```

Enable the site:

```bash
sudo ln -s /etc/nginx/sites-available/megapayer-rpc /etc/nginx/sites-enabled/
sudo nginx -t  # Test configuration
sudo systemctl reload nginx
```

## Step 3: Install Certbot for SSL

```bash
sudo apt install -y certbot python3-certbot-nginx
```

## Step 4: Obtain SSL Certificate

```bash
sudo certbot --nginx -d rpc.megapayer-testnet.net
```

Follow the prompts:
- Enter your email address
- Agree to terms of service
- Choose whether to redirect HTTP to HTTPS (recommended: Yes)

## Step 5: Configure Firewall

```bash
# Allow HTTP and HTTPS
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw status
```

## Step 6: Verify Setup

Test the RPC endpoint:

```bash
# Test HTTP (should redirect to HTTPS)
curl http://rpc.megapayer-testnet.net

# Test HTTPS
curl -X POST https://rpc.megapayer-testnet.net \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "eth_chainId", "params": []}'

# Expected response: {"jsonrpc":"2.0","result":"0x4f10","id":1}
```

## Step 7: Update MetaMask Configuration

Use the new domain in MetaMask:

**Network Name:** MegapayerIstanbul test network  
**RPC URL:** `https://rpc.megapayer-testnet.net`  
**Chain ID:** `20240`  
**Currency Symbol:** `MPC`

## Optional: Set Up Multiple RPC Endpoints

If you want separate endpoints for Node 1 and Node 2:

### Node 1 RPC (Port 9944)
```nginx
server {
    listen 443 ssl;
    server_name rpc1.megapayer-testnet.net;
    
    ssl_certificate /etc/letsencrypt/live/rpc1.megapayer-testnet.net/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/rpc1.megapayer-testnet.net/privkey.pem;
    
    location / {
        proxy_pass http://127.0.0.1:9944;
        # ... same proxy settings as above
    }
}
```

### Node 2 RPC (Port 9945)
```nginx
server {
    listen 443 ssl;
    server_name rpc2.megapayer-testnet.net;
    
    ssl_certificate /etc/letsencrypt/live/rpc2.megapayer-testnet.net/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/rpc2.megapayer-testnet.net/privkey.pem;
    
    location / {
        proxy_pass http://127.0.0.1:9945;
        # ... same proxy settings as above
    }
}
```

## SSL Certificate Auto-Renewal

Certbot sets up automatic renewal. Verify it's working:

```bash
sudo certbot renew --dry-run
```

## Troubleshooting

### Check Nginx Status
```bash
sudo systemctl status nginx
sudo nginx -t
```

### Check Nginx Logs
```bash
sudo tail -f /var/log/nginx/error.log
sudo tail -f /var/log/nginx/access.log
```

### Test RPC Connection
```bash
# Direct connection to node
curl -X POST http://127.0.0.1:9944 \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "eth_chainId", "params": []}'

# Through domain
curl -X POST https://rpc.megapayer-testnet.net \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "eth_chainId", "params": []}'
```

### Check DNS Resolution
```bash
nslookup rpc.megapayer-testnet.net
dig rpc.megapayer-testnet.net
```

## Security Notes

- Keep SSL certificates updated (auto-renewal is configured)
- Consider rate limiting for production use
- Monitor access logs for suspicious activity
- Use firewall rules to restrict access if needed

