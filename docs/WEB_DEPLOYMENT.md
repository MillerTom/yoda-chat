# Jan Web App Deployment Guide

This guide explains how to deploy Jan as a web application to various hosting platforms.

## Overview

Jan can run as a **standalone web application** in addition to its desktop app. The web version:
- Runs entirely in the browser
- Requires no installation
- Can be deployed to any static hosting service
- Can connect to remote AI APIs (OpenAI, Claude, etc.)
- Does **not** include local model execution (desktop-only feature)

## Building the Web App

### Development Build

```bash
# Using Make
make dev-web-app

# OR using Yarn
yarn install
yarn build:core
yarn build:extensions-web
yarn dev:web-app
```

This starts a development server at `http://localhost:3001`

### Production Build

```bash
# Using Make
make build-web-app

# OR using Yarn
yarn install
yarn build:core
yarn build:extensions-web
yarn build:web-app
```

The production-ready files will be in `web-app/dist-web/`

## Configuration

### Environment Variables

Create a `.env` file in the `web-app` directory:

```env
# Google Analytics (Optional)
GA_MEASUREMENT_ID=G-XXXXXXXXXX

# PostHog Analytics (Optional)
POSTHOG_KEY=phc_xxxxxxxxxxxxxxxxxxxxx
POSTHOG_HOST=https://app.posthog.com

# Model Catalog URL (Optional)
MODEL_CATALOG_URL=https://your-model-catalog-url.com
```

### Build Configuration

The web build is configured in `web-app/vite.config.web.ts`:

- **Output directory**: `dist-web/`
- **Build target**: Modern browsers (esnext)
- **Features**: SPA mode with client-side routing
- **External packages**: Tauri dependencies are excluded

## Deployment Options

### Option 1: Netlify

**One-Click Deploy:**

