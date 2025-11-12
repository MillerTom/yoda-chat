# Jan Documentation Index

Welcome to the Jan (Yoda-Chat) documentation! This index helps you find the right guide for your needs.

## 🎯 Start Here

**New to the project?** Start with one of these:

1. **[ANALYSIS_SUMMARY.md](./ANALYSIS_SUMMARY.md)** ⭐ **START HERE**
   - Quick overview of what Jan is
   - Answers to key questions
   - Technology stack summary
   - 5-minute read

2. **[QUICKSTART.md](./QUICKSTART.md)** ⚡
   - Get up and running in 5 minutes
   - Setup instructions for desktop and web
   - Common issues and solutions
   - Perfect for first-time contributors

## 📚 Complete Guides

### For Contributors

3. **[COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md)** 🤝
   - **Complete collaboration workflow**
   - Git branching strategy
   - Component-specific guides
   - Testing and deployment
   - Best practices
   - 📖 30-minute read

### For Developers

4. **[docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md)** 🏗️
   - **Technical deep dive**
   - Component architecture
   - Data flow diagrams
   - Communication patterns
   - Extension system
   - State management
   - 📖 45-minute read

### For Deployment

5. **[docs/WEB_DEPLOYMENT.md](./docs/WEB_DEPLOYMENT.md)** 🌐
   - **Web app deployment guide**
   - Multiple hosting options
   - Netlify, Vercel, Docker, self-hosted
   - Configuration and optimization
   - Security considerations
   - 📖 20-minute read

### Project Documentation

6. **[README.md](./README.md)** 📄
   - Project overview
   - Features list
   - Installation instructions
   - Links to all guides

7. **[CONTRIBUTING.md](./CONTRIBUTING.md)** 📝
   - Contribution guidelines
   - Code standards
   - Pull request process

## 🗺️ Documentation Map

```
Documentation Structure:
├── README.md                    # Project overview
├── ANALYSIS_SUMMARY.md          # Executive summary ⭐
├── QUICKSTART.md                # 5-minute start guide ⚡
├── COLLABORATION_GUIDE.md       # Complete contributor guide 🤝
├── CONTRIBUTING.md              # Contribution guidelines
├── DOCUMENTATION_INDEX.md       # This file
└── docs/
    ├── ARCHITECTURE.md          # Technical architecture 🏗️
    ├── WEB_DEPLOYMENT.md        # Web deployment guide 🌐
    └── README.md                # Docs website info
```

## 📖 Reading Paths

### Path 1: Quick Start (15 minutes)
1. [ANALYSIS_SUMMARY.md](./ANALYSIS_SUMMARY.md) - 5 min
2. [QUICKSTART.md](./QUICKSTART.md) - 10 min
3. Start coding! 🚀

### Path 2: Contributor (45 minutes)
1. [ANALYSIS_SUMMARY.md](./ANALYSIS_SUMMARY.md) - 5 min
2. [QUICKSTART.md](./QUICKSTART.md) - 10 min
3. [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) - 30 min
4. Pick an issue and contribute! 💪

### Path 3: Technical Deep Dive (90 minutes)
1. [ANALYSIS_SUMMARY.md](./ANALYSIS_SUMMARY.md) - 5 min
2. [QUICKSTART.md](./QUICKSTART.md) - 10 min
3. [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) - 30 min
4. [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) - 45 min
5. Master the codebase! 🧠

### Path 4: Deployment Focus (30 minutes)
1. [ANALYSIS_SUMMARY.md](./ANALYSIS_SUMMARY.md) - 5 min
2. [QUICKSTART.md](./QUICKSTART.md) - 10 min
3. [docs/WEB_DEPLOYMENT.md](./docs/WEB_DEPLOYMENT.md) - 20 min
4. Deploy your instance! 🌐

## 🎓 Learning by Role

### Frontend Developer
**Focus on:**
1. [QUICKSTART.md](./QUICKSTART.md) - Setup
2. [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) - Section: "Working on the Frontend"
3. [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) - Section: "Web App Layer"

**Key directories:**
- `web-app/src/components/` - UI components
- `web-app/src/routes/` - Page routes
- `web-app/src/stores/` - State management

### Backend Developer (Desktop)
**Focus on:**
1. [QUICKSTART.md](./QUICKSTART.md) - Setup
2. [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) - Section: "Working on Tauri Backend"
3. [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) - Section: "Tauri Backend Layer"

**Key directories:**
- `src-tauri/src/` - Rust backend
- `src-tauri/plugins/` - Native plugins

### Extension Developer
**Focus on:**
1. [QUICKSTART.md](./QUICKSTART.md) - Setup
2. [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) - Section: "Working on Extensions"
3. [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) - Section: "Extension System"

