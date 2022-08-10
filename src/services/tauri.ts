import { invoke, InvokeArgs } from '@tauri-apps/api/tauri'
import * as dialog from '@tauri-apps/api/dialog';
import * as shell from '@tauri-apps/api/shell';
import * as path from '@tauri-apps/api/path';
import { appWindow } from '@tauri-apps/api/window'
import type { WebviewWindow } from '@tauri-apps/api/window';
import type { TUser } from "../types"
const open = async (...paths: string[]) => {
  const p = await path.join(...paths)
  shell.open(p);
}

interface IApi {
  invoke<T>(cmd: string, args?: InvokeArgs): Promise<T>;
  dialog: {
    open: (options?: dialog.OpenDialogOptions) => Promise<null | string | string[]>;
  };
  open: (...paths: string[]) => void;
  window: WebviewWindow;
  testMode: boolean;
}

const tauriApi: IApi = {
  invoke,
  dialog,
  open,
  window: appWindow,
  testMode: false
}

function fakeInvoke<T>(cmd: string, args?: InvokeArgs): Promise<any> {
  switch (cmd) {
    case "login": {
      let { url, token } = args.portal as any
      if (url === token) {
        return Promise.resolve<TUser>({ id: token, display_name: url })
      }
      return Promise.reject("In the thest environment url and token must be the identical");
    }
    case "show_submission": {
      return Promise.resolve([])
    }
    case "list_projects": {
      const { name } = args
      return Promise.resolve([])
    }
    default:
      console.log("Default handler for cmd %s with args %o", cmd, args);
      return Promise.resolve({});
  }
}
const fakeApi: IApi = {
  invoke: fakeInvoke,
  dialog: {
    open: () => Promise.resolve("/path/to/project"),
  },
  open: (...path) => { console.log("[fake] Opening %o", path) },
  window: appWindow,
  testMode: true,
}

function chooseApi(): IApi {

  return window.__TAURI_IPC__ ? tauriApi : fakeApi
}

export default chooseApi()
