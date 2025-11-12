# Database Analysis: Jan (Yoda-Chat) Codebase

**Date:** 2025-11-12  
**Repository:** MillerTom/yoda-chat (fork of janhq/jan)

## Executive Summary

**YES**, this codebase **DOES have SQL database support**. Specifically, it uses **SQLite** databases with an optional extension called **sqlite-vec** for vector similarity search operations.

## Database Implementation Details

### 1. Database Technology Stack

The project uses **SQLite** as its primary database solution with the following components:

- **Primary Database:** SQLite (via `rusqlite` crate in Rust)
- **Vector Extension:** sqlite-vec (optional, for accelerated similarity search)
- **Fallback:** Linear search when sqlite-vec is unavailable
- **Language:** Implementation in Rust, exposed via Tauri plugins to TypeScript/JavaScript

### 2. Database Dependencies

#### Rust Dependencies (Cargo.toml)
```toml
# Main vector DB plugin
rusqlite = { version = "0.32", features = ["bundled", "load_extension"] }

# Mobile feature (optional)
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"], optional = true }
```

**Location:** `src-tauri/plugins/tauri-plugin-vector-db/Cargo.toml`

### 3. Database Purpose

The SQLite database is used specifically for **Vector Database** functionality:

- **Storage:** Vector embeddings for RAG (Retrieval-Augmented Generation)
- **Search:** Similarity search using cosine similarity or ANN (Approximate Nearest Neighbor)
- **File Management:** Tracking uploaded files and their text chunks
- **Embeddings:** Storing high-dimensional vector representations of text

### 4. Database Schema

The vector database creates the following tables in each collection:

#### Files Table
```sql
CREATE TABLE IF NOT EXISTS files (
    id TEXT PRIMARY KEY,
    path TEXT UNIQUE NOT NULL,
    name TEXT,
    type TEXT,
    size INTEGER,
    chunk_count INTEGER DEFAULT 0
)
```

#### Chunks Table
```sql
CREATE TABLE IF NOT EXISTS chunks (
    id TEXT PRIMARY KEY,
    text TEXT NOT NULL,
    embedding BLOB NOT NULL,
    file_id TEXT,
    chunk_file_order INTEGER,
    FOREIGN KEY (file_id) REFERENCES files(id)
)
```

#### Vector Search Table (when sqlite-vec is available)
```sql
CREATE VIRTUAL TABLE IF NOT EXISTS chunks_vec 
USING vec0(embedding float[dimension])
```

**Indexes:**
- `idx_chunks_id` on `chunks(id)`
- `idx_chunks_file_id` on `chunks(file_id)`
- `idx_chunks_file_order` on `chunks(file_id, chunk_file_order)`

### 5. Database Location

**Default Storage Path:**
```
{USER_DATA_DIR}/Jan/data/db/
```

Where `{USER_DATA_DIR}` is platform-specific:
- **macOS:** `~/Library/Application Support/`
- **Windows:** `C:\Users\{username}\AppData\Roaming\`
- **Linux:** `~/.local/share/`

**Collection Files:**
Each collection is stored as a separate database file:
```
{base_dir}/{collection_name}.db
```

**Implementation:** `src-tauri/plugins/tauri-plugin-vector-db/src/state.rs:9-16`

### 6. Database Operations

The implementation provides the following operations:

#### Collection Management
- `create_collection(name, dimension)` - Create a new vector database
- `delete_collection(name)` - Remove a collection database file
- `get_status()` - Check if sqlite-vec ANN is available

#### File Operations
- `create_file(collection, path, metadata)` - Register a file for processing
- `delete_file(collection, file_id)` - Remove file and its chunks
- `list_attachments(collection, limit)` - List all files in collection

#### Chunk Operations
- `insert_chunks(collection, file_id, chunks[])` - Store text chunks with embeddings
- `delete_chunks(collection, chunk_ids[])` - Remove specific chunks
- `get_chunks(collection, file_id, start, end)` - Retrieve chunks by order
- `chunk_text(text, size, overlap)` - Split text into chunks

#### Search Operations
- `search_collection(collection, query_embedding, limit, threshold, mode, file_ids)` - Search for similar chunks
  - **ANN mode:** Uses sqlite-vec for fast approximate search
  - **Linear mode:** Full scan with cosine similarity
  - **Auto mode:** Uses ANN if available, falls back to linear

### 7. sqlite-vec Extension

The codebase includes support for **sqlite-vec**, a SQLite extension that enables efficient vector similarity search:

**Features:**
- Accelerated Approximate Nearest Neighbor (ANN) search
- Native vector operations in SQL
- Significantly faster than linear search for large datasets

**Load Paths (checked in order):**
1. `./src-tauri/resources/bin/sqlite-vec` (development)
2. `./resources/bin/sqlite-vec` (development)
3. `{exe_dir}/resources/bin/sqlite-vec` (production)
4. `{exe_dir}/../Resources/bin/sqlite-vec` (macOS app bundle)

**Fallback Behavior:**
If sqlite-vec cannot be loaded, the system automatically falls back to linear search using cosine similarity calculations in Rust code.

### 8. Integration Points

#### TypeScript/JavaScript Integration
The vector database is exposed to the frontend via:
- `@janhq/tauri-plugin-vector-db-api` package
- Tauri invoke commands from TypeScript

**Extension:** `extensions/vector-db-extension/`

#### RAG Extension
The vector database is used by the RAG (Retrieval-Augmented Generation) extension:
- `extensions/rag-extension/`
- Settings in `extensions/rag-extension/settings.json`

**Search Modes:**
```json
{
  "search_mode": [
    { "name": "Auto", "value": "auto" },
    { "name": "ANN (sqlite-vec)", "value": "ann" },
    { "name": "Linear (cosine)", "value": "linear" }
  ]
}
```

### 9. Code Structure

**Main Implementation Files:**
```
src-tauri/plugins/tauri-plugin-vector-db/
├── src/
│   ├── lib.rs          - Plugin initialization
│   ├── state.rs        - Database path management
│   ├── db.rs           - Core database operations (631 lines)
│   ├── commands.rs     - Tauri command handlers
│   ├── utils.rs        - Vector utilities (cosine similarity)
│   └── error.rs        - Error handling
├── Cargo.toml          - Rust dependencies
└── build.rs            - Build configuration
```

### 10. Key Features

**Strengths:**
1. ✅ Embedded database (no separate server required)
2. ✅ Optional acceleration with sqlite-vec
3. ✅ Graceful degradation to linear search
4. ✅ Transaction support
5. ✅ File and chunk tracking
6. ✅ Foreign key relationships
7. ✅ Proper indexing for performance

**Design Decisions:**
- One database file per collection (not one global database)
- Embeddings stored as BLOBs (little-endian byte arrays)
- UUID-based primary keys
- Optional ANN search with automatic fallback

## Conclusion

This codebase has a **sophisticated SQL database implementation** using SQLite for vector storage and similarity search. The database is specifically designed for RAG functionality, storing text chunks and their embeddings to enable semantic search capabilities in the Jan AI application.

The implementation is well-structured with:
- Clear separation of concerns (Rust backend, TypeScript frontend)
- Robust error handling
- Performance optimization (indexes, optional ANN)
- Production-ready features (transactions, foreign keys)

**Answer to the question:** Yes, this codebase has SQL database support via SQLite with vector search capabilities.
