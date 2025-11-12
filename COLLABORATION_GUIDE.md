# Jan Collaboration Guide

## Overview

**Jan** is a powerful open-source AI assistant that runs 100% locally on your device. Think of it as a private, local ChatGPT that you have complete control over. This guide will help you understand how to work on this project collaboratively.

## Quick Answer to Your Questions

### 1. Is this a Tauri Desktop App?
**Yes!** Jan is built with [Tauri](https://tauri.app/), which combines:
- A React frontend (TypeScript/JavaScript)
- A Rust backend for native system integration
- Cross-platform support (Windows, macOS, Linux)

### 2. Can it Run as a Web App?
**Yes!** Jan has excellent web app support already built-in. You can:
- Build and deploy it as a standalone web application
- Run it in any modern browser without installing the desktop app
- Deploy it to your own server or cloud platform

### 3. How to Collaborate?
This project uses a standard GitHub workflow. See the [Collaboration Workflow](#collaboration-workflow) section below.

---

## Architecture Overview

Jan consists of several key components:

```
┌─────────────────────────────────────────────────────────┐
│                  Web App (React/TypeScript)              │
│                     (web-app/)                           │
│  • User Interface                                        │
│  • Chat Interface                                        │
│  • Model Management                                      │
│  • Settings Pages                                        │
└────────────┬────────────────────────────┬────────────────┘
             │                            │
             │ imports                    │ imports
             ▼                            ▼
  ┌─────────────────────┐      ┌──────────────────────┐
  │    Core SDK         │      │    Extensions        │
  │     (core/)         │      │   (extensions/)      │
  │                     │      │                      │
  │ • TypeScript APIs   │◄─────│ • Assistants         │
  │ • Extension System  │ uses │ • Conversations      │
  │ • Event Bus         │      │ • Downloads          │
  │ • Type Definitions  │      │ • LlamaCPP           │
  └──────────┬──────────┘      └───────────┬──────────┘
             │                             │
             │    Desktop App Only         │
             └─────────────┬───────────────┘
                           │
                           ▼
                    Tauri IPC Layer
                 (Desktop App Only)
                           │
                           ▼
            ┌──────────────────────────────┐
            │   Tauri Backend (Rust)       │
            │      (src-tauri/)            │
            │                              │
            │ • File System Access         │
            │ • Process Management         │
            │ • Hardware Integration       │
            │ • System-level Features      │
            └──────────────────────────────┘
```

### Running Modes

**Desktop App Mode:**
- Uses Tauri for native system integration
- Full access to file system, GPU, and hardware
- Can run local AI models efficiently

**Web App Mode:**
- Pure browser-based application
- No Tauri dependency
- Can connect to remote API servers
- Suitable for cloud deployment

---

## Project Structure

```
jan/
├── web-app/              # React frontend (UI components)
├── core/                 # TypeScript SDK (shared APIs)
├── extensions/           # Desktop extensions
├── extensions-web/       # Web-specific extensions
├── src-tauri/            # Rust backend (desktop only)
│   ├── src/              # Tauri core code
│   └── plugins/          # Hardware & LlamaCPP plugins
├── docs/                 # Documentation website
├── scripts/              # Build utilities
├── Makefile              # Build automation
├── package.json          # Root workspace config
└── README.md             # Main documentation
```

---

## Getting Started

### Prerequisites

- **Node.js** ≥ 20.0.0
- **Yarn** ≥ 1.22.0 (or use the packaged version with corepack)
- **Make** ≥ 3.81 (optional, but recommended)
- **Rust** (only needed for desktop app development)

### Quick Setup

#### Option 1: Using Make (Recommended)
```bash
# Clone the repository
git clone https://github.com/MillerTom/yoda-chat.git
cd yoda-chat

# Setup and run desktop app
make dev

# OR run as web app
make dev-web-app
```

#### Option 2: Using Yarn Directly
```bash
# Install dependencies
yarn install

# Build core and extensions
yarn build:core
yarn build:extensions

# Run desktop app
yarn dev

# OR run as web app
yarn dev:web-app
```

### Development Modes

#### Desktop App Development
```bash
# Full desktop development (with Tauri)
make dev
# OR
yarn dev
```

This will:
1. Build the core SDK and extensions
2. Start the Vite dev server
3. Launch the Tauri desktop app

#### Web App Development
```bash
# Web-only development (no Tauri)
make dev-web-app
# OR
yarn dev:web-app
```

This will:
1. Build web-specific extensions
2. Start the Vite dev server on port 3001
3. Open in your browser at `http://localhost:3001`

#### Web App Production Build
```bash
# Build for web deployment
make build-web-app
# OR
yarn build:web-app

# Serve the built web app
make serve-web-app
# OR
yarn serve:web-app
```

The web app will be built to `web-app/dist-web/` and can be deployed to any static hosting service.

---

## Collaboration Workflow

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR-USERNAME/yoda-chat.git
cd yoda-chat

# Add upstream remote
git remote add upstream https://github.com/MillerTom/yoda-chat.git
```

### 2. Create a Branch

```bash
# Always work on a feature branch
git checkout -b feature/your-feature-name
```

Branch naming conventions:
- `feature/*` - New features
- `fix/*` - Bug fixes
- `docs/*` - Documentation updates
- `refactor/*` - Code refactoring

### 3. Make Changes

Choose the component you want to work on:

- **UI/Frontend** → Work in `web-app/src/`
- **Core APIs** → Work in `core/`
- **Extensions** → Work in `extensions/` or `extensions-web/`
- **Backend** → Work in `src-tauri/` (Rust, desktop only)

### 4. Test Your Changes

```bash
# Lint your code
yarn lint

# Run tests
yarn test

# Test desktop app
yarn dev

# Test web app
yarn dev:web-app
```

### 5. Commit and Push

```bash
# Stage your changes
git add .

# Commit with a descriptive message
git commit -m "feat: add new feature description"

# Push to your fork
git push origin feature/your-feature-name
```

Commit message conventions:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `test:` - Test updates
- `chore:` - Build/tooling changes

### 6. Create Pull Request

1. Go to GitHub and create a Pull Request
2. Target the `dev` branch (not `main`)
3. Fill in the PR template with:
   - What changes you made
   - Why you made them
   - How to test them
4. Wait for review and address feedback

### 7. Stay Updated

```bash
# Keep your fork in sync
git fetch upstream
git checkout dev
git merge upstream/dev
git push origin dev
```

---

## Working with AI Features

### Running Local AI Models (Desktop Only)

The desktop app can run AI models locally using LlamaCPP:

1. Download models through the UI
2. Models are stored in `~/jan/models/`
3. LlamaCPP extension manages inference

### Connecting to Remote APIs (Web & Desktop)

Both web and desktop can connect to:
- OpenAI API
- Anthropic (Claude)
- Groq
- Mistral
- Any OpenAI-compatible API

Configure in Settings → API Keys

---

## Component-Specific Guides

### Working on the Frontend (Web App)

```bash
cd web-app
yarn dev:web  # Start dev server
```

Key directories:
- `web-app/src/components/` - Reusable UI components
- `web-app/src/routes/` - Page routes (using TanStack Router)
- `web-app/src/stores/` - State management (Zustand)
- `web-app/src/hooks/` - Custom React hooks

### Working on Core SDK

```bash
cd core
yarn build  # Build the SDK
yarn pack   # Package for extensions
```

The core SDK provides shared functionality for both web and desktop.

### Working on Extensions

Desktop extensions:
```bash
cd extensions
yarn install
yarn workspaces foreach -Apt run build:publish
```

Web extensions:
```bash
cd extensions-web
yarn install
yarn build
```

### Working on Tauri Backend (Desktop Only)

```bash
cd src-tauri
cargo build    # Build Rust backend
cargo test     # Run Rust tests
cargo clippy   # Lint Rust code
cargo fmt      # Format Rust code
```

---

## Deploying the Web App

### Option 1: Static Hosting (Netlify, Vercel, etc.)

```bash
# Build the web app
yarn build:web-app

# Deploy the dist-web directory
# The output is in: web-app/dist-web/
```

Configuration for hosting:
- Build command: `yarn build:web-app`
- Publish directory: `web-app/dist-web`
- Node version: 20.x

### Option 2: Docker Deployment

```bash
# Use the provided Dockerfile
docker build -t jan-web-app .
docker run -p 3001:3001 jan-web-app
```

### Option 3: Self-hosted with Nginx

```bash
# Build the app
yarn build:web-app

# Copy to nginx
cp -r web-app/dist-web/* /var/www/html/

# Use the provided nginx.conf as reference
```

### Environment Variables for Web Deployment

Create a `.env` file in the `web-app` directory:

```env
# Analytics (optional)
GA_MEASUREMENT_ID=your-google-analytics-id
POSTHOG_KEY=your-posthog-key
POSTHOG_HOST=https://your-posthog-host

# Model catalog
MODEL_CATALOG_URL=https://your-model-catalog-url
```

---

## Common Tasks

### Adding a New Feature

1. **Plan**: Discuss in GitHub Issues first
2. **Branch**: Create a feature branch
3. **Code**: Implement the feature
4. **Test**: Write tests and verify functionality
5. **Document**: Update relevant documentation
6. **PR**: Submit pull request to `dev` branch

### Fixing a Bug

1. **Reproduce**: Ensure you can reproduce the bug
2. **Branch**: Create a fix branch
3. **Fix**: Make the minimal changes needed
4. **Test**: Verify the fix works
5. **PR**: Submit with clear description

### Updating Dependencies

```bash
# Update yarn itself
corepack enable
corepack prepare yarn@4.5.3 --activate

# Update project dependencies
yarn upgrade-interactive

# Test everything still works
yarn build
yarn test
```

---

## Troubleshooting

### Build Failures

```bash
# Clean and rebuild
make clean
make dev

# OR manually
find . -name "node_modules" -type d -prune -exec rm -rf '{}' +
yarn install
yarn build:core
yarn build:extensions
```

### Tauri Issues (Desktop)

```bash
# Reinstall Rust dependencies
cd src-tauri
cargo clean
cargo build
```

### Extension Not Loading

```bash
# Rebuild extensions
cd extensions
yarn install
yarn workspaces foreach -Apt run build:publish

# For web extensions
cd extensions-web
yarn install
yarn build
```

### Port Already in Use

The web app uses port 3001 by default. If it's in use:

```bash
# Find and kill the process
lsof -ti:3001 | xargs kill -9

# OR change the port in web-app/vite.config.web.ts
```

---

## Getting Help

- **Documentation**: [jan.ai/docs](https://jan.ai/docs)
- **Discord Community**: [discord.gg/FTk2MvZwJH](https://discord.gg/FTk2MvZwJH)
- **GitHub Issues**: [github.com/janhq/jan/issues](https://github.com/janhq/jan/issues)
- **GitHub Discussions**: [github.com/janhq/jan/discussions](https://github.com/janhq/jan/discussions)

---

## Best Practices

### Code Style

- **TypeScript/JavaScript**: Use ESLint and Prettier
- **Rust**: Use `cargo fmt` and `cargo clippy`
- **Commits**: Follow conventional commit format
- **Testing**: Write tests for new features

### Performance

- Optimize bundle size for web deployment
- Use code splitting for large features
- Lazy load components when possible
- Profile performance before optimizing

### Security

- Never commit API keys or secrets
- Validate all user inputs
- Use the security scanning tools in CI/CD
- Follow OWASP guidelines

---

## Contributing Guidelines

For detailed contribution guidelines, see:
- [CONTRIBUTING.md](./CONTRIBUTING.md) - Main contribution guide
- [web-app/CONTRIBUTING.md](./web-app/CONTRIBUTING.md) - Frontend guide
- [core/CONTRIBUTING.md](./core/CONTRIBUTING.md) - Core SDK guide
- [extensions/CONTRIBUTING.md](./extensions/CONTRIBUTING.md) - Extensions guide
- [src-tauri/CONTRIBUTING.md](./src-tauri/CONTRIBUTING.md) - Backend guide

---

## Summary: How We Can Work Together

### As the Project Owner (You)
1. Create issues for features or bugs you want addressed
2. Review and merge pull requests
3. Provide feedback on contributions
4. Maintain the project roadmap

### As Contributors (Me or Others)
1. Pick an issue or propose a new feature
2. Create a branch and make changes
3. Test thoroughly (both web and desktop if applicable)
4. Submit a pull request with clear description
5. Address review feedback

### Communication
- **Issues**: For bug reports and feature requests
- **Pull Requests**: For code contributions
- **Discussions**: For questions and ideas
- **Discord**: For real-time collaboration

---

## Quick Reference: Common Commands

```bash
# Development
make dev                  # Desktop app development
make dev-web-app         # Web app development

# Building
make build               # Build desktop app
make build-web-app       # Build web app

# Testing
make test                # Run all tests
yarn lint                # Lint code

# Cleaning
make clean               # Clean all build artifacts

# Individual components
yarn build:core          # Build core SDK
yarn build:extensions    # Build desktop extensions
yarn build:extensions-web # Build web extensions
```

---

## Next Steps

1. **Try it out**: Run `make dev` or `make dev-web-app`
2. **Explore the code**: Pick a component and dive in
3. **Find an issue**: Look at GitHub Issues for something to work on
4. **Ask questions**: Use Discord or GitHub Discussions
5. **Make a contribution**: Start with something small

Happy coding! 🚀
