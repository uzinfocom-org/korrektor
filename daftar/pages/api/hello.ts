import type { NextApiRequest, NextApiResponse } from "next";

type Data = {
  message: string;
};

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  const address = process.env.BACKEND || "http://localhost:3001";
  const data = await fetch(address + "/");
  res.status(200).json({ message: await data.text() });
}
