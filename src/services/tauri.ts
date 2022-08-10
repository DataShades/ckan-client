import { invoke } from '@tauri-apps/api/tauri'
import * as dialog from '@tauri-apps/api/dialog';
import * as shell from '@tauri-apps/api/shell';
import * as path from '@tauri-apps/api/path';
import { appWindow } from '@tauri-apps/api/window'

const open = async (...paths: string[]) => {
  const p = await path.join(...paths)
  shell.open(p);
}

interface IApi {

}

const tauriApi: IApi = {
  invoke,
  dialog,
  open,
  window: appWindow
}
const fakeApi: IApi = {
  invoke,
  dialog,
  open,
  window: appWindow
}

function chooseApi(): IApi {
  return window.__TAURI_IPC__ ? tauriApi : fakeApi
}

export default chooseApi()
