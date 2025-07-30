import { invoke, InvokeArgs } from "@tauri-apps/api/tauri";
import * as dialog from "@tauri-apps/api/dialog";
import * as shell from "@tauri-apps/api/shell";
import * as path from "@tauri-apps/api/path";
import { appWindow } from "@tauri-apps/api/window";
import type { WebviewWindow } from "@tauri-apps/api/window";
import type { TProject, TSource, TUser } from "../types";
const open = async (...paths: string[]) => {
  const p = await path.join(...paths);
  shell.open(p);
};

interface IApi {
  invoke<T>(cmd: string, args?: InvokeArgs): Promise<T>;
  dialog: {
    open: (
      options?: dialog.OpenDialogOptions,
    ) => Promise<null | string | string[]>;
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
  testMode: false,
};

function fakeInvoke<T>(cmd: string, args?: InvokeArgs): Promise<any> {
  switch (cmd) {
    case "login": {
      let { url, token } = args.portal as any;
      if (url === token) {
        return Promise.resolve<TUser>({ id: token, display_name: url });
      }
      return Promise.reject(
        "In the thest environment url and token must be the identical",
      );
    }
    case "show_submission": {
      return Promise.resolve([
        {
          extras: {
            type: "validated-dataset",
            errors: {},
            dataset: "A-dataset",
          },
        },
        {
          extras: {
            type: "validated-resource",
            errors: {},
            dataset: "A-dataset",
            resource: "B resource",
          },
        },
        {
          extras: {
            type: "validated-resource",
            errors: {},
            dataset: "A-dataset",
            resource: "C resource",
          },
        },
        {
          extras: {
            type: "validated-dataset",
            errors: {},
            dataset: "B dataset",
          },
        },
        {
          extras: {
            type: "validated-resource",
            errors: {},
            dataset: "B dataset",
            resource: "second B resource",
          },
        },
      ]);
    }
    case "list_projects": {
      const { name } = args;

      let projects: TProject[] = [
        { id: "1", name: "proj-1", title: "Project" },
        { id: "2", name: "proj-2", title: "Hello" },
        { id: "3", name: "proj-3", title: "World" },
        { id: "4", name: "proj-4", title: "and" },
        { id: "5", name: "proj-5", title: "Hello World" },
        { id: "6", name: "proj-6", title: "another proj" },
        { id: "7", name: "proj-7", title: "no-no" },
        { id: "8", name: "proj-8", title: "and final" },
      ];

      return Promise.resolve(
        projects.filter(
          (p) => p.name.includes(name) || p.title.toLowerCase().includes(name),
        ),
      );
    }
    case "read_source_path":
      const { path } = args;
      if (path === "/path/to/project") {
        return Promise.resolve<TSource>({
          path,
          metadata: {
            dataset_type: "1..18",
            title: "<dataset title>",
            name: "<dataset name(url)>",
            notes: "<dataset description>",
            publication_date: "<YYYY-MM-DD>",
            tag_string: "tag-1,tag-2",
            spatial_data: "yes|no",
            license_id: "unspecified",
            // "spatial": "{\"type\": \"Polygon\", \"coordinates\": [[[151,-32],[152, -32],[152,-31]]]}",
            dataset_status: "final|draft|updated",
            update_freq: "daily|weekly|monthly|quarterly|yearly|as_required",
            // "additional_lga": "a",
            author: "<prepared by>",
            url: "<source>",
            data_comment: "<comment>",
            access_level: "open|registered|internal|restricted",
          },
          datasets: [
            {
              path: "a",
              name: "A-dataset",
              metadata: {
                dataset_type: "1..18",
                title: "<dataset title>",
                name: "<dataset name(url)>",
                notes: "<dataset description>",
                publication_date: "<YYYY-MM-DD>",
                tag_string: "tag-1,tag-2",
                spatial_data: "yes|no",
                license_id: "unspecified",
                // "spatial": "{\"type\": \"Polygon\", \"coordinates\": [[[151,-32],[152, -32],[152,-31]]]}",
                dataset_status: "final|draft|updated",
                update_freq:
                  "daily|weekly|monthly|quarterly|yearly|as_required",
                // "additional_lga": "a",
                author: "<prepared by>",
                url: "<source>",
                data_comment: "<comment>",
                access_level: "open|registered|internal|restricted",
              },

              resources: [
                {
                  path: "a/b",
                  name: "B resource",
                  metadata: {},
                  size: 1024 * 1024 * 5,
                },
                {
                  path: "a/c",
                  name: "C resource",
                  metadata: {},
                  size: 1024 * 1024 * 5,
                },
              ],
            },
            {
              path: "b",
              name: "B dataset",
              metadata: { title: "B dataset's title" },
              resources: [
                {
                  path: "b/b",
                  name: "second B resource",
                  metadata: { description: "details" },
                  size: 1024 * 1024 * 18,
                },
              ],
            },
          ],
        });
      }
      break;
    case "register_upload":
      debugger;

      return Promise.resolve({
        author_id: "e9036063-5a6b-47b7-b2b2-9d4b3bb1b7a4",
        data: {
          bytes_uploaded: 0,
          chunks_uploaded: 0,
          completed: false,
          dataset: "d",
          key: "resources/fdp-submission/f777b04c-5e35-46d6-a43c-28182b9e7a0e/hello",
          name: "hello",
          size: 100,
          upload_id:
            "air3ExVoAsfTTUBhxlbF4apIubDDsDMYy035oQcxKGs8kFoWHrBpIfe0tLCwjOQIGdZfdEl8i4xYYMk59i_dlCfm1_GXRpR_DlJ0vEOj30BDeZlkf2jWM7F80Sa8jbqMqPJGPXsWbkJdyAgO8Z47xw--",
        },
        extras: {
          category: "submission",
          dataset: "d",
          resource: "hello",
          type: "client-upload",
        },
        id: "a2e66290-f0b5-4586-98d4-42799f85526f",
        modified_at: "2025-07-30T16:58:12.338855",
        name: "fdp:submission:upload:d:hello",
        parent_id: null,
      });

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
  open: (...path) => {
    console.log("[fake] Opening %o", path);
  },
  window: appWindow,
  testMode: true,
};

function chooseApi(): IApi {
  return window.__TAURI_IPC__ ? tauriApi : fakeApi;
}

export default chooseApi();
