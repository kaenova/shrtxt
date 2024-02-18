use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType,
};

pub struct EmbeddingModel {
    model: SentenceEmbeddingsModel,
}

impl EmbeddingModel {
    // A constructor to create a new Book
    pub fn new(model_type: SentenceEmbeddingsModelType) -> Self {
        let downloaded_model = SentenceEmbeddingsBuilder::remote(model_type).create_model();
        let model: SentenceEmbeddingsModel;
        match downloaded_model {
            Err(e) => panic!("Cannot download model {}", e),
            Ok(v) => model = v,
        }

        EmbeddingModel { model: model }
    }

    // An instance method
    pub fn encode(&self, input: &str) -> Option<Vec<f32>> {
        let model_input: [&str; 1] = [input];
        let embeddings = self.model.encode(&model_input);
        match embeddings {
            Err(e) => {
                panic!("Cannot crate embeddings: {}", e)
            }
            Ok(v) => v.last().cloned(),
        }
    }
}
