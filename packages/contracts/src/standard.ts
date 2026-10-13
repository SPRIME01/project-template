import type { StandardSchemaV1 } from "@standard-schema/spec";

// Demonstrates library-side schema interoperability without coupling the
// consuming code to Valibot's parser API.
export async function validateStandard<T extends StandardSchemaV1>(
  schema: T,
  input: unknown,
): Promise<StandardSchemaV1.InferOutput<T>> {
  const result = await schema["~standard"].validate(input);
  if (result.issues) {
    throw new Error(result.issues.map((issue) => issue.message).join("; "));
  }
  return result.value as StandardSchemaV1.InferOutput<T>;
}
