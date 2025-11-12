# Projects PostgreSQL Backend

This module implements PostgreSQL-based storage for projects (folders) in the Yoda Chat application.

## Architecture

### Backend (Rust)

The backend is organized into three main components:

1. **Database Layer** (`db.rs`)
   - Uses `sqlx` for direct SQL queries (no ORM)
   - Implements CRUD operations for projects
   - Handles connection pooling and error handling
   - Provides type-safe database interactions

2. **RPC Layer** (`commands.rs`)
   - Exposes Tauri commands that wrap database operations
   - Provides a clean API response structure
   - Manages database state through Tauri's state management
   - Handles initialization and health checks

3. **Schema** (`schema.sql`)
   - PostgreSQL schema with best practices:
     - UUID primary keys using `uuid-ossp` extension
     - Soft delete support (deleted_at column)
     - Automatic timestamp updates via triggers
     - Indexes for performance
     - Constraints for data integrity

### Frontend (TypeScript)

The frontend implementation is in `/web-app/src/services/projects/`:

- **`postgresql.ts`**: PostgreSQL service implementation
  - Uses Tauri's `invoke` to call backend commands
  - Implements the `ProjectsService` interface
  - Provides automatic initialization
  - Handles error cases gracefully

- **`default.ts`**: localStorage fallback implementation
  - Used for web platform or as fallback
  - Same interface as PostgreSQL service

- **`types.ts`**: Shared TypeScript interfaces
  - `ProjectsService` interface
  - `ThreadFolder` type definition

## Database Schema

```sql
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ DEFAULT NULL
);
```

### Features:
- **Soft Delete**: Records are marked as deleted, not removed
- **Auto Timestamps**: `updated_at` automatically updates on changes
- **UUID IDs**: Distributed, collision-resistant identifiers
- **Indexes**: Fast queries on `updated_at` and `name`

## API Endpoints (Tauri Commands)

### Initialize Database
```typescript
invoke('init_project_database'): Promise<ApiResponse<string>>
```
Initializes the database connection and creates tables if needed.

### Health Check
```typescript
invoke('check_project_database_health'): Promise<ApiResponse<boolean>>
```
Verifies the database connection is working.

### Create Project
```typescript
invoke('create_project', { name: string }): Promise<ApiResponse<Project>>
```
Creates a new project with the given name.

### Get All Projects
```typescript
invoke('get_all_projects'): Promise<ApiResponse<Project[]>>
```
Retrieves all projects (excluding soft-deleted ones).

### Get Project by ID
```typescript
invoke('get_project_by_id', { id: string }): Promise<ApiResponse<Project>>
```
Retrieves a specific project by UUID.

### Update Project
```typescript
invoke('update_project', { id: string, name: string }): Promise<ApiResponse<Project>>
```
Updates a project's name.

### Delete Project
```typescript
invoke('delete_project', { id: string }): Promise<ApiResponse<void>>
```
Soft deletes a project (sets `deleted_at` timestamp).

## Configuration

### Database URL

The database connection URL can be configured via environment variable:

```bash
DATABASE_URL=postgresql://username:password@localhost:5432/database_name
```

Default: `postgresql://postgres:postgres@localhost:5432/yoda_chat`

### Connection Pool

- Max connections: 5
- Configured in `ProjectDatabase::new()`

## Error Handling

The module provides comprehensive error handling:

- `ProjectDbError::DatabaseError` - SQL/connection errors
- `ProjectDbError::ProjectNotFound` - Invalid project ID
- `ProjectDbError::InvalidData` - Validation errors
- `ProjectDbError::ConnectionError` - Connection failures

All errors are propagated to the frontend via the `ApiResponse` structure.

## Testing

Run tests with:

```bash
cd src-tauri
cargo test --lib project -- --ignored
```

Note: Tests require a running PostgreSQL instance. Set `TEST_DATABASE_URL` environment variable if needed.

## Usage Example

### Frontend

```typescript
import { services } from '@/services'

// Initialize (happens automatically on app start for Tauri desktop)
await services.projects().initialize()

// Create a project
const project = await services.projects().addProject('My Project')

// Get all projects
const projects = await services.projects().getProjects()

// Update a project
await services.projects().updateProject(project.id, 'Updated Name')

// Delete a project
await services.projects().deleteProject(project.id)
```

### Backend (Rust)

The backend is automatically wired up through Tauri's command system. No manual initialization needed in application code - it's handled by the service hub.

## Best Practices Implemented

1. **SQL Best Practices**:
   - Parameterized queries (prevents SQL injection)
   - Proper indexing for performance
   - Constraints for data integrity
   - Soft deletes for data recovery

2. **Rust Best Practices**:
   - Error types with `thiserror`
   - Async/await throughout
   - Type safety with strong typing
   - Connection pooling

3. **API Design**:
   - Consistent response structure
   - Clear error messages
   - Health check endpoint
   - Graceful degradation

## Future Enhancements

Potential improvements for future versions:

- [ ] Add project metadata (description, tags, etc.)
- [ ] Implement project sharing/collaboration
- [ ] Add full-text search on project names
- [ ] Implement migration system for schema changes
- [ ] Add project archiving (separate from soft delete)
- [ ] Implement audit logging
- [ ] Add batch operations for better performance
