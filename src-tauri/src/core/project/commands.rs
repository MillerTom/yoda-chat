use super::db::{CreateProject, Project, ProjectDatabase, ProjectDbError, UpdateProject, get_database_url};
use serde::Serialize;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

pub struct ProjectDbState {
    pub db: Arc<Mutex<Option<ProjectDatabase>>>,
}

impl ProjectDbState {
    pub fn new() -> Self {
        Self {
            db: Arc::new(Mutex::new(None)),
        }
    }
}

/// Initialize the PostgreSQL database connection
#[tauri::command]
pub async fn init_project_database(
    state: State<'_, ProjectDbState>,
) -> Result<ApiResponse<String>, String> {
    let database_url = get_database_url();
    
    match ProjectDatabase::new(&database_url).await {
        Ok(db) => {
            // Initialize schema
            if let Err(e) = db.init_schema().await {
                return Ok(ApiResponse::error(format!("Failed to initialize schema: {}", e)));
            }
            
            // Store the database connection
            let mut db_lock = state.db.lock().await;
            *db_lock = Some(db);
            
            Ok(ApiResponse::success("Database initialized successfully".to_string()))
        }
        Err(e) => Ok(ApiResponse::error(format!("Failed to connect to database: {}", e))),
    }
}

/// Check if the database connection is healthy
#[tauri::command]
pub async fn check_project_database_health(
    state: State<'_, ProjectDbState>,
) -> Result<ApiResponse<bool>, String> {
    let db_lock = state.db.lock().await;
    
    match db_lock.as_ref() {
        Some(db) => match db.health_check().await {
            Ok(_) => Ok(ApiResponse::success(true)),
            Err(e) => Ok(ApiResponse::error(format!("Database health check failed: {}", e))),
        },
        None => Ok(ApiResponse::error("Database not initialized".to_string())),
    }
}

/// Create a new project
#[tauri::command]
pub async fn create_project(
    name: String,
    state: State<'_, ProjectDbState>,
) -> Result<ApiResponse<Project>, String> {
    let db_lock = state.db.lock().await;
    
    match db_lock.as_ref() {
        Some(db) => {
            match db.create_project(CreateProject { name }).await {
                Ok(project) => Ok(ApiResponse::success(project)),
                Err(e) => Ok(ApiResponse::error(format!("Failed to create project: {}", e))),
            }
        }
        None => Ok(ApiResponse::error("Database not initialized".to_string())),
    }
}

/// Get all projects
#[tauri::command]
pub async fn get_all_projects(
    state: State<'_, ProjectDbState>,
) -> Result<ApiResponse<Vec<Project>>, String> {
    let db_lock = state.db.lock().await;
    
    match db_lock.as_ref() {
        Some(db) => {
            match db.get_all_projects().await {
                Ok(projects) => Ok(ApiResponse::success(projects)),
                Err(e) => Ok(ApiResponse::error(format!("Failed to get projects: {}", e))),
            }
        }
        None => Ok(ApiResponse::error("Database not initialized".to_string())),
    }
}

/// Get a project by ID
#[tauri::command]
pub async fn get_project_by_id(
    id: String,
    state: State<'_, ProjectDbState>,
) -> Result<ApiResponse<Project>, String> {
    let db_lock = state.db.lock().await;
    
    match db_lock.as_ref() {
        Some(db) => {
            match db.get_project_by_id(&id).await {
                Ok(project) => Ok(ApiResponse::success(project)),
                Err(ProjectDbError::ProjectNotFound(_)) => {
                    Ok(ApiResponse::error(format!("Project not found: {}", id)))
                }
                Err(e) => Ok(ApiResponse::error(format!("Failed to get project: {}", e))),
            }
        }
        None => Ok(ApiResponse::error("Database not initialized".to_string())),
    }
}

/// Update a project
#[tauri::command]
pub async fn update_project(
    id: String,
    name: String,
    state: State<'_, ProjectDbState>,
) -> Result<ApiResponse<Project>, String> {
    let db_lock = state.db.lock().await;
    
    match db_lock.as_ref() {
        Some(db) => {
            match db.update_project(&id, UpdateProject { name }).await {
                Ok(project) => Ok(ApiResponse::success(project)),
                Err(ProjectDbError::ProjectNotFound(_)) => {
                    Ok(ApiResponse::error(format!("Project not found: {}", id)))
                }
                Err(e) => Ok(ApiResponse::error(format!("Failed to update project: {}", e))),
            }
        }
        None => Ok(ApiResponse::error("Database not initialized".to_string())),
    }
}

/// Delete a project (soft delete)
#[tauri::command]
pub async fn delete_project(
    id: String,
    state: State<'_, ProjectDbState>,
) -> Result<ApiResponse<()>, String> {
    let db_lock = state.db.lock().await;
    
    match db_lock.as_ref() {
        Some(db) => {
            match db.delete_project(&id).await {
                Ok(_) => Ok(ApiResponse::success(())),
                Err(ProjectDbError::ProjectNotFound(_)) => {
                    Ok(ApiResponse::error(format!("Project not found: {}", id)))
                }
                Err(e) => Ok(ApiResponse::error(format!("Failed to delete project: {}", e))),
            }
        }
        None => Ok(ApiResponse::error("Database not initialized".to_string())),
    }
}
