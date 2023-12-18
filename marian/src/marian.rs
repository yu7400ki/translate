use crate::opus_mt::OpusMT;
use crate::utils::JSErr;
use anyhow::Error as E;
use candle::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::marian::{Config, MTModel};
use tokenizers::Tokenizer;
use wasm_bindgen::prelude::*;

type Result<T> = std::result::Result<T, JsValue>;
const DEV: Device = Device::Cpu;

#[wasm_bindgen]
pub struct Marian {
    config: Config,
    model: MTModel,
    tokenizer: Tokenizer,
    tokenizer_dec: Tokenizer,
}

#[wasm_bindgen]
impl Marian {
    #[wasm_bindgen(constructor)]
    pub fn new(model: &[u8], tokenizer: &[u8], tokenizer_dec: &[u8]) -> Result<Marian> {
        let config = Config::opus_mt_ja_en();
        let vb = {
            let model = model.to_vec();
            VarBuilder::from_buffered_safetensors(model, DType::F32, &DEV).map_err(JSErr::msg)?
        };
        let model = MTModel::new(&config, vb).map_err(JSErr::msg)?;
        let tokenizer = Tokenizer::from_bytes(tokenizer).map_err(JSErr::msg)?;
        let tokenizer_dec = Tokenizer::from_bytes(tokenizer_dec).map_err(JSErr::msg)?;
        Ok(Marian {
            config,
            model,
            tokenizer,
            tokenizer_dec,
        })
    }

    pub fn translate(&mut self, src: &str) -> Result<String> {
        let mut tokens = self.tokenize(src).map_err(JSErr::msg)?;
        let token_ids = self.generate(&mut tokens).map_err(JSErr::msg)?;
        let decoded = self.decode(&token_ids).map_err(JSErr::msg)?;
        Ok(decoded)
    }

    fn tokenize(&self, src: &str) -> anyhow::Result<Tensor> {
        let mut tokens = self
            .tokenizer
            .encode(src, true)
            .map_err(E::msg)?
            .get_ids()
            .to_vec();
        tokens.push(self.config.eos_token_id);
        Ok(Tensor::new(tokens.as_slice(), &DEV)?.unsqueeze(0)?)
    }

    fn generate(&mut self, tokens: &mut Tensor) -> anyhow::Result<Vec<u32>> {
        let config = &self.config;
        let model = &mut self.model;
        let mut logits_processor = LogitsProcessor::new(0, None, None);
        let encoder_xs = model.encoder().forward(tokens, 0)?;
        let mut token_ids = vec![config.decoder_start_token_id];
        for index in 0..512 {
            let context_size = if index >= 1 { 1 } else { token_ids.len() };
            let start_pos = token_ids.len().saturating_sub(context_size);
            let input_ids = Tensor::new(&token_ids[start_pos..], &DEV)?.unsqueeze(0)?;
            let logits = model.decode(&input_ids, &encoder_xs, start_pos)?;
            let logits = logits.squeeze(0)?;
            let logits = logits.get(logits.dim(0)? - 1)?;
            let token = logits_processor.sample(&logits)?;
            if token == config.eos_token_id || token == config.forced_eos_token_id {
                break;
            }
            token_ids.push(token);
        }
        model.reset_kv_cache();
        Ok(token_ids)
    }

    fn decode(&self, token_ids: &Vec<u32>) -> anyhow::Result<String> {
        let decoded = self.tokenizer_dec.decode(&token_ids[1..], true);
        Ok(decoded.map_err(E::msg)?)
    }
}
