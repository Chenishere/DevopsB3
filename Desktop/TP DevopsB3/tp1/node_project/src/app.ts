import express, { Request, Response } from "express";

const app = express();

app.get("/ping", (req: Request, res: Response) => {
  const headers = req.headers;
  res.status(200).json({ headers });
});

app.use((req: Request, res: Response) => {
  res.status(404).json({ error: "404" });
});

const PORT = process.env.PING_LISTEN_PORT || 80;

app.listen(PORT, () => {
  console.log(`Server is running on port ${PORT}`);
});
