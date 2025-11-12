# Jan Architecture Documentation

This document explains the technical architecture of Jan, helping developers understand how components interact.

## Overview

Jan is a hybrid application that can run as both a desktop app (using Tauri) and a web app (browser-only). The architecture is designed to maximize code reuse between both modes.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                       User Interface                         │
│                  (React + TypeScript)                        │
│                                                              │
│  Components: Chat, Settings, Model Hub, Assistants          │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Renders UI
                     │
┌────────────────────┴────────────────────────────────────────┐
│                    Application Layer                         │
│                       (web-app/)                             │
│                                                              │
│  • TanStack Router (routing)                                 │
│  • Zustand (state management)                                │
│  • React Query (data fetching)                               │
│  • Custom hooks (business logic)                             │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Uses
                     │
┌────────────────────┴────────────────────────────────────────┐
│                    Core SDK Layer                            │
│                        (core/)                               │
│                                                              │
│  • Type definitions                                          │
│  • Extension system                                          │
│  • Event bus                                                 │
│  • API interfaces                                            │
└────────┬───────────────────────────────────┬────────────────┘
         │                                   │
         │ Desktop Mode                      │ Web Mode
         │                                   │
         ▼                                   ▼
┌─────────────────────┐           ┌──────────────────────┐
│  Extensions         │           │  Extensions-Web      │
│  (extensions/)      │           │  (extensions-web/)   │
│                     │           │                      │
│  • Assistant        │           │  • Conversational    │
│  • Conversational   │           │  • Web-specific      │
│  • Download         │           │    features          │
│  • LlamaCPP         │           │                      │
│  • Vector DB        │           └──────────────────────┘
│  • RAG              │
└──────────┬──────────┘
           │
           │ Desktop Only
           │
           ▼
┌─────────────────────────────────────────────────────────────┐
│                    Tauri IPC Layer                           │
│                  (Desktop Mode Only)                         │
│                                                              │
│  Invokes: invoke('command', data)                            │
│  Events: listen('event', handler)                            │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │
┌────────────────────┴────────────────────────────────────────┐
│                   Tauri Backend (Rust)                       │
│                      (src-tauri/)                            │
│                                                              │
│  • Command handlers                                          │
│  • File system access                                        │
│  • Process management                                        │
│  • Window management                                         │
│  • System integration                                        │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Uses
                     │
┌────────────────────┴────────────────────────────────────────┐
│                    Tauri Plugins (Rust)                      │
│                  (src-tauri/plugins/)                        │
│                                                              │
│  ┌────────────────────┐        ┌─────────────────────┐      │
│  │  Hardware Plugin   │        │  LlamaCPP Plugin    │      │
│  │                    │        │                     │      │
│  │  • CPU detection   │        │  • Model loading    │      │
│  │  • GPU detection   │        │  • Inference        │      │
│  │  • Memory info     │        │  • Process mgmt     │      │
│  │  • System stats    │        │  • Streaming        │      │
│  └────────────────────┘        └─────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

## Component Details

### 1. Web App Layer (web-app/)

**Purpose**: Frontend UI and application logic

**Key Technologies**:
- React 19 (UI components)
- TypeScript (type safety)
- Vite (build tool)
- TanStack Router (routing)
- Zustand (state management)
- Tailwind CSS (styling)

**Directory Structure**:
```
web-app/
├── src/
│   ├── components/      # Reusable UI components
│   ├── routes/          # Page routes
│   ├── stores/          # Zustand state stores
│   ├── hooks/           # Custom React hooks
│   ├── utils/           # Utility functions
│   └── types/           # TypeScript types
├── public/              # Static assets
├── vite.config.ts       # Vite config (desktop)
└── vite.config.web.ts   # Vite config (web)
```

**Responsibilities**:
- Render UI components
- Handle user interactions
- Manage application state
- Route navigation
- Call Core SDK APIs

### 2. Core SDK Layer (core/)

**Purpose**: Shared APIs and business logic

**Key Technologies**:
- TypeScript
- Extension system
- Event bus

**Directory Structure**:
```
core/
├── src/
│   ├── api/            # API interfaces
│   ├── events/         # Event definitions
│   ├── extensions/     # Extension system
│   ├── types/          # Shared types
│   └── utils/          # Shared utilities
└── index.ts            # Main export
```

**Responsibilities**:
- Define API contracts
- Provide extension framework
- Event bus for communication
- Type definitions
- Platform abstraction

### 3. Extensions Layer

#### Desktop Extensions (extensions/)

**Purpose**: Desktop-specific features

**Available Extensions**:
- `assistant-extension` - Assistant management
- `conversational-extension` - Chat functionality
- `download-extension` - Model downloads
- `llamacpp-extension` - Local model inference
- `vector-db-extension` - Vector database
- `rag-extension` - RAG capabilities

#### Web Extensions (extensions-web/)

