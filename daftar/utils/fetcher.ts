// @ts-ignore
export const fetcher = (url: string) => fetch(url).then((res) => res.json())

// @ts-ignore
export const textFetcher = (...args) => fetch(...args).then(res => res.text())
