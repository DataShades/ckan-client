import { invoke, InvokeArgs } from '@tauri-apps/api/tauri'
import * as dialog from '@tauri-apps/api/dialog';
import * as shell from '@tauri-apps/api/shell';
import * as path from '@tauri-apps/api/path';
import { appWindow } from '@tauri-apps/api/window'
import type { WebviewWindow } from '@tauri-apps/api/window';
import type { TProject, TUser } from "../types"
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

      let projects: TProject[] = [
        { id: "1", name: 'proj-1', title: "Project" },
        { id: "2", name: 'proj-2', title: "Hello" },
        { id: "3", name: 'proj-3', title: "World" },
        { id: "4", name: 'proj-4', title: "and" },
        { id: "5", name: 'proj-5', title: "Hello World" },
        { id: "6", name: 'proj-6', title: "another proj" },
        { id: "7", name: 'proj-7', title: "no-no" },
        { id: "8", name: 'proj-8', title: "and final" },
      ]


      return Promise.resolve(projects.filter(p => p.name.includes(name) || p.title.toLowerCase().includes(name)))
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