**Purpose**: Web-specific features

**Available Extensions**:
- `conversational-web` - Web chat functionality
- Web-specific adapters

### 4. Tauri Backend Layer (src-tauri/)

**Purpose**: Native system integration (desktop only)

**Key Technologies**:
- Rust
- Tauri 2.x framework
- System APIs

**Directory Structure**:
```
src-tauri/
├── src/
│   ├── core/           # Core Tauri commands
│   ├── utils/          # Utility modules
│   └── main.rs         # Entry point
├── plugins/            # Tauri plugins
│   ├── hardware/       # Hardware detection
│   └── llamacpp/       # LlamaCPP integration
├── resources/          # Bundled resources
└── Cargo.toml          # Rust dependencies
```

**Responsibilities**:
- Handle IPC commands
- File system operations
- Process management
- Window management
- System integration
- Hardware access

### 5. Tauri Plugins Layer (src-tauri/plugins/)

**Purpose**: Specialized native functionality

**Available Plugins**:

#### Hardware Plugin
- CPU detection and info
- GPU detection (NVIDIA, AMD, Intel)
- Memory statistics
- System information

#### LlamaCPP Plugin
- Model loading and management
- Inference execution
- Process management
- Streaming responses

## Data Flow

### Desktop Mode: User Message to AI Response

```
1. User types message
   ↓
2. React component updates state
   ↓
3. Store dispatches action
   ↓
4. Extension handles message
   ↓
5. Extension calls Core SDK API
   ↓
6. Core SDK invokes Tauri command
   ↓
7. Rust backend receives command
   ↓
8. Backend calls LlamaCPP plugin
   ↓
9. Plugin runs inference
   ↓
10. Response streams back through layers
   ↓
11. UI updates with response
```

### Web Mode: User Message to AI Response

```
1. User types message
   ↓
2. React component updates state
   ↓
3. Store dispatches action
   ↓
4. Extension handles message
   ↓
5. Extension calls remote API (fetch)
   ↓
6. API processes request
   ↓
7. Response streams back
   ↓
8. UI updates with response
```

## Communication Patterns

### 1. Frontend to Backend (Desktop)

**Method**: Tauri IPC

```typescript
// Frontend (TypeScript)
import { invoke } from '@tauri-apps/api/core'

const result = await invoke('command_name', {
  param1: 'value1',
  param2: 'value2'
})
```

```rust
// Backend (Rust)
#[tauri::command]
async fn command_name(param1: String, param2: String) -> Result<Value, String> {
    // Implementation
}
```

### 2. Backend to Frontend (Desktop)

**Method**: Tauri Events

```rust
// Backend (Rust)
use tauri::Emitter;

app_handle.emit("event_name", payload)?;
```

```typescript
// Frontend (TypeScript)
import { listen } from '@tauri-apps/api/event'

const unlisten = await listen('event_name', (event) => {
  console.log('Received:', event.payload)
})
```

### 3. Component Communication (Frontend)

**Method**: Zustand stores + React Context

```typescript
// Define store
const useStore = create((set) => ({
  messages: [],
  addMessage: (msg) => set((state) => ({
    messages: [...state.messages, msg]
  }))
}))

// Use in component
function ChatComponent() {
  const addMessage = useStore((state) => state.addMessage)
  // ...
}
```

### 4. Extension Communication

**Method**: Core SDK Event Bus

```typescript
// Publish event
events.emit('message:created', { id, content })

// Subscribe to event
events.on('message:created', (data) => {
  console.log('New message:', data)
})
```

## State Management

### Frontend State (Zustand)

```typescript
// Example: Chat store
interface ChatState {
  conversations: Conversation[]
  activeConversation: string | null
  messages: Message[]
  isLoading: boolean
  
  // Actions
  addMessage: (message: Message) => void
  setActiveConversation: (id: string) => void
}

const useChatStore = create<ChatState>((set) => ({
  // State
  conversations: [],
  activeConversation: null,
  messages: [],
  isLoading: false,
  
  // Actions
  addMessage: (message) => set((state) => ({
    messages: [...state.messages, message]
  })),
  setActiveConversation: (id) => set({
    activeConversation: id
  })
}))
```

### Backend State (Rust)

```rust
// Managed state
pub struct AppState {
    pub models: Arc<Mutex<HashMap<String, Model>>>,
    pub config: Arc<RwLock<Config>>,
}

#[tauri::command]
async fn get_model(
    state: State<'_, AppState>,
    model_id: String
) -> Result<Model, String> {
    let models = state.models.lock().await;
    models.get(&model_id)
        .cloned()
        .ok_or_else(|| "Model not found".to_string())
}
```

## Build Modes

### Desktop Build

```bash
# Development
yarn dev

# Production
yarn build

# Process:
1. Build Core SDK
2. Build Extensions
3. Build Web App (with Tauri APIs)
4. Bundle with Tauri
5. Output: Native executable
```