**Key directories:**
- `extensions/` - Desktop extensions
- `extensions-web/` - Web extensions
- `core/` - Core SDK

### DevOps Engineer
**Focus on:**
1. [QUICKSTART.md](./QUICKSTART.md) - Setup
2. [docs/WEB_DEPLOYMENT.md](./docs/WEB_DEPLOYMENT.md) - All sections
3. [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) - Section: "Deploying the Web App"

**Key files:**
- `Makefile` - Build automation
- `web-app/vite.config.web.ts` - Web build config
- `Dockerfile` - Docker configuration
- `nginx.conf` - Web server config

## 🔍 Find Specific Information

### Setup & Installation
- **First time setup**: [QUICKSTART.md](./QUICKSTART.md)
- **Prerequisites**: [QUICKSTART.md](./QUICKSTART.md) or [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md)
- **Common issues**: [QUICKSTART.md](./QUICKSTART.md) - "Common Setup Issues"

### Development
- **Git workflow**: [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) - "Collaboration Workflow"
- **Code standards**: [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) - "Best Practices"
- **Testing**: [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) - "Testing"

### Architecture
- **Component overview**: [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md)
- **Data flow**: [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) - "Data Flow"
- **Communication patterns**: [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) - "Communication Patterns"

### Deployment
- **Web deployment**: [docs/WEB_DEPLOYMENT.md](./docs/WEB_DEPLOYMENT.md)
- **Desktop build**: [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) - "Build"
- **Docker**: [docs/WEB_DEPLOYMENT.md](./docs/WEB_DEPLOYMENT.md) - "Option 4: Docker"

### Commands Reference
- **Make commands**: [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) - "Quick Reference"
- **Yarn commands**: [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) - "Common Commands"
- **Build commands**: [QUICKSTART.md](./QUICKSTART.md)

## ❓ Common Questions

### "Is Jan a desktop app or web app?"
**Both!** See [ANALYSIS_SUMMARY.md](./ANALYSIS_SUMMARY.md)

### "How do I get started?"
Start with [QUICKSTART.md](./QUICKSTART.md)

### "How do I contribute?"
Follow [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md)

### "How do I deploy the web app?"
See [docs/WEB_DEPLOYMENT.md](./docs/WEB_DEPLOYMENT.md)

### "How does the architecture work?"
Read [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md)

### "What's the difference between desktop and web mode?"
See [ANALYSIS_SUMMARY.md](./ANALYSIS_SUMMARY.md) - "Comparison: Desktop vs Web"

### "Can I run local AI models?"
**Desktop only** - See [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) - "Working with AI Features"

### "How do I add a new feature?"
See [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md) - "Adding a New Feature"

### "Where do I report bugs?"
GitHub Issues: https://github.com/janhq/jan/issues

## 🔗 External Resources

### Official
- **Jan Website**: https://jan.ai/
- **Official Docs**: https://jan.ai/docs
- **Discord**: https://discord.gg/FTk2MvZwJH
- **GitHub**: https://github.com/janhq/jan

### Technologies
- **Tauri**: https://tauri.app/
- **React**: https://react.dev/
- **Vite**: https://vitejs.dev/
- **TypeScript**: https://www.typescriptlang.org/
- **Rust**: https://www.rust-lang.org/

## 📊 Documentation Statistics

- **Total guides**: 6 documents
- **Total content**: ~63,000 words
- **Estimated reading time**: 2-3 hours for everything
- **Quick start time**: 5 minutes
- **Code examples**: 50+
- **Diagrams**: 5+

## 🆕 What's New

This documentation was created to answer key questions:
- ✅ Is this a Tauri app? (Yes)
- ✅ Can it run as a web app? (Yes)
- ✅ How to collaborate? (Standard GitHub workflow)

## 🚀 Next Steps

1. **Read** [ANALYSIS_SUMMARY.md](./ANALYSIS_SUMMARY.md) (5 min)
2. **Try** [QUICKSTART.md](./QUICKSTART.md) (10 min)
3. **Choose** your path:
   - Want to contribute? → [COLLABORATION_GUIDE.md](./COLLABORATION_GUIDE.md)
   - Want to understand architecture? → [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md)
   - Want to deploy? → [docs/WEB_DEPLOYMENT.md](./docs/WEB_DEPLOYMENT.md)

## 📝 Documentation Feedback

Found an issue with the docs? Want to suggest improvements?

- **Open an issue**: https://github.com/MillerTom/yoda-chat/issues
- **Start a discussion**: https://github.com/MillerTom/yoda-chat/discussions
- **Join Discord**: https://discord.gg/FTk2MvZwJH

## 📄 License

Documentation is part of the Jan project, licensed under Apache 2.0.

---

**Happy learning and coding!** 🎉
