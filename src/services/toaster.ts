import { writable, Writable } from "svelte/store";

const uid = () => window.crypto.getRandomValues(new Uint32Array(1))[0]

export enum Color {
  primary = "primary",
  secondary = "secondary",
  success = "success",
  danger = "danger",
  warning = "warning",
  info = "info",
  light = "light",
  dark = "dark",
}

type ToastId = any;

export class Toast {
  public readonly id: ToastId;
  public readonly Color = Color;

  public color: Color = Color.info;

  constructor(public body: string, public header: string = "") {
    this.id = uid();
  }

}

export class ToasterService {
  private store: Writable<Toast[]>;
  constructor() {
    this.store = writable([])
  }
  subscribe(cb, invalidator) {
    return this.store.subscribe(cb, invalidator);
  }

  add(body: string, header?: string): Toast {
    const toast = new Toast(body, header);
    this.store.update(toasts => [...toasts, toast])
    return toast;
  }

  pop(id: ToastId) {
    this.store.update(toasts => toasts.filter(toast => toast.id !== id))
  }

  info(body: string, header: string = "") {
    const toast = this.add(body, header)
    toast.color = Color.info;
    return toast;
  }

  error(body: string, header: string = "") {
    const toast = this.add(body, header)
    toast.color = Color.danger;
    return toast;
  }

}

export default new ToasterService()
