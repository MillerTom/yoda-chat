use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectDbError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Project not found: {0}")]
    ProjectNotFound(String),
    #[error("Invalid project data: {0}")]
    InvalidData(String),
    #[error("Connection error: {0}")]
    ConnectionError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub created_at: i64,  // Unix timestamp in milliseconds
    pub updated_at: i64,  // Unix timestamp in milliseconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProject {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProject {
    pub name: String,
}

pub struct ProjectDatabase {
    pool: PgPool,
}

impl ProjectDatabase {
    /// Create a new database connection pool
    pub async fn new(database_url: &str) -> Result<Self, ProjectDbError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(|e| ProjectDbError::ConnectionError(e.to_string()))?;

        Ok(Self { pool })
    }

    /// Initialize the database schema
    pub async fn init_schema(&self) -> Result<(), ProjectDbError> {
        let schema = include_str!("schema.sql");
        
        sqlx::query(schema)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Create a new project
    pub async fn create_project(&self, create: CreateProject) -> Result<Project, ProjectDbError> {
        let name = create.name.trim();
        if name.is_empty() {
            return Err(ProjectDbError::InvalidData("Project name cannot be empty".to_string()));
        }

        let row = sqlx::query(
            r#"
            INSERT INTO projects (name)
            VALUES ($1)
            RETURNING id, name, 
                      EXTRACT(EPOCH FROM created_at)::BIGINT * 1000 as created_at,
                      EXTRACT(EPOCH FROM updated_at)::BIGINT * 1000 as updated_at
            "#
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;

        Ok(Project {
            id: row.get::<uuid::Uuid, _>("id").to_string(),
            name: row.get("name"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    /// Get all projects (excluding soft-deleted ones)
    pub async fn get_all_projects(&self) -> Result<Vec<Project>, ProjectDbError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name,
                   EXTRACT(EPOCH FROM created_at)::BIGINT * 1000 as created_at,
                   EXTRACT(EPOCH FROM updated_at)::BIGINT * 1000 as updated_at
            FROM projects
            WHERE deleted_at IS NULL
            ORDER BY updated_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let projects = rows
            .iter()
            .map(|row| Project {
                id: row.get::<uuid::Uuid, _>("id").to_string(),
                name: row.get("name"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(projects)
    }

    /// Get a project by ID
    pub async fn get_project_by_id(&self, id: &str) -> Result<Project, ProjectDbError> {
        let uuid = uuid::Uuid::parse_str(id)
            .map_err(|_| ProjectDbError::InvalidData("Invalid UUID format".to_string()))?;

        let row = sqlx::query(
            r#"
            SELECT id, name,
                   EXTRACT(EPOCH FROM created_at)::BIGINT * 1000 as created_at,
                   EXTRACT(EPOCH FROM updated_at)::BIGINT * 1000 as updated_at
            FROM projects
            WHERE id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(uuid)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Project {
                id: row.get::<uuid::Uuid, _>("id").to_string(),
                name: row.get("name"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }),
            None => Err(ProjectDbError::ProjectNotFound(id.to_string())),
        }
    }

    /// Update a project
    pub async fn update_project(&self, id: &str, update: UpdateProject) -> Result<Project, ProjectDbError> {
        let name = update.name.trim();
        if name.is_empty() {
            return Err(ProjectDbError::InvalidData("Project name cannot be empty".to_string()));
        }

        let uuid = uuid::Uuid::parse_str(id)
            .map_err(|_| ProjectDbError::InvalidData("Invalid UUID format".to_string()))?;

        let row = sqlx::query(
            r#"
            UPDATE projects
            SET name = $2
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, name,
                      EXTRACT(EPOCH FROM created_at)::BIGINT * 1000 as created_at,
                      EXTRACT(EPOCH FROM updated_at)::BIGINT * 1000 as updated_at
            "#
        )
        .bind(uuid)
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Project {
                id: row.get::<uuid::Uuid, _>("id").to_string(),
                name: row.get("name"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }),
            None => Err(ProjectDbError::ProjectNotFound(id.to_string())),
        }
    }

    /// Delete a project (soft delete)
    pub async fn delete_project(&self, id: &str) -> Result<(), ProjectDbError> {
        let uuid = uuid::Uuid::parse_str(id)
            .map_err(|_| ProjectDbError::InvalidData("Invalid UUID format".to_string()))?;

        let result = sqlx::query(
            r#"
            UPDATE projects
            SET deleted_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(uuid)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(ProjectDbError::ProjectNotFound(id.to_string()));
        }

        Ok(())
    }

    /// Hard delete a project (for testing/cleanup)
    #[allow(dead_code)]
    pub async fn hard_delete_project(&self, id: &str) -> Result<(), ProjectDbError> {
        let uuid = uuid::Uuid::parse_str(id)
            .map_err(|_| ProjectDbError::InvalidData("Invalid UUID format".to_string()))?;

        sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(uuid)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Check database connection
    pub async fn health_check(&self) -> Result<(), ProjectDbError> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await?;
        Ok(())
    }
}

/// Get database URL from environment or use default
pub fn get_database_url() -> String {
    env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:postgres@localhost:5432/yoda_chat".to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> ProjectDatabase {
        let db_url = env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/yoda_chat_test".to_string());
        
        let db = ProjectDatabase::new(&db_url).await.expect("Failed to connect to test database");
        db.init_schema().await.expect("Failed to initialize schema");
        db
    }

    #[tokio::test]
    #[ignore] // Only run with --ignored flag when PostgreSQL is available
    async fn test_create_project() {
        let db = setup_test_db().await;
        
        let project = db.create_project(CreateProject {
            name: "Test Project".to_string(),
        }).await.expect("Failed to create project");

        assert_eq!(project.name, "Test Project");
        assert!(!project.id.is_empty());
        
        // Cleanup
        db.hard_delete_project(&project.id).await.ok();
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_all_projects() {
        let db = setup_test_db().await;
        
        let project1 = db.create_project(CreateProject {
            name: "Project 1".to_string(),
        }).await.expect("Failed to create project");
        
        let projects = db.get_all_projects().await.expect("Failed to get projects");
        assert!(!projects.is_empty());
        
        // Cleanup
        db.hard_delete_project(&project1.id).await.ok();
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_project() {
        let db = setup_test_db().await;
        
        let project = db.create_project(CreateProject {
            name: "Original Name".to_string(),
        }).await.expect("Failed to create project");
        
        let updated = db.update_project(&project.id, UpdateProject {
            name: "Updated Name".to_string(),
        }).await.expect("Failed to update project");
        
        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.id, project.id);
        
        // Cleanup
        db.hard_delete_project(&project.id).await.ok();
    }

    #[tokio::test]
    #[ignore]
    async fn test_delete_project() {
        let db = setup_test_db().await;
        
        let project = db.create_project(CreateProject {
            name: "To Delete".to_string(),
        }).await.expect("Failed to create project");
        
        db.delete_project(&project.id).await.expect("Failed to delete project");
        
        let result = db.get_project_by_id(&project.id).await;
        assert!(result.is_err());
        
        // Cleanup
        db.hard_delete_project(&project.id).await.ok();
    }
}
