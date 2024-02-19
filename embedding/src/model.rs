use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType,
};

use crate::utils;

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
    pub fn encode(&self, inputs: &Vec<&str>, batch_size: i64) -> Vec<Vec<f32>> {
        
        let batches = utils::batch(inputs, batch_size);
        let mut final_results: Vec<Vec<f32>> = Vec::new();

        println!("inputs: {:?}", inputs);
        println!("batches: {:?}", batches);

        for batch in batches {
            let results = self.model.encode(batch.as_slice());
            match results {
                Ok(v) => {
                    for result in v {
                        final_results.push(result);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        final_results
    }
}
