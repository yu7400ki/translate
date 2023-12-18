type Options = {
  onProgress?: (received: number, total: number) => void;
} & RequestInit;

export default async function fetchWithProgress(
  url: string | URL | Request,
  { onProgress, ...options }: Options = {},
): Promise<Response> {
  const response = await fetch(url, options);

  const reader = response.body?.getReader();
  if (!reader) {
    throw new Error("Response body is not readable");
  }

  return new Promise<Response>((resolve, reject) => {
    const chunks: Uint8Array[] = [];
    let total = 0;

    function pump(): void {
      reader?.read().then(({ done, value }) => {
        if (done) {
          resolve(
            new Response(
              new Blob(chunks, { type: response.headers.get("Content-Type") ?? "" }),
              response,
            ),
          );
          return;
        }

        if (value) {
          chunks.push(value);
          const received = value.length;
          total += received;
          if (onProgress) {
            onProgress(received, total);
          }
        }

        pump();
      }, reject);
    }

    pump();
  });
}

export const fetchContentLength = async (
  url: string | URL | Request,
  options?: RequestInit,
): Promise<number | null> => {
  const response = await fetch(url, options);

  const contentLength = response.headers.get("Content-Length");
  return contentLength ? parseInt(contentLength, 10) : null;
};
