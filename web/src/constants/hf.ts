export const REPOSITORY = "yu7400ki/candle-opusmt";
export const FILES = [
  "opus-mt-ja-en-model.safetensors",
  "opus-mt-ja-en-tokenizer-ja.json",
  "opus-mt-ja-en-tokenizer-en.json",
];
export const URLS = FILES.map(
  (file) => `https://huggingface.co/${REPOSITORY}/resolve/main/${file}`,
);
