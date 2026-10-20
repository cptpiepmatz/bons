import { pipeline } from "@huggingface/transformers";
import { categories } from "../generated/categories.ts";

function createClassifier() {
  return pipeline(
    "zero-shot-classification",
    "onnx-community/multilingual-MiniLMv2-L6-mnli-xnli-ONNX",
    { dtype: "q8" },
  );
}

let classifier: Awaited<ReturnType<typeof createClassifier>> | undefined;

void createClassifier()
  .then((model) => classifier = model)
  .catch((error) => console.warn("Optional classifier unavailable", error));

// Does not wait for initialization; inference is asynchronous when ready.
export function suggestCategory(text: string) {
  if (!classifier) return undefined;

  return classifier(text, Object.values(categories).map((labels) => labels.de));
}
