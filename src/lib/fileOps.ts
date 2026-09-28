import { invoke } from '@tauri-apps/api/core'

export async function readFile(path: string): Promise<string> {
  return await invoke<string>('read_file_content', { path })
}

export async function writeFile(path: string, content: string): Promise<void> {
  await invoke('write_file_content', { path, content })
}

export async function pathExists(path: string): Promise<boolean> {
  return await invoke<boolean>('path_exists', { path })
}

export async function getHomeDir(): Promise<string> {
  return await invoke<string>('home_dir')
}
