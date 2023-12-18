import * as Comlink from "comlink";
import init from "marian";
import { Marian } from "marian";
import { URLS } from "./constants/hf";
import fetchWithProgress from "./utils/fetch-with-progress";

export default class SessionWorker {
  marian: Marian | null = null;

  async initialize(onProgress: (received: number, total: number) => void) {
    await init();

    let total = 0;
    const handleProgress = (received: number) => {
      total += received;
      onProgress(received, total);
    };

    const responses = await Promise.all(
      URLS.map((url) => fetchWithProgress(url, { onProgress: handleProgress })),
    );
    const [model, tokenizer, tokenizerDec] = await Promise.all(
      responses.map((response) => response.arrayBuffer()),
    );

    this.marian = new Marian(
      new Uint8Array(model),
      new Uint8Array(tokenizer),
      new Uint8Array(tokenizerDec),
    );
  }

  translate(src: string) {
    if (!this.marian) {
      throw new Error("Session is not initialized");
    }

    return this.marian.translate(src);
  }
}

Comlink.expose(SessionWorker);
