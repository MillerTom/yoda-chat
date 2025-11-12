/**
 * PostgreSQL Projects Service - Backend implementation
 * Uses Tauri commands to interact with PostgreSQL database
 */

import { invoke } from '@tauri-apps/api/core'
import type { ProjectsService, ThreadFolder } from './types'

interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: string
}

interface Project {
  id: string
  name: string
  created_at: number
  updated_at: number
}

export class PostgreSQLProjectsService implements ProjectsService {
  private initialized = false

  /**
   * Initialize the database connection
   * This should be called once when the app starts
   */
  async initialize(): Promise<void> {
    if (this.initialized) {
      return
    }

    try {
      const response = await invoke<ApiResponse<string>>(
        'init_project_database'
      )

      if (!response.success) {
        console.error('Failed to initialize project database:', response.error)
        throw new Error(response.error || 'Failed to initialize database')
      }

      this.initialized = true
      console.log('Project database initialized successfully')
    } catch (error) {
      console.error('Error initializing project database:', error)
      throw error
    }
  }

  /**
   * Check if database is healthy
   */
  async healthCheck(): Promise<boolean> {
    try {
      const response = await invoke<ApiResponse<boolean>>(
        'check_project_database_health'
      )
      return response.success && (response.data ?? false)
    } catch (error) {
      console.error('Database health check failed:', error)
      return false
    }
  }

  private convertProjectToThreadFolder(project: Project): ThreadFolder {
    return {
      id: project.id,
      name: project.name,
      updated_at: project.updated_at,
    }
  }

  async getProjects(): Promise<ThreadFolder[]> {
    try {
      await this.ensureInitialized()

      const response = await invoke<ApiResponse<Project[]>>(
        'get_all_projects'
      )

      if (!response.success) {
        console.error('Failed to get projects:', response.error)
        return []
      }

      return (response.data || []).map(this.convertProjectToThreadFolder)
    } catch (error) {
      console.error('Error getting projects:', error)
      return []
    }
  }

  async addProject(name: string): Promise<ThreadFolder> {
    try {
      await this.ensureInitialized()

      const response = await invoke<ApiResponse<Project>>('create_project', {
        name,
      })

      if (!response.success || !response.data) {
        throw new Error(response.error || 'Failed to create project')
      }

      return this.convertProjectToThreadFolder(response.data)
    } catch (error) {
      console.error('Error adding project:', error)
      throw error
    }
  }

  async updateProject(id: string, name: string): Promise<void> {
    try {
      await this.ensureInitialized()

      const response = await invoke<ApiResponse<Project>>('update_project', {
        id,
        name,
      })

      if (!response.success) {
        throw new Error(response.error || 'Failed to update project')
      }
    } catch (error) {
      console.error('Error updating project:', error)
      throw error
    }
  }

  async deleteProject(id: string): Promise<void> {
    try {
      await this.ensureInitialized()

      const response = await invoke<ApiResponse<void>>('delete_project', {
        id,
      })

      if (!response.success) {
        throw new Error(response.error || 'Failed to delete project')
      }
    } catch (error) {
      console.error('Error deleting project:', error)
      throw error
    }
  }

  async getProjectById(id: string): Promise<ThreadFolder | undefined> {
    try {
      await this.ensureInitialized()

      const response = await invoke<ApiResponse<Project>>(
        'get_project_by_id',
        { id }
      )

      if (!response.success || !response.data) {
        return undefined
      }

      return this.convertProjectToThreadFolder(response.data)
    } catch (error) {
      console.error('Error getting project by id:', error)
      return undefined
    }
  }

  async setProjects(projects: ThreadFolder[]): Promise<void> {
    try {
      await this.ensureInitialized()

      // For bulk updates, we need to delete all existing projects and re-create them
      // First get all existing projects
      const existing = await this.getProjects()

      // Delete projects that are not in the new list
      for (const project of existing) {
        if (!projects.find((p) => p.id === project.id)) {
          await this.deleteProject(project.id)
        }
      }

      // Update or create projects
      for (const project of projects) {
        const existingProject = existing.find((p) => p.id === project.id)
        if (existingProject) {
          if (existingProject.name !== project.name) {
            await this.updateProject(project.id, project.name)
          }
        } else {
          // Note: This won't preserve the original ID since the backend generates UUIDs
          // This is a limitation of the current implementation
          await this.addProject(project.name)
        }
      }
    } catch (error) {
      console.error('Error setting projects:', error)
      throw error
    }
  }

  private async ensureInitialized(): Promise<void> {
    if (!this.initialized) {
      await this.initialize()
    }
  }
}
