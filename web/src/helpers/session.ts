import type ISession from "@/session.worker";
import SessionWorker from "@/session.worker?worker";
import * as Comlink from "comlink";

export class Session {
  worker: Worker;
  session: Comlink.Remote<ISession> | null = null;

  constructor() {
    this.worker = new SessionWorker();
  }

  async initialize(onProgress: (received: number, total: number) => void) {
    const wrapped = Comlink.wrap<typeof ISession>(this.worker);
    this.session = await new wrapped();
    await this.session.initialize(Comlink.proxy(onProgress));
  }

  translate(src: string) {
    if (!this.session) {
      throw new Error("Session is not initialized");
    }

    return this.session.translate(src);
  }

  terminate() {
    this.worker.terminate();
  }
}