[![Deploy to Netlify](https://www.netlify.com/img/deploy/button.svg)](https://app.netlify.com/start)

**Manual Deploy:**

1. Build the app locally:
   ```bash
   make build-web-app
   ```

2. Deploy via Netlify CLI:
   ```bash
   npm install -g netlify-cli
   netlify deploy --prod --dir=web-app/dist-web
   ```

**Netlify Configuration** (`netlify.toml`):

```toml
[build]
  command = "yarn build:web-app"
  publish = "web-app/dist-web"

[build.environment]
  NODE_VERSION = "20"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```

### Option 2: Vercel

**Deploy Steps:**

1. Install Vercel CLI:
   ```bash
   npm install -g vercel
   ```

2. Build and deploy:
   ```bash
   make build-web-app
   vercel --prod
   ```

**Vercel Configuration** (`vercel.json`):

```json
{
  "version": 2,
  "buildCommand": "yarn build:web-app",
  "outputDirectory": "web-app/dist-web",
  "framework": null,
  "routes": [
    {
      "src": "/(.*)",
      "dest": "/"
    }
  ]
}
```

### Option 3: GitHub Pages

1. Build the app:
   ```bash
   make build-web-app
   ```

2. Deploy to GitHub Pages:
   ```bash
   # Install gh-pages
   npm install -g gh-pages

   # Deploy
   gh-pages -d web-app/dist-web
   ```

3. Enable GitHub Pages in repository settings:
   - Go to Settings → Pages
   - Select `gh-pages` branch
   - Save

**GitHub Actions** (`.github/workflows/deploy-web.yml`):

```yaml
name: Deploy Web App

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '20'
          
      - name: Install dependencies
        run: yarn install
        
      - name: Build web app
        run: make build-web-app
        
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./web-app/dist-web
```

### Option 4: Docker

**Dockerfile for Web App:**

```dockerfile
# Build stage
FROM node:20-alpine AS builder

WORKDIR /app

# Copy package files
COPY package.json yarn.lock ./
COPY core/package.json ./core/
COPY extensions-web/package.json ./extensions-web/
COPY web-app/package.json ./web-app/

# Install dependencies
RUN corepack enable && \
    corepack prepare yarn@4.5.3 --activate && \
    yarn install

# Copy source code
COPY . .

# Build the application
RUN yarn build:core && \
    yarn build:extensions-web && \
    yarn build:web-app

# Production stage
FROM nginx:alpine

# Copy built files
COPY --from=builder /app/web-app/dist-web /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose port
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
```

**nginx.conf:**

```nginx
server {
    listen 80;
    server_name localhost;
    root /usr/share/nginx/html;
    index index.html;

    # Enable gzip compression
    gzip on;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;

    # SPA routing - serve index.html for all routes
    location / {
        try_files $uri $uri/ /index.html;
    }

    # Cache static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
}
```

**Build and Run:**

```bash
# Build the Docker image
docker build -t jan-web-app -f Dockerfile.web .

# Run the container
docker run -p 3001:80 jan-web-app

# Access at http://localhost:3001
```

### Option 5: AWS S3 + CloudFront

1. Build the app:
   ```bash
   make build-web-app
   ```

2. Create S3 bucket:
   ```bash
   aws s3 mb s3://jan-web-app
   ```

3. Upload files:
   ```bash
   aws s3 sync web-app/dist-web/ s3://jan-web-app --delete
   ```

4. Configure S3 for static website hosting:
   - Enable static website hosting
   - Set index document: `index.html`
   - Set error document: `index.html`

5. Create CloudFront distribution:
   - Origin: Your S3 bucket
   - Default root object: `index.html`
   - Custom error responses: 404 → /index.html (200)

### Option 6: Self-Hosted (VPS)

**Setup on Ubuntu/Debian:**

```bash
# Install dependencies
sudo apt update
sudo apt install nginx certbot python3-certbot-nginx

# Build the app (on your local machine)
make build-web-app

# Upload to server
scp -r web-app/dist-web/* user@your-server:/var/www/jan/

# Configure nginx
sudo nano /etc/nginx/sites-available/jan
```

**Nginx configuration:**

```nginx
server {
    listen 80;
    server_name your-domain.com;

    root /var/www/jan;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    # Enable gzip
    gzip on;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml;

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
}
```

**Enable site and SSL:**

```bash
# Enable site
sudo ln -s /etc/nginx/sites-available/jan /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx

# Setup SSL with Let's Encrypt
sudo certbot --nginx -d your-domain.com
```

## Post-Deployment Configuration

### 1. API Keys Setup

Users will need to configure their API keys in the deployed app:

- Go to Settings → API Keys
- Add keys for OpenAI, Anthropic, Groq, etc.
- Keys are stored in browser localStorage (never sent to your server)

### 2. Model Catalog

If you want to provide a custom model catalog:

1. Set `MODEL_CATALOG_URL` environment variable during build
2. Host a JSON file with model definitions
3. Follow the schema in `src-tauri/static/models.json`

### 3. Analytics

If you configured Google Analytics or PostHog:

- Ensure the tracking IDs are correct
- Test in production environment
- Check privacy compliance (GDPR, etc.)

## Performance Optimization

### Build Optimizations

The production build already includes:
- Code splitting
- Tree shaking
- Minification
- Compression

### Additional Optimizations

1. **Enable CDN**: Use CloudFront, Cloudflare, or similar

2. **Configure Caching**:
   ```nginx
   # Cache static assets for 1 year
   location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
       expires 1y;
       add_header Cache-Control "public, immutable";
   }
   ```

3. **Enable Compression**:
   ```nginx
   gzip on;
   gzip_comp_level 6;
   gzip_types text/plain text/css application/json application/javascript text/xml application/xml;
   ```

4. **Use HTTP/2**:
   ```nginx
   listen 443 ssl http2;
   ```

## Monitoring and Maintenance

### Health Checks

Create a health check endpoint:

```javascript
// In your nginx config
location /health {
    return 200 "OK\n";
    add_header Content-Type text/plain;
}
```

### Error Tracking

1. **Browser Console Errors**: Monitor via analytics
2. **Sentry Integration**: Add Sentry for error tracking
3. **Custom Logging**: Implement custom error logging

### Updates

To deploy updates:

```bash
# 1. Build new version
make build-web-app

# 2. Deploy to your platform
# (Netlify/Vercel deploy automatically on git push)

# 3. For manual deployments:
rsync -av web-app/dist-web/ user@server:/var/www/jan/
```

## Troubleshooting

### "Page not found" on refresh

**Problem**: Direct navigation to routes fails

**Solution**: Configure SPA fallback to index.html

- **Netlify**: Use `_redirects` file
- **Nginx**: Use `try_files $uri $uri/ /index.html;`
- **Apache**: Use `.htaccess` with RewriteRules

### "Module not found" errors

**Problem**: Missing dependencies or build issues

**Solution**:
```bash
# Clean and rebuild
make clean
yarn install
make build-web-app
```

### Performance issues

**Problem**: Slow loading times

**Solution**:
1. Enable gzip/brotli compression
2. Use CDN
3. Enable browser caching
4. Check bundle size with `yarn build:web-app --analyze`

### API connection issues

**Problem**: Can't connect to AI APIs

**Solution**:
1. Check CORS settings on API provider
2. Verify API keys are configured correctly
3. Check browser console for errors
4. Ensure HTTPS is enabled (required for some APIs)

## Security Considerations

1. **HTTPS Only**: Always use HTTPS in production
2. **API Keys**: Never commit API keys to the repository
3. **CSP Headers**: Configure Content Security Policy
4. **CORS**: Properly configure Cross-Origin Resource Sharing
5. **Updates**: Keep dependencies updated

## Comparison: Desktop vs Web

| Feature | Desktop App | Web App |
|---------|-------------|---------|
| Installation | Required | None |
| Local Models | ✅ Yes | ❌ No |
| Remote APIs | ✅ Yes | ✅ Yes |
| File System | ✅ Full access | ❌ Limited |
| GPU Acceleration | ✅ Yes | ❌ No |
| Privacy | 🔒 100% Local | 🌐 Browser-based |
| Updates | Auto-update | Auto-load |
| Offline | ✅ Full support | 🔌 Online only |
| Cross-platform | Win/Mac/Linux | Any browser |

## Resources

- **Main Documentation**: [README.md](../README.md)
- **Collaboration Guide**: [COLLABORATION_GUIDE.md](../COLLABORATION_GUIDE.md)
- **Quick Start**: [QUICKSTART.md](../QUICKSTART.md)
- **Web App Vite Config**: [web-app/vite.config.web.ts](../web-app/vite.config.web.ts)

## Support

For deployment issues:
- Check [GitHub Issues](https://github.com/janhq/jan/issues)
- Ask in [Discord](https://discord.gg/FTk2MvZwJH) #deployment channel
- Review [Documentation](https://jan.ai/docs)

---

**Happy deploying!** 🚀
