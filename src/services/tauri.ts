import { invoke } from '@tauri-apps/api/tauri'
import { emit, listen } from '@tauri-apps/api/event'


export default {
  invoke,
  emit, listen
}
