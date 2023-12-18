use candle_transformers::models::marian::Config;

pub trait OpusMT {
    fn opus_mt_ja_en() -> Self;
}

impl OpusMT for Config {
    fn opus_mt_ja_en() -> Self {
        Self {
            activation_function: candle_nn::Activation::Swish,
            d_model: 512,
            decoder_attention_heads: 8,
            decoder_ffn_dim: 2048,
            decoder_layers: 6,
            decoder_start_token_id: 60715,
            decoder_vocab_size: Some(60716),
            encoder_attention_heads: 8,
            encoder_ffn_dim: 2048,
            encoder_layers: 6,
            eos_token_id: 0,
            forced_eos_token_id: 0,
            is_encoder_decoder: true,
            max_position_embeddings: 512,
            pad_token_id: 60715,
            scale_embedding: true,
            share_encoder_decoder_embeddings: true,
            use_cache: true,
            vocab_size: 60716,
        }
    }
}
