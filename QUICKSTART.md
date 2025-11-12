# Jan Quick Start Guide

Get up and running with Jan in 5 minutes!

## What is Jan?

Jan is an open-source AI assistant that runs 100% locally on your device. It's like ChatGPT, but private and under your complete control.

## Two Ways to Run Jan

### 🖥️ Desktop App (Full Features)
- Runs AI models locally on your computer
- Full access to file system and GPU acceleration
- Best for privacy and offline use

### 🌐 Web App (Browser-Based)
- Runs in any modern browser
- No installation required
- Great for trying out or remote access

---

## Quick Start: Desktop App

### Prerequisites
- Node.js 20+ ([download](https://nodejs.org))
- Rust ([install](https://rustup.rs))

### 3 Commands to Start
```bash
git clone https://github.com/MillerTom/yoda-chat.git
cd yoda-chat
make dev
```

That's it! Jan will open in a desktop window.

### What Just Happened?
1. ✅ Installed all dependencies
2. ✅ Built the core SDK and extensions
3. ✅ Started the development server
4. ✅ Launched the Tauri desktop app

---

## Quick Start: Web App

### Prerequisites
- Node.js 20+ ([download](https://nodejs.org))

### 3 Commands to Start
```bash
git clone https://github.com/MillerTom/yoda-chat.git
cd yoda-chat
make dev-web-app
```

Then open your browser to: **http://localhost:3001**

### What Just Happened?
1. ✅ Installed all dependencies
2. ✅ Built web-specific extensions
3. ✅ Started the Vite dev server
4. ✅ Ready in your browser!

---

## First Time Setup (No Make)

If you don't have `make` installed, use these commands instead:

### For Desktop App:
```bash
# 1. Clone and enter directory
git clone https://github.com/MillerTom/yoda-chat.git
cd yoda-chat

# 2. Install dependencies
yarn install

# 3. Build required components
yarn build:tauri:plugin:api
yarn build:core
yarn build:extensions

# 4. Run the app
yarn dev
```

### For Web App:
```bash
# 1. Clone and enter directory
git clone https://github.com/MillerTom/yoda-chat.git
cd yoda-chat

# 2. Install dependencies
yarn install

# 3. Build required components
yarn build:core
yarn build:extensions-web

# 4. Run the web app
yarn dev:web-app
```

---

## Testing Your Setup

### Desktop App Checklist
- [ ] Desktop window opens
- [ ] You see the Jan chat interface
- [ ] No error messages in terminal
- [ ] Console shows "App ready"

### Web App Checklist
- [ ] Browser opens to localhost:3001
- [ ] You see the Jan chat interface
- [ ] No error messages in terminal
- [ ] No browser console errors

---

## Common Setup Issues

### "command not found: make"

**On macOS:**
```bash
xcode-select --install
```

**On Linux:**
```bash
sudo apt-get install build-essential  # Debian/Ubuntu
sudo yum install make                 # RedHat/CentOS
```

**On Windows:**
Use the yarn commands instead (no Make required)

### "Rust not found"

Only needed for desktop app. Install from: https://rustup.rs

```bash
# Verify installation
rustc --version
cargo --version
```

### "Node version too old"

```bash
# Check your version
node --version

# Should be 20.0.0 or higher
# Download latest from: https://nodejs.org
```

### "Port 3001 already in use"

```bash
# Find and kill the process
lsof -ti:3001 | xargs kill -9

# Then try again
make dev-web-app
```

### "Build failed" or "ENOENT errors"

```bash
# Clean everything and start fresh
make clean
make dev

# OR without make
find . -name "node_modules" -type d -prune -exec rm -rf '{}' +
yarn install
yarn build:core
yarn build:extensions
yarn dev
```

---

## Next Steps

### Explore Features
1. **Chat with AI**: Use the chat interface
2. **Try Models**: Download and run local models (desktop)
3. **Connect APIs**: Add OpenAI, Claude, or other API keys
4. **Customize**: Explore settings and preferences

### Make Your First Change
1. **Pick a file**: Try editing a UI component in `web-app/src/`
2. **Save**: Changes auto-reload in development mode
3. **Test**: Verify your changes work
4. **Learn**: Explore the codebase further

### Join the Community
- **Discord**: [discord.gg/FTk2MvZwJH](https://discord.gg/FTk2MvZwJH)
- **GitHub Issues**: Report bugs or request features
- **Documentation**: [jan.ai/docs](https://jan.ai/docs)

---

## Development Workflow

### Making Changes

1. **Edit code** in your favorite editor
2. **Save file** - changes auto-reload
3. **Check browser/app** - see your changes live
4. **Repeat** until satisfied

### Hot Reload
- Frontend changes reload automatically
- Backend changes require restart (desktop only)
- Extension changes require rebuild

### Useful Dev Commands

```bash
# Check for errors
yarn lint

# Run tests
yarn test

# Build for production
make build              # Desktop
make build-web-app     # Web

# Clean and rebuild
make clean
make dev
```

---

## Understanding the Structure

### Key Directories

```
yoda-chat/
├── web-app/         ← Frontend code (React/TypeScript)
├── core/            ← Shared APIs and SDK
├── extensions/      ← Desktop features
├── extensions-web/  ← Web-specific features
├── src-tauri/       ← Rust backend (desktop only)
├── docs/            ← Documentation website
└── Makefile         ← Build automation
```

### Where to Make Changes

**Want to change the UI?** → `web-app/src/components/`
**Want to add a feature?** → `extensions/` or `extensions-web/`
**Want to modify core logic?** → `core/`
**Want to work on system integration?** → `src-tauri/` (desktop)

---

## Pro Tips

### Speed Up Development

```bash
# Terminal 1: Watch core changes
cd core
yarn build --watch

# Terminal 2: Watch extensions
cd extensions-web
yarn build --watch

# Terminal 3: Run dev server
yarn dev:web-app
```

### Debug Like a Pro

**Frontend debugging:**
- Open browser DevTools (F12)
- Check Console for errors
- Use React DevTools extension

**Backend debugging (desktop):**
- Check terminal output
- Look at Rust logs in src-tauri
- Use `cargo test` for Rust tests

### Stay Organized

```bash
# Create a branch for your work
git checkout -b my-feature

# Make small, focused commits
git add .
git commit -m "feat: add new button"

# Push when ready
git push origin my-feature
```

---

## Production Deployment

### Desktop App

```bash
# Build executable
make build

# Find your app in:
# - macOS: src-tauri/target/release/bundle/
# - Windows: src-tauri/target/release/
# - Linux: src-tauri/target/release/
```

### Web App

```bash
# Build static files
make build-web-app

# Deploy the dist-web folder to:
# - Netlify, Vercel, GitHub Pages
# - Your own server with nginx
# - Docker container

# Output is in: web-app/dist-web/
```

---

## Get Help

### Something not working?

1. **Check the logs**: Look at terminal output for errors
2. **Clean build**: Run `make clean` then try again
3. **Search issues**: Check GitHub for similar problems
4. **Ask the community**: Discord is very helpful

### Want to contribute?

See the full [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) for details on:
- Git workflow
- Code standards
- Testing requirements
- Pull request process

---

## Summary: Your First 5 Minutes

```bash
# If you have 'make':
git clone https://github.com/MillerTom/yoda-chat.git
cd yoda-chat
make dev-web-app        # Easiest to start with web

# If you don't have 'make':
git clone https://github.com/MillerTom/yoda-chat.git
cd yoda-chat
yarn install
yarn build:core
yarn build:extensions-web
yarn dev:web-app

# Open browser to http://localhost:3001
# Start exploring and making changes!
```

---

## Resources

- 📖 **Full Guide**: [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md)
- 🤝 **Contributing**: [CONTRIBUTING.md](./CONTRIBUTING.md)
- 📚 **Documentation**: [jan.ai/docs](https://jan.ai/docs)
- 💬 **Community**: [Discord](https://discord.gg/FTk2MvZwJH)
- 🐛 **Issues**: [GitHub Issues](https://github.com/janhq/jan/issues)

---

**Welcome to Jan! Let's build the future of local AI together.** 🚀
