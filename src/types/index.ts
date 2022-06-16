
export type TPortal = {
  url: string | null,
  token: string | null
}

export type TUser = {
  id: string,
  display_name: string,
}

export type TProject = {
  id: string,
  name: string,
  title: string,
}

export type TMetadata = Object;
export type TSource = {
  path: string,
  metadata: TMetadata | null,
  datasets: TDataset[],
}
export type TDataset = {
  path: string,
  name: string,
  metadata: TMetadata | null,
  resources: TResource[],
}
export type TResource = {
  path: string,
  name: string,
  metadata: TMetadata | null,
  size: number,
}
