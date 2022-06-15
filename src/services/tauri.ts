import { invoke } from '@tauri-apps/api/tauri'
import * as dialog from '@tauri-apps/api/dialog';
import * as shell  from '@tauri-apps/api/shell';
import * as path  from '@tauri-apps/api/path';
import { emit, listen } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window'

const open = async (...paths: string[]) => {
  const p = await path.join(...paths)
  shell.open(p);
}

export default {
  invoke,
  emit, listen,
  dialog,
  open,
  window: appWindow
}