**Output**:
- macOS: `.dmg` or `.app`
- Windows: `.exe` or `.msi`
- Linux: `.deb`, `.AppImage`, etc.

### Web Build

```bash
# Development
yarn dev:web-app

# Production
yarn build:web-app

# Process:
1. Build Core SDK
2. Build Web Extensions
3. Build Web App (exclude Tauri APIs)
4. Output: Static files
```

**Output**:
- `dist-web/` directory
- Static HTML, CSS, JS
- Can be hosted anywhere

## Configuration

### Desktop Configuration (tauri.conf.json)

```json
{
  "build": {
    "frontendDist": "../web-app/dist",
    "devUrl": "http://localhost:1420",
    "beforeDevCommand": "yarn dev:web",
    "beforeBuildCommand": "yarn build:web"
  },
  "app": {
    "windows": [...],
    "security": {...}
  }
}
```

### Web Configuration (vite.config.web.ts)

```typescript
export default defineConfig({
  build: {
    outDir: './dist-web',
    rollupOptions: {
      external: [
        // Exclude Tauri packages
        '@tauri-apps/api',
        // ...
      ]
    }
  },
  define: {
    IS_TAURI: false,
    IS_WEB_APP: true,
    // ...
  }
})
```

## Platform Detection

The app uses compile-time constants to detect platform:

```typescript
// Generated at build time
declare const IS_TAURI: boolean
declare const IS_WEB_APP: boolean
declare const IS_MACOS: boolean
declare const IS_WINDOWS: boolean
declare const IS_LINUX: boolean

// Usage
if (IS_TAURI) {
  // Use Tauri APIs
  const result = await invoke('command')
} else {
  // Use web APIs
  const result = await fetch('/api/command')
}
```

## Extension System

### Extension Interface

```typescript
interface Extension {
  name: string
  version: string
  
  // Lifecycle
  onLoad(): Promise<void>
  onUnload(): Promise<void>
  
  // Optional features
  onMessage?(message: Message): Promise<void>
  onCommand?(command: string, args: any): Promise<any>
}
```

### Registering an Extension

```typescript
// Extension implementation
class MyExtension implements Extension {
  name = 'my-extension'
  version = '1.0.0'
  
  async onLoad() {
    console.log('Extension loaded')
  }
  
  async onUnload() {
    console.log('Extension unloaded')
  }
}

// Register
extensionManager.register(new MyExtension())
```

## Security Model

### Desktop Security

- **Sandboxed**: Tauri provides process isolation
- **CSP**: Content Security Policy enforced
- **IPC Validation**: All commands validated
- **File System**: Limited to app directories

### Web Security

- **HTTPS Only**: Enforced in production
- **CSP Headers**: Strict content policy
- **CORS**: Properly configured
- **No Secrets**: API keys in browser only

## Performance Considerations

### Code Splitting

```typescript
// Route-based code splitting
const Chat = lazy(() => import('./routes/Chat'))
const Settings = lazy(() => import('./routes/Settings'))

// Component-based code splitting
const HeavyComponent = lazy(() => import('./components/Heavy'))
```

### Lazy Loading

```typescript
// Lazy load extensions
const loadExtension = async (name: string) => {
  const module = await import(`../extensions/${name}`)
  return module.default
}
```

### Bundle Optimization

- Tree shaking enabled
- Minification in production
- Code splitting by route
- Dynamic imports for large components

## Testing Architecture

### Frontend Tests (Vitest)

```typescript
// Component test
describe('ChatComponent', () => {
  it('renders messages', () => {
    render(<ChatComponent />)
    expect(screen.getByText('Hello')).toBeInTheDocument()
  })
})

// Store test
describe('ChatStore', () => {
  it('adds messages', () => {
    const store = useChatStore.getState()
    store.addMessage(mockMessage)
    expect(store.messages).toHaveLength(1)
  })
})
```

### Backend Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_command() {
        let result = command_name("test".to_string()).await;
        assert!(result.is_ok());
    }
}
```

## Debugging

### Frontend Debugging

- Chrome DevTools
- React DevTools
- Zustand DevTools
- Network tab for API calls

### Backend Debugging (Desktop)

- Rust debugging with `println!` or `dbg!`
- Logs in terminal
- Tauri DevTools
- `RUST_LOG` environment variable

## Resources

- **Tauri Docs**: https://tauri.app/
- **React Docs**: https://react.dev/
- **Vite Docs**: https://vitejs.dev/
- **TanStack Router**: https://tanstack.com/router
- **Zustand Docs**: https://zustand-demo.pmnd.rs/

---

For more information, see:
- [COLLABORATION_GUIDE.md](../COLLABORATION_GUIDE.md)
- [QUICKSTART.md](../QUICKSTART.md)
- [WEB_DEPLOYMENT.md](./WEB_DEPLOYMENT.md)
