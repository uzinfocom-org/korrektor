import useSWR from 'swr'
import { fetcher } from "../utils/fetcher";


export default function Hello() {
  const { data, error } = useSWR(`/api/hello`, fetcher)

  if (error) return <>Failed to load data from backend</>
  if (!data) return <>Loading...</>

  return <>{data.message}</>
}
