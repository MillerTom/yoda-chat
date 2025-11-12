# Jan Codebase Analysis Summary

## Your Questions Answered

### 1. What is this codebase?
**Jan** is a powerful open-source AI assistant (fork of [janhq/jan](https://github.com/janhq/jan)) that can run 100% locally on your device. It's like ChatGPT, but private and under your complete control.

### 2. Is it a Tauri desktop app?
**Yes!** Jan is built with Tauri, combining:
- **Frontend**: React + TypeScript (Vite, TanStack Router, Zustand)
- **Backend**: Rust (Tauri framework)
- **Platform**: Cross-platform (Windows, macOS, Linux)

### 3. Can it run as a web app?
**Absolutely!** Jan has first-class web app support:
- ✅ Can be deployed as a standalone web application
- ✅ Runs in any modern browser
- ✅ No installation required
- ✅ Already has build scripts: `make dev-web-app` and `make build-web-app`
- ✅ Configuration ready in `web-app/vite.config.web.ts`

**Limitations of web version:**
- ❌ No local AI model execution (desktop-only feature via LlamaCPP)
- ✅ Can still connect to remote APIs (OpenAI, Claude, Groq, etc.)

### 4. How can we collaborate on this?
Standard GitHub workflow is ready! See [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) for complete details.

---

## Architecture Overview

```
┌─────────────────────────────────────────────┐
│         Web App (React + TypeScript)        │
│              (web-app/)                     │
│  • Chat UI, Model Management, Settings      │
└─────────────┬───────────────────────────────┘
              │
              ├── Desktop Mode: Uses Tauri IPC
              │   ↓
              │   Rust Backend (src-tauri/)
              │   • File system, GPU, Local models
              │
              └── Web Mode: Browser only
                  • Remote APIs, Cloud features
```

### Component Structure

```
yoda-chat/
├── web-app/           # React frontend (UI)
├── core/              # TypeScript SDK (shared APIs)
├── extensions/        # Desktop-specific features
├── extensions-web/    # Web-specific features
├── src-tauri/         # Rust backend (desktop only)
├── docs/              # Documentation
├── Makefile           # Build automation
└── package.json       # Monorepo configuration
```

---

## Getting Started (Quick Commands)

### Desktop App Development
```bash
git clone https://github.com/MillerTom/yoda-chat.git
cd yoda-chat
make dev
```

### Web App Development
```bash
git clone https://github.com/MillerTom/yoda-chat.git
cd yoda-chat
make dev-web-app
```

### Web App Production Build
```bash
make build-web-app
# Output: web-app/dist-web/
# Deploy to: Netlify, Vercel, GitHub Pages, your server, etc.
```

---

## Key Features

### Desktop App (Full Features)
- ✅ Run AI models locally (Llama, Gemma, Qwen, etc.)
- ✅ GPU acceleration support
- ✅ Full file system access
- ✅ System integration
- ✅ Auto-updater
- ✅ Connect to remote APIs

### Web App (Browser-Based)
- ✅ No installation required
- ✅ Connect to remote APIs (OpenAI, Claude, etc.)
- ✅ Cross-platform (any browser)
- ✅ Easy deployment
- ❌ No local model execution
- ❌ Limited file system access

---

## Technology Stack

### Frontend
- **Framework**: React 19
- **Language**: TypeScript
- **Build Tool**: Vite
- **Routing**: TanStack Router
- **State**: Zustand
- **Styling**: Tailwind CSS
- **UI Components**: Radix UI

### Backend (Desktop Only)
- **Framework**: Tauri 2.x
- **Language**: Rust
- **Features**: File system, process management, hardware integration

### Build System
- **Package Manager**: Yarn 4 (workspaces)
- **Build Tool**: Make + Vite
- **Testing**: Vitest
- **Linting**: ESLint + Prettier (TS/JS), Clippy (Rust)

---

## Collaboration Workflow

### 1. Fork & Clone
```bash
git clone https://github.com/YOUR-USERNAME/yoda-chat.git
cd yoda-chat
```

### 2. Create Branch
```bash
git checkout -b feature/your-feature
```

### 3. Make Changes
Choose your area:
- **Frontend/UI**: `web-app/src/`
- **Core APIs**: `core/`
- **Extensions**: `extensions/` or `extensions-web/`
- **Backend**: `src-tauri/` (Rust, desktop only)

### 4. Test
```bash
yarn lint
yarn test
yarn dev          # Desktop
yarn dev:web-app  # Web
```

### 5. Submit PR
- Target: `dev` branch (not `main`)
- Include: Clear description, screenshots if UI change
- Wait for review

---

## Documentation Created

I've created three comprehensive guides for you:

1. **[QUICKSTART.md](./QUICKSTART.md)**
   - 5-minute getting started guide
   - Setup for desktop and web
   - Common issues and solutions
   - First steps for new contributors

2. **[COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md)**
   - Complete collaboration workflow
   - Git branching strategy
   - Component-specific guides
   - Best practices and standards
   - Deployment options

3. **[docs/WEB_DEPLOYMENT.md](./docs/WEB_DEPLOYMENT.md)**
   - Web app deployment guide
   - Multiple hosting options (Netlify, Vercel, Docker, etc.)
   - Configuration and optimization
   - Performance and security
   - Troubleshooting

---

## Available Make Commands

```bash
# Development
make dev              # Desktop app development
make dev-web-app      # Web app development

# Building
make build            # Build desktop app
make build-web-app    # Build web app

# Testing
make test             # Run all tests
make lint             # Lint code

# Cleaning
make clean            # Clean all build artifacts
```

Or use Yarn directly:
```bash
yarn dev              # Desktop
yarn dev:web-app      # Web
yarn build:web-app    # Build web
yarn test             # Test
yarn lint             # Lint
```

---

## Web Deployment Options

Jan can be deployed to:

### Static Hosting
- **Netlify**: One-click deploy
- **Vercel**: Automatic deployments
- **GitHub Pages**: Free hosting
- **Cloudflare Pages**: Global CDN

### Container
- **Docker**: Full containerization
- **Kubernetes**: Scalable deployment

### Self-Hosted
- **VPS**: Full control (Ubuntu/Debian + Nginx)
- **AWS S3**: S3 + CloudFront
- **Azure**: Static Web Apps
- **GCP**: Cloud Storage + Load Balancer

See [docs/WEB_DEPLOYMENT.md](./docs/WEB_DEPLOYMENT.md) for detailed instructions.

---

## Development Environment

### Prerequisites
- **Node.js**: ≥ 20.0.0
- **Yarn**: ≥ 1.22.0 (or use corepack)
- **Rust**: Latest stable (desktop only)
- **Make**: ≥ 3.81 (optional but recommended)

### IDE Recommendations
- **VS Code** with extensions:
  - ESLint
  - Prettier
  - Rust Analyzer (for src-tauri)
  - Tailwind CSS IntelliSense

---

## Testing Strategy

### Frontend Tests
```bash
cd web-app
yarn test
```

### Core SDK Tests
```bash
cd core
yarn test
```

### Rust Tests (Desktop)
```bash
cd src-tauri
cargo test
```

### Integration Tests
```bash
make test  # Runs all tests
```

---

## Common Development Tasks

### Adding a New Feature
1. Create issue on GitHub
2. Create feature branch
3. Implement feature
4. Write tests
5. Update documentation
6. Submit PR to `dev` branch

### Fixing a Bug
1. Reproduce the bug
2. Create fix branch
3. Make minimal changes
4. Test the fix
5. Submit PR with clear description

### Updating Dependencies
```bash
yarn upgrade-interactive
make test  # Verify everything works
```

---

## Project Health

### Existing Setup
- ✅ Monorepo with Yarn workspaces
- ✅ Desktop app (Tauri)
- ✅ Web app support (Vite)
- ✅ Build automation (Make)
- ✅ Testing framework (Vitest)
- ✅ Linting (ESLint, Clippy)
- ✅ CI/CD ready structure
- ✅ Documentation (existing + new guides)

### What's Working
- ✅ Development builds
- ✅ Production builds
- ✅ Testing infrastructure
- ✅ Extension system
- ✅ Web deployment ready

---

## Next Steps for Collaboration

### For You (Project Owner)
1. Review the new documentation files
2. Try running: `make dev-web-app`
3. Explore the codebase structure
4. Create issues for features you want
5. Review and merge this PR

### For Contributors
1. Read [QUICKSTART.md](./QUICKSTART.md)
2. Read [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md)
3. Pick an issue or propose a feature
4. Follow the contribution workflow
5. Submit PRs for review

---

## Resources

### Documentation
- 📚 [Main README](./README.md) - Project overview
- 🚀 [Quick Start Guide](./QUICKSTART.md) - Get started in 5 minutes
- 🤝 [Collaboration Guide](./COLLABORATION_GUIDE.md) - Complete contributor guide
- 🌐 [Web Deployment](./docs/WEB_DEPLOYMENT.md) - Deploy as web app
- 📖 [Official Docs](https://jan.ai/docs) - End-user documentation

### Community
- 💬 [Discord](https://discord.gg/FTk2MvZwJH) - Community chat
- 🐛 [GitHub Issues](https://github.com/janhq/jan/issues) - Bug reports
- 💡 [GitHub Discussions](https://github.com/janhq/jan/discussions) - Questions & ideas

### Component-Specific
- [web-app/CONTRIBUTING.md](./web-app/CONTRIBUTING.md) - Frontend guide
- [core/CONTRIBUTING.md](./core/CONTRIBUTING.md) - Core SDK guide
- [extensions/CONTRIBUTING.md](./extensions/CONTRIBUTING.md) - Extensions guide
- [src-tauri/CONTRIBUTING.md](./src-tauri/CONTRIBUTING.md) - Backend guide

---

## Summary

**Jan is a mature, well-structured project that:**
1. ✅ Is a Tauri desktop app
2. ✅ Can run as a web app
3. ✅ Has excellent build tooling
4. ✅ Is ready for collaboration
5. ✅ Has comprehensive documentation

**You can:**
- Run it locally (desktop or web) in minutes
- Deploy it as a web app to any hosting platform
- Contribute through standard GitHub workflow
- Build new features using the extension system

**The new documentation provides:**
- Quick start instructions
- Complete collaboration workflow
- Web deployment guides
- Troubleshooting help
- Best practices

**Let's build together!** 🚀
